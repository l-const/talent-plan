use chrono::Utc;
use serde::{Deserialize, Serialize};

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
    file_id: u8,
    value_size: u32,
    value_pos: u32,
    timestamp: chrono::DateTime<Utc>,
}
