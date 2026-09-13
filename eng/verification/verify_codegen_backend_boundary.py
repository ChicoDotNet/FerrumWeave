#!/usr/bin/env python3
"""Certify the rustc -> FerrumWeave codegen boundary and first MIR lowering slice.

Default mode is a boundary sensor: a successful managed artifact is GREEN, while a
FerrumWeave-owned MIR diagnostic proves rustc reached the backend but the product
slice is still RED.

`--require-artifact` is the product gate. It requires source mutation causality,
managed execution through C#, and a negative Rust type-check test.
"""

from __future__ import annotations

import argparse
import hashlib
import shutil
import subprocess
import tempfile
from pathlib import Path

LOWERING_MARKER = "FERRUMWEAVE_MIR_LOWERING_FAILED"
ASSEMBLY_FILE = "FerrumWeave.Generated.dll"


def rust_source(value: int) -> str:
    return '#[no_mangle]\n' f'pub extern "C" fn answer() -> i32 {{ {value} }}\n'


def invalid_rust_source() -> str:
    return (
        '#[no_mangle]\n'
        'pub extern "C" fn answer() -> i32 { "not-an-i32" }\n'
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


def assert_managed_shape(path: Path) -> None:
    data = path.read_bytes()
    if not data.startswith(b"MZ"):
        raise AssertionError(f"{path} is not a PE image")
    if b"BSJB" not in data:
        raise AssertionError(f"{path} does not contain a CLR metadata root")
    for expected in [b"FerrumWeave.Generated", b"FerrumWeave", b"RustApi", b"Answer"]:
        if expected not in data:
            raise AssertionError(f"{path} is missing managed metadata name {expected!r}")


def execute_from_csharp(artifact: Path, expected: int, root: Path) -> None:
    consumer = root / f"consumer_{expected}"
    consumer.mkdir()
    referenced = consumer / ASSEMBLY_FILE
    shutil.copyfile(artifact, referenced)

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
            f"C# consumer failed for expected value {expected}:\n{run.stdout}\n{run.stderr}"
        )
    observed = run.stdout.strip().splitlines()[-1] if run.stdout.strip() else ""
    if observed != str(expected):
        raise AssertionError(
            f"source-causality failure: expected {expected}, managed consumer observed {observed!r}"
        )


def product_gate(toolchain: str, backend: Path, work: Path) -> int:
    artifacts: dict[int, Path] = {}
    for value in (137, 211):
        result, artifact = compile_source(
            toolchain,
            backend,
            work,
            f"answer_{value}",
            rust_source(value),
        )
        if result.returncode != 0:
            print(f"RED: FerrumWeave failed to compile Rust source returning {value}")
            print(result.stdout)
            print(result.stderr)
            return 1
        if not artifact.is_file():
            print(f"RED: rustc exited successfully but no managed artifact exists for {value}")
            return 1
        try:
            assert_managed_shape(artifact)
            execute_from_csharp(artifact, value, work)
        except AssertionError as exc:
            print(f"RED: {exc}")
            return 1
        artifacts[value] = artifact

    first_hash = hashlib.sha256(artifacts[137].read_bytes()).hexdigest()
    second_hash = hashlib.sha256(artifacts[211].read_bytes()).hexdigest()
    if first_hash == second_hash:
        print("RED: mutating only Rust source 137 -> 211 did not change the managed artifact")
        return 1

    invalid_result, invalid_artifact = compile_source(
        toolchain,
        backend,
        work,
        "invalid_answer",
        invalid_rust_source(),
    )
    invalid_output = f"{invalid_result.stdout}\n{invalid_result.stderr}"
    if invalid_result.returncode == 0:
        print("RED: invalid Rust unexpectedly compiled through FerrumWeave")
        return 1
    if invalid_artifact.exists():
        print("RED: invalid Rust left behind a managed artifact")
        return 1
    if LOWERING_MARKER in invalid_output:
        print("RED: invalid Rust reached FerrumWeave lowering instead of failing in rustc semantics")
        return 1

    print("GREEN: FerrumWeave owns the first source-causal MIR -> managed artifact slice")
    print("  Rust mutation: 137 -> 211 changed both assembly and managed observable")
    print("  Consumer: C# observed 137 and 211 through FerrumWeave.RustApi.Answer()")
    print("  Negative: Rust type error failed before a managed artifact was produced")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--toolchain", required=True)
    parser.add_argument("--backend", type=Path, required=True)
    parser.add_argument("--require-artifact", action="store_true")
    args = parser.parse_args()

    backend = args.backend.resolve()
    if not backend.is_file():
        print(f"ERROR: FerrumWeave backend was not built: {backend}")
        return 2

    with tempfile.TemporaryDirectory(prefix="ferrumweave-codegen-boundary-") as temp:
        work = Path(temp)
        if args.require_artifact:
            return product_gate(args.toolchain, backend, work)

        result, artifact = compile_source(
            args.toolchain,
            backend,
            work,
            "boundary_answer",
            rust_source(137),
        )
        combined = f"{result.stdout}\n{result.stderr}"
        if result.returncode == 0 and artifact.is_file():
            print("GREEN: rustc loaded FerrumWeave and FerrumWeave produced a managed artifact")
            return 0
        if LOWERING_MARKER in combined:
            print("GREEN boundary / RED product: rustc reached FerrumWeave MIR lowering")
            print(combined)
            return 0

        print("ERROR: compilation failed before a recognizable FerrumWeave codegen boundary")
        print(combined)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
