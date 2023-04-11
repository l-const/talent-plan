//! A Bitcask instance is a directory, and we
//! enforce that only one operating system process will open that Bitcask for writing at a given time. You can think
//! of that process effectively as the ”database server”. At any moment, one file is ”active” in that directory for
//! writing by the server. When that file meets a size threshold it will be closed and a new active file will be created.
//! Once a file is closed, either purposefully or due to server exit, it is considered immutable and will never be
//! opened for writing again
//!
//! The active file is only written by appending, which means that sequential writes do not require disk seeking.

//! When a write occurs, the keydir is atomically updated with the location of the newest data. The old data is
//! still present on disk, but any new reads will use the latest version available in the keydir. As we’ll see later, the
//! merge process will eventually remove the old value.
//!
//! Reading a value is simple, and doesn’t ever require more than a single disk seek. We look up the key in our
//! keydir, and from there we read the data using the file id, position, and size that are returned from that lookup. In
//! many cases, the operating system’s filesystem read-ahead cache makes this a much faster operation than would
//! be otherwise expected

use serde_json::Deserializer;

// use crate::domain::LogPointer;
use crate::{
    domain::{BufReaderWithPos, BufWriterWithPos, KvsCommand, LogPointer},
    error::Result,
};
use std::io::{Seek, SeekFrom, Write};
use std::{collections::BTreeMap, fs, io::Read};
use std::{collections::HashMap, fs::File, path::PathBuf};

// Threshold in bytes which needs to be exceeded in order to do a compaction operation.
static COMPACTION_THRESHOLD: u64 = 1024 * 1024;
static LOG_FILETYPE_SUFFIX: &'static str = ".log";
static KEY_NOT_FOUND: &'static str = "Key not found";

/// A new in memory key-value store
/// Key/value pairs are persisted to disk in log files. Log files are named after
/// monotonically increasing generation numbers with a `log` extension name.
/// A `BTreeMap` in memory stores the keys and the value locations for fast query.
#[derive(Debug)]
pub struct KvStore {
    /// A keydir is simply a hash
    /// table that maps every key in a Bitcask to a fixed-size structure giving the file, offset, and size of the most recently
    /// written entry for that key

    /// Directory for the log and other data
    path: PathBuf,

    index: BTreeMap<String, LogPointer>,
    // writer of the current log.
    writer: BufWriterWithPos<File>,
    // map generation number to the file reader.
    readers: HashMap<u64, BufReaderWithPos<File>>,
    current_file_idx: u64,
    // the number of bytes representing "stale" commands that could be
    // deleted during a compaction.
    uncompacted: u64,
}

impl KvStore {
    /// Set a new entry to the KvStore
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut store = KvStore::open("./").unwrap();
    /// store.set(String::from("key"), String::from("value"));
    /// assert!(store.get(String::from("key")).is_ok());
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let set_cmd = KvsCommand::Set(crate::domain::Set {
            key: key.clone(),
            value: value.clone(),
        });
        let pos = self.writer.pos;
        serde_json::to_writer(&mut self.writer, &set_cmd)?;
        self.writer.flush()?;
        if let Some(old_cmd) = self
            .index
            .insert(key, (self.current_file_idx, (pos..self.writer.pos)).into())
        {
            self.uncompacted += old_cmd.value_size;
        }

        if self.uncompacted > COMPACTION_THRESHOLD {
            self.compact()?;
        }

