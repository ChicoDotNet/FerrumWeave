use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

use ferrumweave_cil::{
    PROBE_ASSEMBLY_NAME, PROBE_GREETING, ProbePaths, emit_probe_assembly, write_probe_artifacts,
};

#[test]
fn hello_ferrumweave_cli_contract() {
    let output = Command::new(env!("CARGO_BIN_EXE_ferrumweave"))
        .output()
        .expect("FerrumWeave binary should start");

    assert!(output.status.success(), "binary should exit successfully");
    assert!(
        output.stderr.is_empty(),
        "binary should not write to stderr"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");
    let normalized = stdout.trim_end_matches(&['\r', '\n'][..]);
    assert_eq!(normalized, "Hello FerrumWeave");
}

#[test]
fn r01_probe_is_managed_assembly() {
    let image = emit_probe_assembly();
    let layout = inspect_probe(&image);

    assert_eq!(layout.optional_magic, 0x010B, "R01 emits a PE32 CLI image");
    assert_ne!(layout.cli_rva, 0, "CLI data directory must be present");
    assert_eq!(layout.cli_header_size, 0x48);
    assert_eq!(
        layout.cor_flags & 0x1,
        0x1,
        "COMIMAGE_FLAGS_ILONLY must be set"
    );
    assert_eq!(layout.entry_point_token, 0x0600_0001);
    assert_eq!(read_u32(&image, layout.metadata_offset), 0x424A_5342);

    let tables = metadata_stream(&image, layout.metadata_offset, "#~");
    assert!(
        table_row_count(tables, 0x06) >= 1,
        "at least the managed entry-point MethodDef is expected"
    );
    assert_eq!(
        table_row_count(tables, 0x0A),
        1,
        "one MemberRef is expected"
    );
    assert_eq!(
        table_row_count(tables, 0x20),
        1,
        "one Assembly row is expected"
    );
    assert_eq!(
        table_row_count(tables, 0x23),
        1,
        "one AssemblyRef is expected"
    );

    let strings = metadata_stream(&image, layout.metadata_offset, "#Strings");
    assert!(contains_ascii(strings, PROBE_ASSEMBLY_NAME));
    assert!(contains_ascii(strings, "System.Console"));
    assert!(contains_ascii(strings, "WriteLine"));
}

#[test]
fn r01_probe_executes_on_coreclr() {
    with_probe(|paths| {
        let output = run_dotnet(&paths.assembly);
        assert_dotnet_greeting(&output);
    });
}

#[test]
fn r01_probe_is_portable_il_only_artifact() {
    let image = emit_probe_assembly();
    let layout = inspect_probe(&image);

    assert_eq!(
        layout.machine, 0x014C,
        "AnyCPU managed PE convention should be I386"
    );
    assert_eq!(layout.cor_flags & 0x1, 0x1, "artifact must be IL-only");
    assert_eq!(layout.cor_flags & 0x2, 0, "32BITREQUIRED must remain clear");

    with_probe(|paths| {
        let output = run_dotnet(&paths.assembly);
        assert_dotnet_greeting(&output);
    });
}

#[test]
fn r01_probe_has_no_native_implementation() {
    let image = emit_probe_assembly();
    let layout = inspect_probe(&image);

    assert_eq!(
        layout.native_entry_point_rva, 0,
        "PE native entry point must be empty"
    );
    assert_eq!(
        layout.import_directory_rva, 0,
        "R01 must not import a native bootstrap"
    );
    assert_eq!(layout.import_directory_size, 0);
    assert_eq!(
        layout.cor_flags & 0x10,
        0,
        "NATIVE_ENTRYPOINT flag must be clear"
    );
    assert_eq!(
        layout.entry_point_token >> 24,
        0x06,
        "entry point must be a MethodDef token"
    );
}

fn with_probe(test: impl FnOnce(&ProbePaths)) {
    let directory = unique_temp_directory();
    let paths = write_probe_artifacts(&directory).expect("R01 probe artifacts should be written");
    test(&paths);
    fs::remove_dir_all(directory).expect("temporary R01 directory should be removable");
}

fn run_dotnet(assembly: &Path) -> Output {
    Command::new("dotnet")
        .arg(assembly)
        .env("DOTNET_NOLOGO", "1")
        .env("DOTNET_CLI_TELEMETRY_OPTOUT", "1")
        .env("DOTNET_SKIP_FIRST_TIME_EXPERIENCE", "1")
        .output()
        .expect("dotnet should be available and start the managed probe")
}

fn assert_dotnet_greeting(output: &Output) {
    assert!(
        output.status.success(),
        "dotnet should execute the R01 probe successfully; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "managed probe should not write to stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout.clone()).expect("probe stdout should be UTF-8");
    let normalized = stdout.trim_end_matches(&['\r', '\n'][..]);
    assert_eq!(normalized, PROBE_GREETING);
}

fn unique_temp_directory() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("ferrumweave-r01-{}-{nanos}", std::process::id()))
}

