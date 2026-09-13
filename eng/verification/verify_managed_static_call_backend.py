#!/usr/bin/env python3
"""Certify source-causal managed static calls through the FerrumWeave backend.

This verifier does not use rustc_codegen_clr. Rust source selects a managed
System.Math method through a narrow marker function. FerrumWeave must observe
that call in MIR, emit the corresponding CLR MemberRef/call, and produce an
artifact whose behavior changes when only the Rust source changes.
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import tempfile
from pathlib import Path

ASSEMBLY_FILE = "FerrumWeave.Generated.dll"


def rust_source(marker: str, argument: int) -> str:
    return (
        "#[inline(never)]\n"
        "fn ferrumweave_system_math_abs(value: i32) -> i32 { value }\n\n"
        "#[inline(never)]\n"
        "fn ferrumweave_system_math_sign(value: i32) -> i32 { value }\n\n"
        "#[no_mangle]\n"
        f'pub extern "C" fn answer() -> i32 {{ {marker}({argument}) }}\n'
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


def assert_managed_target(artifact: Path, method: str) -> None:
    data = artifact.read_bytes()
    for expected in [
        b"MZ",
        b"BSJB",
        b"FerrumWeave.Generated",
        b"FerrumWeave",
        b"RustApi",
        b"Answer",
        b"System",
        b"Math",
        method.encode("ascii"),
    ]:
        if expected not in data:
            raise AssertionError(
                f"managed artifact for {method} is missing {expected!r}"
            )


def execute_from_csharp(artifact: Path, expected: int, root: Path, name: str) -> None:
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
        raise AssertionError(
            f"C# consumer failed for {name}:\n{run.stdout}\n{run.stderr}"
        )
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

    # Positive constants keep this slice focused on MIR Call lowering rather than
    # also requiring UnaryOp lowering. Abs still distinguishes argument mutation,
    # while switching only the Rust-selected marker to Sign changes the managed
    # method and observable from 137 to 1.
    cases = [
        ("abs_137", "ferrumweave_system_math_abs", 137, "Abs", 137),
        ("abs_211", "ferrumweave_system_math_abs", 211, "Abs", 211),
        ("sign_137", "ferrumweave_system_math_sign", 137, "Sign", 1),
    ]

    with tempfile.TemporaryDirectory(prefix="ferrumweave-managed-static-") as temp:
        work = Path(temp)
        images: dict[str, bytes] = {}

        for name, marker, argument, managed_method, expected in cases:
            result, artifact = compile_source(
                args.toolchain,
                backend,
                work,
                name,
                rust_source(marker, argument),
            )
            if result.returncode != 0:
                print(
                    f"RED: FerrumWeave cannot lower Rust MIR call {marker}({argument})"
                )
                print(result.stdout)
                print(result.stderr)
                return 1
            if not artifact.is_file():
                print(f"RED: compilation produced no managed artifact for {name}")
                return 1

            try:
                assert_managed_target(artifact, managed_method)
                execute_from_csharp(artifact, expected, work, name)
            except AssertionError as exc:
                print(f"RED: {exc}")
                return 1

            images[name] = artifact.read_bytes()

        if images["abs_137"] == images["abs_211"]:
            print("RED: changing only the Rust call argument 137 -> 211 did not change the assembly")
            return 1
        if images["abs_137"] == images["sign_137"]:
            print("RED: changing only the Rust-selected method Abs -> Sign did not change the assembly")
            return 1

    print("GREEN: FerrumWeave source-causally lowers a managed static System.* call")
    print("  MIR-selected method: Math.Abs -> Math.Sign changes MemberRef and observable 137 -> 1")
    print("  MIR-selected argument: 137 -> 211 changes managed observable 137 -> 211")
    print("  rustc_codegen_clr was not used in the product path")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
