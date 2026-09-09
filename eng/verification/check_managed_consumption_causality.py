#!/usr/bin/env python3
"""Fail closed until managed API consumption is caused by real Rust source."""

from __future__ import annotations

import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LEDGER = ROOT / "tests" / "r05" / "contracts.toml"
SOURCE_CAUSAL_CONTRACT = "FW-R05-DOTNET-009"


def fail(message: str) -> int:
    print(f"ERROR: {message}", file=sys.stderr)
    return 1


def main() -> int:
    data = tomllib.loads(LEDGER.read_text(encoding="utf-8"))
    contracts = data.get("contracts", [])
    contract = next(
        (item for item in contracts if item.get("id") == SOURCE_CAUSAL_CONTRACT),
        None,
    )

    if contract is None:
        return fail(
            f"{SOURCE_CAUSAL_CONTRACT} is missing; R05 cannot be certified without a source-causality gate"
        )

    implemented = bool(contract.get("implemented", False))
    status = str(data.get("status", "unknown"))
    proof = str(contract.get("proof", "")).strip()
    evidence_level = str(contract.get("evidence_level", "")).strip()

    if status == "done" and not implemented:
        return fail(
            f"R05 is marked done while {SOURCE_CAUSAL_CONTRACT} is not implemented"
        )

    if not implemented:
        print(
            f"RED       {SOURCE_CAUSAL_CONTRACT}: real Rust source -> rustc/codegen -> managed API causality is not yet proven"
        )
        print(
            "Preserved evidence: metadata resolution and managed IL emission remain artifact-proven; they are not discarded."
        )
        return 1

    if evidence_level != "source-causal-certified":
        return fail(
            f"{SOURCE_CAUSAL_CONTRACT} is implemented but evidence_level is {evidence_level!r}; expected 'source-causal-certified'"
        )

    if not proof or proof.startswith("pending:"):
        return fail(
            f"{SOURCE_CAUSAL_CONTRACT} is implemented without replayable proof"
        )

    print(
        f"COVERED   {SOURCE_CAUSAL_CONTRACT}: source-causal managed API consumption is certified"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
