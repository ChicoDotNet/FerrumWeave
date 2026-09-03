#![forbid(unsafe_code)]

#[path = "lib.rs"]
mod base;
pub use base::*;

const EXTERNAL_PE_OFFSET: usize = 0x80;
const EXTERNAL_OPTIONAL_HEADER_SIZE: usize = 0xE0;
const EXTERNAL_HEADERS_SIZE: usize = 0x200;
const EXTERNAL_FILE_ALIGNMENT: usize = 0x200;
const EXTERNAL_SECTION_ALIGNMENT: u32 = 0x2000;
const EXTERNAL_SECTION_RVA: u32 = 0x2000;
const EXTERNAL_CLR_HEADER_SIZE: usize = 0x48;
const EXTERNAL_METHOD_DEF_TOKEN_MAIN: u32 = 0x0600_0001;
const EXTERNAL_MEMBER_REF_TOKEN_CALL: u32 = 0x0A00_0001;
const EXTERNAL_MEMBER_REF_TOKEN_WRITELINE: u32 = 0x0A00_0002;

/// Emits a minimal managed executable that calls a public static string-returning
/// method from an independently compiled managed assembly and writes its result
/// through `System.Console.WriteLine`.
///
/// The emitted image contains real ECMA-335 `AssemblyRef`, `TypeRef`, and
/// `MemberRef` rows for the external dependency. No native interop or P/Invoke is
/// involved; CoreCLR resolves the managed reference when the probe executes.
#[must_use]
pub fn emit_probe_assembly_with_external_static_call(
    assembly_name: &str,
    namespace: &str,
    type_name: &str,
    method_name: &str,
) -> Vec<u8> {
    let method_body = build_external_main_method_body();
    let method_offset = EXTERNAL_CLR_HEADER_SIZE;
    let method_rva = EXTERNAL_SECTION_RVA + external_to_u32(method_offset);

    let metadata =
        build_external_metadata(method_rva, assembly_name, namespace, type_name, method_name);
    let metadata_offset = external_align_usize(method_offset + method_body.len(), 4);
    let metadata_rva = EXTERNAL_SECTION_RVA + external_to_u32(metadata_offset);
    let section_virtual_size = metadata_offset + metadata.len();
    let section_raw_size = external_align_usize(section_virtual_size, EXTERNAL_FILE_ALIGNMENT);

    let mut section = vec![0_u8; section_raw_size];
    section[method_offset..method_offset + method_body.len()].copy_from_slice(&method_body);
    section[metadata_offset..metadata_offset + metadata.len()].copy_from_slice(&metadata);

    write_external_clr_header(
        &mut section[..EXTERNAL_CLR_HEADER_SIZE],
        metadata_rva,
        external_to_u32(metadata.len()),
    );

    let mut image = vec![0_u8; EXTERNAL_HEADERS_SIZE];
    write_external_pe_headers(
        &mut image,
        external_to_u32(section_virtual_size),
        external_to_u32(section_raw_size),
    );
    image.extend_from_slice(&section);
    image
}

fn build_external_main_method_body() -> Vec<u8> {
    const CODE_SIZE: u8 = 11;
    let mut code = Vec::with_capacity(usize::from(CODE_SIZE) + 1);
    code.push((CODE_SIZE << 2) | 0b10);

    // call string [ExternalAssembly]Namespace.Type::Method()
    code.push(0x28);
    external_push_u32(&mut code, EXTERNAL_MEMBER_REF_TOKEN_CALL);

    // call void [System.Console]System.Console::WriteLine(string)
    code.push(0x28);
    external_push_u32(&mut code, EXTERNAL_MEMBER_REF_TOKEN_WRITELINE);

    code.push(0x2A); // ret
    code
}

