#![forbid(unsafe_code)]

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub const PROBE_ASSEMBLY_NAME: &str = "FerrumWeave.Probe";
pub const PROBE_ASSEMBLY_FILE: &str = "FerrumWeave.Probe.dll";
pub const PROBE_RUNTIME_CONFIG_FILE: &str = "FerrumWeave.Probe.runtimeconfig.json";
pub const PROBE_GREETING: &str = "Hello FerrumWeave";

const PE_OFFSET: usize = 0x80;
const OPTIONAL_HEADER_SIZE: usize = 0xE0;
const HEADERS_SIZE: usize = 0x200;
const FILE_ALIGNMENT: usize = 0x200;
const SECTION_ALIGNMENT: u32 = 0x2000;
const SECTION_RVA: u32 = 0x2000;
const CLR_HEADER_SIZE: usize = 0x48;
const METHOD_DEF_TOKEN_MAIN: u32 = 0x0600_0001;
const MEMBER_REF_TOKEN_WRITELINE: u32 = 0x0A00_0001;
const MEMBER_REF_TOKEN_OBJECT_CTOR: u32 = 0x0A00_0002;
const MEMBER_REF_TOKEN_OBJECT_TOSTRING: u32 = 0x0A00_0003;
const MEMBER_REF_TOKEN_ENVIRONMENT_GET_CURRENT_DIRECTORY: u32 = 0x0A00_0004;
const MEMBER_REF_TOKEN_ENVIRONMENT_SET_CURRENT_DIRECTORY: u32 = 0x0A00_0005;
const USER_STRING_TOKEN_GREETING: u32 = 0x7000_0001;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProbePaths {
    pub assembly: PathBuf,
    pub runtime_config: PathBuf,
}

/// Emits the deliberately tiny managed probe assembly.
///
/// The byte layout intentionally contains only the ECMA-335 structures required
/// by the currently certified vertical slices. R05 extends the original R01
/// probe with real managed construction, instance dispatch, and property access
/// while preserving the executable `System.Console.WriteLine` path.
#[must_use]
pub fn emit_probe_assembly() -> Vec<u8> {
    let method_body = build_main_method_body();
    let method_offset = CLR_HEADER_SIZE;
    let method_rva = SECTION_RVA + to_u32(method_offset);

    let metadata = build_metadata(method_rva);
    let metadata_offset = align_usize(method_offset + method_body.len(), 4);
    let metadata_rva = SECTION_RVA + to_u32(metadata_offset);
    let section_virtual_size = metadata_offset + metadata.len();
    let section_raw_size = align_usize(section_virtual_size, FILE_ALIGNMENT);

    let mut section = vec![0_u8; section_raw_size];
    section[method_offset..method_offset + method_body.len()].copy_from_slice(&method_body);
    section[metadata_offset..metadata_offset + metadata.len()].copy_from_slice(&metadata);

    write_clr_header(
        &mut section[..CLR_HEADER_SIZE],
        metadata_rva,
        to_u32(metadata.len()),
    );

    let mut image = vec![0_u8; HEADERS_SIZE];
    write_pe_headers(
        &mut image,
        to_u32(section_virtual_size),
        to_u32(section_raw_size),
    );
    image.extend_from_slice(&section);
    image
}

#[must_use]
pub fn probe_runtime_config() -> String {
    concat!(
        "{\n",
        "  \"runtimeOptions\": {\n",
        "    \"tfm\": \"net10.0\",\n",
        "    \"framework\": {\n",
        "      \"name\": \"Microsoft.NETCore.App\",\n",
        "      \"version\": \"10.0.0\"\n",
        "    }\n",
        "  }\n",
        "}\n"
    )
    .to_owned()
}

