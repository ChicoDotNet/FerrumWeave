#!/usr/bin/env python3
"""Enforce FerrumWeave's Rust line-coverage policy."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("report", type=Path)
    parser.add_argument("--minimum", type=float, default=80.0)
    parser.add_argument("--preferred-maximum", type=float, default=96.0)
    args = parser.parse_args()

    data = json.loads(args.report.read_text(encoding="utf-8"))
    try:
        percent = float(data["data"][0]["totals"]["lines"]["percent"])
    except (KeyError, IndexError, TypeError, ValueError) as exc:
        raise SystemExit(f"Unable to read line coverage from {args.report}: {exc}") from exc

    print(f"Rust line coverage: {percent:.2f}%")
    print(f"Policy: minimum {args.minimum:.2f}%; preferred operating band {args.minimum:.2f}%–{args.preferred_maximum:.2f}%")

    if percent < args.minimum:
        print(f"ERROR: coverage is below the {args.minimum:.2f}% gate.")
        return 1

    if percent > args.preferred_maximum:
        print(
            "NOTICE: coverage is above the preferred operating band. "
            "This is allowed; do not add untested code or low-value tests merely to force the percentage downward."
        )
    else:
        print("Coverage is inside the preferred operating band.")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
