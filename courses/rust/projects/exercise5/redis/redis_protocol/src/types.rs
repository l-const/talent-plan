// In RESP, the first byte determines the data type:

// For Simple Strings, the first byte of the reply is "+"
// For Errors, the first byte of the reply is "-"
// For Integers, the first byte of the reply is ":"
// For Bulk Strings, the first byte of the reply is "$"
// For Arrays, the first byte of the reply is "*"
// RESP can represent a Null value using a special variation of Bulk Strings or Array as specified later.

// In RESP, different parts of the protocol are always terminated with "\r\n" (CRLF).

/// Byte prefix before a simple string type.
pub const SIMPLESTRING_BYTE: u8 = b'+';
/// Byte prefix before an error type.
pub const ERROR_BYTE: u8 = b'-';
/// Byte prefix before an integer type.
pub const INTEGER_BYTE: u8 = b':';
/// Byte prefix before a bulk string type.
pub const BULKSTRING_BYTE: u8 = b'$';
/// Byte prefix before an array type.
pub const ARRAY_BYTE: u8 = b'*';

/// The binary representation of NULL in RESP2.
pub const NULL: &'static str = "$-1\r\n";



#[derive(Debug)]
pub(crate) enum RespValue {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<RespValue>),
    Null,
}

