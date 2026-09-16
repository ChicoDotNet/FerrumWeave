#![forbid(unsafe_code)]

//! Managed executable emission for the first Rust `main` -> CLR entry-point slice.
//!
//! The source-language contract remains owned by rustc/codegen lowering. This
//! module owns only the CLR projection: a public static `<RootNamespace>.Program.Main`
//! entry adapter calls a compiler-owned private Rust-main method, while optional
//! i32 exports remain available through the already-established managed API surface.

use crate::I32ConstantExport;

const PE_OFFSET: usize = 0x80;
const OPTIONAL_HEADER_SIZE: usize = 0xE0;
const HEADERS_SIZE: usize = 0x200;
const FILE_ALIGNMENT: usize = 0x200;
const SECTION_ALIGNMENT: u32 = 0x2000;
const SECTION_RVA: u32 = 0x2000;
const CLR_HEADER_SIZE: usize = 0x48;
const METHOD_DEF_TOKEN_MAIN: u32 = 0x0600_0001;
const METHOD_DEF_TOKEN_RUST_MAIN: u32 = 0x0600_0002;

#[must_use]
pub fn emit_managed_executable_with_i32_constant_exports(
    assembly_name: &str,
    root_namespace: &str,
    api_namespace: &str,
    api_type_name: &str,
    exports: &[I32ConstantExport<'_>],
) -> Vec<u8> {
    let main_body = build_main_adapter_body();
    let rust_main_body = build_rust_main_body();
    let export_bodies: Vec<Vec<u8>> = exports
        .iter()
        .map(|export| build_constant_body(export.value))
        .collect();

    let mut method_rvas = Vec::with_capacity(2 + exports.len());
    let mut next_offset = CLR_HEADER_SIZE;
    for body in std::iter::once(&main_body)
        .chain(std::iter::once(&rust_main_body))
        .chain(export_bodies.iter())
    {
        method_rvas.push(SECTION_RVA + to_u32(next_offset));
        next_offset = align_usize(next_offset + body.len(), 4);
    }

    let metadata = build_metadata(
        &method_rvas,
        assembly_name,
        root_namespace,
        api_namespace,
        api_type_name,
        exports,
    );
    let metadata_offset = next_offset;
    let metadata_rva = SECTION_RVA + to_u32(metadata_offset);
    let section_virtual_size = metadata_offset + metadata.len();
    let section_raw_size = align_usize(section_virtual_size, FILE_ALIGNMENT);

    let mut section = vec![0_u8; section_raw_size];
    let mut offset = CLR_HEADER_SIZE;
    for body in std::iter::once(&main_body)
        .chain(std::iter::once(&rust_main_body))
        .chain(export_bodies.iter())
    {
        section[offset..offset + body.len()].copy_from_slice(body);
        offset = align_usize(offset + body.len(), 4);
    }
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

fn build_main_adapter_body() -> Vec<u8> {
    // call void <RootNamespace>.Program::__FerrumWeaveRustMain(); ret
    let mut body = vec![(6 << 2) | 0b10, 0x28];
    body.extend_from_slice(&METHOD_DEF_TOKEN_RUST_MAIN.to_le_bytes());
    body.push(0x2A);
    body
}

fn build_rust_main_body() -> Vec<u8> {
    // R10's first executable slice accepts only a source-causal no-op Rust main.
    // Richer main bodies are rejected by codegen lowering until their semantics
    // have an explicit executable contract.
    vec![(1 << 2) | 0b10, 0x2A]
}

fn build_constant_body(value: i32) -> Vec<u8> {
    let mut body = vec![(6 << 2) | 0b10, 0x20];
    body.extend_from_slice(&value.to_le_bytes());
    body.push(0x2A);
    body
}

fn build_metadata(
    method_rvas: &[u32],
    assembly_identity: &str,
    root_namespace_value: &str,
    api_namespace_value: &str,
    api_type_name_value: &str,
    exports: &[I32ConstantExport<'_>],
) -> Vec<u8> {
    debug_assert_eq!(method_rvas.len(), 2 + exports.len());

    let mut strings = vec![0_u8];
    let module_name = push_string(&mut strings, &format!("{assembly_identity}.dll"));
    let object_name = push_string(&mut strings, "Object");
    let system_namespace = push_string(&mut strings, "System");
    let module_type_name = push_string(&mut strings, "<Module>");
    let program_type_name = push_string(&mut strings, "Program");
    let root_namespace = push_string(&mut strings, root_namespace_value);
    let main_name = push_string(&mut strings, "Main");
    let rust_main_name = push_string(&mut strings, "__FerrumWeaveRustMain");
    let api_type_name = push_string(&mut strings, api_type_name_value);
    let api_namespace = push_string(&mut strings, api_namespace_value);
    let export_names: Vec<u16> = exports
        .iter()
        .map(|export| push_string(&mut strings, export.method_name))
        .collect();
    let assembly_name = push_string(&mut strings, assembly_identity);
    let system_runtime_name = push_string(&mut strings, "System.Runtime");
    pad_vec(&mut strings, 4);

    let guid = vec![
        0x46, 0x57, 0x52, 0x31, 0x30, 0x45, 0x4E, 0x54, 0x52, 0x59, 0x30, 0x30, 0x30, 0x30,
        0x30, 0x31,
    ];

    let mut blobs = vec![0_u8];
    let void_signature = push_blob(&mut blobs, &[0x00, 0x00, 0x01]);
    let i32_signature = push_blob(&mut blobs, &[0x00, 0x00, 0x08]);
    let system_public_key_token = push_blob(
        &mut blobs,
        &[0xB0, 0x3F, 0x5F, 0x7F, 0x11, 0xD5, 0x0A, 0x3A],
    );
    pad_vec(&mut blobs, 4);

    let mut tables = Vec::new();
    push_u32(&mut tables, 0);
    tables.extend_from_slice(&[2, 0, 0, 1]);
    let valid_tables =
        (1_u64 << 0) | (1_u64 << 1) | (1_u64 << 2) | (1_u64 << 6) | (1_u64 << 32) | (1_u64 << 35);
    push_u64(&mut tables, valid_tables);
    push_u64(&mut tables, 0);
    for count in [
        1_u32,
        1,
        3,
        u32::try_from(2 + exports.len()).expect("method count fits u32"),
        1,
        1,
    ] {
        push_u32(&mut tables, count);
    }

    // Module.
    push_u16(&mut tables, 0);
    push_u16(&mut tables, module_name);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);

    // TypeRef row 1: [System.Runtime]System.Object.
    push_u16(&mut tables, 6);
    push_u16(&mut tables, object_name);
    push_u16(&mut tables, system_namespace);

    // TypeDef row 1: global <Module>, with no owned methods.
    push_u32(&mut tables, 0);
    push_u16(&mut tables, module_type_name);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 1);

    // TypeDef row 2: public <RootNamespace>.Program : System.Object.
    push_u32(&mut tables, 0x0010_0001);
    push_u16(&mut tables, program_type_name);
    push_u16(&mut tables, root_namespace);
    push_u16(&mut tables, 5);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 1);

    // TypeDef row 3: public FerrumWeave.RustApi : System.Object.
    // MethodList row 3 means Program owns Main + __FerrumWeaveRustMain.
    push_u32(&mut tables, 0x0010_0001);
    push_u16(&mut tables, api_type_name);
    push_u16(&mut tables, api_namespace);
    push_u16(&mut tables, 5);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 3);

    // MethodDef row 1: public static void Program.Main().
    push_method_def(
        &mut tables,
        method_rvas[0],
        0x0096,
        main_name,
        void_signature,
    );

    // MethodDef row 2: private static void Program.__FerrumWeaveRustMain().
    push_method_def(
        &mut tables,
        method_rvas[1],
        0x0091,
        rust_main_name,
        void_signature,
    );

    for ((rva, name), _export) in method_rvas[2..]
        .iter()
        .zip(export_names.iter())
        .zip(exports.iter())
    {
        push_method_def(&mut tables, *rva, 0x0096, *name, i32_signature);
    }

    // Assembly.
    push_u32(&mut tables, 0x0000_8004);
    for value in [1_u16, 0, 0, 0] {
        push_u16(&mut tables, value);
    }
    push_u32(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, assembly_name);
    push_u16(&mut tables, 0);

    // AssemblyRef: System.Runtime.
    for value in [10_u16, 0, 0, 0] {
        push_u16(&mut tables, value);
    }
    push_u32(&mut tables, 0);
    push_u16(&mut tables, system_public_key_token);
    push_u16(&mut tables, system_runtime_name);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
    pad_vec(&mut tables, 4);

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
    push_u32(&mut metadata, 0x424A_5342);
    push_u16(&mut metadata, 1);
    push_u16(&mut metadata, 1);
    push_u32(&mut metadata, 0);
    push_u32(&mut metadata, to_u32(version.len()));
    metadata.extend_from_slice(version);
    push_u16(&mut metadata, 0);
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