fn build_external_metadata(
    method_rva: u32,
    external_assembly_name: &str,
    external_namespace: &str,
    external_type_name: &str,
    external_method_name: &str,
) -> Vec<u8> {
    let mut strings = vec![0_u8];
    let module_name = external_push_string(&mut strings, PROBE_ASSEMBLY_FILE);
    let console_name = external_push_string(&mut strings, "Console");
    let system_namespace = external_push_string(&mut strings, "System");
    let user_type_name = external_push_string(&mut strings, external_type_name);
    let user_namespace = external_push_string(&mut strings, external_namespace);
    let module_type_name = external_push_string(&mut strings, "<Module>");
    let main_name = external_push_string(&mut strings, "Main");
    let external_method_name = external_push_string(&mut strings, external_method_name);
    let writeline_name = external_push_string(&mut strings, "WriteLine");
    let probe_assembly_name = external_push_string(&mut strings, PROBE_ASSEMBLY_NAME);
    let system_console_assembly_name = external_push_string(&mut strings, "System.Console");
    let external_assembly_name = external_push_string(&mut strings, external_assembly_name);
    external_pad_vec(&mut strings, 4);

    let guid = vec![
        0x46, 0x57, 0x52, 0x30, 0x35, 0x45, 0x58, 0x54, 0x43, 0x41, 0x4C, 0x4C, 0x30, 0x30, 0x30,
        0x31,
    ];

    let mut blobs = vec![0_u8];
    let main_signature = external_push_blob(&mut blobs, &[0x00, 0x00, 0x01]);
    let external_call_signature = external_push_blob(&mut blobs, &[0x00, 0x00, 0x0E]);
    let writeline_signature = external_push_blob(&mut blobs, &[0x00, 0x01, 0x01, 0x0E]);
    let system_public_key_token = external_push_blob(
        &mut blobs,
        &[0xB0, 0x3F, 0x5F, 0x7F, 0x11, 0xD5, 0x0A, 0x3A],
    );
    external_pad_vec(&mut blobs, 4);

    let mut tables = Vec::new();
    external_push_u32(&mut tables, 0);
    tables.extend_from_slice(&[2, 0, 0, 1]);

    let valid_tables = (1_u64 << 0)
        | (1_u64 << 1)
        | (1_u64 << 2)
        | (1_u64 << 6)
        | (1_u64 << 10)
        | (1_u64 << 32)
        | (1_u64 << 35);
    external_push_u64(&mut tables, valid_tables);
    external_push_u64(&mut tables, 0);

    for count in [1_u32, 2, 1, 1, 2, 1, 2] {
        external_push_u32(&mut tables, count);
    }

    // Module (0x00).
    external_push_u16(&mut tables, 0);
    external_push_u16(&mut tables, module_name);
    external_push_u16(&mut tables, 1);
    external_push_u16(&mut tables, 0);
    external_push_u16(&mut tables, 0);

    // TypeRef row 1: [System.Console]System.Console.
    external_push_u16(&mut tables, 6); // AssemblyRef row 1, ResolutionScope tag 2.
    external_push_u16(&mut tables, console_name);
    external_push_u16(&mut tables, system_namespace);

    // TypeRef row 2: [ExternalAssembly]Namespace.Type.
    external_push_u16(&mut tables, 10); // AssemblyRef row 2, ResolutionScope tag 2.
    external_push_u16(&mut tables, user_type_name);
    external_push_u16(&mut tables, user_namespace);

    // TypeDef (0x02): global <Module>.
    external_push_u32(&mut tables, 0);
    external_push_u16(&mut tables, module_type_name);
    external_push_u16(&mut tables, 0);
    external_push_u16(&mut tables, 0);
    external_push_u16(&mut tables, 1);
    external_push_u16(&mut tables, 1);

    // MethodDef (0x06): public static void Main().
    external_push_u32(&mut tables, method_rva);
    external_push_u16(&mut tables, 0);
    external_push_u16(&mut tables, 0x0096);
    external_push_u16(&mut tables, main_name);
    external_push_u16(&mut tables, main_signature);
    external_push_u16(&mut tables, 1);

    // MemberRef row 1: external static string method(). TypeRef row 2.
    external_push_u16(&mut tables, 17); // (2 << 3) | TypeRef tag 1.
    external_push_u16(&mut tables, external_method_name);
    external_push_u16(&mut tables, external_call_signature);

    // MemberRef row 2: System.Console.WriteLine(string). TypeRef row 1.
    external_push_u16(&mut tables, 9); // (1 << 3) | TypeRef tag 1.
    external_push_u16(&mut tables, writeline_name);
    external_push_u16(&mut tables, writeline_signature);

    // Assembly (0x20): FerrumWeave.Probe, version 1.0.0.0.
    external_push_u32(&mut tables, 0x0000_8004);
    external_push_u16(&mut tables, 1);
    external_push_u16(&mut tables, 0);
    external_push_u16(&mut tables, 0);
    external_push_u16(&mut tables, 0);
    external_push_u32(&mut tables, 0);
    external_push_u16(&mut tables, 0);
    external_push_u16(&mut tables, probe_assembly_name);
    external_push_u16(&mut tables, 0);

    // AssemblyRef row 1: System.Console, .NET 10 framework assembly.
    external_push_u16(&mut tables, 10);
    external_push_u16(&mut tables, 0);
    external_push_u16(&mut tables, 0);
    external_push_u16(&mut tables, 0);
    external_push_u32(&mut tables, 0);
    external_push_u16(&mut tables, system_public_key_token);
    external_push_u16(&mut tables, system_console_assembly_name);
    external_push_u16(&mut tables, 0);
    external_push_u16(&mut tables, 0);

    // AssemblyRef row 2: independently compiled fixture, default 1.0.0.0.
    external_push_u16(&mut tables, 1);
    external_push_u16(&mut tables, 0);
    external_push_u16(&mut tables, 0);
    external_push_u16(&mut tables, 0);
    external_push_u32(&mut tables, 0);
    external_push_u16(&mut tables, 0);
    external_push_u16(&mut tables, external_assembly_name);
    external_push_u16(&mut tables, 0);
    external_push_u16(&mut tables, 0);
    external_pad_vec(&mut tables, 4);

    let streams = [
        ("#~", tables),
        ("#Strings", strings),
        ("#GUID", guid),
        ("#Blob", blobs),
    ];

    let version = b"v4.0.30319\0\0";
    let fixed_header_size = 16 + version.len() + 4;
    let stream_headers_size: usize = streams
        .iter()
        .map(|(name, _)| 8 + external_align_usize(name.len() + 1, 4))
        .sum();
    let data_start = external_align_usize(fixed_header_size + stream_headers_size, 4);

    let mut offsets = Vec::with_capacity(streams.len());
    let mut next_offset = data_start;
    for (_, data) in &streams {
        offsets.push(next_offset);
        next_offset += data.len();
    }

    let mut metadata = Vec::with_capacity(next_offset);
    external_push_u32(&mut metadata, 0x424A_5342);
    external_push_u16(&mut metadata, 1);
    external_push_u16(&mut metadata, 1);
    external_push_u32(&mut metadata, 0);
    external_push_u32(&mut metadata, external_to_u32(version.len()));
    metadata.extend_from_slice(version);
    external_push_u16(&mut metadata, 0);
    external_push_u16(
        &mut metadata,
        u16::try_from(streams.len()).expect("stream count fits u16"),
    );

    for ((name, data), offset) in streams.iter().zip(offsets.iter()) {
        external_push_u32(&mut metadata, external_to_u32(*offset));
        external_push_u32(&mut metadata, external_to_u32(data.len()));
        metadata.extend_from_slice(name.as_bytes());
        metadata.push(0);
        external_pad_vec(&mut metadata, 4);
    }

    metadata.resize(data_start, 0);
    for (_, data) in streams {
        metadata.extend_from_slice(&data);
    }
    metadata
}

