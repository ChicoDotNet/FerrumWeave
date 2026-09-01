#!/usr/bin/env python3
"""Certify the R02 real-Rust-to-CLR vertical slice."""

from __future__ import annotations

import argparse
import json
import struct
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
FIXTURE_DIR = ROOT / "tests" / "fixtures" / "r02"
VALID_SOURCE = FIXTURE_DIR / "managed_console.rs"
INVALID_SOURCE = FIXTURE_DIR / "borrow_invalid.rs"
LEDGER = ROOT / "tests" / "r02" / "contracts.toml"
GREETING = "Hello FerrumWeave"
EXPECTED_CONTRACTS = {
    "FW-R02-CLR-001": "r02_real_rust_to_managed_assembly",
    "FW-R02-CLR-002": "r02_borrow_checker_rejects_invalid_source",
    "FW-R02-CLR-003": "r02_managed_program_runs_on_linux",
    "FW-R02-CLR-004": "r02_managed_program_runs_on_windows",
    "FW-R02-CLR-005": "r02_has_no_source_language_substitution",
}


def fail(message: str) -> None:
    raise AssertionError(message)


def run(command: list[str], cwd: Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        command,
        cwd=cwd,
        text=True,
        capture_output=True,
        check=False,
    )


def compiler_command(toolchain: str, backend: Path, linker: Path, source: Path, output: Path) -> list[str]:
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


def read_u16(data: bytes, offset: int) -> int:
    return struct.unpack_from("<H", data, offset)[0]


def read_u32(data: bytes, offset: int) -> int:
    return struct.unpack_from("<I", data, offset)[0]


def rva_to_offset(data: bytes, section_table: int, section_count: int, rva: int) -> int:
    for index in range(section_count):
        section = section_table + index * 40
        virtual_size = read_u32(data, section + 8)
        virtual_address = read_u32(data, section + 12)
        raw_size = read_u32(data, section + 16)
        raw_pointer = read_u32(data, section + 20)
        span = max(virtual_size, raw_size)
        if virtual_address <= rva < virtual_address + span:
            return raw_pointer + (rva - virtual_address)
    fail(f"RVA 0x{rva:08X} does not map to any PE section")
    return 0


def inspect_managed_pe(path: Path) -> None:
    data = path.read_bytes()
    if data[:2] != b"MZ":
        fail("R02 output is not a PE image")

    pe = read_u32(data, 0x3C)
    if data[pe : pe + 4] != b"PE\0\0":
        fail("R02 output has no PE signature")

    coff = pe + 4
    section_count = read_u16(data, coff + 2)
    optional_size = read_u16(data, coff + 16)
    optional = coff + 20
    magic = read_u16(data, optional)
    if magic == 0x10B:
        data_directories = optional + 96
    elif magic == 0x20B:
        data_directories = optional + 112
    else:
        fail(f"unsupported PE optional-header magic 0x{magic:04X}")

    cli_rva = read_u32(data, data_directories + 14 * 8)
    cli_size = read_u32(data, data_directories + 14 * 8 + 4)
    if cli_rva == 0 or cli_size < 72:
        fail("R02 output has no valid CLI header directory")

    section_table = optional + optional_size
    cli = rva_to_offset(data, section_table, section_count, cli_rva)
    cli_header_size = read_u32(data, cli)
    metadata_rva = read_u32(data, cli + 8)
    metadata_size = read_u32(data, cli + 12)
    cor_flags = read_u32(data, cli + 16)
    entry_point = read_u32(data, cli + 20)

    if cli_header_size < 72:
        fail("CLI header is smaller than IMAGE_COR20_HEADER")
    if metadata_rva == 0 or metadata_size == 0:
        fail("CLI header does not reference CLR metadata")
    if cor_flags & 0x1 == 0:
        fail("R02 artifact is not marked ILONLY")
    if cor_flags & 0x10:
        fail("R02 artifact declares a native entry point")
    if entry_point >> 24 != 0x06:
        fail(f"R02 entry point is not a MethodDef token: 0x{entry_point:08X}")

    metadata = rva_to_offset(data, section_table, section_count, metadata_rva)
    if data[metadata : metadata + 4] != b"BSJB":
        fail("R02 CLR metadata root signature is missing")


def parse_ledger_scalar(value: str) -> object:
    value = value.strip()
    if value == "true":
        return True
    if value == "false":
        return False
    if value.startswith('"') and value.endswith('"'):
        try:
            return json.loads(value)
        except json.JSONDecodeError as error:
            fail(f"invalid quoted value in R02 contract ledger: {error}")
    try:
        return float(value) if "." in value else int(value)
    except ValueError:
        fail(f"unsupported value in R02 contract ledger: {value!r}")
    return value