pub fn write_probe_artifacts(directory: impl AsRef<Path>) -> io::Result<ProbePaths> {
    let directory = directory.as_ref();
    fs::create_dir_all(directory)?;

    let assembly = directory.join(PROBE_ASSEMBLY_FILE);
    let runtime_config = directory.join(PROBE_RUNTIME_CONFIG_FILE);

    fs::write(&assembly, emit_probe_assembly())?;
    fs::write(&runtime_config, probe_runtime_config())?;

    Ok(ProbePaths {
        assembly,
        runtime_config,
    })
}

fn build_main_method_body() -> Vec<u8> {
    let mut code = Vec::with_capacity(35);

    // Tiny method header: low bits 0b10 + code size in the upper six bits.
    const CODE_SIZE: u8 = 34;
    code.push((CODE_SIZE << 2) | 0b10);

    // ldstr "Hello FerrumWeave"
    code.push(0x72);
    push_u32(&mut code, USER_STRING_TOKEN_GREETING);

    // call void [System.Console]System.Console::WriteLine(string)
    code.push(0x28);
    push_u32(&mut code, MEMBER_REF_TOKEN_WRITELINE);

    // newobj instance void [System.Runtime]System.Object::.ctor()
    code.push(0x73);
    push_u32(&mut code, MEMBER_REF_TOKEN_OBJECT_CTOR);

    // Keep one object reference while dispatching the public instance method.
    code.push(0x25); // dup

    // callvirt instance string [System.Runtime]System.Object::ToString()
    code.push(0x6F);
    push_u32(&mut code, MEMBER_REF_TOKEN_OBJECT_TOSTRING);

    // Discard the returned string and the retained object reference.
    code.push(0x26);
    code.push(0x26);

    // Read then write the same portable managed property value.
    code.push(0x28);
    push_u32(
        &mut code,
        MEMBER_REF_TOKEN_ENVIRONMENT_GET_CURRENT_DIRECTORY,
    );
    code.push(0x28);
    push_u32(
        &mut code,
        MEMBER_REF_TOKEN_ENVIRONMENT_SET_CURRENT_DIRECTORY,
    );

    // ret
    code.push(0x2A);
    code
}

