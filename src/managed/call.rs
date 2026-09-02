use super::base::{ManagedMetadataError, ManagedMethodRef};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedMemberRef {
    pub namespace: String,
    pub type_name: String,
    pub method_name: String,
    pub token: u32,
}

pub fn resolve_public_static_member_ref(
    image: &[u8],
    namespace: &str,
    type_name: &str,
    method_name: &str,
) -> Result<ManagedMemberRef, ManagedMetadataError> {
    let metadata = metadata_root(image)?;
    let streams = metadata_streams(metadata)?;
    let tables = streams
        .iter()
        .find(|stream| stream.name == "#~" || stream.name == "#-")
        .ok_or(ManagedMetadataError::Malformed(
            "missing metadata tables stream",
        ))?;
    let strings = streams
        .iter()
        .find(|stream| stream.name == "#Strings")
        .ok_or(ManagedMetadataError::Malformed("missing #Strings heap"))?;

    let tables_data = subslice(metadata, tables.offset, tables.size)?;
    let strings_data = subslice(metadata, strings.offset, strings.size)?;
    let layout = TableLayout::parse(tables_data)?;

    let mut type_ref_row = None;
    for row in 0..layout.rows[1] as usize {
        let offset = layout.type_ref_start + row * layout.type_ref_row_size;
        let name_index = read_index(
            tables_data,
            offset + layout.resolution_scope_size,
            layout.string_index_size,
        )?;
        let namespace_index = read_index(
            tables_data,
            offset + layout.resolution_scope_size + layout.string_index_size,
            layout.string_index_size,
        )?;
        if heap_string(strings_data, name_index)? == type_name
            && heap_string(strings_data, namespace_index)? == namespace
        {
            type_ref_row = Some(row as u32 + 1);
            break;
        }
    }

    let type_ref_row = type_ref_row.ok_or(ManagedMetadataError::MethodNotFound)?;
    let expected_parent = (type_ref_row << 3) | 1;

    for row in 0..layout.rows[10] as usize {
        let offset = layout.member_ref_start + row * layout.member_ref_row_size;
        let parent = read_index(tables_data, offset, layout.member_ref_parent_size)? as u32;
        let name_index = read_index(
            tables_data,
            offset + layout.member_ref_parent_size,
            layout.string_index_size,
        )?;
        if parent == expected_parent && heap_string(strings_data, name_index)? == method_name {
            return Ok(ManagedMemberRef {
                namespace: namespace.to_owned(),
                type_name: type_name.to_owned(),
                method_name: method_name.to_owned(),
                token: 0x0A00_0000 | (row as u32 + 1),
            });
        }
    }

    Err(ManagedMetadataError::MethodNotFound)
}

pub fn method_calls_member_ref(
    image: &[u8],
    method: &ManagedMethodRef,
    target: &ManagedMemberRef,
) -> Result<bool, ManagedMetadataError> {
    if method.method_row == 0 {
        return Err(ManagedMetadataError::Malformed("MethodDef row is zero"));
    }

    let metadata = metadata_root(image)?;
    let streams = metadata_streams(metadata)?;
    let tables = streams
        .iter()
        .find(|stream| stream.name == "#~" || stream.name == "#-")
        .ok_or(ManagedMetadataError::Malformed(
            "missing metadata tables stream",
        ))?;
    let tables_data = subslice(metadata, tables.offset, tables.size)?;
    let layout = TableLayout::parse(tables_data)?;

    if method.method_row > layout.rows[6] {
        return Err(ManagedMetadataError::Malformed(
            "MethodDef row is out of range",
        ));
    }

    let method_offset = layout.method_def_start
        + (method.method_row as usize - 1) * layout.method_def_row_size;
    let method_rva = read_u32(tables_data, method_offset)?;
    let body_offset = rva_to_file_offset(image, method_rva)?;
    let body = method_code(image, body_offset)?;

    let mut cursor = 0usize;
    while cursor < body.len() {
        match body[cursor] {
            0x28 => {
                let token = read_u32(body, cursor + 1)?;
                if token == target.token {
                    return Ok(true);
                }
                cursor += 5;
            }
            0x72 => cursor += 5,
            0x2A => cursor += 1,
            _ => {
                return Err(ManagedMetadataError::Unsupported(
                    "IL opcode is not projected by the R05 call inspector",
                ));
            }
        }
    }

    Ok(false)
}

#[derive(Debug)]
struct StreamHeader {
    name: String,
    offset: usize,
    size: usize,
}

#[derive(Debug)]
struct TableLayout {
    rows: [u32; 64],
    string_index_size: usize,
    resolution_scope_size: usize,
    type_ref_start: usize,
    type_ref_row_size: usize,
    method_def_start: usize,
    method_def_row_size: usize,
    member_ref_start: usize,
    member_ref_row_size: usize,
    member_ref_parent_size: usize,
}

