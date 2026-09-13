#!/usr/bin/env python3
"""Falsify FerrumWeave i32 argument/local flow through rustc MIR.

The C# consumer and its arguments stay fixed. Only Rust source changes which
argument is copied through a local into the return place. The contract passes
only when FerrumWeave's own CodegenBackend lowers that MIR data flow into the
managed method signature/body.
"""

from __future__ import annotations

import argparse
import hashlib
import shutil
import subprocess
import tempfile
from pathlib import Path

ASSEMBLY_FILE = "FerrumWeave.Generated.dll"


def rust_source(selected: str) -> str:
    if selected not in {"left", "right"}:
        raise ValueError(selected)
    return (
        '#[no_mangle]\n'
        'pub extern "C" fn answer(left: i32, right: i32) -> i32 {\n'
        f'    let selected = {selected};\n'
        '    selected\n'
        '}\n'
    )


def compile_source(toolchain: str, backend: Path, work: Path, selected: str) -> Path:
    source = work / f"select_{selected}.rs"
    artifact = work / f"select_{selected}.dll"
    source.write_text(rust_source(selected), encoding="utf-8")
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
            str(artifact),
        ],
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        raise AssertionError(
            f"FerrumWeave could not lower Rust local flow selecting {selected}:\n"
            f"{result.stdout}\n{result.stderr}"
        )
    if not artifact.is_file():
        raise AssertionError(f"no managed artifact was produced for Rust selection {selected}")
    return artifact


def execute_from_csharp(artifact: Path, expected: int, root: Path, label: str) -> None:
    consumer = root / f"consumer_{label}"
    consumer.mkdir()
    shutil.copyfile(artifact, consumer / ASSEMBLY_FILE)
    (consumer / "Consumer.csproj").write_text(
        '<Project Sdk="Microsoft.NET.Sdk">\n'
        '  <PropertyGroup>\n'
        '    <OutputType>Exe</OutputType>\n'
        '    <TargetFramework>net10.0</TargetFramework>\n'
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
        'System.Console.WriteLine(FerrumWeave.RustApi.Answer(137, 211));\n',
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
            f"C# consumer failed for Rust selection {label}:\n{run.stdout}\n{run.stderr}"
        )
    observed = run.stdout.strip().splitlines()[-1] if run.stdout.strip() else ""
    if observed != str(expected):
        raise AssertionError(
            f"Rust selected {label}, expected managed observable {expected}, got {observed!r}"
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

    try:
        with tempfile.TemporaryDirectory(prefix="ferrumweave-i32-local-flow-") as temp:
            work = Path(temp)
            left_artifact = compile_source(args.toolchain, backend, work, "left")
            right_artifact = compile_source(args.toolchain, backend, work, "right")

            execute_from_csharp(left_artifact, 137, work, "left")
            execute_from_csharp(right_artifact, 211, work, "right")

            left_hash = hashlib.sha256(left_artifact.read_bytes()).hexdigest()
            right_hash = hashlib.sha256(right_artifact.read_bytes()).hexdigest()
            if left_hash == right_hash:
                raise AssertionError(
                    "changing only Rust local selection left -> right did not change the managed artifact"
                )
    except AssertionError as exc:
        print(f"RED: {exc}")
        return 1

    print("GREEN: FerrumWeave source-causally lowers i32 arguments/local flow")
    print("  Fixed C# call: Answer(137, 211)")
    print("  Rust-only mutation: selected = left -> selected = right")
    print("  Managed observable: 137 -> 211")
    print("  rustc_codegen_clr was not used in the product path")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