fn build_metadata(method_rva: u32) -> Vec<u8> {
    let mut strings = vec![0_u8];
    let module_name = push_string(&mut strings, PROBE_ASSEMBLY_FILE);
    let console_name = push_string(&mut strings, "Console");
    let int128_name = push_string(&mut strings, "Int128");
    let uint128_name = push_string(&mut strings, "UInt128");
    let object_name = push_string(&mut strings, "Object");
    let environment_name = push_string(&mut strings, "Environment");
    let system_namespace = push_string(&mut strings, "System");
    let module_type_name = push_string(&mut strings, "<Module>");
    let main_name = push_string(&mut strings, "Main");
    let probe_i32_name = push_string(&mut strings, "ProbeI32");
    let probe_boolean_name = push_string(&mut strings, "ProbeBoolean");
    let probe_string_name = push_string(&mut strings, "ProbeString");
    let probe_object_name = push_string(&mut strings, "ProbeObject");
    let probe_i32_array_name = push_string(&mut strings, "ProbeI32Array");
    let probe_int128_name = push_string(&mut strings, "ProbeInt128");
    let probe_uint128_name = push_string(&mut strings, "ProbeUInt128");
    let writeline_name = push_string(&mut strings, "WriteLine");
    let ctor_name = push_string(&mut strings, ".ctor");
    let tostring_name = push_string(&mut strings, "ToString");
    let get_current_directory_name = push_string(&mut strings, "get_CurrentDirectory");
    let set_current_directory_name = push_string(&mut strings, "set_CurrentDirectory");
    let assembly_name = push_string(&mut strings, PROBE_ASSEMBLY_NAME);
    let system_console_assembly_name = push_string(&mut strings, "System.Console");
    let system_runtime_assembly_name = push_string(&mut strings, "System.Runtime");
    pad_vec(&mut strings, 4);

    let mut user_strings = vec![0_u8];
    let greeting_offset = push_user_string(&mut user_strings, PROBE_GREETING);
    debug_assert_eq!(greeting_offset, 1);
    pad_vec(&mut user_strings, 4);

    // A deterministic MVID keeps the probe reproducible byte-for-byte.
    let guid = vec![
        0x46, 0x57, 0x52, 0x30, 0x31, 0x43, 0x49, 0x4C, 0x50, 0x52, 0x4F, 0x42, 0x45, 0x30, 0x30,
        0x31,
    ];

    let mut blobs = vec![0_u8];
    let main_signature = push_blob(&mut blobs, &[0x00, 0x00, 0x01]);
    let probe_i32_signature = push_blob(&mut blobs, &[0x00, 0x01, 0x01, 0x08]);
    let probe_boolean_signature = push_blob(&mut blobs, &[0x00, 0x01, 0x01, 0x02]);
    let probe_string_signature = push_blob(&mut blobs, &[0x00, 0x01, 0x01, 0x0E]);
    let probe_object_signature = push_blob(&mut blobs, &[0x00, 0x01, 0x01, 0x1C]);
    let probe_i32_array_signature = push_blob(&mut blobs, &[0x00, 0x01, 0x01, 0x1D, 0x08]);
    // ELEMENT_TYPE_VALUETYPE + TypeDefOrRef-coded TypeRef rows 2 and 3.
    let probe_int128_signature = push_blob(&mut blobs, &[0x00, 0x01, 0x01, 0x11, 0x09]);
    let probe_uint128_signature = push_blob(&mut blobs, &[0x00, 0x01, 0x01, 0x11, 0x0D]);
    let writeline_signature = push_blob(&mut blobs, &[0x00, 0x01, 0x01, 0x0E]);
    let object_ctor_signature = push_blob(&mut blobs, &[0x20, 0x00, 0x01]);
    let object_tostring_signature = push_blob(&mut blobs, &[0x20, 0x00, 0x0E]);
    let environment_get_current_directory_signature =
        push_blob(&mut blobs, &[0x00, 0x00, 0x0E]);
    let environment_set_current_directory_signature =
        push_blob(&mut blobs, &[0x00, 0x01, 0x01, 0x0E]);
    let system_public_key_token = push_blob(
        &mut blobs,
        &[0xB0, 0x3F, 0x5F, 0x7F, 0x11, 0xD5, 0x0A, 0x3A],
    );
    pad_vec(&mut blobs, 4);

    let mut tables = Vec::new();
    push_u32(&mut tables, 0); // Reserved.
    tables.extend_from_slice(&[2, 0, 0, 1]); // Major, minor, heap sizes, reserved.

    let valid_tables = (1_u64 << 0)
        | (1_u64 << 1)
        | (1_u64 << 2)
        | (1_u64 << 6)
        | (1_u64 << 10)
        | (1_u64 << 32)
        | (1_u64 << 35);
    push_u64(&mut tables, valid_tables);
    push_u64(&mut tables, 0); // Sorted mask.

    // Row counts, in table-id order for every set bit in the valid mask.
    for count in [1_u32, 5, 1, 8, 5, 1, 2] {
        push_u32(&mut tables, count);
    }

    // Module (0x00).
    push_u16(&mut tables, 0); // Generation.
    push_u16(&mut tables, module_name);
    push_u16(&mut tables, 1); // MVID GUID index.
    push_u16(&mut tables, 0); // EncId.
    push_u16(&mut tables, 0); // EncBaseId.

    // TypeRef row 1: [System.Console]System.Console.
    // ResolutionScope tag 2 is AssemblyRef; row 1 => (1 << 2) | 2 = 6.
    push_u16(&mut tables, 6);
    push_u16(&mut tables, console_name);
    push_u16(&mut tables, system_namespace);

    // TypeRef rows 2-3: [System.Runtime]System.Int128 / System.UInt128.
    // AssemblyRef row 2 => (2 << 2) | 2 = 10.
    for type_name in [int128_name, uint128_name] {
        push_u16(&mut tables, 10);
        push_u16(&mut tables, type_name);
        push_u16(&mut tables, system_namespace);
    }

    // TypeRef row 4: [System.Runtime]System.Object.
    push_u16(&mut tables, 10);
    push_u16(&mut tables, object_name);
    push_u16(&mut tables, system_namespace);

    // TypeRef row 5: [System.Runtime]System.Environment.
    push_u16(&mut tables, 10);
    push_u16(&mut tables, environment_name);
    push_u16(&mut tables, system_namespace);

    // TypeDef (0x02): the required global <Module> type.
    push_u32(&mut tables, 0);
    push_u16(&mut tables, module_type_name);
    push_u16(&mut tables, 0); // Namespace.
    push_u16(&mut tables, 0); // Extends.
    push_u16(&mut tables, 1); // First field (one-past-empty table).
    push_u16(&mut tables, 1); // First method.

    // MethodDef (0x06): public static void Main().
    push_u32(&mut tables, method_rva);
    push_u16(&mut tables, 0); // IL + managed are the zero/default implementation flags.
    push_u16(&mut tables, 0x0096); // Public | Static | HideBySig.
    push_u16(&mut tables, main_name);
    push_u16(&mut tables, main_signature);
    push_u16(&mut tables, 1); // First parameter (one-past-empty table).

    for (name, signature) in [
        (probe_i32_name, probe_i32_signature),
        (probe_boolean_name, probe_boolean_signature),
        (probe_string_name, probe_string_signature),
        (probe_object_name, probe_object_signature),
        (probe_i32_array_name, probe_i32_array_signature),
        (probe_int128_name, probe_int128_signature),
        (probe_uint128_name, probe_uint128_signature),
    ] {
        // Representative R04 CTS probes deliberately share the tiny method body.
        // They are reflection contracts, not executable product entry points.
        push_u32(&mut tables, method_rva);
        push_u16(&mut tables, 0);
        push_u16(&mut tables, 0x0096);
        push_u16(&mut tables, name);
        push_u16(&mut tables, signature);
        push_u16(&mut tables, 1);
    }

    // MemberRef row 1: System.Console.WriteLine(string).
    // MemberRefParent tag 1 is TypeRef; row 1 => (1 << 3) | 1 = 9.
    push_u16(&mut tables, 9);
    push_u16(&mut tables, writeline_name);
    push_u16(&mut tables, writeline_signature);

    // MemberRef row 2: System.Object::.ctor(). TypeRef row 4 => (4 << 3) | 1 = 33.
    push_u16(&mut tables, 33);
    push_u16(&mut tables, ctor_name);
    push_u16(&mut tables, object_ctor_signature);

    // MemberRef row 3: System.Object::ToString().
    push_u16(&mut tables, 33);
    push_u16(&mut tables, tostring_name);
    push_u16(&mut tables, object_tostring_signature);

    // MemberRef rows 4-5: System.Environment.CurrentDirectory accessors.
    // TypeRef row 5 => (5 << 3) | 1 = 41.
    push_u16(&mut tables, 41);
    push_u16(&mut tables, get_current_directory_name);
    push_u16(
        &mut tables,
        environment_get_current_directory_signature,
    );
    push_u16(&mut tables, 41);
    push_u16(&mut tables, set_current_directory_name);
    push_u16(
        &mut tables,
        environment_set_current_directory_signature,
    );

    // Assembly (0x20).
    push_u32(&mut tables, 0x0000_8004); // SHA-1, conventional ECMA-335 value.
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u32(&mut tables, 0); // Flags.
    push_u16(&mut tables, 0); // Public key blob.
    push_u16(&mut tables, assembly_name);
    push_u16(&mut tables, 0); // Culture.

    for assembly_ref_name in [system_console_assembly_name, system_runtime_assembly_name] {
        // AssemblyRef (0x23): .NET 10 framework assembly.
        push_u16(&mut tables, 10);
        push_u16(&mut tables, 0);
        push_u16(&mut tables, 0);
        push_u16(&mut tables, 0);
        push_u32(&mut tables, 0);
        push_u16(&mut tables, system_public_key_token);
        push_u16(&mut tables, assembly_ref_name);
        push_u16(&mut tables, 0); // Culture.
        push_u16(&mut tables, 0); // Hash value.
    }
    pad_vec(&mut tables, 4);

    let streams = [
        ("#~", tables),
        ("#Strings", strings),
        ("#US", user_strings),
        ("#GUID", guid),
        ("#Blob", blobs),
    ];

    let version = b"v4.0.30319\0\0";
    let fixed_header_size = 16 + version.len() + 4;
    let stream_headers_size: usize = streams
        .iter()
        .map(|(name, _)| 8 + align_usize(name.len() + 1, 4))
        .sum();
    let data_start = align_usize(fixed_header_size + stream_headers_size, 4);

    let mut offsets = Vec::with_capacity(streams.len());
    let mut next_offset = data_start;
    for (_, data) in &streams {
        offsets.push(next_offset);
        next_offset += data.len();
    }

    let mut metadata = Vec::with_capacity(next_offset);
    push_u32(&mut metadata, 0x424A_5342); // BSJB.
    push_u16(&mut metadata, 1);
    push_u16(&mut metadata, 1);
    push_u32(&mut metadata, 0);
    push_u32(&mut metadata, to_u32(version.len()));
    metadata.extend_from_slice(version);
    push_u16(&mut metadata, 0); // Flags.
    push_u16(
        &mut metadata,
        u16::try_from(streams.len()).expect("stream count fits u16"),
    );

    for ((name, data), offset) in streams.iter().zip(offsets.iter()) {
        push_u32(&mut metadata, to_u32(*offset));
        push_u32(&mut metadata, to_u32(data.len()));
        metadata.extend_from_slice(name.as_bytes());
        metadata.push(0);
        pad_vec(&mut metadata, 4);
    }

    metadata.resize(data_start, 0);
    for (_, data) in streams {
        metadata.extend_from_slice(&data);
    }

    metadata
}

