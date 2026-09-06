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
const FIELD_TOKEN_RELEASE_COUNT: u32 = 0x0400_0001;

pub const R07_RESOURCE_TYPE_NAME: &str = "RustResource";
pub const R07_DISPOSE_METHOD_NAME: &str = "Dispose";
pub const R07_RELEASE_COUNT_METHOD_NAME: &str = "ReleaseCount";

#[must_use]
pub fn emit_r07_disposable_assembly() -> Vec<u8> {
    let ctor_body = build_resource_constructor_body();
    let ctor_offset = CLR_HEADER_SIZE;
    let ctor_rva = SECTION_RVA + to_u32(ctor_offset);

    let dispose_body = build_dispose_body();
    let dispose_offset = align_usize(ctor_offset + ctor_body.len(), 4);
    let dispose_rva = SECTION_RVA + to_u32(dispose_offset);

    let release_count_body = build_release_count_body();
    let release_count_offset = align_usize(dispose_offset + dispose_body.len(), 4);
    let release_count_rva = SECTION_RVA + to_u32(release_count_offset);

    let metadata_offset = align_usize(release_count_offset + release_count_body.len(), 4);
    let metadata = build_metadata(ctor_rva, dispose_rva, release_count_rva);
    let metadata_rva = SECTION_RVA + to_u32(metadata_offset);
    let section_virtual_size = metadata_offset + metadata.len();
    let section_raw_size = align_usize(section_virtual_size, FILE_ALIGNMENT);

    let mut section = vec![0_u8; section_raw_size];
    section[ctor_offset..ctor_offset + ctor_body.len()].copy_from_slice(&ctor_body);
    section[dispose_offset..dispose_offset + dispose_body.len()].copy_from_slice(&dispose_body);
    section[release_count_offset..release_count_offset + release_count_body.len()]
        .copy_from_slice(&release_count_body);
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

pub fn write_r07_disposable_artifact(directory: impl AsRef<Path>) -> io::Result<PathBuf> {
    let directory = directory.as_ref();
    fs::create_dir_all(directory)?;
    let assembly = directory.join(PROBE_ASSEMBLY_FILE);
    fs::write(&assembly, emit_r07_disposable_assembly())?;
    Ok(assembly)
}

fn build_resource_constructor_body() -> Vec<u8> {
    let mut body = Vec::with_capacity(15);
    const CODE_SIZE: u8 = 14;
    body.push((CODE_SIZE << 2) | 0b10);
    body.push(0x02); // ldarg.0
    body.push(0x28); // call instance void System.Object::.ctor()
    push_u32(&mut body, MEMBER_REF_TOKEN_OBJECT_CTOR);
    body.push(0x02); // ldarg.0
    body.push(0x16); // ldc.i4.0
    body.push(0x7D); // stfld int32 FerrumWeave.RustResource::releaseCount
    push_u32(&mut body, FIELD_TOKEN_RELEASE_COUNT);
    body.push(0x2A); // ret
    body
}

fn build_dispose_body() -> Vec<u8> {
    let mut body = Vec::with_capacity(17);
    const CODE_SIZE: u8 = 16;
    body.push((CODE_SIZE << 2) | 0b10);
    body.push(0x02); // ldarg.0
    body.push(0x7B); // ldfld int32 releaseCount
    push_u32(&mut body, FIELD_TOKEN_RELEASE_COUNT);
    body.push(0x2D); // brtrue.s -> ret when already released
    body.push(0x07);
    body.push(0x02); // ldarg.0
    body.push(0x17); // ldc.i4.1
    body.push(0x7D); // stfld int32 releaseCount
    push_u32(&mut body, FIELD_TOKEN_RELEASE_COUNT);
    body.push(0x2A); // ret
    body
}

fn build_release_count_body() -> Vec<u8> {
    let mut body = Vec::with_capacity(8);
    const CODE_SIZE: u8 = 7;
    body.push((CODE_SIZE << 2) | 0b10);
    body.push(0x02); // ldarg.0
    body.push(0x7B); // ldfld int32 releaseCount
    push_u32(&mut body, FIELD_TOKEN_RELEASE_COUNT);
    body.push(0x2A); // ret
    body
}

fn build_metadata(ctor_rva: u32, dispose_rva: u32, release_count_rva: u32) -> Vec<u8> {
    let mut strings = vec![0_u8];
    let module_name = push_string(&mut strings, PROBE_ASSEMBLY_FILE);
    let object_name = push_string(&mut strings, "Object");
    let idisposable_name = push_string(&mut strings, "IDisposable");
    let system_namespace = push_string(&mut strings, "System");
    let module_type_name = push_string(&mut strings, "<Module>");
    let resource_name = push_string(&mut strings, R07_RESOURCE_TYPE_NAME);
    let ferrumweave_namespace = push_string(&mut strings, "FerrumWeave");
    let release_count_field_name = push_string(&mut strings, "releaseCount");
    let ctor_name = push_string(&mut strings, ".ctor");
    let dispose_name = push_string(&mut strings, R07_DISPOSE_METHOD_NAME);
    let release_count_name = push_string(&mut strings, R07_RELEASE_COUNT_METHOD_NAME);
    let assembly_name = push_string(&mut strings, "FerrumWeave.Probe");
    let system_runtime_name = push_string(&mut strings, "System.Runtime");
    pad_vec(&mut strings, 4);

    let guid = vec![
        0x46, 0x57, 0x52, 0x30, 0x37, 0x44, 0x49, 0x53, 0x50, 0x4F, 0x53, 0x41, 0x42, 0x4C, 0x45,
        0x31,
    ];

    let mut blobs = vec![0_u8];
    let field_i32_signature = push_blob(&mut blobs, &[0x06, 0x08]);
    let instance_void_signature = push_blob(&mut blobs, &[0x20, 0x00, 0x01]);
    let instance_i32_signature = push_blob(&mut blobs, &[0x20, 0x00, 0x08]);
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
        | (1_u64 << 4)
        | (1_u64 << 6)
        | (1_u64 << 9)
        | (1_u64 << 10)
        | (1_u64 << 32)
        | (1_u64 << 35);
    push_u64(&mut tables, valid_tables);
    push_u64(&mut tables, 0);

    for count in [1_u32, 2, 2, 1, 3, 1, 1, 1, 1] {
        push_u32(&mut tables, count);
    }

    // Module (0x00).
    push_u16(&mut tables, 0);
    push_u16(&mut tables, module_name);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);

    // TypeRef rows 1-2: System.Object and System.IDisposable from System.Runtime.
    for type_name in [object_name, idisposable_name] {
        push_u16(&mut tables, 6); // AssemblyRef row 1, ResolutionScope tag 2.
        push_u16(&mut tables, type_name);
        push_u16(&mut tables, system_namespace);
    }

    // TypeDef row 1: <Module>.
    push_u32(&mut tables, 0);
    push_u16(&mut tables, module_type_name);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 1);

    // TypeDef row 2: public class FerrumWeave.RustResource : System.Object.
    push_u32(&mut tables, 0x0010_0001);
    push_u16(&mut tables, resource_name);
    push_u16(&mut tables, ferrumweave_namespace);
    push_u16(&mut tables, 5); // TypeRef row 1 encoded as TypeDefOrRef.
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 1);

    // Field row 1: private int32 releaseCount.
    push_u16(&mut tables, 0x0001);
    push_u16(&mut tables, release_count_field_name);
    push_u16(&mut tables, field_i32_signature);

    // MethodDef row 1: public specialname rtspecialname instance void .ctor().
    push_u32(&mut tables, ctor_rva);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0x1886);
    push_u16(&mut tables, ctor_name);
    push_u16(&mut tables, instance_void_signature);
    push_u16(&mut tables, 1);

    // MethodDef row 2: public virtual final newslot instance void Dispose().
    push_u32(&mut tables, dispose_rva);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0x01E6);
    push_u16(&mut tables, dispose_name);
    push_u16(&mut tables, instance_void_signature);
    push_u16(&mut tables, 1);

    // MethodDef row 3: public instance int32 ReleaseCount().
    push_u32(&mut tables, release_count_rva);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0x0086);
    push_u16(&mut tables, release_count_name);
    push_u16(&mut tables, instance_i32_signature);
    push_u16(&mut tables, 1);

    // InterfaceImpl row 1: RustResource implements System.IDisposable.
    push_u16(&mut tables, 2); // TypeDef row 2.
    push_u16(&mut tables, 9); // TypeRef row 2 encoded as TypeDefOrRef.

    // MemberRef row 1: instance void System.Object::.ctor().
    push_u16(&mut tables, 9); // TypeRef row 1 encoded as MemberRefParent.
    push_u16(&mut tables, ctor_name);
    push_u16(&mut tables, instance_void_signature);

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

fn push_compressed_unsigned(buffer: &mut Vec<u8>, value: u32) {
    match value {
        0..=0x7F => buffer.push(u8::try_from(value).expect("7-bit value fits u8")),
        0x80..=0x3FFF => {
            buffer.push(u8::try_from((value >> 8) | 0x80).expect("14-bit prefix fits u8"));
            buffer.push(u8::try_from(value & 0xFF).expect("low byte fits u8"));
        }
        _ => panic!("R07 emitter only needs compact metadata values"),
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
    u16::try_from(value).expect("R07 metadata index fits u16")
}

fn to_u32(value: usize) -> u32 {
    u32::try_from(value).expect("R07 metadata value fits u32")
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
    fn disposable_image_is_deterministic_and_names_the_contract_surface() {
        let image = emit_r07_disposable_assembly();
        assert_eq!(image, emit_r07_disposable_assembly());
        for expected in [
            R07_RESOURCE_TYPE_NAME,
            R07_DISPOSE_METHOD_NAME,
            R07_RELEASE_COUNT_METHOD_NAME,
            "IDisposable",
        ] {
            assert!(
                image
                    .windows(expected.len())
                    .any(|window| window == expected.as_bytes())
            );
        }
    }
}
