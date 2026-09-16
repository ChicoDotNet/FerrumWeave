#!/usr/bin/env python3
"""Falsify mixed constant/direct-call crate aggregation through FerrumWeave.

One rustc invocation must preserve both source-owned exports in one managed
artifact. The fixed consumer also requires the direct-call export to retain a
managed call opcode. rustc_codegen_clr is not part of this product-path claim.
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import tempfile
from pathlib import Path

ASSEMBLY_FILE = "FerrumWeave.Generated.dll"
SOURCE = '''#[no_mangle]
pub extern "C" fn constant_value() -> i32 {
    137
}

#[inline(never)]
fn helper(left: i32, right: i32) -> i32 {
    left + right
}

#[no_mangle]
pub extern "C" fn compute_result(left: i32, right: i32) -> i32 {
    helper(left, right)
}
'''


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
        with tempfile.TemporaryDirectory(prefix="ferrumweave-mixed-direct-call-") as temp:
            work = Path(temp)
            source = work / "mixed.rs"
            artifact = work / ASSEMBLY_FILE
            source.write_text(SOURCE, encoding="utf-8")
            result = subprocess.run(
                [
                    "rustc", f"+{args.toolchain}", "-Z", f"codegen-backend={backend}",
                    "--edition", "2021", "--crate-type", "lib", str(source), "-o", str(artifact),
                ],
                text=True, capture_output=True, check=False,
            )
            if result.returncode != 0:
                raise AssertionError(
                    "FerrumWeave could not lower mixed constant/direct-call crate:\n"
                    f"{result.stdout}\n{result.stderr}"
                )
            if not artifact.is_file():
                raise AssertionError("no managed artifact was produced")

            consumer = work / "consumer"
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
                'var compute = type.GetMethod("ComputeResult");\n'
                'if (constant is null || compute is null)\n'
                '    throw new System.Exception("mixed crate metadata lost a source-owned export");\n'
                'var il = compute.GetMethodBody()?.GetILAsByteArray();\n'
                'if (il is null || System.Array.IndexOf(il, (byte)0x28) < 0)\n'
                '    throw new System.Exception("ComputeResult lost its managed call opcode");\n'
                'System.Console.WriteLine($"{FerrumWeave.RustApi.ConstantValue()},{FerrumWeave.RustApi.ComputeResult(137, 74)}");\n',
                encoding="utf-8",
            )
            run = subprocess.run(
                ["dotnet", "run", "--project", str(consumer / "Consumer.csproj"), "--nologo"],
                text=True, capture_output=True, check=False,
            )
            if run.returncode != 0:
                raise AssertionError(
                    "managed metadata does not preserve constant and direct-call source-owned identities:\n"
                    f"{run.stdout}\n{run.stderr}"
                )
            lines = [line.strip() for line in run.stdout.splitlines() if line.strip()]
            if not lines or lines[-1] != "137,211":
                raise AssertionError(f"unexpected mixed managed observable: {lines[-1] if lines else '<none>'}")
    except AssertionError as exc:
        print(f"RED: {exc}")
        return 1

    print("GREEN: FerrumWeave aggregates constant + direct Rust call in one managed artifact")
    print("  Rust-owned exports: constant_value + compute_result")
    print("  Fixed managed observables: 137,211")
    print("  ComputeResult IL retains a managed call opcode")
    print("  rustc_codegen_clr was not used in the product path")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
