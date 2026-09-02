//! Minimal managed-metadata projection primitives for R05.
//!
//! This module deliberately starts at the CLR-shaped boundary: Rust reads ECMA-335
//! metadata and identifies an existing managed method. It does not load a native
//! library, use P/Invoke, or shell out to C#/.NET as the implementation.

const METHOD_ATTRIBUTES_MEMBER_ACCESS_MASK: u16 = 0x0007;
const METHOD_ATTRIBUTES_PUBLIC: u16 = 0x0006;
const METHOD_ATTRIBUTES_STATIC: u16 = 0x0010;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedMethodRef {
    pub namespace: String,
    pub type_name: String,
    pub method_name: String,
    pub method_row: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManagedMetadataError {
    Malformed(&'static str),
    Unsupported(&'static str),
    MethodNotFound,
}

#[must_use]
pub fn resolve_public_static_method(
    image: &[u8],
    namespace: &str,
    type_name: &str,
    method_name: &str,
) -> Result<ManagedMethodRef, ManagedMetadataError> {
    let metadata = metadata_root(image)?;
    let streams = metadata_streams(metadata)?;
    let tables = streams
        .iter()
        .find(|stream| stream.name == "#~" || stream.name == "#-")
        .ok_or(ManagedMetadataError::Malformed("missing metadata tables stream"))?;
    let strings = streams
        .iter()
        .find(|stream| stream.name == "#Strings")
        .ok_or(ManagedMetadataError::Malformed("missing #Strings heap"))?;

    let tables_data = subslice(metadata, tables.offset, tables.size)?;
    let strings_data = subslice(metadata, strings.offset, strings.size)?;
    resolve_from_tables(tables_data, strings_data, namespace, type_name, method_name)
}

#[derive(Debug)]
struct StreamHeader {
    name: String,
    offset: usize,
    size: usize,
}

fn resolve_from_tables(
    tables: &[u8],
    strings: &[u8],
    namespace: &str,
    type_name: &str,
    method_name: &str,
) -> Result<ManagedMethodRef, ManagedMetadataError> {
    if tables.len() < 24 {
        return Err(ManagedMetadataError::Malformed("tables stream header is truncated"));
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

    for unsupported in 3..=5 {
        if rows[unsupported] != 0 {
            return Err(ManagedMetadataError::Unsupported(
                "metadata tables between TypeDef and MethodDef are not projected yet",
            ));
        }
    }

    let string_index_size = if heap_sizes & 0x01 != 0 { 4 } else { 2 };
    let guid_index_size = if heap_sizes & 0x02 != 0 { 4 } else { 2 };
    let module_row_size = 2 + string_index_size + guid_index_size * 3;
    let resolution_scope_size = coded_index_size(&rows, &[0, 26, 35, 1], 2);
    let type_ref_row_size = resolution_scope_size + string_index_size * 2;
    let type_def_or_ref_size = coded_index_size(&rows, &[2, 1, 27], 2);
    let field_index_size = table_index_size(rows[4]);
    let method_index_size = table_index_size(rows[6]);
    let type_def_row_size =
        4 + string_index_size * 2 + type_def_or_ref_size + field_index_size + method_index_size;

    cursor = cursor
        .checked_add(module_row_size * rows[0] as usize)
        .and_then(|value| value.checked_add(type_ref_row_size * rows[1] as usize))
        .ok_or(ManagedMetadataError::Malformed("metadata table offset overflow"))?;

    let type_def_start = cursor;
    let method_def_start = type_def_start
        .checked_add(type_def_row_size * rows[2] as usize)
        .ok_or(ManagedMetadataError::Malformed("metadata table offset overflow"))?;

    let blob_index_size = if heap_sizes & 0x04 != 0 { 4 } else { 2 };
    let param_index_size = table_index_size(rows[8]);
    let method_def_row_size = 8 + string_index_size + blob_index_size + param_index_size;

    for type_row in 0..rows[2] as usize {
        let row_offset = type_def_start + type_row * type_def_row_size;
        let name_index = read_index(tables, row_offset + 4, string_index_size)?;
        let namespace_index = read_index(tables, row_offset + 4 + string_index_size, string_index_size)?;
        if heap_string(strings, name_index)? != type_name
            || heap_string(strings, namespace_index)? != namespace
        {
            continue;
        }

        let method_list_offset = row_offset
            + 4
            + string_index_size * 2
            + type_def_or_ref_size
            + field_index_size;
        let first_method = read_index(tables, method_list_offset, method_index_size)? as u32;
        let next_first_method = if type_row + 1 < rows[2] as usize {
            let next_offset = method_list_offset + type_def_row_size;
            read_index(tables, next_offset, method_index_size)? as u32
        } else {
            rows[6] + 1
        };

        for method_row in first_method..next_first_method {
            if method_row == 0 || method_row > rows[6] {
                return Err(ManagedMetadataError::Malformed("invalid MethodList index"));
            }
            let row_offset = method_def_start + (method_row as usize - 1) * method_def_row_size;
            let flags = read_u16(tables, row_offset + 6)?;
            let name_offset = row_offset + 8;
            let name_index = read_index(tables, name_offset, string_index_size)?;
            if heap_string(strings, name_index)? != method_name {
                continue;
            }
            let is_public = flags & METHOD_ATTRIBUTES_MEMBER_ACCESS_MASK == METHOD_ATTRIBUTES_PUBLIC;
            let is_static = flags & METHOD_ATTRIBUTES_STATIC != 0;
            if is_public && is_static {
                return Ok(ManagedMethodRef {
                    namespace: namespace.to_owned(),
                    type_name: type_name.to_owned(),
                    method_name: method_name.to_owned(),
                    method_row,
                });
            }
        }
    }

    Err(ManagedMetadataError::MethodNotFound)
}

fn metadata_root(image: &[u8]) -> Result<&[u8], ManagedMetadataError> {
    let pe_offset = read_u32(image, 0x3c)? as usize;
    if subslice(image, pe_offset, 4)? != b"PE\0\0" {
        return Err(ManagedMetadataError::Malformed("missing PE signature"));
    }

    let coff = pe_offset + 4;
    let section_count = read_u16(image, coff + 2)? as usize;
    let optional_size = read_u16(image, coff + 16)? as usize;
    let optional = coff + 20;
    let magic = read_u16(image, optional)?;
    let data_directory = match magic {
        0x10b => optional + 96,
        0x20b => optional + 112,
        _ => return Err(ManagedMetadataError::Unsupported("unsupported PE optional header")),
    };

    let cli_directory = data_directory + 14 * 8;
    let cli_rva = read_u32(image, cli_directory)?;
    let section_table = optional + optional_size;
    let cli_offset = rva_to_file_offset(image, section_table, section_count, cli_rva)?;
    let metadata_rva = read_u32(image, cli_offset + 8)?;
    let metadata_size = read_u32(image, cli_offset + 12)? as usize;
    let metadata_offset = rva_to_file_offset(image, section_table, section_count, metadata_rva)?;
    subslice(image, metadata_offset, metadata_size)
}

fn metadata_streams(metadata: &[u8]) -> Result<Vec<StreamHeader>, ManagedMetadataError> {
    if read_u32(metadata, 0)? != 0x424a_5342 {
        return Err(ManagedMetadataError::Malformed("missing CLR metadata signature"));
    }
    let version_len = read_u32(metadata, 12)? as usize;
    let mut cursor = 16usize
        .checked_add(version_len)
        .ok_or(ManagedMetadataError::Malformed("metadata header overflow"))?;
    cursor = align4(cursor);
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

fn rva_to_file_offset(
    image: &[u8],
    section_table: usize,
    section_count: usize,
    rva: u32,
) -> Result<usize, ManagedMetadataError> {
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
    Err(ManagedMetadataError::Malformed("RVA does not map to a PE section"))
}

fn coded_index_size(rows: &[u32; 64], tables: &[usize], tag_bits: u32) -> usize {
    let max_rows = tables.iter().map(|table| rows[*table]).max().unwrap_or(0);
    if max_rows < (1u32 << (16 - tag_bits)) { 2 } else { 4 }
}

fn table_index_size(rows: u32) -> usize {
    if rows < 0x1_0000 { 2 } else { 4 }
}

fn heap_string(heap: &[u8], index: usize) -> Result<&str, ManagedMetadataError> {
    if index >= heap.len() {
        return Err(ManagedMetadataError::Malformed("string heap index is out of range"));
    }
    let end = heap[index..]
        .iter()
        .position(|byte| *byte == 0)
        .ok_or(ManagedMetadataError::Malformed("unterminated string heap entry"))?
        + index;
    std::str::from_utf8(&heap[index..end])
        .map_err(|_| ManagedMetadataError::Malformed("string heap entry is not UTF-8"))
}

fn read_index(data: &[u8], offset: usize, size: usize) -> Result<usize, ManagedMetadataError> {
    match size {
        2 => Ok(read_u16(data, offset)? as usize),
        4 => Ok(read_u32(data, offset)? as usize),
        _ => Err(ManagedMetadataError::Malformed("invalid metadata index width")),
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

fn subslice(
    data: &[u8],
    offset: usize,
    size: usize,
) -> Result<&[u8], ManagedMetadataError> {
    let end = offset
        .checked_add(size)
        .ok_or(ManagedMetadataError::Malformed("slice offset overflow"))?;
    data.get(offset..end)
        .ok_or(ManagedMetadataError::Malformed("truncated managed image"))
}

const fn align4(value: usize) -> usize {
    (value + 3) & !3
}

#[cfg(test)]
mod tests {
    use super::{ManagedMetadataError, resolve_public_static_method};

    #[test]
    fn resolves_public_static_method_from_real_managed_metadata() {
        let image = ferrumweave_cil::emit_probe_assembly();

        let method = resolve_public_static_method(&image, "", "<Module>", "Main")
            .expect("R05 resolver should find the managed entry point");

        assert_eq!(method.namespace, "");
        assert_eq!(method.type_name, "<Module>");
        assert_eq!(method.method_name, "Main");
        assert_eq!(method.method_row, 1);
    }

    #[test]
    fn rejects_missing_method_instead_of_fabricating_a_projection() {
        let image = ferrumweave_cil::emit_probe_assembly();

        assert_eq!(
            resolve_public_static_method(&image, "", "<Module>", "DoesNotExist"),
            Err(ManagedMetadataError::MethodNotFound)
        );
    }
}
