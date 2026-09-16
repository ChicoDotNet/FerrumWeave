#!/usr/bin/env python3
"""Certify that real Rust source causally selects and drives a managed static call."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
FIXTURE = ROOT / "tests" / "fixtures" / "managed_static_call.rs"
BASE_VALUE = "const PROBE_VALUE: i32 = 137;"
MUTATED_VALUE = "const PROBE_VALUE: i32 = 211;"
BASE_METHOD = '        "WriteLine",'
MUTATED_METHOD = '        "Write",'


def fail(message: str) -> None:
    raise AssertionError(message)


def run(command: list[str], cwd: Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(command, cwd=cwd, text=True, capture_output=True, check=False)


def clr_command(
    toolchain: str, backend: Path, linker: Path, source: Path, output: Path
) -> list[str]:
    return [
        "rustc",
        f"+{toolchain}",
        "-O",
        "-Z",
        f"codegen-backend={backend}",
        "-C",
        f"linker={linker}",
        "--edition",
        "2021",
        "-Ctarget-feature=+x87+sse",
        str(source),
        "-o",
        str(output),
    ]


def write_runtime_config(assembly: Path) -> None:
    payload = {
        "runtimeOptions": {
            "tfm": "net10.0",
            "framework": {"name": "Microsoft.NETCore.App", "version": "10.0.0"},
        }
    }
    assembly.with_suffix(".runtimeconfig.json").write_text(
        json.dumps(payload, indent=2) + "\n", encoding="utf-8"
    )


def compile_and_execute(
    *,
    toolchain: str,
    backend: Path,
    linker: Path,
    source_text: str,
    stem: str,
    work: Path,
) -> str:
    source = work / f"{stem}.rs"
    assembly = work / f"{stem}.exe"
    source.write_text(source_text, encoding="utf-8")

    compiled = run(clr_command(toolchain, backend, linker, source, assembly), ROOT)
    if compiled.returncode != 0:
        fail(
            f"{stem} CLR compilation failed:\nstdout:\n{compiled.stdout}\nstderr:\n{compiled.stderr}"
        )
    if not assembly.is_file():
        fail(f"{stem} produced no managed assembly")

    write_runtime_config(assembly)
    executed = run(["dotnet", str(assembly)], work)
    if executed.returncode != 0:
        fail(
            f"{stem} managed execution failed:\nstdout:\n{executed.stdout}\nstderr:\n{executed.stderr}"
        )
    if executed.stderr:
        fail(f"{stem} wrote to stderr:\n{executed.stderr}")
    return executed.stdout.replace("\r\n", "\n")


def replace_once(source: str, old: str, new: str, label: str) -> str:
    if source.count(old) != 1:
        fail(f"{label} mutation expected exactly one source occurrence of {old!r}")
    return source.replace(old, new, 1)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--toolchain", required=True)
    parser.add_argument("--backend", type=Path, required=True)
    parser.add_argument("--linker", type=Path, required=True)
    args = parser.parse_args()

    if not FIXTURE.is_file():
        fail(f"managed static-call fixture not found: {FIXTURE}")
    source = FIXTURE.read_text(encoding="utf-8")

    for marker in (
        BASE_VALUE,
        BASE_METHOD,
        '        "System.Console",',
        "rustc_clr_interop_managed_call1_",
    ):
        if marker not in source:
            fail(f"fixture is missing required causal marker: {marker!r}")

    value_mutation = replace_once(source, BASE_VALUE, MUTATED_VALUE, "value")
    method_mutation = replace_once(source, BASE_METHOD, MUTATED_METHOD, "method")

    with tempfile.TemporaryDirectory(prefix="ferrumweave-managed-causality-") as temp:
        work = Path(temp)
        baseline = compile_and_execute(
            toolchain=args.toolchain,
            backend=args.backend,
            linker=args.linker,
            source_text=source,
            stem="baseline",
            work=work,
        )
        mutated_value = compile_and_execute(
            toolchain=args.toolchain,
            backend=args.backend,
            linker=args.linker,
            source_text=value_mutation,
            stem="value_mutation",
            work=work,
        )
        mutated_method = compile_and_execute(
            toolchain=args.toolchain,
            backend=args.backend,
            linker=args.linker,
            source_text=method_mutation,
            stem="method_mutation",
            work=work,
        )

    if baseline != "137\n":
        fail(f"baseline Rust source produced unexpected managed output: {baseline!r}")
    if mutated_value != "211\n":
        fail(
            "changing only the Rust source value did not change the managed observable "
            f"from 137 to 211: {mutated_value!r}"
        )
    if mutated_method != "137":
        fail(
            "changing only the Rust source managed method from WriteLine to Write did not "
            f"remove the newline: {mutated_method!r}"
        )

    print("COVERED   managed static-call source causality")
    print("baseline source: System.Console.WriteLine(137) -> '137\\n'")
    print("value mutation: 137 -> 211 changed only Rust source and produced '211\\n'")
    print("method mutation: WriteLine -> Write changed only Rust source and produced '137'")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        raise SystemExit(1) from error