def load_contract_ledger(path: Path) -> dict[str, object]:
    """Parse only the tiny TOML subset owned by the R02 contract ledger.

    Keeping this parser deliberately scoped avoids requiring Python 3.11's
    tomllib on GitHub-hosted runners while refusing unsupported TOML syntax.
    """

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
            fail(f"unsupported R02 ledger syntax on line {line_number}: {raw_line!r}")
        key = key.strip()
        if not key:
            fail(f"empty key in R02 contract ledger on line {line_number}")

        target = current if current is not None else data
        if key in target:
            fail(f"duplicate key {key!r} in R02 contract ledger on line {line_number}")
        target[key] = parse_ledger_scalar(raw_value)

    return data


def verify_ledger() -> None:
    data = load_contract_ledger(LEDGER)
    contracts = data.get("contracts", [])
    if not isinstance(contracts, list):
        fail("R02 contract ledger contracts collection is invalid")

    found = {contract["id"]: contract for contract in contracts if isinstance(contract, dict)}
    if len(found) != len(contracts):
        fail("R02 contract ledger contains a malformed contract entry")
    if set(found) != set(EXPECTED_CONTRACTS):
        fail(f"R02 contract ledger mismatch: {sorted(found)}")
    if float(data.get("minimum_percent", 0.0)) != 100.0:
        fail("R02 functional-contract target must remain 100%")

    for contract_id, test_name in EXPECTED_CONTRACTS.items():
        contract = found[contract_id]
        if not contract.get("implemented", False):
            fail(f"{contract_id} is not marked implemented")
        if contract.get("test") != test_name:
            fail(f"{contract_id} is mapped to an unexpected verifier")


def verify_no_source_substitution() -> None:
    files = {path.name for path in FIXTURE_DIR.iterdir() if path.is_file()}
    expected = {VALID_SOURCE.name, INVALID_SOURCE.name}
    if files != expected:
        fail(f"R02 fixture directory must contain exactly {sorted(expected)}; found {sorted(files)}")

    forbidden = {".cs", ".vb", ".fs", ".fsx", ".cpp", ".c"}
    offenders = [path for path in FIXTURE_DIR.rglob("*") if path.is_file() and path.suffix.lower() in forbidden]
    if offenders:
        fail(f"R02 source-language substitution detected: {offenders}")


def verify_borrow_checker(toolchain: str, backend: Path, linker: Path, work: Path) -> None:
    output = work / "borrow-invalid.exe"
    result = run(compiler_command(toolchain, backend, linker, INVALID_SOURCE, output), ROOT)
    if result.returncode == 0:
        fail("invalid Rust borrow unexpectedly compiled")
    if "E0502" not in result.stderr:
        fail(f"invalid Rust failed for the wrong reason; expected E0502:\n{result.stderr}")
    if output.exists():
        fail("borrow-check failure produced an executable artifact")


def write_runtime_config(assembly: Path) -> None:
    config = assembly.with_suffix(".runtimeconfig.json")
    payload = {
        "runtimeOptions": {
            "tfm": "net10.0",
            "framework": {"name": "Microsoft.NETCore.App", "version": "10.0.0"},
        }
    }
    config.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def verify_execution(assembly: Path) -> None:
    write_runtime_config(assembly)
    result = run(["dotnet", str(assembly)], assembly.parent)
    if result.returncode != 0:
        fail(f"CoreCLR execution failed:\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}")
    if result.stderr:
        fail(f"managed R02 program wrote to stderr:\n{result.stderr}")
    if result.stdout.rstrip("\r\n") != GREETING:
        fail(f"unexpected R02 stdout: {result.stdout!r}")


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

    verify_ledger()
    verify_no_source_substitution()

    with tempfile.TemporaryDirectory(prefix="ferrumweave-r02-") as temporary:
        work = Path(temporary)
        assembly = work / "FerrumWeave.R02.exe"

        result = run(compiler_command(args.toolchain, backend, linker, VALID_SOURCE, assembly), ROOT)
        if result.returncode != 0:
            fail(f"real Rust -> CLR compilation failed:\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}")
        if not assembly.is_file():
            fail("rustc/codegen backend reported success without producing the R02 artifact")

        inspect_managed_pe(assembly)
        verify_borrow_checker(args.toolchain, backend, linker, work)
        verify_execution(assembly)

    platform_contract = "FW-R02-CLR-004" if sys.platform.startswith("win") else "FW-R02-CLR-003"
    print("COVERED   FW-R02-CLR-001: real Rust compiled to managed PE/CLI")
    print("COVERED   FW-R02-CLR-002: rustc rejected invalid borrow with E0502")
    print(f"COVERED   {platform_contract}: managed Rust executed on this CI platform")
    print("COVERED   FW-R02-CLR-005: direct .rs -> rustc path; no source-language substitute")
    print("R02 matrix contract target: 5/5 = 100.00% when Linux and Windows jobs both pass")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        raise SystemExit(1)
