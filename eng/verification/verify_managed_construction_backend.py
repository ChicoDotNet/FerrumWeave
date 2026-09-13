#!/usr/bin/env python3
"""Certify managed object construction through the FerrumWeave rustc backend.

This is intentionally a construction-only contract. Rust source selects a
narrow managed-constructor marker and supplies an i32 payload that remains the
public return value. FerrumWeave must observe the marker call in MIR and emit a
real CLR `newobj` for the selected constructor. The C# consumer stays fixed;
only Rust source changes. `rustc_codegen_clr` is not part of this path.
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import tempfile
from pathlib import Path

ASSEMBLY_FILE = "FerrumWeave.Generated.dll"


def rust_source(marker: str, value: int) -> str:
    return (
        "#[inline(never)]\n"
        "fn ferrumweave_system_object_new(value: i32) -> i32 { value }\n\n"
        "#[inline(never)]\n"
        "fn ferrumweave_system_text_string_builder_new(value: i32) -> i32 { value }\n\n"
        "#[no_mangle]\n"
        f'pub extern "C" fn answer() -> i32 {{ {marker}({value}) }}\n'
    )


def compile_source(
    toolchain: str,
    backend: Path,
    work: Path,
    name: str,
    source_text: str,
) -> tuple[subprocess.CompletedProcess[str], Path]:
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


def assert_constructor_target(artifact: Path, required_strings: list[bytes]) -> None:
    data = artifact.read_bytes()
    for expected in [b"MZ", b"BSJB", b"FerrumWeave.Generated", b"RustApi", b"Answer", *required_strings]:
        if expected not in data:
            raise AssertionError(f"managed construction artifact is missing {expected!r}")


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
        "if (!il.Contains((byte)0x73)) throw new Exception(\"Answer contains no managed newobj opcode\");\n"
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
        raise AssertionError(f"C# construction consumer failed for {name}:\n{run.stdout}\n{run.stderr}")
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

    cases = [
        ("object_137", "ferrumweave_system_object_new", 137, [b"System", b"Object", b".ctor"]),
        ("object_211", "ferrumweave_system_object_new", 211, [b"System", b"Object", b".ctor"]),
        (
            "string_builder_137",
            "ferrumweave_system_text_string_builder_new",
            137,
            [b"System", b"Text", b"StringBuilder", b".ctor"],
        ),
    ]

    with tempfile.TemporaryDirectory(prefix="ferrumweave-managed-construction-") as temp:
        work = Path(temp)
        images: dict[str, bytes] = {}
        for name, marker, value, required_strings in cases:
            result, artifact = compile_source(
                args.toolchain,
                backend,
                work,
                name,
                rust_source(marker, value),
            )
            if result.returncode != 0:
                print(f"RED: FerrumWeave cannot lower managed construction marker {marker}({value})")
                print(result.stdout)
                print(result.stderr)
                return 1
            if not artifact.is_file():
                print(f"RED: compilation produced no managed construction artifact for {name}")
                return 1
            try:
                assert_constructor_target(artifact, required_strings)
                execute_and_inspect(artifact, value, work, name)
            except AssertionError as exc:
                print(f"RED: {exc}")
                return 1
            images[name] = artifact.read_bytes()

        if images["object_137"] == images["object_211"]:
            print("RED: changing only Rust payload 137 -> 211 did not change the assembly")
            return 1
        if images["object_137"] == images["string_builder_137"]:
            print("RED: changing only Rust constructor Object -> StringBuilder did not change the assembly")
            return 1

    print("GREEN: FerrumWeave source-causally emits managed object construction")
    print("  Answer contains CLR newobj and executes successfully under CoreCLR")
    print("  Rust constructor marker mutation Object -> StringBuilder changes constructor metadata")
    print("  Rust payload mutation 137 -> 211 changes the executable observable")
    print("  rustc_codegen_clr was not used in the product path")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
