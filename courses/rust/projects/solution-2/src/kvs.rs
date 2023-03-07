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

use crate::{error::KvsError, ser::KvsCommand};
use std::io::{BufWriter, Write, BufReader, Read, Seek};
use std::{collections::HashMap, fs::File, path::PathBuf};

/// A new in memory key-value store
#[derive(Debug)]
pub struct KvStore {
    /// A keydir is simply a hash
    /// table that maps every key in a Bitcask to a fixed-size structure giving the file, offset, and size of the most recently
    /// written entry for that key
    map: HashMap<String, String>,
    writer: BufWriter<File>,
    reader: BufReader<File>,
}

/// Crate level result type
pub type Result<T> = std::result::Result<T, KvsError<String>>;

const KEY_NOT_FOUND: &'static str = "Key not found";

fn key_not_found() -> KvsError<String> {
    KvsError {
        msg: KEY_NOT_FOUND.into(),
    }
}

impl KvStore {
    /// Set a new entry to the KvStore
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set(String::from("key"), String::from("value"));
    /// assert!(store.get(String::from("key")).is_ok());
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let set_cmd = KvsCommand::Set(crate::ser::Set {
            key: key.clone(),
            value: value.clone(),
        });
        if let Some(_) = self.map.insert(key, value) {
            self.write_log(set_cmd)?;
            Ok(())
        } else {
            self.write_log(set_cmd)?;
            Ok(())
        }
    }
    /// Get a value from the KvStore by specifying the key
    /// Returns the Ok(value) or [`None`] if the key does not exist
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// let result = store.get(String::from("key"));
    /// assert!(result.is_ok());
    /// ```
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        self.build_index();
        match self.map.get(&key) {
            Some(value) => Ok(Some(value.to_string())),
            None => Ok(None),
        }
    }
    /// Remove a value from the KvStore
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set(String::from("key"), String::from("value"));
    /// store.remove(String::from("key"));
    /// assert!(store.get(String::from("key")).is_ok());
    /// ```
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.build_index();
        //TODO: maybe check when writing the rm cmd.
        if self.map.is_empty() {
            return Err(key_not_found());
        }
        if let Some(_) = self.map.remove(&key) {
            let rm_cmd = KvsCommand::Rm(crate::ser::Rm { key: key.clone() });
            self.write_log(rm_cmd)?;
            Ok(())
        } else {
            Err(key_not_found())
        }
    }

    /// TODO:
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut path_buf: PathBuf = path.into();
        let log_file_handle = Self::create_file(path_buf.clone())?;
        path_buf.push(".log");
        let reader = BufReader::new(File::open(path_buf)?);
        let writer = std::io::BufWriter::new(log_file_handle);
        Ok(Self {
            map: HashMap::new(),
            writer,
            reader,
        })
    }

    pub(crate) fn create_file(mut log_path: PathBuf) -> Result<File> {
        if !std::path::Path::exists(&log_path) {
            std::fs::create_dir(&log_path)?;
        }
        log_path.push(".log");
        let log_file_handle = File::options()
            .create(true)
            .read(true)
            .append(true)
            .open(log_path)?;
        Ok(log_file_handle)
    }

    pub(crate) fn write_log(&mut self, cmd: KvsCommand) -> Result<()> {
        let str = serde_json::to_string(&cmd)?;
        self.writer.write_all(str.as_bytes())?;
        self.writer.flush()?;
        Ok(())
    }


    pub(crate) fn read_log(&mut self) -> Result<Vec<KvsCommand>> {
        let cmds: Vec<KvsCommand> = serde_json::from_reader(&mut self.reader)?;
        // self.reader.stream_position();
        Ok(cmds)
    }

    pub(crate) fn build_index(&mut self) {
        let cmds = self.read_log().unwrap();
        for cmd in cmds.into_iter() {
            if let KvsCommand::Set(set_cmd) = cmd {
                let key = set_cmd.key;
                let value = set_cmd.value;
                self.map.insert(key, value);
            }
       }
    }

    pub(crate) fn close(&mut self) -> Result<()> {
        self.writer.flush()?;
        Ok(())
    }

}