impl TableLayout {
    fn parse(tables: &[u8]) -> Result<Self, ManagedMetadataError> {
        if tables.len() < 24 {
            return Err(ManagedMetadataError::Malformed(
                "tables stream header is truncated",
            ));
        }

        let heap_sizes = tables[6];
        let valid = read_u64(tables, 8)?;
        let mut cursor = 24usize;
        let mut rows = [0u32; 64];
        for (table, row_count) in rows.iter_mut().enumerate() {
            if valid & (1u64 << table) != 0 {
                *row_count = read_u32(tables, cursor)?;
                cursor += 4;
            }
        }

        if rows[3..=5].iter().any(|count| *count != 0)
            || rows[7..=9].iter().any(|count| *count != 0)
        {
            return Err(ManagedMetadataError::Unsupported(
                "intermediate metadata tables are not projected yet",
            ));
        }

        let string_index_size = if heap_sizes & 0x01 != 0 { 4 } else { 2 };
        let guid_index_size = if heap_sizes & 0x02 != 0 { 4 } else { 2 };
        let blob_index_size = if heap_sizes & 0x04 != 0 { 4 } else { 2 };
        let module_row_size = 2 + string_index_size + guid_index_size * 3;
        let resolution_scope_size = coded_index_size(&rows, &[0, 26, 35, 1], 2);
        let type_ref_row_size = resolution_scope_size + string_index_size * 2;
        let type_def_or_ref_size = coded_index_size(&rows, &[2, 1, 27], 2);
        let field_index_size = table_index_size(rows[4]);
        let method_index_size = table_index_size(rows[6]);
        let type_def_row_size =
            4 + string_index_size * 2 + type_def_or_ref_size + field_index_size + method_index_size;
        let param_index_size = table_index_size(rows[8]);
        let method_def_row_size = 8 + string_index_size + blob_index_size + param_index_size;
        let member_ref_parent_size = coded_index_size(&rows, &[2, 1, 26, 6, 27], 3);
        let member_ref_row_size = member_ref_parent_size + string_index_size + blob_index_size;

        let type_ref_start = cursor
            .checked_add(module_row_size * rows[0] as usize)
            .ok_or(ManagedMetadataError::Malformed(
                "metadata table offset overflow",
            ))?;
        let type_def_start = type_ref_start
            .checked_add(type_ref_row_size * rows[1] as usize)
            .ok_or(ManagedMetadataError::Malformed(
                "metadata table offset overflow",
            ))?;
        let method_def_start = type_def_start
            .checked_add(type_def_row_size * rows[2] as usize)
            .ok_or(ManagedMetadataError::Malformed(
                "metadata table offset overflow",
            ))?;
        let member_ref_start = method_def_start
            .checked_add(method_def_row_size * rows[6] as usize)
            .ok_or(ManagedMetadataError::Malformed(
                "metadata table offset overflow",
            ))?;

        Ok(Self {
            rows,
            string_index_size,
            resolution_scope_size,
            type_ref_start,
            type_ref_row_size,
            method_def_start,
            method_def_row_size,
            member_ref_start,
            member_ref_row_size,
            member_ref_parent_size,
        })
    }
}

fn method_code(image: &[u8], body_offset: usize) -> Result<&[u8], ManagedMetadataError> {
    let first = *image
        .get(body_offset)
        .ok_or(ManagedMetadataError::Malformed("missing method header"))?;
    match first & 0x03 {
        0x02 => {
            let code_size = (first >> 2) as usize;
            subslice(image, body_offset + 1, code_size)
        }
        0x03 => {
            let flags_and_size = read_u16(image, body_offset)?;
            let header_size = ((flags_and_size >> 12) as usize) * 4;
            if header_size < 12 {
                return Err(ManagedMetadataError::Malformed(
                    "fat method header is too small",
                ));
            }
            let code_size = read_u32(image, body_offset + 4)? as usize;
            subslice(image, body_offset + header_size, code_size)
        }
        _ => Err(ManagedMetadataError::Unsupported(
            "unsupported managed method header",
        )),
    }
}

fn metadata_root(image: &[u8]) -> Result<&[u8], ManagedMetadataError> {
    let pe_offset = read_u32(image, 0x3c)? as usize;
    if subslice(image, pe_offset, 4)? != b"PE\0\0" {
        return Err(ManagedMetadataError::Malformed("missing PE signature"));
    }

    let coff = pe_offset + 4;
    let optional_size = read_u16(image, coff + 16)? as usize;
    let optional = coff + 20;
    let magic = read_u16(image, optional)?;
    let data_directory = match magic {
        0x10b => optional + 96,
        0x20b => optional + 112,
        _ => {
            return Err(ManagedMetadataError::Unsupported(
                "unsupported PE optional header",
            ));
        }
    };

    let cli_directory = data_directory + 14 * 8;
    let cli_rva = read_u32(image, cli_directory)?;
    let cli_offset = rva_to_file_offset(image, cli_rva)?;
    let metadata_rva = read_u32(image, cli_offset + 8)?;
    let metadata_size = read_u32(image, cli_offset + 12)? as usize;
    let metadata_offset = rva_to_file_offset(image, metadata_rva)?;
    subslice(image, metadata_offset, metadata_size)
}

