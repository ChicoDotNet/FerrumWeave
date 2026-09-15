#!/usr/bin/env python3
"""Falsify specialized-emitter substitution for constant + control-flow exports.

Both shapes are already FerrumWeave-backend-proven independently. One rustc
invocation must preserve both in one managed artifact. rustc_codegen_clr is not
used as product evidence.
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import tempfile
from pathlib import Path

ASSEMBLY_FILE = "FerrumWeave.Generated.dll"
SOURCE = '''#[no_mangle]
pub extern "C" fn constant_value() -> i32 { 137 }

#[no_mangle]
pub extern "C" fn choose_value(selector: i32, left: i32, right: i32) -> i32 {
    if selector == 0 { left } else { right }
}
'''


def compile_crate(toolchain: str, backend: Path, work: Path) -> Path:
    source = work / "mixed_constant_control_flow.rs"
    artifact = work / ASSEMBLY_FILE
    source.write_text(SOURCE, encoding="utf-8")
    result = subprocess.run(
        ["rustc", f"+{toolchain}", "-Z", f"codegen-backend={backend}",
         "--edition", "2021", "--crate-type", "lib", str(source), "-o", str(artifact)],
        text=True, capture_output=True, check=False,
    )
    if result.returncode != 0:
        raise AssertionError(
            "FerrumWeave could not compile mixed constant/control-flow exports:\n"
            f"{result.stdout}\n{result.stderr}"
        )
    if not artifact.is_file():
        raise AssertionError("mixed Rust crate produced no managed artifact")
    return artifact


def run_consumer(artifact: Path, root: Path) -> tuple[int, int, int]:
    consumer = root / "consumer"
    consumer.mkdir()
    shutil.copyfile(artifact, consumer / ASSEMBLY_FILE)
    (consumer / "Consumer.csproj").write_text(
        '<Project Sdk="Microsoft.NET.Sdk">\n'
        '  <PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net10.0</TargetFramework></PropertyGroup>\n'
        '  <ItemGroup><Reference Include="FerrumWeave.Generated"><HintPath>FerrumWeave.Generated.dll</HintPath><Private>true</Private></Reference></ItemGroup>\n'
        '</Project>\n', encoding="utf-8",
    )
    (consumer / "Program.cs").write_text(
        'var type = typeof(FerrumWeave.RustApi);\n'
        'var constant = type.GetMethod("ConstantValue");\n'
        'var choose = type.GetMethod("ChooseValue");\n'
        'if (constant is null || choose is null)\n'
        '    throw new System.Exception($"missing exports: ConstantValue={constant is not null}, ChooseValue={choose is not null}");\n'
        'System.Console.WriteLine($"{FerrumWeave.RustApi.ConstantValue()},{FerrumWeave.RustApi.ChooseValue(0, 137, 211)},{FerrumWeave.RustApi.ChooseValue(1, 137, 211)}");\n',
        encoding="utf-8",
    )
    run = subprocess.run(
        ["dotnet", "run", "--project", str(consumer / "Consumer.csproj"), "--nologo"],
        text=True, capture_output=True, check=False,
    )
    if run.returncode != 0:
        raise AssertionError(
            "managed consumer could not observe mixed Rust exports:\n"
            f"{run.stdout}\n{run.stderr}"
        )
    lines = [line.strip() for line in run.stdout.splitlines() if line.strip()]
    if not lines:
        raise AssertionError("managed consumer produced no observable")
    try:
        constant, zero, nonzero = lines[-1].split(",", maxsplit=2)
        return int(constant), int(zero), int(nonzero)
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
        with tempfile.TemporaryDirectory(prefix="ferrumweave-mixed-control-flow-") as temp:
            work = Path(temp)
            artifact = compile_crate(args.toolchain, backend, work)
            data = artifact.read_bytes()
            if b"ConstantValue" not in data or b"ChooseValue" not in data:
                raise AssertionError(
                    "managed metadata does not preserve constant and control-flow source-owned identities"
                )
            observed = run_consumer(artifact, work)
            if observed != (137, 137, 211):
                raise AssertionError(f"Rust source did not control mixed observables: {observed}")
    except AssertionError as exc:
        print(f"RED: {exc}")
        return 1

    print("GREEN: FerrumWeave preserves constant + control-flow exports in one managed artifact")
    print("  ConstantValue() -> 137; ChooseValue(0,137,211) -> 137; ChooseValue(1,137,211) -> 211")
    print("  rustc_codegen_clr was not used in the product path")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