fn write_clr_header(header: &mut [u8], metadata_rva: u32, metadata_size: u32) {
    debug_assert_eq!(header.len(), CLR_HEADER_SIZE);
    write_u32_at(header, 0x00, to_u32(CLR_HEADER_SIZE));
    write_u16_at(header, 0x04, 2);
    write_u16_at(header, 0x06, 5);
    write_u32_at(header, 0x08, metadata_rva);
    write_u32_at(header, 0x0C, metadata_size);
    write_u32_at(header, 0x10, 0x0000_0001); // COMIMAGE_FLAGS_ILONLY.
    write_u32_at(header, 0x14, METHOD_DEF_TOKEN_MAIN);
    // Remaining data directories are intentionally zero for the probe.
}

fn write_pe_headers(headers: &mut [u8], section_virtual_size: u32, section_raw_size: u32) {
    debug_assert_eq!(headers.len(), HEADERS_SIZE);

    headers[0..2].copy_from_slice(b"MZ");
    write_u32_at(headers, 0x3C, to_u32(PE_OFFSET));

    headers[PE_OFFSET..PE_OFFSET + 4].copy_from_slice(b"PE\0\0");
    let coff = PE_OFFSET + 4;
    write_u16_at(headers, coff, 0x014C); // IMAGE_FILE_MACHINE_I386 / AnyCPU convention.
    write_u16_at(headers, coff + 2, 1); // One .text section.
    write_u32_at(headers, coff + 4, 0); // Deterministic timestamp.
    write_u32_at(headers, coff + 8, 0);
    write_u32_at(headers, coff + 12, 0);
    write_u16_at(headers, coff + 16, to_u16(OPTIONAL_HEADER_SIZE));
    write_u16_at(headers, coff + 18, 0x2022); // Executable | large-address-aware | DLL.

    let optional = coff + 20;
    write_u16_at(headers, optional, 0x010B); // PE32.
    headers[optional + 2] = 0;
    headers[optional + 3] = 0;
    write_u32_at(headers, optional + 4, section_raw_size); // SizeOfCode.
    write_u32_at(headers, optional + 8, 0);
    write_u32_at(headers, optional + 12, 0);
    write_u32_at(headers, optional + 16, 0); // No native entry point.
    write_u32_at(headers, optional + 20, SECTION_RVA);
    write_u32_at(headers, optional + 24, 0);
    write_u32_at(headers, optional + 28, 0x0040_0000); // ImageBase.
    write_u32_at(headers, optional + 32, SECTION_ALIGNMENT);
    write_u32_at(headers, optional + 36, to_u32(FILE_ALIGNMENT));
    write_u16_at(headers, optional + 40, 4);
    write_u16_at(headers, optional + 42, 0);
    write_u16_at(headers, optional + 44, 0);
    write_u16_at(headers, optional + 46, 0);
    write_u16_at(headers, optional + 48, 4);
    write_u16_at(headers, optional + 50, 0);
    write_u32_at(headers, optional + 52, 0);

    let image_size = align_u32(SECTION_RVA + section_virtual_size, SECTION_ALIGNMENT);
    write_u32_at(headers, optional + 56, image_size);
    write_u32_at(headers, optional + 60, to_u32(HEADERS_SIZE));
    write_u32_at(headers, optional + 64, 0); // Checksum.
    write_u16_at(headers, optional + 68, 3); // Windows CUI; CLR ignores OS-specific UI details.
    write_u16_at(headers, optional + 70, 0x0100); // NX compatible; no relocations required.
    write_u32_at(headers, optional + 72, 0x0010_0000);
    write_u32_at(headers, optional + 76, 0x0000_1000);
    write_u32_at(headers, optional + 80, 0x0010_0000);
    write_u32_at(headers, optional + 84, 0x0000_1000);
    write_u32_at(headers, optional + 88, 0);
    write_u32_at(headers, optional + 92, 16);

    // IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR (index 14).
    let cli_directory = optional + 96 + (14 * 8);
    write_u32_at(headers, cli_directory, SECTION_RVA);
    write_u32_at(headers, cli_directory + 4, to_u32(CLR_HEADER_SIZE));

    let section = optional + OPTIONAL_HEADER_SIZE;
    headers[section..section + 8].copy_from_slice(b".text\0\0\0");
    write_u32_at(headers, section + 8, section_virtual_size);
    write_u32_at(headers, section + 12, SECTION_RVA);
    write_u32_at(headers, section + 16, section_raw_size);
    write_u32_at(headers, section + 20, to_u32(HEADERS_SIZE));
    write_u32_at(headers, section + 24, 0);
    write_u32_at(headers, section + 28, 0);
    write_u16_at(headers, section + 32, 0);
    write_u16_at(headers, section + 34, 0);
    write_u32_at(headers, section + 36, 0x6000_0020); // Code | execute | read.
}

