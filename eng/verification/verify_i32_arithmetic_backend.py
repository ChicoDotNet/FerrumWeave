#!/usr/bin/env python3
"""Falsify FerrumWeave i32 arithmetic lowering through rustc MIR.

The C# consumer arguments stay fixed. Rust-only mutations change the arithmetic
operator or the exported Rust symbol. The contract passes only when
FerrumWeave's own CodegenBackend lowers the MIR operation and preserves the
rustc-owned public export identity into managed metadata.
"""

from __future__ import annotations

import argparse
import hashlib
import shutil
import subprocess
import tempfile
from pathlib import Path

ASSEMBLY_FILE = "FerrumWeave.Generated.dll"


def rust_source(operator: str, export_symbol: str = "answer") -> str:
    if operator not in {"+", "-"}:
        raise ValueError(operator)
    return (
        '#[no_mangle]\n'
        f'pub extern "C" fn {export_symbol}(left: i32, right: i32) -> i32 {{\n'
        f'    let result = left {operator} right;\n'
        '    result\n'
        '}\n'
    )


def compile_source(
    toolchain: str,
    backend: Path,
    work: Path,
    label: str,
    operator: str,
    export_symbol: str = "answer",
) -> Path:
    source = work / f"arithmetic_{label}.rs"
    artifact = work / f"arithmetic_{label}.dll"
    source.write_text(rust_source(operator, export_symbol), encoding="utf-8")
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
            f"FerrumWeave could not lower Rust i32 arithmetic {label}:\n"
            f"{result.stdout}\n{result.stderr}"
        )
    if not artifact.is_file():
        raise AssertionError(f"no managed artifact was produced for Rust arithmetic {label}")
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
        f'System.Console.WriteLine(FerrumWeave.RustApi.{method_name}(137, 74));\n',
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
            f"C# consumer failed for Rust arithmetic {label}:\n{run.stdout}\n{run.stderr}"
        )
    observed = run.stdout.strip().splitlines()[-1] if run.stdout.strip() else ""
    if observed != str(expected):
        raise AssertionError(
            f"Rust arithmetic {label} expected managed observable {expected}, got {observed!r}"
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
        with tempfile.TemporaryDirectory(prefix="ferrumweave-i32-arithmetic-") as temp:
            work = Path(temp)
            add_artifact = compile_source(args.toolchain, backend, work, "add", "+")
            sub_artifact = compile_source(args.toolchain, backend, work, "sub", "-")
            renamed_artifact = compile_source(
                args.toolchain,
                backend,
                work,
                "renamed",
                "+",
                export_symbol="compute_result",
            )

            execute_from_csharp(add_artifact, 211, work, "add")
            execute_from_csharp(sub_artifact, 63, work, "sub")
            execute_from_csharp(
                renamed_artifact,
                211,
                work,
                "renamed",
                method_name="ComputeResult",
            )

            add_hash = hashlib.sha256(add_artifact.read_bytes()).hexdigest()
            sub_hash = hashlib.sha256(sub_artifact.read_bytes()).hexdigest()
            renamed_hash = hashlib.sha256(renamed_artifact.read_bytes()).hexdigest()
            if add_hash == sub_hash:
                raise AssertionError(
                    "changing only Rust operator + -> - did not change the managed artifact"
                )
            if add_hash == renamed_hash:
                raise AssertionError(
                    "changing only Rust export answer -> compute_result did not change managed metadata"
                )
    except AssertionError as exc:
        print(f"RED: {exc}")
        return 1

    print("GREEN: FerrumWeave source-causally lowers i32 arithmetic")
    print("  Fixed C# arguments: (137, 74)")
    print("  Rust-only arithmetic mutation: left + right -> left - right")
    print("  Managed observable: 211 -> 63")
    print("  Rust-only export mutation: answer -> compute_result")
    print("  Managed consumer mutation: Answer(...) -> ComputeResult(...)")
    print("  rustc_codegen_clr was not used in the product path")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
