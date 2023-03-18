use bytes::Bytes;
use bytes_utils::Str;

#[derive(Debug)]
pub(crate) enum FrameKind {
    SimpleString,
    Error,
    Integer,
    BulkString,
    Array,
    Null,
}

#[derive(Debug)]
pub(crate) enum Frame {
    SimpleString(Bytes),
    Error(Str),
    Integer(i64),
    BulkString(Bytes),
    Array(Vec<Frame>),
    Null,
}

