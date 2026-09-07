#![forbid(unsafe_code)]

const PE_OFFSET: usize = 0x80;
const OPTIONAL_HEADER_SIZE: usize = 0xE0;
const HEADERS_SIZE: usize = 0x200;
const FILE_ALIGNMENT: usize = 0x200;
const SECTION_ALIGNMENT: u32 = 0x2000;
const SECTION_RVA: u32 = 0x2000;
const CLR_HEADER_SIZE: usize = 0x48;
const ENTRY_POINT_TOKEN: u32 = 0x0600_0002;
const MEMBER_REF_TOKEN_CONSOLE_WRITELINE: u32 = 0x0A00_0001;
const USER_STRING_TOKEN_MAIN_MESSAGE: u32 = 0x7000_0001;

pub fn emit_console_assembly(assembly_name: &str, message: &str) -> Vec<u8> {
    let answer_body = build_answer_body();
    let answer_offset = CLR_HEADER_SIZE;
    let answer_rva = SECTION_RVA + to_u32(answer_offset);

    let main_body = build_main_body();
    let main_offset = align_usize(answer_offset + answer_body.len(), 4);
    let main_rva = SECTION_RVA + to_u32(main_offset);

    let metadata_offset = align_usize(main_offset + main_body.len(), 4);
    let metadata = build_metadata(assembly_name, message, answer_rva, main_rva);
    let metadata_rva = SECTION_RVA + to_u32(metadata_offset);
    let section_virtual_size = metadata_offset + metadata.len();
    let section_raw_size = align_usize(section_virtual_size, FILE_ALIGNMENT);

    let mut section = vec![0_u8; section_raw_size];
    section[answer_offset..answer_offset + answer_body.len()].copy_from_slice(&answer_body);
    section[main_offset..main_offset + main_body.len()].copy_from_slice(&main_body);
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

fn build_answer_body() -> Vec<u8> {
    vec![0x0E, 0x1F, 42, 0x2A]
}

fn build_main_body() -> Vec<u8> {
    let mut body = Vec::with_capacity(12);
    body.push((11 << 2) | 0b10);
    body.push(0x72); // ldstr
    push_u32(&mut body, USER_STRING_TOKEN_MAIN_MESSAGE);
    body.push(0x28); // call void System.Console::WriteLine(string)
    push_u32(&mut body, MEMBER_REF_TOKEN_CONSOLE_WRITELINE);
    body.push(0x2A); // ret
    body
}

fn build_metadata(assembly_name: &str, message: &str, answer_rva: u32, main_rva: u32) -> Vec<u8> {
    let mut strings = vec![0_u8];
    let module_name = push_string(&mut strings, &format!("{assembly_name}.dll"));
    let object_name = push_string(&mut strings, "Object");
    let console_name = push_string(&mut strings, "Console");
    let system_namespace = push_string(&mut strings, "System");
    let module_type_name = push_string(&mut strings, "<Module>");
    let rust_api_name = push_string(&mut strings, "RustApi");
    let program_name = push_string(&mut strings, "Program");
    let ferrumweave_namespace = push_string(&mut strings, "FerrumWeave");
    let answer_name = push_string(&mut strings, "Answer");
    let main_name = push_string(&mut strings, "Main");
    let write_line_name = push_string(&mut strings, "WriteLine");
    let assembly_name_index = push_string(&mut strings, assembly_name);
    let system_runtime_name = push_string(&mut strings, "System.Runtime");
    let system_console_name = push_string(&mut strings, "System.Console");
    pad_vec(&mut strings, 4);

    let mut user_strings = vec![0_u8];
    let main_message = push_user_string(&mut user_strings, message);
    assert_eq!(main_message, 1, "main message must be first user string");
    pad_vec(&mut user_strings, 4);

    let guid = vec![
        0x46, 0x57, 0x52, 0x30, 0x38, 0x45, 0x58, 0x45, 0x43, 0x55, 0x54, 0x41, 0x42, 0x4C,
        0x45, 0x31,
    ];

    let mut blobs = vec![0_u8];
    let answer_signature = push_blob(&mut blobs, &[0x00, 0x00, 0x08]);
    let main_signature = push_blob(&mut blobs, &[0x00, 0x00, 0x01]);
    let write_line_signature = push_blob(&mut blobs, &[0x00, 0x01, 0x01, 0x0E]);
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
    for count in [1_u32, 2, 3, 2, 1, 1, 2] {
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

    // TypeRef row 2: [System.Console]System.Console.
    push_u16(&mut tables, 10);
    push_u16(&mut tables, console_name);
    push_u16(&mut tables, system_namespace);

    // TypeDef row 1: <Module> owns no methods.
    push_u32(&mut tables, 0);
    push_u16(&mut tables, module_type_name);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 1);

    // TypeDef row 2: public abstract sealed FerrumWeave.RustApi : Object.
    push_u32(&mut tables, 0x0010_0181);
    push_u16(&mut tables, rust_api_name);
    push_u16(&mut tables, ferrumweave_namespace);
    push_u16(&mut tables, 5);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 1);

    // TypeDef row 3: public abstract sealed FerrumWeave.Program : Object.
    push_u32(&mut tables, 0x0010_0181);
    push_u16(&mut tables, program_name);
    push_u16(&mut tables, ferrumweave_namespace);
    push_u16(&mut tables, 5);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 2);

    // MethodDef row 1: public static int32 RustApi.Answer().
    push_u32(&mut tables, answer_rva);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0x0096);
    push_u16(&mut tables, answer_name);
    push_u16(&mut tables, answer_signature);
    push_u16(&mut tables, 1);

    // MethodDef row 2: public static void Program.Main().
    push_u32(&mut tables, main_rva);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0x0096);
    push_u16(&mut tables, main_name);
    push_u16(&mut tables, main_signature);
    push_u16(&mut tables, 1);

    // MemberRef row 1: void [System.Console]System.Console::WriteLine(string).
    push_u16(&mut tables, 17);
    push_u16(&mut tables, write_line_name);
    push_u16(&mut tables, write_line_signature);

    // Assembly.
    push_u32(&mut tables, 0x0000_8004);
    push_u16(&mut tables, 1);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u32(&mut tables, 0);
    push_u16(&mut tables, 0);
    push_u16(&mut tables, assembly_name_index);
    push_u16(&mut tables, 0);

    // AssemblyRef row 1: System.Runtime 10.0.0.0.
    push_assembly_ref(
        &mut tables,
        system_public_key_token,
        system_runtime_name,
    );
    // AssemblyRef row 2: System.Console 10.0.0.0.
    push_assembly_ref(
        &mut tables,
        system_public_key_token,
        system_console_name,
    );
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
    push_u16(&mut metadata, to_u16(streams.len()));

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

