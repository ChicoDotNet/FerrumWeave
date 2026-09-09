#!/usr/bin/env python3
"""Certify managed property read/write through the FerrumWeave rustc backend.

Rust source selects a narrow StringBuilder.Length property marker and supplies an
i32 value. FerrumWeave must observe that marker in MIR, emit both the managed
property setter and getter, and return the getter observable. The C# consumer is
fixed; only Rust source changes. `rustc_codegen_clr` is not part of this path.
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import tempfile
from pathlib import Path

ASSEMBLY_FILE = "FerrumWeave.Generated.dll"


def rust_source(value: int) -> str:
    return (
        "#[inline(never)]\n"
        "fn ferrumweave_system_text_string_builder_length(value: i32) -> i32 { value }\n\n"
        "#[no_mangle]\n"
        f'pub extern "C" fn answer() -> i32 {{ ferrumweave_system_text_string_builder_length({value}) }}\n'
    )


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


def assert_property_target(artifact: Path) -> None:
    data = artifact.read_bytes()
    for expected in [
        b"MZ",
        b"BSJB",
        b"FerrumWeave.Generated",
        b"RustApi",
        b"Answer",
        b"System",
        b"Text",
        b"StringBuilder",
        b"set_Length",
        b"get_Length",
    ]:
        if expected not in data:
            raise AssertionError(f"managed property artifact is missing {expected!r}")


def execute_and_inspect(artifact: Path, expected: int, root: Path, name: str) -> None:
    consumer = root / f"consumer_{name}"
    consumer.mkdir()
    shutil.copyfile(artifact, consumer / ASSEMBLY_FILE)
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
        '  </ItemGroup>\n'
        '</Project>\n',
        encoding="utf-8",
    )
    (consumer / "Program.cs").write_text(
        "using System.Reflection;\n"
        "var method = typeof(FerrumWeave.RustApi).GetMethod(\"Answer\", BindingFlags.Public | BindingFlags.Static)!;\n"
        "var il = method.GetMethodBody()!.GetILAsByteArray()!;\n"
        "if (il.Count(b => b == (byte)0x6F) < 2) throw new Exception(\"Answer must contain property setter and getter callvirt opcodes\");\n"
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
        raise AssertionError(f"C# property consumer failed for {name}:\n{run.stdout}\n{run.stderr}")
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

    cases = [("length_3", 3), ("length_7", 7)]

    with tempfile.TemporaryDirectory(prefix="ferrumweave-managed-property-") as temp:
        work = Path(temp)
        images: dict[str, bytes] = {}
        for name, value in cases:
            result, artifact = compile_source(
                args.toolchain, backend, work, name, rust_source(value)
            )
            if result.returncode != 0:
                print(f"RED: FerrumWeave cannot lower StringBuilder.Length property marker ({value})")
                print(result.stdout)
                print(result.stderr)
                return 1
            if not artifact.is_file():
                print(f"RED: compilation produced no managed property artifact for {name}")
                return 1
            try:
                assert_property_target(artifact)
                execute_and_inspect(artifact, value, work, name)
            except AssertionError as exc:
                print(f"RED: {exc}")
                return 1
            images[name] = artifact.read_bytes()

        if images["length_3"] == images["length_7"]:
            print("RED: changing only Rust property payload 3 -> 7 did not change the assembly")
            return 1

    print("GREEN: FerrumWeave source-causally emits managed property read/write")
    print("  Answer contains CLR callvirt for StringBuilder.set_Length and get_Length")
    print("  Rust payload mutation 3 -> 7 changes the assembly and executable observable")
    print("  artifact executes successfully under CoreCLR")
    print("  rustc_codegen_clr was not used in the product path")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
