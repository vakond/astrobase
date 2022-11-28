//! astrobase-server persistent key-value database storage.

use super::{Error, Result};

use std::fs::{File, OpenOptions};
use std::path::Path;

const SEP: &str = "\t";
const DELETED: &str = "\0";

/// Represents the storage.
pub struct Storage {
    file: File,
}

impl Storage {
    /// Opens the storage for reading only.
    pub fn open(filename: &Path) -> Result<Self> {
        let file = File::open(filename).map_err(|e| Error::OpenFile(e, filename.into()))?;
        Ok(Storage { file })
    }

    /// Opens the storage for append (creates new file if missing).
    pub fn open_w(filename: &Path) -> Result<Self> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filename)
            .map_err(|e| Error::OpenFile(e, filename.into()))?;
        Ok(Storage { file })
    }

    /// Searches the key and returns the value or empty string if not found.
    /// We have to scan the entire file because only the last record with given key is actual.
    pub fn find_last(&self, key: &str) -> Result<String> {
        use std::io::{BufRead as _, BufReader};

        let mut value = String::default();
        let reader = BufReader::new(&self.file);
        for line in reader.lines() {
            if let Some(v) = parse(&line?, key)? {
                value = v;
            }
        }

        if value == DELETED {
            value.clear();
        }
        Ok(value)
    }

    /// Writes new record.
    pub fn push(&mut self, key: &str, value: &str) -> Result<String> {
        use std::io::{BufWriter, Write as _};

        let mut writer = BufWriter::new(&self.file);
        writeln!(&mut writer, "{}{}{}", key, SEP, value)?;
        writer.flush()?;

        Ok(String::default())
    }

    /// Adds new record with special value to mark a key as deleted.
    pub fn mark_deleted(&mut self, key: &str) -> Result<String> {
        self.push(key, DELETED)
    }

    /// Collects garbage — removes duplicates and deleted records.
    #[allow(unused)]
    #[allow(clippy::unused_self)]
    pub fn compact(&mut self) -> Result<()> {
        unimplemented!()
    }
}

/// Parses a string retrieving key and value.
/// Returns None if another key present.
fn parse(record: &str, key: &str) -> Result<Option<String>> {
    let mut pair = record.split(SEP);
    let head = pair
        .next()
        .ok_or_else(|| Error::RecordInvalid(record.into()))?;
    if head == key {
        return Ok(Some(
            pair.next()
                .ok_or_else(|| Error::RecordInvalid(record.into()))?
                .into(),
        ));
    }
    Ok(None)
}
