#![forbid(unsafe_code)]

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::PROBE_ASSEMBLY_FILE;

const PE_OFFSET: usize = 0x80;
const OPTIONAL_HEADER_SIZE: usize = 0xE0;
const HEADERS_SIZE: usize = 0x200;
const FILE_ALIGNMENT: usize = 0x200;
const SECTION_ALIGNMENT: u32 = 0x2000;
const SECTION_RVA: u32 = 0x2000;
const CLR_HEADER_SIZE: usize = 0x48;
const MEMBER_REF_TOKEN_OBJECT_CTOR: u32 = 0x0A00_0001;
const MEMBER_REF_TOKEN_NULLABLE_I32_CTOR: u32 = 0x0A00_0002;
const TYPE_SPEC_TOKEN_NULLABLE_I32: u32 = 0x1B00_0001;
const USER_STRING_TOKEN_OPTION_SOME: u32 = 0x7000_0001;

pub const R06_NAMESPACE: &str = "FerrumWeave";
pub const R06_TYPE_NAME: &str = "RustApi";
pub const R06_INSTANCE_TYPE_NAME: &str = "RustValue";
pub const R06_STATIC_METHOD_NAME: &str = "Answer";
pub const R06_STATIC_ANSWER: i32 = 42;
pub const R07_OPTION_SOME_METHOD_NAME: &str = "OptionSomeString";
pub const R07_OPTION_NONE_METHOD_NAME: &str = "OptionNoneString";
pub const R07_OPTION_SOME_I32_METHOD_NAME: &str = "OptionSomeI32";
pub const R07_OPTION_NONE_I32_METHOD_NAME: &str = "OptionNoneI32";
pub const R07_OPTION_SOME_VALUE: &str = "FerrumWeave";

