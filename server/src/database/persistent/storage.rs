//! astrobase-server persistent key-value database storage.

use anyhow::{anyhow, Context as _};
use std::fs::{File, OpenOptions};
use std::io::{BufRead as _, BufReader, BufWriter, Write as _};
use std::path::Path;

const SEP: &str = "\t";
const DELETED: &str = "\0";

/// Represents the storage.
pub struct Storage {
    file: File,
}

impl Storage {
    /// Opens the storage for reading only.
    pub fn open(filename: &Path) -> anyhow::Result<Self> {
        let text = format!("No such file {:?}", filename);
        let file = File::open(filename).context(text)?;
        Ok(Storage { file })
    }

    /// Opens the storage for append (creates new file if missing).
    pub fn open_w(filename: &Path) -> anyhow::Result<Self> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filename)?;
        Ok(Storage { file })
    }

    /// Searches the key and returns the value or empty string if not found.
    /// We have to scan the entire file because only the last record with given key is actual.
    pub fn find_last(&self, key: &str) -> anyhow::Result<String> {
        let mut value = String::default();
        let reader = BufReader::new(&self.file);
        for line in reader.lines() {
            let record = line?;
            let mut parts = record.split(SEP);
            let head = parts
                .next()
                .ok_or_else(|| anyhow!("Invalid record '{}'", record))?;
            if head == key {
                value = parts
                    .next()
                    .ok_or_else(|| anyhow!("Invalid record '{}'", record))?
                    .into();
            }
        }
        if value == DELETED {
            value.clear();
        }
        Ok(value)
    }

    /// Appends new record to the end.
    pub fn append(&mut self, key: &str, value: &str) -> anyhow::Result<String> {
        let mut writer = BufWriter::new(&self.file);
        writeln!(&mut writer, "{}\t{}", key, value)?;
        writer.flush()?;
        Ok(String::default())
    }

    /// Appends new record with special value to mark a key as deleted.
    pub fn mark_deleted(&mut self, key: &str) -> anyhow::Result<String> {
        self.append(key, DELETED)
    }

    /// Collects garbage — removes duplicates and deleted records.
    #[allow(unused)]
    pub fn compact(&mut self) -> anyhow::Result<()> {
        unimplemented!()
    }
}
