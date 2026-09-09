#!/usr/bin/env python3
"""Falsify direct Rust function-call lowering through the FerrumWeave backend.

The C# consumer and inputs stay fixed. Only the body of a non-inlined Rust helper
changes from addition to subtraction. A passing artifact must therefore both
contain a managed call in Answer and change its observable through Rust callee
semantics. rustc_codegen_clr is not part of this product-path contract.
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import tempfile
from pathlib import Path

ASSEMBLY_FILE = "FerrumWeave.Generated.dll"
SOURCE_TEMPLATE = '''#[inline(never)]
fn helper(left: i32, right: i32) -> i32 {{
    left {operator} right
}}

#[no_mangle]
pub extern "C" fn answer(left: i32, right: i32) -> i32 {{
    helper(left, right)
}}
'''


def compile_variant(toolchain: str, backend: Path, work: Path, name: str, operator: str) -> Path:
    source = work / f"{name}.rs"
    artifact = work / f"{name}.dll"
    source.write_text(SOURCE_TEMPLATE.format(operator=operator), encoding="utf-8")
    result = subprocess.run(
        [
            "rustc", f"+{toolchain}", "-Z", f"codegen-backend={backend}",
            "--edition", "2021", "--crate-type", "lib", str(source), "-o", str(artifact),
        ],
        text=True, capture_output=True, check=False,
    )
    if result.returncode != 0:
        raise AssertionError(
            f"FerrumWeave could not lower direct Rust function call ({name}):\n"
            f"{result.stdout}\n{result.stderr}"
        )
    if not artifact.is_file():
        raise AssertionError(f"no managed artifact was produced for {name}")
    return artifact


def run_consumer(artifact: Path, root: Path, name: str) -> int:
    consumer = root / f"consumer-{name}"
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
        'var method = typeof(FerrumWeave.RustApi).GetMethod("Answer");\n'
        'var il = method?.GetMethodBody()?.GetILAsByteArray();\n'
        'if (il is null || System.Array.IndexOf(il, (byte)0x28) < 0)\n'
        '    throw new System.Exception("Answer does not contain a managed call opcode");\n'
        'System.Console.WriteLine(FerrumWeave.RustApi.Answer(137, 74));\n',
        encoding="utf-8",
    )
    run = subprocess.run(
        ["dotnet", "run", "--project", str(consumer / "Consumer.csproj"), "--nologo"],
        text=True, capture_output=True, check=False,
    )
    if run.returncode != 0:
        raise AssertionError(
            f"managed consumer failed for {name}:\n{run.stdout}\n{run.stderr}"
        )
    lines = [line.strip() for line in run.stdout.splitlines() if line.strip()]
    if not lines:
        raise AssertionError(f"managed consumer produced no observable for {name}")
    try:
        return int(lines[-1])
    except ValueError as exc:
        raise AssertionError(f"unexpected managed observable for {name}: {lines[-1]!r}") from exc


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
        with tempfile.TemporaryDirectory(prefix="ferrumweave-direct-call-") as temp:
            work = Path(temp)
            add_artifact = compile_variant(args.toolchain, backend, work, "helper_add", "+")
            sub_artifact = compile_variant(args.toolchain, backend, work, "helper_sub", "-")

            if add_artifact.read_bytes() == sub_artifact.read_bytes():
                raise AssertionError("Rust-only helper mutation did not change the managed artifact")

            add_observed = run_consumer(add_artifact, work, "add")
            sub_observed = run_consumer(sub_artifact, work, "sub")
            if add_observed != 211 or sub_observed != 63:
                raise AssertionError(
                    "Rust helper mutation did not control the managed observable: "
                    f"add={add_observed}, sub={sub_observed}"
                )
    except AssertionError as exc:
        print(f"RED: {exc}")
        return 1

    print("GREEN: FerrumWeave source-causally lowers a direct Rust function call")
    print("  Fixed C# call: Answer(137, 74)")
    print("  Rust-only mutation: helper left + right -> left - right")
    print("  Managed observable: 211 -> 63")
    print("  Answer IL contains a managed call opcode")
    print("  rustc_codegen_clr was not used in the product path")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