fn metadata_streams(metadata: &[u8]) -> Result<Vec<StreamHeader>, ManagedMetadataError> {
    if read_u32(metadata, 0)? != 0x424a_5342 {
        return Err(ManagedMetadataError::Malformed(
            "missing CLR metadata signature",
        ));
    }
    let version_len = read_u32(metadata, 12)? as usize;
    let mut cursor = align4(
        16usize
            .checked_add(version_len)
            .ok_or(ManagedMetadataError::Malformed("metadata header overflow"))?,
    );
    let stream_count = read_u16(metadata, cursor + 2)? as usize;
    cursor += 4;

    let mut streams = Vec::with_capacity(stream_count);
    for _ in 0..stream_count {
        let offset = read_u32(metadata, cursor)? as usize;
        let size = read_u32(metadata, cursor + 4)? as usize;
        cursor += 8;
        let name_end = metadata[cursor..]
            .iter()
            .position(|byte| *byte == 0)
            .ok_or(ManagedMetadataError::Malformed("unterminated stream name"))?
            + cursor;
        let name = std::str::from_utf8(&metadata[cursor..name_end])
            .map_err(|_| ManagedMetadataError::Malformed("stream name is not UTF-8"))?
            .to_owned();
        cursor = align4(name_end + 1);
        streams.push(StreamHeader { name, offset, size });
    }
    Ok(streams)
}

fn rva_to_file_offset(image: &[u8], rva: u32) -> Result<usize, ManagedMetadataError> {
    let pe_offset = read_u32(image, 0x3c)? as usize;
    let coff = pe_offset + 4;
    let section_count = read_u16(image, coff + 2)? as usize;
    let optional_size = read_u16(image, coff + 16)? as usize;
    let section_table = coff + 20 + optional_size;

    for index in 0..section_count {
        let section = section_table + index * 40;
        let virtual_size = read_u32(image, section + 8)?;
        let virtual_address = read_u32(image, section + 12)?;
        let raw_size = read_u32(image, section + 16)?;
        let raw_pointer = read_u32(image, section + 20)?;
        let span = virtual_size.max(raw_size);
        if rva >= virtual_address && rva < virtual_address.saturating_add(span) {
            return Ok((raw_pointer + (rva - virtual_address)) as usize);
        }
    }

    Err(ManagedMetadataError::Malformed(
        "RVA does not map to a PE section",
    ))
}

fn coded_index_size(rows: &[u32; 64], tables: &[usize], tag_bits: u32) -> usize {
    let max_rows = tables.iter().map(|table| rows[*table]).max().unwrap_or(0);
    if max_rows < (1u32 << (16 - tag_bits)) {
        2
    } else {
        4
    }
}

fn table_index_size(rows: u32) -> usize {
    if rows < 0x1_0000 {
        2
    } else {
        4
    }
}

fn heap_string(heap: &[u8], index: usize) -> Result<&str, ManagedMetadataError> {
    if index >= heap.len() {
        return Err(ManagedMetadataError::Malformed(
            "string heap index is out of range",
        ));
    }
    let end = heap[index..]
        .iter()
        .position(|byte| *byte == 0)
        .ok_or(ManagedMetadataError::Malformed(
            "unterminated string heap entry",
        ))?
        + index;
    std::str::from_utf8(&heap[index..end])
        .map_err(|_| ManagedMetadataError::Malformed("string heap entry is not UTF-8"))
}

fn read_index(data: &[u8], offset: usize, size: usize) -> Result<usize, ManagedMetadataError> {
    match size {
        2 => Ok(read_u16(data, offset)? as usize),
        4 => Ok(read_u32(data, offset)? as usize),
        _ => Err(ManagedMetadataError::Malformed(
            "invalid metadata index width",
        )),
    }
}

fn read_u16(data: &[u8], offset: usize) -> Result<u16, ManagedMetadataError> {
    let bytes = subslice(data, offset, 2)?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32(data: &[u8], offset: usize) -> Result<u32, ManagedMetadataError> {
    let bytes = subslice(data, offset, 4)?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_u64(data: &[u8], offset: usize) -> Result<u64, ManagedMetadataError> {
    let bytes = subslice(data, offset, 8)?;
    Ok(u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]))
}

fn subslice(data: &[u8], offset: usize, size: usize) -> Result<&[u8], ManagedMetadataError> {
    let end = offset
        .checked_add(size)
        .ok_or(ManagedMetadataError::Malformed("slice offset overflow"))?;
    data.get(offset..end)
        .ok_or(ManagedMetadataError::Malformed("truncated managed image"))
}

const fn align4(value: usize) -> usize {
    (value + 3) & !3
}