fn push_string(heap: &mut Vec<u8>, value: &str) -> u16 {
    let index = to_u16(heap.len());
    heap.extend_from_slice(value.as_bytes());
    heap.push(0);
    index
}

fn push_blob(heap: &mut Vec<u8>, value: &[u8]) -> u16 {
    let index = to_u16(heap.len());
    push_compressed_unsigned(heap, to_u32(value.len()));
    heap.extend_from_slice(value);
    index
}

fn push_user_string(heap: &mut Vec<u8>, value: &str) -> u32 {
    let index = to_u32(heap.len());
    let utf16: Vec<u16> = value.encode_utf16().collect();
    let payload_size = utf16.len() * 2 + 1;
    push_compressed_unsigned(heap, to_u32(payload_size));
    for unit in utf16 {
        push_u16(heap, unit);
    }
    heap.push(0); // ECMA-335 terminal flag; ASCII probe needs no special handling.
    index
}

fn push_compressed_unsigned(buffer: &mut Vec<u8>, value: u32) {
    match value {
        0..=0x7F => buffer.push(u8::try_from(value).expect("7-bit value fits u8")),
        0x80..=0x3FFF => {
            buffer.push(u8::try_from((value >> 8) | 0x80).expect("14-bit prefix fits u8"));
            buffer.push(u8::try_from(value & 0xFF).expect("low byte fits u8"));
        }
        0x4000..=0x1FFF_FFFF => {
            buffer.push(u8::try_from((value >> 24) | 0xC0).expect("29-bit prefix fits u8"));
            buffer.push(u8::try_from((value >> 16) & 0xFF).expect("byte fits u8"));
            buffer.push(u8::try_from((value >> 8) & 0xFF).expect("byte fits u8"));
            buffer.push(u8::try_from(value & 0xFF).expect("byte fits u8"));
        }
        _ => panic!("ECMA-335 compressed unsigned integer is too large: {value}"),
    }
}

