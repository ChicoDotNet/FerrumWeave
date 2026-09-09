#![forbid(unsafe_code)]

//! Managed construction emission for FerrumWeave-owned MIR lowering.
//!
//! The compiler layer supplies the selected constructor and payload. This module
//! only translates that lowered operation into ECMA-335 metadata and IL.

const PE_OFFSET: usize = 0x80;
const OPTIONAL_HEADER_SIZE: usize = 0xE0;
const HEADERS_SIZE: usize = 0x200;
const FILE_ALIGNMENT: usize = 0x200;
const SECTION_ALIGNMENT: u32 = 0x2000;
const SECTION_RVA: u32 = 0x2000;
const CLR_HEADER_SIZE: usize = 0x48;
const MEMBER_REF_TOKEN_CTOR: u32 = 0x0A00_0001;

const ASSEMBLY_NAME: &str = "FerrumWeave.Generated";
const ASSEMBLY_FILE: &str = "FerrumWeave.Generated.dll";
const NAMESPACE: &str = "FerrumWeave";
const TYPE_NAME: &str = "RustApi";
const METHOD_NAME: &str = "Answer";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagedConstructor {
    Object,
    StringBuilder,
}

impl ManagedConstructor {
    const fn type_name(self) -> &'static str {
        match self {
            Self::Object => "Object",
            Self::StringBuilder => "StringBuilder",
        }
    }

    const fn namespace(self) -> &'static str {
        match self {
            Self::Object => "System",
            Self::StringBuilder => "System.Text",
        }
    }
}

#[must_use]
pub fn emit_i32_export_with_managed_construction(
    constructor: ManagedConstructor,
    payload: i32,
) -> Vec<u8> {
    let method_body = build_method_body(payload);
    let method_offset = CLR_HEADER_SIZE;
    let method_rva = SECTION_RVA + to_u32(method_offset);
    let metadata = build_metadata(method_rva, constructor);
    let metadata_offset = align_usize(method_offset + method_body.len(), 4);
    let metadata_rva = SECTION_RVA + to_u32(metadata_offset);
    let section_virtual_size = metadata_offset + metadata.len();
    let section_raw_size = align_usize(section_virtual_size, FILE_ALIGNMENT);

    let mut section = vec![0_u8; section_raw_size];
    section[method_offset..method_offset + method_body.len()].copy_from_slice(&method_body);
    section[metadata_offset..metadata_offset + metadata.len()].copy_from_slice(&metadata);
    write_clr_header(&mut section[..CLR_HEADER_SIZE], metadata_rva, to_u32(metadata.len()));

    let mut image = vec![0_u8; HEADERS_SIZE];
    write_pe_headers(&mut image, to_u32(section_virtual_size), to_u32(section_raw_size));
    image.extend_from_slice(&section);
    image
}

fn build_method_body(payload: i32) -> Vec<u8> {
    const CODE_SIZE: u8 = 12;
    let mut body = Vec::with_capacity(usize::from(CODE_SIZE) + 1);
    body.push((CODE_SIZE << 2) | 0b10);
    body.push(0x73); // newobj instance void <selected>::.ctor()
    push_u32(&mut body, MEMBER_REF_TOKEN_CTOR);
    body.push(0x26); // pop: construction is the side effect under certification
    body.push(0x20); // ldc.i4 payload
    body.extend_from_slice(&payload.to_le_bytes());
    body.push(0x2A); // ret
    body
}

