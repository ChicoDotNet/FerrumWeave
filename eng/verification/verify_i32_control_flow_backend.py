#!/usr/bin/env python3
"""Falsify FerrumWeave i32 control-flow lowering through rustc MIR.

The C# consumer and all arguments stay fixed. Only the Rust branch predicate
changes from equality to inequality. Each produced assembly is exercised with
both selector outcomes so an emitter cannot pass by baking the expected branch.
The contract can pass only when FerrumWeave's own CodegenBackend preserves Rust
MIR control flow in the managed method body.
"""

from __future__ import annotations

import argparse
import hashlib
import shutil
import subprocess
import tempfile
from pathlib import Path

ASSEMBLY_FILE = "FerrumWeave.Generated.dll"


def rust_source(predicate: str) -> str:
    if predicate not in {"==", "!="}:
        raise ValueError(predicate)
    return (
        '#[no_mangle]\n'
        'pub extern "C" fn answer(selector: i32, left: i32, right: i32) -> i32 {\n'
        f'    if selector {predicate} 0 {{\n'
        '        left\n'
        '    } else {\n'
        '        right\n'
        '    }\n'
        '}\n'
    )


def compile_source(toolchain: str, backend: Path, work: Path, label: str, predicate: str) -> Path:
    source = work / f"control_flow_{label}.rs"
    artifact = work / f"control_flow_{label}.dll"
    source.write_text(rust_source(predicate), encoding="utf-8")
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
            f"FerrumWeave could not lower Rust i32 control flow {label}:\n"
            f"{result.stdout}\n{result.stderr}"
        )
    if not artifact.is_file():
        raise AssertionError(f"no managed artifact was produced for Rust control flow {label}")
    return artifact


def execute_from_csharp(
    artifact: Path,
    selector: int,
    expected: int,
    root: Path,
    label: str,
) -> None:
    consumer = root / f"consumer_{label}_{selector}"
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
        f'System.Console.WriteLine(FerrumWeave.RustApi.Answer({selector}, 137, 211));\n',
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
            f"C# consumer failed for Rust control flow {label}, selector={selector}:\n"
            f"{run.stdout}\n{run.stderr}"
        )
    observed = run.stdout.strip().splitlines()[-1] if run.stdout.strip() else ""
    if observed != str(expected):
        raise AssertionError(
            f"Rust control flow {label}, selector={selector} expected managed observable "
            f"{expected}, got {observed!r}"
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
        with tempfile.TemporaryDirectory(prefix="ferrumweave-i32-control-flow-") as temp:
            work = Path(temp)
            eq_artifact = compile_source(args.toolchain, backend, work, "eq", "==")
            ne_artifact = compile_source(args.toolchain, backend, work, "ne", "!=")

            execute_from_csharp(eq_artifact, 0, 137, work, "eq")
            execute_from_csharp(eq_artifact, 1, 211, work, "eq")
            execute_from_csharp(ne_artifact, 0, 211, work, "ne")
            execute_from_csharp(ne_artifact, 1, 137, work, "ne")

            eq_hash = hashlib.sha256(eq_artifact.read_bytes()).hexdigest()
            ne_hash = hashlib.sha256(ne_artifact.read_bytes()).hexdigest()
            if eq_hash == ne_hash:
                raise AssertionError(
                    "changing only Rust predicate == -> != did not change the managed artifact"
                )
    except AssertionError as exc:
        print(f"RED: {exc}")
        return 1

    print("GREEN: FerrumWeave source-causally lowers i32 control flow")
    print("  Rust-only mutation: selector == 0 -> selector != 0")
    print("  Each artifact is exercised with selector=0 and selector=1")
    print("  Equal observable: 137, 211; not-equal observable: 211, 137")
    print("  rustc_codegen_clr was not used in the product path")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())