#!/usr/bin/env python3
"""Fail closed until every R05 managed-consumption family is source-causal."""

from __future__ import annotations

import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LEDGER = ROOT / "tests" / "r05" / "contracts.toml"
SOURCE_CAUSAL_CONTRACTS = {
    "FW-R05-DOTNET-009",
    "FW-R05-DOTNET-010",
    "FW-R05-DOTNET-011",
    "FW-R05-DOTNET-012",
    "FW-R05-DOTNET-013",
}


def fail(message: str) -> int:
    print(f"ERROR: {message}", file=sys.stderr)
    return 1


def main() -> int:
    data = tomllib.loads(LEDGER.read_text(encoding="utf-8"))
    contracts = data.get("contracts", [])
    by_id = {str(item.get("id")): item for item in contracts}
    missing_ids = SOURCE_CAUSAL_CONTRACTS - set(by_id)
    if missing_ids:
        return fail(
            "R05 source-causality contract census is incomplete: "
            + ", ".join(sorted(missing_ids))
        )

    status = str(data.get("status", "unknown"))
    uncovered: list[str] = []

    for contract_id in sorted(SOURCE_CAUSAL_CONTRACTS):
        contract = by_id[contract_id]
        implemented = bool(contract.get("implemented", False))
        proof = str(contract.get("proof", "")).strip()
        evidence_level = str(contract.get("evidence_level", "")).strip()

        if not implemented:
            uncovered.append(contract_id)
            print(f"RED       {contract_id}: source-causal evidence is still missing")
            continue
        if evidence_level != "source-causal-certified":
            return fail(
                f"{contract_id} is implemented but evidence_level is {evidence_level!r}; "
                "expected 'source-causal-certified'"
            )
        if not proof or proof.startswith("pending:"):
            return fail(f"{contract_id} is implemented without replayable proof")
        print(f"COVERED   {contract_id}: source-causal evidence is certified")

    if status == "done" and uncovered:
        return fail(
            "R05 is marked done while source-causal contracts remain uncovered: "
            + ", ".join(uncovered)
        )

    if uncovered:
        print(
            "Preserved evidence: artifact-level metadata/IL proofs remain valid; "
            "R05 stays open until every advertised managed-consumption family is causal."
        )
        return 1

    print("COVERED   R05 managed-consumption source causality is complete")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