fn write_external_clr_header(header: &mut [u8], metadata_rva: u32, metadata_size: u32) {
    external_write_u32_at(header, 0x00, external_to_u32(EXTERNAL_CLR_HEADER_SIZE));
    external_write_u16_at(header, 0x04, 2);
    external_write_u16_at(header, 0x06, 5);
    external_write_u32_at(header, 0x08, metadata_rva);
    external_write_u32_at(header, 0x0C, metadata_size);
    external_write_u32_at(header, 0x10, 0x0000_0001);
    external_write_u32_at(header, 0x14, EXTERNAL_METHOD_DEF_TOKEN_MAIN);
}

fn write_external_pe_headers(headers: &mut [u8], section_virtual_size: u32, section_raw_size: u32) {
    headers[0..2].copy_from_slice(b"MZ");
    external_write_u32_at(headers, 0x3C, external_to_u32(EXTERNAL_PE_OFFSET));

    headers[EXTERNAL_PE_OFFSET..EXTERNAL_PE_OFFSET + 4].copy_from_slice(b"PE\0\0");
    let coff = EXTERNAL_PE_OFFSET + 4;
    external_write_u16_at(headers, coff, 0x014C);
    external_write_u16_at(headers, coff + 2, 1);
    external_write_u32_at(headers, coff + 4, 0);
    external_write_u32_at(headers, coff + 8, 0);
    external_write_u32_at(headers, coff + 12, 0);
    external_write_u16_at(
        headers,
        coff + 16,
        external_to_u16(EXTERNAL_OPTIONAL_HEADER_SIZE),
    );
    external_write_u16_at(headers, coff + 18, 0x2022);

    let optional = coff + 20;
    external_write_u16_at(headers, optional, 0x010B);
    external_write_u32_at(headers, optional + 4, section_raw_size);
    external_write_u32_at(headers, optional + 20, EXTERNAL_SECTION_RVA);
    external_write_u32_at(headers, optional + 28, 0x0040_0000);
    external_write_u32_at(headers, optional + 32, EXTERNAL_SECTION_ALIGNMENT);
    external_write_u32_at(
        headers,
        optional + 36,
        external_to_u32(EXTERNAL_FILE_ALIGNMENT),
    );
    external_write_u16_at(headers, optional + 40, 4);
    external_write_u16_at(headers, optional + 48, 4);

    let image_size = external_align_u32(
        EXTERNAL_SECTION_RVA + section_virtual_size,
        EXTERNAL_SECTION_ALIGNMENT,
    );
    external_write_u32_at(headers, optional + 56, image_size);
    external_write_u32_at(
        headers,
        optional + 60,
        external_to_u32(EXTERNAL_HEADERS_SIZE),
    );
    external_write_u16_at(headers, optional + 68, 3);
    external_write_u16_at(headers, optional + 70, 0x0100);
    external_write_u32_at(headers, optional + 72, 0x0010_0000);
    external_write_u32_at(headers, optional + 76, 0x0000_1000);
    external_write_u32_at(headers, optional + 80, 0x0010_0000);
    external_write_u32_at(headers, optional + 84, 0x0000_1000);
    external_write_u32_at(headers, optional + 92, 16);

    let cli_directory = optional + 96 + (14 * 8);
    external_write_u32_at(headers, cli_directory, EXTERNAL_SECTION_RVA);
    external_write_u32_at(
        headers,
        cli_directory + 4,
        external_to_u32(EXTERNAL_CLR_HEADER_SIZE),
    );

    let section = optional + EXTERNAL_OPTIONAL_HEADER_SIZE;
    headers[section..section + 8].copy_from_slice(b".text\0\0\0");
    external_write_u32_at(headers, section + 8, section_virtual_size);
    external_write_u32_at(headers, section + 12, EXTERNAL_SECTION_RVA);
    external_write_u32_at(headers, section + 16, section_raw_size);
    external_write_u32_at(
        headers,
        section + 20,
        external_to_u32(EXTERNAL_HEADERS_SIZE),
    );
    external_write_u32_at(headers, section + 36, 0x6000_0020);
}

