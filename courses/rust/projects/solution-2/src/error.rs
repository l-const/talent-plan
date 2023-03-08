/// Crate level error type
#[derive(Debug)]
pub struct KvsError<T: AsRef<str>> {
    /// A message field describing the error
    pub msg: T,
}

impl From<std::io::Error> for KvsError<String> {
    fn from(err: std::io::Error) -> Self {
        KvsError {
            msg: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for KvsError<String> {
    fn from(err: serde_json::Error) -> Self {
        KvsError {
            msg: err.to_string(),
        }
    }
}

/// Crate level result type
pub type Result<T> = std::result::Result<T, KvsError<String>>;
