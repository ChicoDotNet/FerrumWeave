#!/usr/bin/env python3
"""Measure declared functional-contract coverage by executing the mapped tests."""

from __future__ import annotations

import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "tests" / "functional" / "contracts.toml"


def run(command: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        command,
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=False,
    )


def main() -> int:
    data = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))
    contracts = data.get("contracts", [])
    minimum = float(data.get("minimum_percent", 96.0))

    if not contracts:
        print("ERROR: functional contract ledger is empty.")
        return 1

    listed = run(["cargo", "test", "--locked", "--test", "functional", "--", "--list", "--format", "terse"])
    if listed.returncode != 0:
        sys.stdout.write(listed.stdout)
        sys.stderr.write(listed.stderr)
        return listed.returncode

    available = {
        line.removesuffix(": test").strip()
        for line in listed.stdout.splitlines()
        if line.strip().endswith(": test")
    }

    covered = 0
    failures: list[str] = []

    for contract in contracts:
        contract_id = contract["id"]
        implemented = bool(contract.get("implemented", False))
        test_name = str(contract.get("test", "")).strip()

        if not implemented:
            print(f"UNCOVERED {contract_id}: not implemented")
            continue

        if not test_name or test_name not in available:
            print(f"UNCOVERED {contract_id}: mapped test {test_name!r} was not found")
            failures.append(contract_id)
            continue

        result = run(
            [
                "cargo",
                "test",
                "--locked",
                "--test",
                "functional",
                test_name,
                "--",
                "--exact",
            ]
        )
        if result.returncode == 0:
            covered += 1
            print(f"COVERED   {contract_id}: {test_name}")
        else:
            print(f"FAILED    {contract_id}: {test_name}")
            sys.stdout.write(result.stdout)
            sys.stderr.write(result.stderr)
            failures.append(contract_id)

    percent = covered * 100.0 / len(contracts)
    print(f"Functional coverage: {covered}/{len(contracts)} = {percent:.2f}%")
    print(f"Policy: minimum {minimum:.2f}%; target 100.00%")

    if percent < minimum:
        print(f"ERROR: functional coverage is below the {minimum:.2f}% gate.")
        return 1
    if failures:
        print("ERROR: one or more implemented contracts failed their mapped test.")
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