#[must_use]
pub fn emit_r06_static_api_assembly() -> Vec<u8> {
    let static_answer_body = build_answer_method_body();
    let static_answer_offset = CLR_HEADER_SIZE;
    let static_answer_rva = SECTION_RVA + to_u32(static_answer_offset);

    let option_some_body = build_option_some_string_body();
    let option_some_offset = align_usize(static_answer_offset + static_answer_body.len(), 4);
    let option_some_rva = SECTION_RVA + to_u32(option_some_offset);

    let option_none_body = build_option_none_string_body();
    let option_none_offset = align_usize(option_some_offset + option_some_body.len(), 4);
    let option_none_rva = SECTION_RVA + to_u32(option_none_offset);

    let option_some_i32_body = build_option_some_i32_body();
    let option_some_i32_offset = align_usize(option_none_offset + option_none_body.len(), 4);
    let option_some_i32_rva = SECTION_RVA + to_u32(option_some_i32_offset);

    let option_none_i32_body = build_option_none_i32_body();
    let option_none_i32_offset =
        align_usize(option_some_i32_offset + option_some_i32_body.len(), 4);
    let option_none_i32_rva = SECTION_RVA + to_u32(option_none_i32_offset);

    let ctor_body = build_constructor_method_body();
    let ctor_offset = align_usize(option_none_i32_offset + option_none_i32_body.len(), 4);
    let ctor_rva = SECTION_RVA + to_u32(ctor_offset);

    let instance_answer_body = build_answer_method_body();
    let instance_answer_offset = align_usize(ctor_offset + ctor_body.len(), 4);
    let instance_answer_rva = SECTION_RVA + to_u32(instance_answer_offset);

    let metadata_offset = align_usize(instance_answer_offset + instance_answer_body.len(), 4);
    let metadata = build_metadata(
        static_answer_rva,
        option_some_rva,
        option_none_rva,
        option_some_i32_rva,
        option_none_i32_rva,
        ctor_rva,
        instance_answer_rva,
    );
    let metadata_rva = SECTION_RVA + to_u32(metadata_offset);
    let section_virtual_size = metadata_offset + metadata.len();
    let section_raw_size = align_usize(section_virtual_size, FILE_ALIGNMENT);

    let mut section = vec![0_u8; section_raw_size];
    section[static_answer_offset..static_answer_offset + static_answer_body.len()]
        .copy_from_slice(&static_answer_body);
    section[option_some_offset..option_some_offset + option_some_body.len()]
        .copy_from_slice(&option_some_body);
    section[option_none_offset..option_none_offset + option_none_body.len()]
        .copy_from_slice(&option_none_body);
    section[option_some_i32_offset..option_some_i32_offset + option_some_i32_body.len()]
        .copy_from_slice(&option_some_i32_body);
    section[option_none_i32_offset..option_none_i32_offset + option_none_i32_body.len()]
        .copy_from_slice(&option_none_i32_body);
    section[ctor_offset..ctor_offset + ctor_body.len()].copy_from_slice(&ctor_body);
    section[instance_answer_offset..instance_answer_offset + instance_answer_body.len()]
        .copy_from_slice(&instance_answer_body);
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

pub fn write_r06_static_api_artifact(directory: impl AsRef<Path>) -> io::Result<PathBuf> {
    let directory = directory.as_ref();
    fs::create_dir_all(directory)?;
    let assembly = directory.join(PROBE_ASSEMBLY_FILE);
    fs::write(&assembly, emit_r06_static_api_assembly())?;
    Ok(assembly)
}

fn build_answer_method_body() -> Vec<u8> {
    let mut body = Vec::with_capacity(4);
    const CODE_SIZE: u8 = 3;
    body.push((CODE_SIZE << 2) | 0b10);
    body.push(0x1F); // ldc.i4.s
    body.push(u8::try_from(R06_STATIC_ANSWER).expect("R06 answer fits signed byte"));
    body.push(0x2A); // ret
    body
}

fn build_option_some_string_body() -> Vec<u8> {
    let mut body = Vec::with_capacity(7);
    const CODE_SIZE: u8 = 6;
    body.push((CODE_SIZE << 2) | 0b10);
    body.push(0x72); // ldstr
    push_u32(&mut body, USER_STRING_TOKEN_OPTION_SOME);
    body.push(0x2A); // ret
    body
}

fn build_option_none_string_body() -> Vec<u8> {
    let mut body = Vec::with_capacity(3);
    const CODE_SIZE: u8 = 2;
    body.push((CODE_SIZE << 2) | 0b10);
    body.push(0x14); // ldnull
    body.push(0x2A); // ret
    body
}

fn build_option_some_i32_body() -> Vec<u8> {
    let mut body = Vec::with_capacity(9);
    const CODE_SIZE: u8 = 8;
    body.push((CODE_SIZE << 2) | 0b10);
    body.push(0x1F); // ldc.i4.s
    body.push(u8::try_from(R06_STATIC_ANSWER).expect("R07 Option value fits signed byte"));
    body.push(0x73); // newobj instance void System.Nullable<int32>::.ctor(int32)
    push_u32(&mut body, MEMBER_REF_TOKEN_NULLABLE_I32_CTOR);
    body.push(0x2A); // ret
    body
}

fn build_option_none_i32_body() -> Vec<u8> {
    let mut body = Vec::with_capacity(8);
    const CODE_SIZE: u8 = 7;
    body.push((CODE_SIZE << 2) | 0b10);
    body.push(0x14); // ldnull
    body.push(0xA5); // unbox.any System.Nullable<int32>; null becomes default Nullable<int32>
    push_u32(&mut body, TYPE_SPEC_TOKEN_NULLABLE_I32);
    body.push(0x2A); // ret
    body
}

fn build_constructor_method_body() -> Vec<u8> {
    let mut body = Vec::with_capacity(8);
    const CODE_SIZE: u8 = 7;
    body.push((CODE_SIZE << 2) | 0b10);
    body.push(0x02); // ldarg.0
    body.push(0x28); // call instance void System.Object::.ctor()
    push_u32(&mut body, MEMBER_REF_TOKEN_OBJECT_CTOR);
    body.push(0x2A); // ret
    body
}

fn build_metadata(
    static_answer_rva: u32,
    option_some_rva: u32,
    option_none_rva: u32,
    option_some_i32_rva: u32,
    option_none_i32_rva: u32,
    ctor_rva: u32,
    instance_answer_rva: u32,
) -> Vec<u8> {
    let mut strings = vec![0_u8];
    let module_name = push_string(&mut strings, PROBE_ASSEMBLY_FILE);
    let object_name = push_string(&mut strings, "Object");
    let nullable_name = push_string(&mut strings, "Nullable`1");
    let system_namespace = push_string(&mut strings, "System");
    let module_type_name = push_string(&mut strings, "<Module>");
    let rust_api_name = push_string(&mut strings, R06_TYPE_NAME);
    let rust_value_name = push_string(&mut strings, R06_INSTANCE_TYPE_NAME);
    let ferrumweave_namespace = push_string(&mut strings, R06_NAMESPACE);
    let answer_name = push_string(&mut strings, R06_STATIC_METHOD_NAME);
    let option_some_name = push_string(&mut strings, R07_OPTION_SOME_METHOD_NAME);
    let option_none_name = push_string(&mut strings, R07_OPTION_NONE_METHOD_NAME);
    let option_some_i32_name = push_string(&mut strings, R07_OPTION_SOME_I32_METHOD_NAME);
    let option_none_i32_name = push_string(&mut strings, R07_OPTION_NONE_I32_METHOD_NAME);
    let ctor_name = push_string(&mut strings, ".ctor");
    let assembly_name = push_string(&mut strings, "FerrumWeave.Probe");
    let system_runtime_name = push_string(&mut strings, "System.Runtime");
    pad_vec(&mut strings, 4);

    let mut user_strings = vec![0_u8];
    let option_some_string = push_user_string(&mut user_strings, R07_OPTION_SOME_VALUE);
    debug_assert_eq!(option_some_string, 1);
    pad_vec(&mut user_strings, 4);

    let guid = vec![
        0x46, 0x57, 0x52, 0x30, 0x36, 0x53, 0x54, 0x41, 0x54, 0x49, 0x43, 0x41, 0x50, 0x49, 0x30,
        0x31,
    ];

    let mut blobs = vec![0_u8];
    let static_answer_signature = push_blob(&mut blobs, &[0x00, 0x00, 0x08]);
    let static_string_signature = push_blob(&mut blobs, &[0x00, 0x00, 0x0E]);
    // DEFAULT, 0 params, GENERICINST VALUETYPE TypeRef(Nullable`1), 1 generic arg, I4.
    let static_nullable_i32_signature =
        push_blob(&mut blobs, &[0x00, 0x00, 0x15, 0x11, 0x09, 0x01, 0x08]);
    let ctor_signature = push_blob(&mut blobs, &[0x20, 0x00, 0x01]);
    let nullable_i32_ctor_signature = push_blob(&mut blobs, &[0x20, 0x01, 0x01, 0x08]);
    let instance_answer_signature = push_blob(&mut blobs, &[0x20, 0x00, 0x08]);
    let nullable_i32_type_signature = push_blob(&mut blobs, &[0x15, 0x11, 0x09, 0x01, 0x08]);
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
        | (1_u64 << 27)
        | (1_u64 << 32)
        | (1_u64 << 35);
    push_u64(&mut tables, valid_tables);
    push_u64(&mut tables, 0);

    for count in [1_u32, 2, 3, 7, 2, 1, 1, 1] {
        push_u32(&mut tables, count);
    }

    // Module (0x00).
    push_u16(&mut tables, 0);
    push_u16(&mut tables, module_name);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);

    // TypeRef row 1: [System.Runtime]System.Object.
    push_u16(&mut tables, 6); // AssemblyRef row 1, ResolutionScope tag 2.
    push_u16(&mut tables, object_name);
    push_u16(&mut tables, system_namespace);

    // TypeRef row 2: [System.Runtime]System.Nullable`1.
    push_u16(&mut tables, 6);
    push_u16(&mut tables, nullable_name);
    push_u16(&mut tables, system_namespace);

    // TypeDef row 1: <Module>. RustApi also starts at method row 1,
    // therefore the module owns no methods.
    push_u32(&mut tables, 0);
    push_u16(&mut tables, module_type_name);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 1);

    // TypeDef row 2: public abstract sealed class FerrumWeave.RustApi : Object.
    push_u32(&mut tables, 0x0010_0181);
    push_u16(&mut tables, rust_api_name);
    push_u16(&mut tables, ferrumweave_namespace);
    push_u16(&mut tables, 5); // TypeDefOrRef: TypeRef row 1 => (1 << 2) | 1.
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 1);

    // TypeDef row 3: public class FerrumWeave.RustValue : Object.
    push_u32(&mut tables, 0x0010_0001);
    push_u16(&mut tables, rust_value_name);
    push_u16(&mut tables, ferrumweave_namespace);
    push_u16(&mut tables, 5);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 6);

    // MethodDef row 1: public static int32 RustApi.Answer().
    push_u32(&mut tables, static_answer_rva);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0x0096);
    push_u16(&mut tables, answer_name);
    push_u16(&mut tables, static_answer_signature);
    push_u16(&mut tables, 1);

    // MethodDef row 2: public static string RustApi.OptionSomeString().
    push_u32(&mut tables, option_some_rva);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0x0096);
    push_u16(&mut tables, option_some_name);
    push_u16(&mut tables, static_string_signature);
    push_u16(&mut tables, 1);

    // MethodDef row 3: public static string RustApi.OptionNoneString().
    push_u32(&mut tables, option_none_rva);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0x0096);
    push_u16(&mut tables, option_none_name);
    push_u16(&mut tables, static_string_signature);
    push_u16(&mut tables, 1);

    // MethodDef row 4: public static System.Nullable<int32> RustApi.OptionSomeI32().
    push_u32(&mut tables, option_some_i32_rva);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0x0096);
    push_u16(&mut tables, option_some_i32_name);
    push_u16(&mut tables, static_nullable_i32_signature);
    push_u16(&mut tables, 1);

    // MethodDef row 5: public static System.Nullable<int32> RustApi.OptionNoneI32().
    push_u32(&mut tables, option_none_i32_rva);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0x0096);
    push_u16(&mut tables, option_none_i32_name);
    push_u16(&mut tables, static_nullable_i32_signature);
    push_u16(&mut tables, 1);

    // MethodDef row 6: public specialname rtspecialname instance void RustValue::.ctor().
    push_u32(&mut tables, ctor_rva);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0x1886);
    push_u16(&mut tables, ctor_name);
    push_u16(&mut tables, ctor_signature);
    push_u16(&mut tables, 1);

    // MethodDef row 7: public instance int32 RustValue.Answer().
    push_u32(&mut tables, instance_answer_rva);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0x0086);
    push_u16(&mut tables, answer_name);
    push_u16(&mut tables, instance_answer_signature);
    push_u16(&mut tables, 1);

    // MemberRef row 1: instance void [System.Runtime]System.Object::.ctor().
    // MemberRefParent tag 1 is TypeRef; row 1 => (1 << 3) | 1 = 9.
    push_u16(&mut tables, 9);
    push_u16(&mut tables, ctor_name);
    push_u16(&mut tables, ctor_signature);

    // MemberRef row 2: instance void System.Nullable<int32>::.ctor(int32).
    // MemberRefParent tag 4 is TypeSpec; row 1 => (1 << 3) | 4 = 12.
    push_u16(&mut tables, 12);
    push_u16(&mut tables, ctor_name);
    push_u16(&mut tables, nullable_i32_ctor_signature);

    // TypeSpec row 1: System.Nullable<int32>.
    push_u16(&mut tables, nullable_i32_type_signature);

    // Assembly (0x20): FerrumWeave.Probe 1.0.0.0.
    push_u32(&mut tables, 0x0000_8004);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u32(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, assembly_name);
    push_u16(&mut tables, 0);

    // AssemblyRef (0x23): System.Runtime 10.0.0.0.
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

