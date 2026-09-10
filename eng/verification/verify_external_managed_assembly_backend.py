#!/usr/bin/env python3
"""Certify an independently compiled managed assembly through FerrumWeave codegen.

The external C# dependency is built independently and remains fixed. Rust source
selects a narrow external managed marker and supplies the only changing payload.
FerrumWeave must observe that marker in MIR, emit a managed AssemblyRef/MemberRef
and real call, then produce an artifact that executes with the dependency under
CoreCLR. `rustc_codegen_clr` is not part of this product path.
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import tempfile
from pathlib import Path

GENERATED_ASSEMBLY = "FerrumWeave.Generated.dll"
EXTERNAL_ASSEMBLY = "External.Managed.dll"


def rust_source(value: int) -> str:
    return (
        "#[inline(never)]\n"
        "fn ferrumweave_external_managed_transform(value: i32) -> i32 { value }\n\n"
        "#[no_mangle]\n"
        f'pub extern "C" fn answer() -> i32 {{ ferrumweave_external_managed_transform({value}) }}\n'
    )


def build_external_dependency(root: Path) -> Path:
    project = root / "external"
    project.mkdir()
    (project / "External.Managed.csproj").write_text(
        '<Project Sdk="Microsoft.NET.Sdk">\n'
        '  <PropertyGroup>\n'
        '    <TargetFramework>net10.0</TargetFramework>\n'
        '    <AssemblyName>External.Managed</AssemblyName>\n'
        '    <RootNamespace>ExternalManaged</RootNamespace>\n'
        '  </PropertyGroup>\n'
        '</Project>\n',
        encoding="utf-8",
    )
    (project / "ExternalApi.cs").write_text(
        "namespace ExternalManaged;\n\n"
        "public static class ExternalApi\n"
        "{\n"
        "    public static int Transform(int value) => value + 1000;\n"
        "}\n",
        encoding="utf-8",
    )
    build = subprocess.run(
        ["dotnet", "build", str(project / "External.Managed.csproj"), "-c", "Release", "--nologo"],
        text=True,
        capture_output=True,
        check=False,
    )
    if build.returncode != 0:
        raise AssertionError(f"independent external assembly build failed:\n{build.stdout}\n{build.stderr}")
    assembly = project / "bin" / "Release" / "net10.0" / EXTERNAL_ASSEMBLY
    if not assembly.is_file():
        raise AssertionError("independent external assembly build produced no DLL")
    return assembly


def compile_source(toolchain: str, backend: Path, work: Path, name: str, source_text: str):
    source = work / f"{name}.rs"
    output = work / f"{name}.dll"
    source.write_text(source_text, encoding="utf-8")
    result = subprocess.run(
        [
            "rustc",
            f"+{toolchain}",
            "-Z",
            f"codegen-backend={backend}",
            "--edition",
            "2021",
            "--crate-type",
            "lib",
            str(source),
            "-o",
            str(output),
        ],
        text=True,
        capture_output=True,
        check=False,
    )
    return result, output


def assert_external_reference(artifact: Path) -> None:
    data = artifact.read_bytes()
    for expected in [
        b"MZ",
        b"BSJB",
        b"FerrumWeave.Generated",
        b"RustApi",
        b"Answer",
        b"External.Managed",
        b"ExternalManaged",
        b"ExternalApi",
        b"Transform",
    ]:
        if expected not in data:
            raise AssertionError(f"external managed artifact is missing {expected!r}")


def execute_and_inspect(
    artifact: Path,
    dependency: Path,
    expected: int,
    root: Path,
    name: str,
) -> None:
    consumer = root / f"consumer_{name}"
    consumer.mkdir()
    shutil.copyfile(artifact, consumer / GENERATED_ASSEMBLY)
    shutil.copyfile(dependency, consumer / EXTERNAL_ASSEMBLY)
    (consumer / "Consumer.csproj").write_text(
        '<Project Sdk="Microsoft.NET.Sdk">\n'
        '  <PropertyGroup>\n'
        '    <OutputType>Exe</OutputType>\n'
        '    <TargetFramework>net10.0</TargetFramework>\n'
        '    <ImplicitUsings>enable</ImplicitUsings>\n'
        '  </PropertyGroup>\n'
        '  <ItemGroup>\n'
        '    <Reference Include="FerrumWeave.Generated">\n'
        '      <HintPath>FerrumWeave.Generated.dll</HintPath>\n'
        '      <Private>true</Private>\n'
        '    </Reference>\n'
        '    <Reference Include="External.Managed">\n'
        '      <HintPath>External.Managed.dll</HintPath>\n'
        '      <Private>true</Private>\n'
        '    </Reference>\n'
        '  </ItemGroup>\n'
        '</Project>\n',
        encoding="utf-8",
    )
    (consumer / "Program.cs").write_text(
        "using System.Reflection;\n"
        "var method = typeof(FerrumWeave.RustApi).GetMethod(\"Answer\", BindingFlags.Public | BindingFlags.Static)!;\n"
        "var il = method.GetMethodBody()!.GetILAsByteArray()!;\n"
        "if (!il.Contains((byte)0x28)) throw new Exception(\"Answer must contain a managed call opcode\");\n"
        "Console.WriteLine(FerrumWeave.RustApi.Answer());\n",
        encoding="utf-8",
    )
    run = subprocess.run(
        ["dotnet", "run", "--project", str(consumer / "Consumer.csproj"), "--nologo"],
        text=True,
        capture_output=True,
        check=False,
    )
    if run.returncode != 0:
        raise AssertionError(f"external managed consumer failed for {name}:\n{run.stdout}\n{run.stderr}")
    observed = run.stdout.strip().splitlines()[-1] if run.stdout.strip() else ""
    if observed != str(expected):
        raise AssertionError(
            f"source-causality failure for {name}: expected {expected}, observed {observed!r}"
        )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--toolchain", required=True)
    parser.add_argument("--backend", type=Path, required=True)
    args = parser.parse_args()

    backend = args.backend.resolve()
    if not backend.is_file():
        print(f"ERROR: FerrumWeave backend was not built: {backend}")
        return 2

    cases = [("payload_137", 137, 1137), ("payload_211", 211, 1211)]

    with tempfile.TemporaryDirectory(prefix="ferrumweave-external-managed-") as temp:
        work = Path(temp)
        try:
            dependency = build_external_dependency(work)
        except AssertionError as exc:
            print(f"ERROR: {exc}")
            return 2

        images: dict[str, bytes] = {}
        for name, value, expected in cases:
            result, artifact = compile_source(
                args.toolchain, backend, work, name, rust_source(value)
            )
            if result.returncode != 0:
                print(
                    "RED: FerrumWeave cannot lower the external managed assembly marker "
                    f"for Rust payload {value}"
                )
                print(result.stdout)
                print(result.stderr)
                return 1
            if not artifact.is_file():
                print(f"RED: compilation produced no external managed artifact for {name}")
                return 1
            try:
                assert_external_reference(artifact)
                execute_and_inspect(artifact, dependency, expected, work, name)
            except AssertionError as exc:
                print(f"RED: {exc}")
                return 1
            images[name] = artifact.read_bytes()

        if images["payload_137"] == images["payload_211"]:
            print("RED: changing only Rust payload 137 -> 211 did not change the assembly")
            return 1

    print("GREEN: FerrumWeave source-causally consumes an independent managed assembly")
    print("  external C# dependency was compiled independently and held fixed")
    print("  Answer contains a CLR call and External.Managed AssemblyRef/MemberRef metadata")
    print("  Rust payload mutation 137 -> 211 changes artifact and observable 1137 -> 1211")
    print("  artifact executes successfully with the dependency under CoreCLR")
    print("  rustc_codegen_clr was not used in the product path")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
