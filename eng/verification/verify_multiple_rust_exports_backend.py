#!/usr/bin/env python3
"""Falsify single-export substitution in the FerrumWeave rustc backend.

A real crate backend must preserve more than one externally visible Rust export
in the same managed artifact. The C# consumer stays fixed. Rust source defines
two independent exports with distinct constants; both CLR methods and both
observables must survive the same rustc -> FerrumWeave codegen_crate invocation.
`rustc_codegen_clr` is not part of this product-path contract.
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import tempfile
from pathlib import Path

ASSEMBLY_FILE = "FerrumWeave.Generated.dll"
SOURCE = '''#[no_mangle]
pub extern "C" fn alpha_value() -> i32 { 137 }

#[no_mangle]
pub extern "C" fn beta_value() -> i32 { 211 }
'''


def compile_crate(toolchain: str, backend: Path, work: Path) -> Path:
    source = work / "multiple_exports.rs"
    artifact = work / ASSEMBLY_FILE
    source.write_text(SOURCE, encoding="utf-8")
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
            "FerrumWeave could not compile a Rust crate containing two exports:\n"
            f"{result.stdout}\n{result.stderr}"
        )
    if not artifact.is_file():
        raise AssertionError("two-export Rust crate produced no managed artifact")
    return artifact


def run_consumer(artifact: Path, root: Path) -> tuple[int, int]:
    consumer = root / "consumer"
    consumer.mkdir()
    shutil.copyfile(artifact, consumer / ASSEMBLY_FILE)
    (consumer / "Consumer.csproj").write_text(
        '<Project Sdk="Microsoft.NET.Sdk">\n'
        '  <PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net10.0</TargetFramework></PropertyGroup>\n'
        '  <ItemGroup><Reference Include="FerrumWeave.Generated"><HintPath>FerrumWeave.Generated.dll</HintPath><Private>true</Private></Reference></ItemGroup>\n'
        '</Project>\n',
        encoding="utf-8",
    )
    (consumer / "Program.cs").write_text(
        'var type = typeof(FerrumWeave.RustApi);\n'
        'var alpha = type.GetMethod("AlphaValue");\n'
        'var beta = type.GetMethod("BetaValue");\n'
        'if (alpha is null || beta is null)\n'
        '    throw new Exception($"missing exports: AlphaValue={alpha is not null}, BetaValue={beta is not null}");\n'
        'Console.WriteLine($"{FerrumWeave.RustApi.AlphaValue()},{FerrumWeave.RustApi.BetaValue()}");\n',
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
            "managed consumer could not observe both Rust exports from one artifact:\n"
            f"{run.stdout}\n{run.stderr}"
        )
    lines = [line.strip() for line in run.stdout.splitlines() if line.strip()]
    if not lines:
        raise AssertionError("managed consumer produced no observable")
    try:
        left, right = lines[-1].split(",", maxsplit=1)
        return int(left), int(right)
    except (ValueError, TypeError) as exc:
        raise AssertionError(f"unexpected managed observable: {lines[-1]!r}") from exc


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
        with tempfile.TemporaryDirectory(prefix="ferrumweave-multiple-exports-") as temp:
            work = Path(temp)
            artifact = compile_crate(args.toolchain, backend, work)
            data = artifact.read_bytes()
            if b"AlphaValue" not in data or b"BetaValue" not in data:
                raise AssertionError(
                    "managed metadata does not contain both source-owned Rust export identities"
                )
            alpha, beta = run_consumer(artifact, work)
            if (alpha, beta) != (137, 211):
                raise AssertionError(
                    f"Rust source did not control both managed observables: alpha={alpha}, beta={beta}"
                )
    except AssertionError as exc:
        print(f"RED: {exc}")
        return 1

    print("GREEN: FerrumWeave preserves multiple Rust exports in one managed artifact")
    print("  one rustc invocation -> one FerrumWeave codegen_crate -> one managed DLL")
    print("  Rust alpha_value() -> CLR AlphaValue() -> 137")
    print("  Rust beta_value() -> CLR BetaValue() -> 211")
    print("  rustc_codegen_clr was not used in the product path")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
