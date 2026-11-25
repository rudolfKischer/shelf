use std::collections::HashSet;

use crate::byte_reader::{ByteReader, OutOfBytes};
use crate::vdf_text::Vdf;

const SUPPORTED_MAGIC: u32 = 0x0756_4429;
const ENTRY_METADATA_BYTES: usize = 60;
const END_OF_ENTRIES_APP_ID: u32 = 0;

const TYPE_OBJECT: u8 = 0;
const TYPE_STRING: u8 = 1;
const TYPE_INT32: u8 = 2;
const TYPE_FLOAT: u8 = 3;
const TYPE_UINT64: u8 = 7;
const TYPE_END: u8 = 8;
const TYPE_INT64: u8 = 10;

#[derive(Debug, PartialEq)]
pub enum AppInfoError {
    UnsupportedVersion(u32),
    Truncated,
    BadStringIndex(u32),
    UnknownValueType(u8),
}

impl From<OutOfBytes> for AppInfoError {
    fn from(_: OutOfBytes) -> Self {
        AppInfoError::Truncated
    }
}

#[derive(Debug)]
pub struct AppInfoEntry {
    pub app_id: u32,
    pub root: Vdf,
}

pub fn read_app_info(
    bytes: &[u8],
    wanted: Option<&HashSet<u32>>,
) -> Result<Vec<AppInfoEntry>, AppInfoError> {
    let mut reader = ByteReader::new(bytes);
    let magic = reader.read_u32()?;
    if magic != SUPPORTED_MAGIC {
        return Err(AppInfoError::UnsupportedVersion(magic));
    }
    reader.skip(std::mem::size_of::<u32>())?;
    let string_table_offset = reader.read_u64()? as usize;
    let strings = read_string_table(bytes, string_table_offset)?;
    read_entries(&mut reader, &strings, wanted)
}

fn read_string_table(bytes: &[u8], offset: usize) -> Result<Vec<String>, AppInfoError> {
    let mut reader = ByteReader::new(bytes);
    reader.seek(offset);
    let count = reader.read_u32()?;
    (0..count)
        .map(|_| reader.read_cstring().map_err(AppInfoError::from))
        .collect()
}

fn read_entries(
    reader: &mut ByteReader,
    strings: &[String],
    wanted: Option<&HashSet<u32>>,
) -> Result<Vec<AppInfoEntry>, AppInfoError> {
    let mut entries = Vec::new();
    loop {
        let app_id = reader.read_u32()?;
        if app_id == END_OF_ENTRIES_APP_ID {
            return Ok(entries);
        }
        let size = reader.read_u32()? as usize;
        let entry_end = reader.position() + size;
        if wanted.map_or(true, |set| set.contains(&app_id)) {
            reader.skip(ENTRY_METADATA_BYTES)?;
            let root = Vdf::Map(read_object(reader, strings)?);
            entries.push(AppInfoEntry { app_id, root });
        }
        reader.seek(entry_end);
    }
}

fn read_object(
    reader: &mut ByteReader,
    strings: &[String],
) -> Result<Vec<(String, Vdf)>, AppInfoError> {
    let mut entries = Vec::new();
    loop {
        let value_type = reader.read_u8()?;
        if value_type == TYPE_END {
            return Ok(entries);
        }
        let key_index = reader.read_u32()?;
        let key = strings
            .get(key_index as usize)
            .ok_or(AppInfoError::BadStringIndex(key_index))?;
        entries.push((key.clone(), read_value(reader, strings, value_type)?));
    }
}

fn read_value(
    reader: &mut ByteReader,
    strings: &[String],
    value_type: u8,
) -> Result<Vdf, AppInfoError> {
    Ok(match value_type {
        TYPE_OBJECT => Vdf::Map(read_object(reader, strings)?),
        TYPE_STRING => Vdf::Text(reader.read_cstring()?),
        TYPE_INT32 => Vdf::Text(reader.read_i32()?.to_string()),
        TYPE_FLOAT => Vdf::Text(reader.read_f32()?.to_string()),
        TYPE_UINT64 => Vdf::Text(reader.read_u64()?.to_string()),
        TYPE_INT64 => Vdf::Text(reader.read_i64()?.to_string()),
        other => return Err(AppInfoError::UnknownValueType(other)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unknown_version() {
        let bytes = [0u8; 16];
        assert_eq!(
            read_app_info(&bytes, None).unwrap_err(),
            AppInfoError::UnsupportedVersion(0)
        );
    }
}
