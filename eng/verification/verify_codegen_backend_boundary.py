#!/usr/bin/env python3
"""Verify that rustc loads FerrumWeave as its codegen backend.

The same verifier supports two phases:
- handshake: success means rustc loaded FerrumWeave and reached codegen_crate;
- product gate: success means FerrumWeave compiled the Rust source to an artifact.
"""

from __future__ import annotations

import argparse
import subprocess
import tempfile
from pathlib import Path

MARKER = "FERRUMWEAVE_BACKEND_REACHED_CODEGEN_CRATE: MIR lowering is not implemented"


def run_compile(toolchain: str, backend: Path, work: Path) -> subprocess.CompletedProcess[str]:
    source = work / "backend_boundary.rs"
    output = work / ("backend_boundary.exe" if __import__("os").name == "nt" else "backend_boundary")
    source.write_text(
        'fn main() { let value: i32 = 137; println!("{value}"); }\n',
        encoding="utf-8",
    )
    return subprocess.run(
        [
            "rustc",
            f"+{toolchain}",
            "-Z",
            f"codegen-backend={backend}",
            "--edition",
            "2021",
            str(source),
            "-o",
            str(output),
        ],
        text=True,
        capture_output=True,
        check=False,
    )


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
        result = run_compile(args.toolchain, backend, work)
        combined = f"{result.stdout}\n{result.stderr}"

        if args.require_artifact:
            if result.returncode == 0:
                print("GREEN: rustc completed code generation through the FerrumWeave backend")
                return 0
            if MARKER in combined:
                print("RED: rustc loaded FerrumWeave, but FerrumWeave MIR lowering is not implemented")
                return 1
            print("ERROR: product gate failed before the expected FerrumWeave lowering boundary")
            print(combined)
            return 2

        if result.returncode == 0:
            print("ERROR: handshake expected the current explicit lowering RED, but compilation succeeded")
            return 2
        if MARKER not in combined:
            print("ERROR: rustc did not reach the FerrumWeave codegen_crate boundary")
            print(combined)
            return 2

        print("GREEN: rustc loaded FerrumWeave and reached codegen_crate")
        print("Expected next state: replace the explicit lowering RED with FerrumWeave-owned MIR lowering")
        return 0


if __name__ == "__main__":
    raise SystemExit(main())
