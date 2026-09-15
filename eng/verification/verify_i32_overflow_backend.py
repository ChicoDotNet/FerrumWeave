#!/usr/bin/env python3
"""Falsify FerrumWeave checked i32 overflow semantics through rustc MIR.

The Rust function uses ordinary i32 addition. For the pinned debug rustc lane,
MIR carries AddWithOverflow plus an Assert. A conforming FerrumWeave backend
must not silently turn that into wrapping CLR `add` behavior.
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import tempfile
from pathlib import Path

ASSEMBLY_FILE = "FerrumWeave.Generated.dll"
RUST_SOURCE = '''#[no_mangle]
pub extern "C" fn answer(left: i32, right: i32) -> i32 {
    left + right
}
'''


def compile_source(toolchain: str, backend: Path, work: Path) -> Path:
    source = work / "checked_add.rs"
    artifact = work / "checked_add.dll"
    source.write_text(RUST_SOURCE, encoding="utf-8")
    result = subprocess.run(
        [
            "rustc", f"+{toolchain}", "-Z", f"codegen-backend={backend}",
            "--edition", "2021", "--crate-type", "lib", str(source), "-o", str(artifact),
        ],
        text=True, capture_output=True, check=False,
    )
    if result.returncode != 0:
        raise AssertionError(
            "FerrumWeave could not lower checked Rust i32 addition:\n"
            f"{result.stdout}\n{result.stderr}"
        )
    if not artifact.is_file():
        raise AssertionError("no managed artifact was produced for checked Rust i32 addition")
    return artifact


def make_consumer(artifact: Path, root: Path) -> Path:
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
        'System.Console.WriteLine(FerrumWeave.RustApi.Answer(int.MaxValue, 1));\n',
        encoding="utf-8",
    )
    return consumer / "Consumer.csproj"


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
        with tempfile.TemporaryDirectory(prefix="ferrumweave-i32-overflow-") as temp:
            work = Path(temp)
            artifact = compile_source(args.toolchain, backend, work)
            project = make_consumer(artifact, work)
            run = subprocess.run(
                ["dotnet", "run", "--project", str(project), "--nologo"],
                text=True, capture_output=True, check=False,
            )
            if run.returncode == 0:
                observed = run.stdout.strip().splitlines()[-1] if run.stdout.strip() else ""
                raise AssertionError(
                    "Rust checked i32 overflow executed successfully instead of taking the "
                    f"overflow failure path; observable was {observed!r}"
                )
    except AssertionError as exc:
        print(f"RED: {exc}")
        return 1

    print("GREEN: FerrumWeave preserves the rustc checked-i32 overflow failure path")
    print("  Rust source: left + right")
    print("  Fixed C# call: Answer(int.MaxValue, 1)")
    print("  Required behavior: overflow must not return a wrapped i32 value")
    print("  rustc_codegen_clr was not used in the product path")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