fn external_push_string(heap: &mut Vec<u8>, value: &str) -> u16 {
    let index = external_to_u16(heap.len());
    heap.extend_from_slice(value.as_bytes());
    heap.push(0);
    index
}

fn external_push_blob(heap: &mut Vec<u8>, value: &[u8]) -> u16 {
    let index = external_to_u16(heap.len());
    external_push_compressed_unsigned(heap, external_to_u32(value.len()));
    heap.extend_from_slice(value);
    index
}

fn external_push_compressed_unsigned(buffer: &mut Vec<u8>, value: u32) {
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

fn external_align_usize(value: usize, alignment: usize) -> usize {
    (value + (alignment - 1)) & !(alignment - 1)
}

fn external_align_u32(value: u32, alignment: u32) -> u32 {
    (value + (alignment - 1)) & !(alignment - 1)
}

fn external_pad_vec(buffer: &mut Vec<u8>, alignment: usize) {
    let target = external_align_usize(buffer.len(), alignment);
    buffer.resize(target, 0);
}

fn external_to_u16(value: usize) -> u16 {
    u16::try_from(value).expect("external probe index fits u16")
}

fn external_to_u32(value: usize) -> u32 {
    u32::try_from(value).expect("external probe size fits u32")
}

fn external_push_u16(buffer: &mut Vec<u8>, value: u16) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn external_push_u32(buffer: &mut Vec<u8>, value: u32) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn external_push_u64(buffer: &mut Vec<u8>, value: u64) {
    buffer.extend_from_slice(&value.to_le_bytes());
}

fn external_write_u16_at(buffer: &mut [u8], offset: usize, value: u16) {
    buffer[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn external_write_u32_at(buffer: &mut [u8], offset: usize, value: u32) {
    buffer[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}