fn build_metadata(method_rva: u32, constructor: ManagedConstructor) -> Vec<u8> {
    let mut strings = vec![0_u8];
    let module_name = push_string(&mut strings, ASSEMBLY_FILE);
    let object_name = push_string(&mut strings, "Object");
    let constructed_type_name = push_string(&mut strings, constructor.type_name());
    let constructed_namespace = push_string(&mut strings, constructor.namespace());
    let system_namespace = push_string(&mut strings, "System");
    let module_type_name = push_string(&mut strings, "<Module>");
    let rust_api_name = push_string(&mut strings, TYPE_NAME);
    let ferrumweave_namespace = push_string(&mut strings, NAMESPACE);
    let answer_name = push_string(&mut strings, METHOD_NAME);
    let ctor_name = push_string(&mut strings, ".ctor");
    let assembly_name = push_string(&mut strings, ASSEMBLY_NAME);
    let system_runtime_name = push_string(&mut strings, "System.Runtime");
    pad_vec(&mut strings, 4);

    let guid = vec![
        0x46, 0x57, 0x43, 0x4F, 0x4E, 0x53, 0x54, 0x52,
        0x55, 0x43, 0x54, 0x30, 0x30, 0x30, 0x30, 0x31,
    ];

    let mut blobs = vec![0_u8];
    let answer_signature = push_blob(&mut blobs, &[0x00, 0x00, 0x08]);
    let ctor_signature = push_blob(&mut blobs, &[0x20, 0x00, 0x01]);
    let system_public_key_token = push_blob(
        &mut blobs,
        &[0xB0, 0x3F, 0x5F, 0x7F, 0x11, 0xD5, 0x0A, 0x3A],
    );
    pad_vec(&mut blobs, 4);

    let mut tables = Vec::new();
    push_u32(&mut tables, 0);
    tables.extend_from_slice(&[2, 0, 0, 1]);
    let valid_tables = (1_u64 << 0)
        | (1_u64 << 1)
        | (1_u64 << 2)
        | (1_u64 << 6)
        | (1_u64 << 10)
        | (1_u64 << 32)
        | (1_u64 << 35);
    push_u64(&mut tables, valid_tables);
    push_u64(&mut tables, 0);
    for count in [1_u32, 2, 2, 1, 1, 1, 1] {
        push_u32(&mut tables, count);
    }

    // Module.
    push_u16(&mut tables, 0);
    push_u16(&mut tables, module_name);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);

    // TypeRef 1: System.Object, used as RustApi base type.
    push_u16(&mut tables, 6);
    push_u16(&mut tables, object_name);
    push_u16(&mut tables, system_namespace);

    // TypeRef 2: selected construction target.
    push_u16(&mut tables, 6);
    push_u16(&mut tables, constructed_type_name);
    push_u16(&mut tables, constructed_namespace);

    // <Module>.
    push_u32(&mut tables, 0);
    push_u16(&mut tables, module_type_name);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 1);

    // FerrumWeave.RustApi extends TypeRef row 1.
    push_u32(&mut tables, 0x0010_0001);
    push_u16(&mut tables, rust_api_name);
    push_u16(&mut tables, ferrumweave_namespace);
    push_u16(&mut tables, 5);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 1);

    // public static int32 Answer().
    push_u32(&mut tables, method_rva);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0x0096);
    push_u16(&mut tables, answer_name);
    push_u16(&mut tables, answer_signature);
    push_u16(&mut tables, 1);

    // MemberRef parent TypeRef row 2: (2 << 3) | TypeRef-tag(1) = 17.
    push_u16(&mut tables, 17);
    push_u16(&mut tables, ctor_name);
    push_u16(&mut tables, ctor_signature);

    // Assembly.
    push_u32(&mut tables, 0x0000_8004);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u32(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, assembly_name);
    push_u16(&mut tables, 0);

    // AssemblyRef System.Runtime.
    push_u16(&mut tables, 10);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
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
    push_u16(&mut metadata, u16::try_from(streams.len()).expect("stream count fits u16"));
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
    write_u32_at(header, 0x00, to_u32(CLR_HEADER_SIZE));
    write_u16_at(header, 0x04, 2);
    write_u16_at(header, 0x06, 5);
    write_u32_at(header, 0x08, metadata_rva);
    write_u32_at(header, 0x0C, metadata_size);
    write_u32_at(header, 0x10, 0x0000_0001);
    write_u32_at(header, 0x14, 0);
}

fn write_pe_headers(headers: &mut [u8], section_virtual_size: u32, section_raw_size: u32) {
    headers[0..2].copy_from_slice(b"MZ");
    write_u32_at(headers, 0x3C, to_u32(PE_OFFSET));
    headers[PE_OFFSET..PE_OFFSET + 4].copy_from_slice(b"PE\0\0");
    let coff = PE_OFFSET + 4;
    write_u16_at(headers, coff, 0x014C);
    write_u16_at(headers, coff + 2, 1);
    write_u32_at(headers, coff + 4, 0);
    write_u32_at(headers, coff + 8, 0);
    write_u32_at(headers, coff + 12, 0);
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
    write_u32_at(headers, optional + 56, align_u32(SECTION_RVA + section_virtual_size, SECTION_ALIGNMENT));
    write_u32_at(headers, optional + 60, to_u32(HEADERS_SIZE));
    write_u16_at(headers, optional + 68, 3);
    write_u16_at(headers, optional + 70, 0x0100);
    write_u32_at(headers, optional + 72, 0x0010_0000);
    write_u32_at(headers, optional + 76, 0x0000_1000);
    write_u32_at(headers, optional + 80, 0x0010_0000);
    write_u32_at(headers, optional + 84, 0x0000_1000);
    write_u32_at(headers, optional + 92, 16);
    let cli_directory = optional + 96 + (14 * 8);
    write_u32_at(headers, cli_directory, SECTION_RVA);
    write_u32_at(headers, cli_directory + 4, to_u32(CLR_HEADER_SIZE));
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
        0..=0x7F => buffer.push(u8::try_from(value).expect("7-bit value fits u8")),
        0x80..=0x3FFF => {
            buffer.push(u8::try_from((value >> 8) | 0x80).expect("14-bit prefix fits u8"));
            buffer.push(u8::try_from(value & 0xFF).expect("low byte fits u8"));
        }
        _ => panic!("managed construction export only needs compact metadata values"),
    }
}
fn align_usize(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}
fn align_u32(value: u32, alignment: u32) -> u32 {
    (value + alignment - 1) & !(alignment - 1)
}
fn pad_vec(buffer: &mut Vec<u8>, alignment: usize) {
    buffer.resize(align_usize(buffer.len(), alignment), 0);
}
fn to_u16(value: usize) -> u16 {
    u16::try_from(value).expect("managed construction metadata index fits u16")
}
fn to_u32(value: usize) -> u32 {
    u32::try_from(value).expect("managed construction image size fits u32")
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
    fn construction_export_is_payload_and_constructor_sensitive() {
        let object_137 = emit_i32_export_with_managed_construction(ManagedConstructor::Object, 137);
        let object_211 = emit_i32_export_with_managed_construction(ManagedConstructor::Object, 211);
        let builder_137 = emit_i32_export_with_managed_construction(ManagedConstructor::StringBuilder, 137);
        assert_ne!(object_137, object_211);
        assert_ne!(object_137, builder_137);
        assert!(object_137.windows(5).any(|window| window == b".ctor"));
        assert!(builder_137.windows(13).any(|window| window == b"StringBuilder"));
    }
}
