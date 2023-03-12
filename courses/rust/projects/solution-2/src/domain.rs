use serde::{Deserialize, Serialize};
use std::io::{BufReader, BufWriter};
use std::io::{Read, Seek, Write};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum KvsCommand {
    Set(Set),
    Rm(Rm),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Set {
    pub(crate) key: String,
    pub(crate) value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Rm {
    pub(crate) key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct LogPointer {
    pub(crate) file_id: u64,
    pub(crate) value_size: u64,
    pub(crate) value_pos: u64,
}

impl From<(u64, std::ops::Range<u64>)> for LogPointer {
    fn from((file_id, range): (u64, std::ops::Range<u64>)) -> Self {
        Self {
            file_id,
            value_size: range.end - range.start,
            value_pos: range.start,
        }
    }
}

#[derive(Debug)]
pub(crate) struct BufReaderWithPos<T: Read + Seek> {
    inner: BufReader<T>,
    pub(crate) pos: u64,
}

impl<RS: Read + Seek> BufReaderWithPos<RS> {
    pub(crate) fn new(mut inner: RS) -> crate::Result<Self> {
        let pos = inner.seek(std::io::SeekFrom::Current(0))?;
        Ok(Self {
            inner: BufReader::new(inner),
            pos,
        })
    }
}

impl<RS: Read + Seek> Seek for BufReaderWithPos<RS> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.inner.seek(pos)
    }
}

impl<RS: Read + Seek> Read for BufReaderWithPos<RS> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.inner.read(buf) {
            Ok(bytes_len) => {
                self.pos += bytes_len as u64;
                return Ok(bytes_len);
            }
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
pub(crate) struct BufWriterWithPos<WS: Write + Seek> {
    inner: BufWriter<WS>,
    pub(crate) pos: u64,
}

impl<WS: Write + Seek> BufWriterWithPos<WS> {
    pub(crate) fn new(mut inner: WS) -> crate::Result<Self> {
        let pos = inner.seek(std::io::SeekFrom::Current(0))?;
        Ok(Self {
            inner: BufWriter::new(inner),
            pos,
        })
    }
}

impl<WS: Write + Seek> Write for BufWriterWithPos<WS> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = self.inner.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl<WS: Write + Seek> Seek for BufWriterWithPos<WS> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.inner.seek(pos)
    }
}
