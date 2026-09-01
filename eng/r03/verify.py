#!/usr/bin/env python3
"""Certify incremental R03 safe-Rust semantics against native rustc behavior."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
FIXTURE_DIR = ROOT / "tests" / "fixtures" / "r03"
LEDGER = ROOT / "tests" / "r03" / "contracts.toml"
KNOWN_CONTRACTS = {
    "FW-R03-SEM-001",
    "FW-R03-SEM-002",
    "FW-R03-SEM-003",
    "FW-R03-SEM-004",
    "FW-R03-SEM-005",
    "FW-R03-SEM-006",
    "FW-R03-SEM-007",
    "FW-R03-SEM-008",
    "FW-R03-SEM-009",
    "FW-R03-SEM-010",
    "FW-R03-NEG-001",
}
EXPECTED_OUTPUTS = {
    "semantics_s01.rs": "42\n30\n1\n42\n7\n42\n42\n42\n",
}
EXPECTED_REJECTIONS = {
    "borrow_invalid.rs": "E0502",
}
VERIFIED_FIXTURES = set(EXPECTED_OUTPUTS) | set(EXPECTED_REJECTIONS)


def fail(message: str) -> None:
    raise AssertionError(message)


def run(command: list[str], cwd: Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(command, cwd=cwd, text=True, capture_output=True, check=False)


def parse_scalar(value: str) -> object:
    value = value.strip()
    if value == "true":
        return True
    if value == "false":
        return False
    if value.startswith('"') and value.endswith('"'):
        try:
            return json.loads(value)
        except json.JSONDecodeError as error:
            fail(f"invalid quoted ledger value: {error}")
    try:
        return float(value) if "." in value else int(value)
    except ValueError:
        fail(f"unsupported ledger scalar: {value!r}")
    return value


def load_ledger(path: Path) -> dict[str, object]:
    data: dict[str, object] = {"contracts": []}
    contracts = data["contracts"]
    assert isinstance(contracts, list)
    current: dict[str, object] | None = None

    for line_number, raw_line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        if line == "[[contracts]]":
            current = {}
            contracts.append(current)
            continue

        key, separator, raw_value = line.partition("=")
        if not separator:
            fail(f"unsupported R03 ledger syntax on line {line_number}: {raw_line!r}")
        key = key.strip()
        if not key:
            fail(f"empty ledger key on line {line_number}")
        target = current if current is not None else data
        if key in target:
            fail(f"duplicate ledger key {key!r} on line {line_number}")
        target[key] = parse_scalar(raw_value)

    return data


def verify_ledger() -> tuple[list[dict[str, object]], float, str]:
    data = load_ledger(LEDGER)
    if data.get("milestone") != "R03":
        fail("R03 contract ledger has the wrong milestone id")
    status = data.get("status")
    if status not in {"in_progress", "done"}:
        fail(f"unsupported R03 milestone status: {status!r}")

    raw_contracts = data.get("contracts")
    if not isinstance(raw_contracts, list):
        fail("R03 contracts collection is invalid")
    contracts = [contract for contract in raw_contracts if isinstance(contract, dict)]
    if len(contracts) != len(raw_contracts):
        fail("R03 ledger contains a malformed contract")

    found = {str(contract.get("id")): contract for contract in contracts}
    if len(found) != len(contracts):
        fail("R03 ledger contains duplicate contract ids")
    if set(found) != KNOWN_CONTRACTS:
        fail(f"R03 known-contract census mismatch: {sorted(found)}")

    implemented = [contract for contract in contracts if contract.get("implemented") is True]
    for contract in implemented:
        fixture = contract.get("fixture")
        if not isinstance(fixture, str) or not fixture:
            fail(f"implemented contract {contract['id']} has no fixture")
        if fixture not in VERIFIED_FIXTURES:
            fail(f"implemented contract {contract['id']} uses an unverified fixture {fixture!r}")

    coverage = 100.0 * len(implemented) / len(contracts)
    minimum_when_done = float(data.get("minimum_percent_when_done", 0.0))
    target_when_done = float(data.get("target_percent_when_done", 0.0))
    if minimum_when_done != 96.0 or target_when_done != 100.0:
        fail("R03 DoD coverage thresholds drifted from the roadmap policy")
    if status == "done" and coverage < minimum_when_done:
        fail(f"R03 marked done at only {coverage:.2f}% functional coverage")
    if status == "done" and coverage != target_when_done:
        fail(f"R03 marked done before reaching the declared 100% target: {coverage:.2f}%")

    return implemented, coverage, str(status)


def clr_command(toolchain: str, backend: Path, linker: Path, source: Path, output: Path) -> list[str]:
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
        "--cfg",
        "ferrumweave_clr",
        "-Ctarget-feature=+x87+sse",
        str(source),
        "-o",
        str(output),
    ]


def native_command(toolchain: str, source: Path, output: Path) -> list[str]:
    return ["rustc", f"+{toolchain}", "-O", "--edition", "2021", str(source), "-o", str(output)]


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


def compile_or_fail(command: list[str], label: str) -> None:
    result = run(command, ROOT)
    if result.returncode != 0:
        fail(f"{label} compilation failed:\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}")


def execute_or_fail(command: list[str], cwd: Path, label: str) -> str:
    result = run(command, cwd)
    if result.returncode != 0:
        fail(f"{label} execution failed:\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}")
    if result.stderr:
        fail(f"{label} wrote to stderr:\n{result.stderr}")
    return result.stdout.replace("\r\n", "\n")


def verify_positive_fixture(
    toolchain: str, backend: Path, linker: Path, fixture_name: str, work: Path
) -> None:
    source = FIXTURE_DIR / fixture_name
    if not source.is_file():
        fail(f"R03 fixture not found: {source}")
    source_text = source.read_text(encoding="utf-8")
    if "unsafe" in source_text:
        fail(f"R03 safe-semantics fixture unexpectedly contains unsafe code: {fixture_name}")

    native = work / ("native.exe" if sys.platform.startswith("win") else "native")
    managed = work / "FerrumWeave.R03.exe"

    compile_or_fail(native_command(toolchain, source, native), f"native parity fixture {fixture_name}")
    native_stdout = execute_or_fail([str(native)], work, f"native parity fixture {fixture_name}")
    expected = EXPECTED_OUTPUTS[fixture_name]
    if native_stdout != expected:
        fail(f"native rustc produced unexpected semantics for {fixture_name}: {native_stdout!r}")

    compile_or_fail(clr_command(toolchain, backend, linker, source, managed), f"CLR fixture {fixture_name}")
    if not managed.is_file():
        fail(f"CLR backend produced no managed artifact for {fixture_name}")
    write_runtime_config(managed)
    clr_stdout = execute_or_fail(["dotnet", str(managed)], work, f"CLR fixture {fixture_name}")
    if clr_stdout != native_stdout:
        fail(
            f"backend semantic mismatch for {fixture_name}: "
            f"native={native_stdout!r}, clr={clr_stdout!r}"
        )


def verify_negative_fixture(
    toolchain: str, backend: Path, linker: Path, fixture_name: str, work: Path
) -> None:
    source = FIXTURE_DIR / fixture_name
    if not source.is_file():
        fail(f"R03 negative fixture not found: {source}")
    output = work / "negative.exe"
    result = run(clr_command(toolchain, backend, linker, source, output), ROOT)
    if result.returncode == 0:
        fail(f"invalid safe-Rust fixture unexpectedly compiled: {fixture_name}")
    expected_code = EXPECTED_REJECTIONS[fixture_name]
    if expected_code not in result.stderr:
        fail(
            f"invalid safe-Rust fixture failed for the wrong reason; expected {expected_code}:\n"
            f"{result.stderr}"
        )
    if output.exists():
        fail(f"negative semantic rejection produced an executable artifact: {fixture_name}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--backend", required=True, type=Path)
    parser.add_argument("--linker", required=True, type=Path)
    parser.add_argument("--toolchain", default="nightly-2025-10-14")
    args = parser.parse_args()

    backend = args.backend.resolve()
    linker = args.linker.resolve()
    if not backend.is_file():
        fail(f"codegen backend not found: {backend}")
    if not linker.is_file():
        fail(f"CIL linker not found: {linker}")

    implemented, coverage, status = verify_ledger()
    positive_fixtures = sorted(
        {str(contract["fixture"]) for contract in implemented if contract["fixture"] in EXPECTED_OUTPUTS}
    )
    negative_fixtures = sorted(
        {str(contract["fixture"]) for contract in implemented if contract["fixture"] in EXPECTED_REJECTIONS}
    )

    with tempfile.TemporaryDirectory(prefix="ferrumweave-r03-") as temporary:
        root = Path(temporary)
        for index, fixture_name in enumerate(positive_fixtures):
            work = root / f"positive-{index}"
            work.mkdir()
            verify_positive_fixture(args.toolchain, backend, linker, fixture_name, work)
        for index, fixture_name in enumerate(negative_fixtures):
            work = root / f"negative-{index}"
            work.mkdir()
            verify_negative_fixture(args.toolchain, backend, linker, fixture_name, work)

    for contract in implemented:
        print(f"COVERED   {contract['id']}: {contract['family']}")
    missing = len(KNOWN_CONTRACTS) - len(implemented)
    print(
        f"R03 milestone progress: {len(implemented)}/{len(KNOWN_CONTRACTS)} "
        f"= {coverage:.2f}% ({missing} known contracts not implemented yet)"
    )
    if status == "done":
        print("R03 ledger is complete; CI certification is the final milestone gate")
    else:
        print("R03 remains in progress; >=96% becomes mandatory when status changes to done")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        raise SystemExit(1)