fn push_method_def(
    tables: &mut Vec<u8>,
    rva: u32,
    flags: u16,
    name: u16,
    signature: u16,
) {
    push_u32(tables, rva);
    push_u16(tables, 0);
    push_u16(tables, flags);
    push_u16(tables, name);
    push_u16(tables, signature);
    push_u16(tables, 1);
}

fn write_clr_header(header: &mut [u8], metadata_rva: u32, metadata_size: u32) {
    write_u32_at(header, 0, to_u32(CLR_HEADER_SIZE));
    write_u16_at(header, 4, 2);
    write_u16_at(header, 6, 5);
    write_u32_at(header, 8, metadata_rva);
    write_u32_at(header, 12, metadata_size);
    write_u32_at(header, 16, 1); // COMIMAGE_FLAGS_ILONLY.
    write_u32_at(header, 20, METHOD_DEF_TOKEN_MAIN);
}

fn write_pe_headers(headers: &mut [u8], section_virtual_size: u32, section_raw_size: u32) {
    headers[0..2].copy_from_slice(b"MZ");
    write_u32_at(headers, 0x3C, to_u32(PE_OFFSET));
    headers[PE_OFFSET..PE_OFFSET + 4].copy_from_slice(b"PE\0\0");
    let coff = PE_OFFSET + 4;
    write_u16_at(headers, coff, 0x014C);
    write_u16_at(headers, coff + 2, 1);
    write_u16_at(headers, coff + 16, to_u16(OPTIONAL_HEADER_SIZE));
    write_u16_at(headers, coff + 18, 0x2022);

    let optional = coff + 20;
    write_u16_at(headers, optional, 0x010B);
    write_u32_at(headers, optional + 4, section_raw_size);
    write_u32_at(headers, optional + 20, SECTION_RVA);
    write_u32_at(headers, optional + 28, 0x0040_0000);
    write_u32_at(headers, optional + 32, SECTION_ALIGNMENT);
    write_u32_at(headers, optional + 36, to_u32(FILE_ALIGNMENT));
    write_u16_at(headers, optional + 40, 4);
    write_u16_at(headers, optional + 48, 4);
    write_u32_at(
        headers,
        optional + 56,
        align_u32(SECTION_RVA + section_virtual_size, SECTION_ALIGNMENT),
    );
    write_u32_at(headers, optional + 60, to_u32(HEADERS_SIZE));
    write_u16_at(headers, optional + 68, 3);
    write_u16_at(headers, optional + 70, 0x0100);
    write_u32_at(headers, optional + 72, 0x0010_0000);
    write_u32_at(headers, optional + 76, 0x1000);
    write_u32_at(headers, optional + 80, 0x0010_0000);
    write_u32_at(headers, optional + 84, 0x1000);
    write_u32_at(headers, optional + 92, 16);

    let cli = optional + 96 + 14 * 8;
    write_u32_at(headers, cli, SECTION_RVA);
    write_u32_at(headers, cli + 4, to_u32(CLR_HEADER_SIZE));

    let section = optional + OPTIONAL_HEADER_SIZE;
    headers[section..section + 8].copy_from_slice(b".text\0\0\0");
    write_u32_at(headers, section + 8, section_virtual_size);
    write_u32_at(headers, section + 12, SECTION_RVA);
    write_u32_at(headers, section + 16, section_raw_size);
    write_u32_at(headers, section + 20, to_u32(HEADERS_SIZE));
    write_u32_at(headers, section + 36, 0x6000_0020);
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

fn push_compressed_unsigned(buffer: &mut Vec<u8>, value: u32) {
    match value {
        0..=0x7F => buffer.push(value as u8),
        0x80..=0x3FFF => {
            buffer.push(((value >> 8) | 0x80) as u8);
            buffer.push((value & 0xFF) as u8);
        }
        _ => panic!("executable export only needs compact metadata values"),
    }
}

fn align_usize(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

fn align_u32(value: u32, alignment: u32) -> u32 {
    (value + alignment - 1) & !(alignment - 1)
}

fn pad_vec(buffer: &mut Vec<u8>, alignment: usize) {
    buffer.resize(align_usize(buffer.len(), alignment), 0)
}

fn to_u16(value: usize) -> u16 {
    u16::try_from(value).expect("metadata index fits u16")
}

fn to_u32(value: usize) -> u32 {
    u32::try_from(value).expect("image size fits u32")
}

fn push_u16(buffer: &mut Vec<u8>, value: u16) {
    buffer.extend_from_slice(&value.to_le_bytes())
}

fn push_u32(buffer: &mut Vec<u8>, value: u32) {
    buffer.extend_from_slice(&value.to_le_bytes())
}

fn push_u64(buffer: &mut Vec<u8>, value: u64) {
    buffer.extend_from_slice(&value.to_le_bytes())
}

fn write_u16_at(buffer: &mut [u8], offset: usize, value: u16) {
    buffer[offset..offset + 2].copy_from_slice(&value.to_le_bytes())
}

fn write_u32_at(buffer: &mut [u8], offset: usize, value: u32) {
    buffer[offset..offset + 4].copy_from_slice(&value.to_le_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executable_has_program_main_and_cli_entry_token() {
        let exports = [I32ConstantExport {
            method_name: "Answer",
            value: 42,
        }];
        let image = emit_managed_executable_with_i32_constant_exports(
            "HelloFerrum",
            "HelloFerrum",
            "FerrumWeave",
            "RustApi",
            &exports,
        );

        assert!(image.windows("HelloFerrum".len()).any(|window| window == b"HelloFerrum"));
        assert!(image.windows("Program".len()).any(|window| window == b"Program"));
        assert!(image.windows("Main".len()).any(|window| window == b"Main"));
        assert!(image.windows("Answer".len()).any(|window| window == b"Answer"));

        let cli_entry_offset = HEADERS_SIZE + 0x14;
        assert_eq!(
            u32::from_le_bytes(
                image[cli_entry_offset..cli_entry_offset + 4]
                    .try_into()
                    .expect("entry token is four bytes")
            ),
            METHOD_DEF_TOKEN_MAIN
        );
    }

    #[test]
    fn executable_can_exist_without_public_api_exports() {
        let image = emit_managed_executable_with_i32_constant_exports(
            "HelloFerrum",
            "HelloFerrum",
            "FerrumWeave",
            "RustApi",
            &[],
        );
        assert!(image.windows("Program".len()).any(|window| window == b"Program"));
        assert!(image.windows("__FerrumWeaveRustMain".len()).any(|window| {
            window == b"__FerrumWeaveRustMain"
        }));
    }
}