        Ok(())
    }

    /// Get a value from the KvStore by specifying the key
    /// Returns the Ok(value) or [`None`] if the key does not exist
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut store = KvStore::open("./").unwrap();
    /// let result = store.get(String::from("key"));
    /// assert!(result.is_ok());
    /// ```
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        match self.index.get(&key) {
            Some(log_pointer) => {
                let reader = self
                    .readers
                    .get_mut(&log_pointer.file_id)
                    .expect("Cannot find log reader");
                reader.seek(SeekFrom::Start(log_pointer.value_pos))?;
                let cmd_reader = reader.take(log_pointer.value_size);
                if let KvsCommand::Set(crate::domain::Set { value, .. }) =
                    serde_json::from_reader(cmd_reader)?
                {
                    return Ok(Some(value));
                } else {
                    return Err("Unexpected deserializes command".into());
                }
            }
            None => Ok(None),
        }
    }
    /// Remove a value from the KvStore
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut store = KvStore::open("./").unwrap();
    /// store.set(String::from("key"), String::from("value"));
    /// store.remove(String::from("key"));
    /// assert!(store.get(String::from("key")).is_ok());
    /// ```
    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.index.contains_key(&key) {
            dbg!("inside");
            let rm_cmd = KvsCommand::Rm(crate::domain::Rm { key: key.clone() });
            serde_json::to_writer(&mut self.writer, &rm_cmd)?;
            self.writer.flush()?;
            let old_cmd = self.index.remove(&key).expect(KEY_NOT_FOUND);
            self.uncompacted += old_cmd.value_size;
            Ok(())
        } else {
            Err(KEY_NOT_FOUND.into())
        }
    }
    /// Opens a `KvStore` with the given path.
    ///
    /// This will create a new directory if the given one does not exist.
    ///
    /// # Errors
    ///
    /// It propagates I/O or deserialization errors during the log replay.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path_buf: PathBuf = path.into();
        fs::create_dir_all(&path_buf)?;

        let mut readers = HashMap::new();
        let mut index = BTreeMap::new();

        let file_idx_list = sorted_file_idx_list(&path_buf)?;
        let mut uncompacted = 0;

        for &file_idx in &file_idx_list {
            let mut reader = BufReaderWithPos::new(File::open(create_path(&path_buf, file_idx))?)?;
            uncompacted += build_index_from_reader(file_idx, &mut reader, &mut index)?;
            readers.insert(file_idx, reader);
        }

        let current_file_idx = file_idx_list.last().unwrap_or(&0) + 1;
        let writer = create_file(&path_buf, current_file_idx, &mut readers)?;

        Ok(KvStore {
            path: path_buf,
            readers,
            index,
            current_file_idx,
            writer,
            uncompacted,
        })
    }

    /// Clears stale entries in the log.
    pub(crate) fn compact(&mut self) -> crate::Result<()> {
        // increase current gen by 2. current_gen + 1 is for the compaction file.
        let cur_file_idx = self.current_file_idx + 1;
        self.current_file_idx += 2;
        self.writer = self.create_file(self.current_file_idx)?;
        let mut compaction_writer = self.create_file(cur_file_idx)?;

        let mut new_pos = 0; // pos in the new log file.
        for log_pointer in &mut self.index.values_mut() {
            let reader = self
                .readers
                .get_mut(&log_pointer.file_id)
                .expect("Cannot find log reader");
            if reader.pos != log_pointer.value_pos {
                reader.seek(SeekFrom::Start(log_pointer.value_pos))?;
            }

            let mut entry_reader = reader.take(log_pointer.value_size);
            let len = std::io::copy(&mut entry_reader, &mut compaction_writer)?;
            *log_pointer = (cur_file_idx, new_pos..new_pos + len).into();
            new_pos += len;
        }
        compaction_writer.flush()?;

        // remove stale log files.
        let stale_gens: Vec<_> = self
            .readers
            .keys()
            .filter(|&&f_idx| f_idx < cur_file_idx)
            .cloned()
            .collect();
        for stale_gen in stale_gens {
            self.readers.remove(&stale_gen);
            fs::remove_file(create_path(&self.path, stale_gen))?;
        }
        self.uncompacted = 0;

        Ok(())
    }

    /// Create a new log file with given generation number and add the reader to the readers map.
    ///
    /// Returns the writer to the log.
    pub(crate) fn create_file(&mut self, file_idx: u64) -> crate::Result<BufWriterWithPos<File>> {
        create_file(&self.path, file_idx, &mut self.readers)
    }
}

fn create_path(dir: &std::path::Path, suffix: u64) -> PathBuf {
    dir.join(format!("{}{}", suffix, LOG_FILETYPE_SUFFIX))
}

fn create_file(
    path: &std::path::Path,
    file_idx: u64,
    readers: &mut HashMap<u64, BufReaderWithPos<File>>,
) -> crate::Result<BufWriterWithPos<File>> {
    let path = create_path(&path, file_idx);
    let writer = BufWriterWithPos::new(
        File::options()
            .create(true)
            .read(true)
            .append(true)
            .open(&path)?,
    )?;
    readers.insert(file_idx, BufReaderWithPos::new(File::open(&path)?)?);
    Ok(writer)
}

fn sorted_file_idx_list(path: &std::path::Path) -> crate::Result<Vec<u64>> {
    let mut file_list: Vec<u64> = fs::read_dir(&path)?
        .flat_map(|res| -> Result<_> { Ok(res?.path()) })
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        .flat_map(|path| {
            path.file_name()
                .and_then(std::ffi::OsStr::to_str)
                .map(|s| s.trim_end_matches(LOG_FILETYPE_SUFFIX))
                .map(str::parse::<u64>)
        })
        .flatten()
        .collect();
    file_list.sort_unstable();
    Ok(file_list)
}

fn build_index_from_reader(
    idx: u64,
    reader: &mut BufReaderWithPos<File>,
    index: &mut BTreeMap<String, LogPointer>,
) -> crate::Result<u64> {
    // Make sure we read from the beginning of the file
    let mut pos = reader.seek(SeekFrom::Start(0))?;
    let mut stream_cmds = Deserializer::from_reader(reader).into_iter::<KvsCommand>();
    let mut uncompacted = 0; // number of bytes that can be saved after a compaction.
    while let Some(Ok(cmd)) = stream_cmds.next() {
        let new_pos = stream_cmds.byte_offset() as u64;
        match cmd {
            KvsCommand::Set(crate::domain::Set { key, .. }) => {
                if let Some(old_cmd) = index.insert(key, (idx, (pos..new_pos)).into()) {
                    uncompacted += old_cmd.value_size;
                }
            }
            KvsCommand::Rm(crate::domain::Rm { key }) => {
                if let Some(old_cmd) = index.remove(&key) {
                    uncompacted += old_cmd.value_size;
                }
                // the "remove" command itself can be deleted in the next compaction.
                // so we add its length to `uncompacted`.
                uncompacted += new_pos - pos;
            }
        }
        pos = new_pos;
    }
    Ok(uncompacted)
}


/// KvsEngine
pub trait KvsEngine {

    /// Set the value of a string key to a string.

    /// Return an error if the value is not written successfully.
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// Get the string value of a string key. If the key does not exist, return None.

    /// Return an error if the value is not read successfully.    
    fn get(&mut self, key: String) -> Result<Option<String>>;
    
    /// Remove a given string key.

    /// Return an error if the key does not exit or value is not read successfully
    fn remove(&mut self, key: String) -> Result<()>;

}