fn push_assembly_ref(tables: &mut Vec<u8>, public_key_token: u16, name: u16) {
    push_u16(tables, 10);
    push_u16(tables, 0);
    push_u16(tables, 0);
    push_u16(tables, 0);
    push_u32(tables, 0);
    push_u16(tables, public_key_token);
    push_u16(tables, name);
    push_u16(tables, 0);
    push_u16(tables, 0);
}

fn write_clr_header(header: &mut [u8], metadata_rva: u32, metadata_size: u32) {
    write_u32_at(header, 0x00, to_u32(CLR_HEADER_SIZE));
    write_u16_at(header, 0x04, 2);
    write_u16_at(header, 0x06, 5);
    write_u32_at(header, 0x08, metadata_rva);
    write_u32_at(header, 0x0C, metadata_size);
    write_u32_at(header, 0x10, 0x0000_0001);
    write_u32_at(header, 0x14, ENTRY_POINT_TOKEN);
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

fn push_user_string(heap: &mut Vec<u8>, value: &str) -> usize {
    let offset = heap.len();
    let mut payload = Vec::with_capacity((value.encode_utf16().count() * 2) + 1);
    for code_unit in value.encode_utf16() {
        payload.extend_from_slice(&code_unit.to_le_bytes());
    }
    payload.push(0);
    push_compressed_u32(heap, to_u32(payload.len()));
    heap.extend_from_slice(&payload);
    offset
}

fn push_string(heap: &mut Vec<u8>, value: &str) -> u16 {
    let offset = to_u16(heap.len());
    heap.extend_from_slice(value.as_bytes());
    heap.push(0);
    offset
}

fn push_blob(heap: &mut Vec<u8>, value: &[u8]) -> u16 {
    let offset = to_u16(heap.len());
    push_compressed_u32(heap, to_u32(value.len()));
    heap.extend_from_slice(value);
    offset
}

fn push_compressed_u32(output: &mut Vec<u8>, value: u32) {
    if value <= 0x7F {
        output.push(u8::try_from(value).expect("compressed value fits u8"));
    } else if value <= 0x3FFF {
        output.push(u8::try_from((value >> 8) | 0x80).expect("compressed prefix fits u8"));
        output.push(u8::try_from(value & 0xFF).expect("compressed tail fits u8"));
    } else {
        output.push(u8::try_from((value >> 24) | 0xC0).expect("compressed prefix fits u8"));
        output.push(u8::try_from((value >> 16) & 0xFF).expect("compressed byte fits u8"));
        output.push(u8::try_from((value >> 8) & 0xFF).expect("compressed byte fits u8"));
        output.push(u8::try_from(value & 0xFF).expect("compressed byte fits u8"));
    }
}

fn pad_vec(buffer: &mut Vec<u8>, alignment: usize) {
    let aligned = align_usize(buffer.len(), alignment);
    buffer.resize(aligned, 0);
}

fn align_usize(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

fn align_u32(value: u32, alignment: u32) -> u32 {
    (value + alignment - 1) & !(alignment - 1)
}

fn to_u16(value: usize) -> u16 {
    u16::try_from(value).expect("value fits u16")
}

fn to_u32(value: usize) -> u32 {
    u32::try_from(value).expect("value fits u32")
}

fn push_u16(output: &mut Vec<u8>, value: u16) {
    output.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(output: &mut Vec<u8>, value: u32) {
    output.extend_from_slice(&value.to_le_bytes());
}

fn push_u64(output: &mut Vec<u8>, value: u64) {
    output.extend_from_slice(&value.to_le_bytes());
}

fn write_u16_at(output: &mut [u8], offset: usize, value: u16) {
    output[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn write_u32_at(output: &mut [u8], offset: usize, value: u32) {
    output[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}
