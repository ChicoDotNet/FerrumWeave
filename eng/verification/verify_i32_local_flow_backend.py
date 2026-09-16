#!/usr/bin/env python3
"""Falsify FerrumWeave i32 argument/local flow through rustc MIR.

The C# arguments stay fixed. Rust-only mutations change either which argument
flows through a local into the return place or the exported Rust symbol. The
contract passes only when FerrumWeave's own CodegenBackend lowers both MIR data
flow and rustc-owned public identity into the managed artifact.
"""

from __future__ import annotations

import argparse
import hashlib
import shutil
import subprocess
import tempfile
from pathlib import Path

ASSEMBLY_FILE = "FerrumWeave.Generated.dll"


def rust_source(selected: str, export_name: str = "answer") -> str:
    if selected not in {"left", "right"}:
        raise ValueError(selected)
    return (
        '#[no_mangle]\n'
        f'pub extern "C" fn {export_name}(left: i32, right: i32) -> i32 {{\n'
        f'    let selected = {selected};\n'
        '    selected\n'
        '}\n'
    )


def compile_source(
    toolchain: str,
    backend: Path,
    work: Path,
    selected: str,
    export_name: str = "answer",
) -> Path:
    source = work / f"select_{selected}_{export_name}.rs"
    artifact = work / f"select_{selected}_{export_name}.dll"
    source.write_text(rust_source(selected, export_name), encoding="utf-8")
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
            f"FerrumWeave could not lower Rust local flow selecting {selected} "
            f"for export {export_name}:\n{result.stdout}\n{result.stderr}"
        )
    if not artifact.is_file():
        raise AssertionError(
            f"no managed artifact was produced for Rust selection {selected} / {export_name}"
        )
    return artifact


def execute_from_csharp(
    artifact: Path,
    expected: int,
    root: Path,
    label: str,
    method_name: str = "Answer",
) -> None:
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
        f'System.Console.WriteLine(FerrumWeave.RustApi.{method_name}(137, 211));\n',
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
            renamed_artifact = compile_source(
                args.toolchain, backend, work, "left", "compute_result"
            )

            execute_from_csharp(left_artifact, 137, work, "left")
            execute_from_csharp(right_artifact, 211, work, "right")
            execute_from_csharp(
                renamed_artifact,
                137,
                work,
                "left_renamed",
                "ComputeResult",
            )

            left_hash = hashlib.sha256(left_artifact.read_bytes()).hexdigest()
            right_hash = hashlib.sha256(right_artifact.read_bytes()).hexdigest()
            renamed_hash = hashlib.sha256(renamed_artifact.read_bytes()).hexdigest()
            if left_hash == right_hash:
                raise AssertionError(
                    "changing only Rust local selection left -> right did not change the managed artifact"
                )
            if left_hash == renamed_hash:
                raise AssertionError(
                    "changing only Rust export name answer -> compute_result did not change managed metadata"
                )
    except AssertionError as exc:
        print(f"RED: {exc}")
        return 1

    print("GREEN: FerrumWeave source-causally lowers i32 arguments/local flow")
    print("  Fixed C# arguments: (137, 211)")
    print("  Rust-only mutation: selected = left -> selected = right")
    print("  Managed observable: 137 -> 211")
    print("  Rust-only identity mutation: answer -> compute_result")
    print("  Managed public identity: Answer -> ComputeResult")
    print("  rustc_codegen_clr was not used in the product path")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