fn write_clr_header(header: &mut [u8], metadata_rva: u32, metadata_size: u32) {
    write_u32_at(header, 0x00, to_u32(CLR_HEADER_SIZE));
    write_u16_at(header, 0x04, 2);
    write_u16_at(header, 0x06, 5);
    write_u32_at(header, 0x08, metadata_rva);
    write_u32_at(header, 0x0C, metadata_size);
    write_u32_at(header, 0x10, 0x0000_0001); // COMIMAGE_FLAGS_ILONLY.
    write_u32_at(header, 0x14, 0); // Library: no managed entry point token.
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
    let image_size = align_u32(SECTION_RVA + section_virtual_size, SECTION_ALIGNMENT);
    write_u32_at(headers, optional + 56, image_size);
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

fn push_user_string(heap: &mut Vec<u8>, value: &str) -> u32 {
    let index = to_u32(heap.len());
    let utf16: Vec<u16> = value.encode_utf16().collect();
    let payload_size = utf16.len() * 2 + 1;
    push_compressed_unsigned(heap, to_u32(payload_size));
    for unit in utf16 {
        push_u16(heap, unit);
    }
    heap.push(0);
    index
}

fn push_compressed_unsigned(buffer: &mut Vec<u8>, value: u32) {
    match value {
        0..=0x7F => buffer.push(u8::try_from(value).expect("7-bit value fits u8")),
        0x80..=0x3FFF => {
            buffer.push(u8::try_from((value >> 8) | 0x80).expect("14-bit prefix fits u8"));
            buffer.push(u8::try_from(value & 0xFF).expect("low byte fits u8"));
        }
        _ => panic!("R06 emitter only needs compact metadata values"),
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
    u16::try_from(value).expect("R06 metadata index fits u16")
}

fn to_u32(value: usize) -> u32 {
    u32::try_from(value).expect("R06 image size fits u32")
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
    fn static_api_image_is_deterministic_and_names_the_public_surface() {
        let first = emit_r06_static_api_assembly();
        assert_eq!(first, emit_r06_static_api_assembly());
        for expected in [
            R06_NAMESPACE,
            R06_TYPE_NAME,
            R06_INSTANCE_TYPE_NAME,
            R06_STATIC_METHOD_NAME,
            R07_OPTION_SOME_METHOD_NAME,
            R07_OPTION_NONE_METHOD_NAME,
            R07_OPTION_SOME_I32_METHOD_NAME,
            R07_OPTION_NONE_I32_METHOD_NAME,
            R07_OPTION_SOME_VALUE,
        ] {
            assert!(
                first
                    .windows(expected.len())
                    .any(|window| window == expected.as_bytes())
            );
        }
    }
}