fn align_usize(value: usize, alignment: usize) -> usize {
    debug_assert!(alignment.is_power_of_two());
    (value + (alignment - 1)) & !(alignment - 1)
}

fn align_u32(value: u32, alignment: u32) -> u32 {
    debug_assert!(alignment.is_power_of_two());
    (value + (alignment - 1)) & !(alignment - 1)
}

fn pad_vec(buffer: &mut Vec<u8>, alignment: usize) {
    let target = align_usize(buffer.len(), alignment);
    buffer.resize(target, 0);
}

fn to_u16(value: usize) -> u16 {
    u16::try_from(value).expect("probe index fits u16")
}

fn to_u32(value: usize) -> u32 {
    u32::try_from(value).expect("probe size fits u32")
}

fn push_u16(buffer: &mut Vec<u8>, value: u16) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(buffer: &mut Vec<u8>, value: u32) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn push_u64(buffer: &mut Vec<u8>, value: u64) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn write_u16_at(buffer: &mut [u8], offset: usize, value: u16) {
    buffer[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn write_u32_at(buffer: &mut [u8], offset: usize, value: u32) {
    buffer[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_emission_is_deterministic() {
        assert_eq!(emit_probe_assembly(), emit_probe_assembly());
    }

    #[test]
    fn runtime_config_targets_dotnet_10_lts() {
        let config = probe_runtime_config();
        assert!(config.contains("\"tfm\": \"net10.0\""));
        assert!(config.contains("\"version\": \"10.0.0\""));
    }

    #[test]
    fn compressed_unsigned_encoding_covers_ecma_widths() {
        let mut bytes = Vec::new();
        push_compressed_unsigned(&mut bytes, 0x7F);
        push_compressed_unsigned(&mut bytes, 0x80);
        push_compressed_unsigned(&mut bytes, 0x4000);
        assert_eq!(bytes, [0x7F, 0x80, 0x80, 0xC0, 0x00, 0x40, 0x00]);
    }

    #[test]
    #[should_panic(expected = "compressed unsigned integer is too large")]
    fn compressed_unsigned_encoding_rejects_out_of_range_values() {
        let mut bytes = Vec::new();
        push_compressed_unsigned(&mut bytes, 0x2000_0000);
    }

    #[test]
    fn emitted_image_contains_expected_managed_names() {
        let image = emit_probe_assembly();
        assert!(
            image
                .windows(PROBE_ASSEMBLY_NAME.len())
                .any(|window| window == PROBE_ASSEMBLY_NAME.as_bytes())
        );
        assert!(
            image
                .windows("System.Console".len())
                .any(|window| window == b"System.Console")
        );
        assert!(
            image
                .windows("Object".len())
                .any(|window| window == b"Object")
        );
        assert!(
            image
                .windows("ToString".len())
                .any(|window| window == b"ToString")
        );
        assert!(
            image
                .windows("Environment".len())
                .any(|window| window == b"Environment")
        );
    }
}