#[derive(Debug)]
struct ProbeLayout {
    machine: u16,
    optional_magic: u16,
    native_entry_point_rva: u32,
    import_directory_rva: u32,
    import_directory_size: u32,
    cli_rva: u32,
    cli_header_size: u32,
    cor_flags: u32,
    entry_point_token: u32,
    metadata_offset: usize,
}

fn inspect_probe(image: &[u8]) -> ProbeLayout {
    assert_eq!(&image[0..2], b"MZ");
    let pe = to_usize(read_u32(image, 0x3C));
    assert_eq!(&image[pe..pe + 4], b"PE\0\0");

    let coff = pe + 4;
    let machine = read_u16(image, coff);
    let optional_size = usize::from(read_u16(image, coff + 16));
    let optional = coff + 20;
    let optional_magic = read_u16(image, optional);
    let native_entry_point_rva = read_u32(image, optional + 16);

    let data_directories = optional + 96;
    let import_directory_rva = read_u32(image, data_directories + 8);
    let import_directory_size = read_u32(image, data_directories + 12);
    let cli_rva = read_u32(image, data_directories + (14 * 8));

    let section = optional + optional_size;
    let section_rva = read_u32(image, section + 12);
    let section_raw = read_u32(image, section + 20);
    let cli_offset = rva_to_offset(cli_rva, section_rva, section_raw);

    let cli_header_size = read_u32(image, cli_offset);
    let metadata_rva = read_u32(image, cli_offset + 8);
    let cor_flags = read_u32(image, cli_offset + 16);
    let entry_point_token = read_u32(image, cli_offset + 20);
    let metadata_offset = rva_to_offset(metadata_rva, section_rva, section_raw);

    ProbeLayout {
        machine,
        optional_magic,
        native_entry_point_rva,
        import_directory_rva,
        import_directory_size,
        cli_rva,
        cli_header_size,
        cor_flags,
        entry_point_token,
        metadata_offset,
    }
}

fn metadata_stream<'a>(image: &'a [u8], metadata: usize, wanted: &str) -> &'a [u8] {
    assert_eq!(read_u32(image, metadata), 0x424A_5342);
    let version_len = to_usize(read_u32(image, metadata + 12));
    let mut cursor = metadata + 16 + version_len;
    let stream_count = usize::from(read_u16(image, cursor + 2));
    cursor += 4;

    for _ in 0..stream_count {
        let stream_offset = to_usize(read_u32(image, cursor));
        let stream_size = to_usize(read_u32(image, cursor + 4));
        let name_start = cursor + 8;
        let name_end = image[name_start..]
            .iter()
            .position(|byte| *byte == 0)
            .map(|relative| name_start + relative)
            .expect("metadata stream name should be NUL terminated");
        let name = std::str::from_utf8(&image[name_start..name_end])
            .expect("metadata stream name should be ASCII/UTF-8");

        if name == wanted {
            let absolute = metadata + stream_offset;
            return &image[absolute..absolute + stream_size];
        }

        let name_bytes = name_end - name_start + 1;
        cursor += 8 + align(name_bytes, 4);
    }

    panic!("metadata stream {wanted} was not found");
}

fn table_row_count(tables: &[u8], wanted_table: u8) -> u32 {
    let valid = read_u64(tables, 8);
    let mut cursor = 24;

    for table in 0_u8..64 {
        if valid & (1_u64 << table) == 0 {
            continue;
        }

        let rows = read_u32(tables, cursor);
        if table == wanted_table {
            return rows;
        }
        cursor += 4;
    }

    0
}

fn contains_ascii(haystack: &[u8], needle: &str) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle.as_bytes())
}

fn rva_to_offset(rva: u32, section_rva: u32, section_raw: u32) -> usize {
    assert!(
        rva >= section_rva,
        "RVA must reside in the R01 .text section"
    );
    to_usize(section_raw + (rva - section_rva))
}

fn align(value: usize, alignment: usize) -> usize {
    (value + (alignment - 1)) & !(alignment - 1)
}

fn read_u16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(
        bytes[offset..offset + 2]
            .try_into()
            .expect("u16 field should be present"),
    )
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(
        bytes[offset..offset + 4]
            .try_into()
            .expect("u32 field should be present"),
    )
}

fn read_u64(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(
        bytes[offset..offset + 8]
            .try_into()
            .expect("u64 field should be present"),
    )
}

fn to_usize(value: u32) -> usize {
    usize::try_from(value).expect("R01 PE offsets fit usize")
}
