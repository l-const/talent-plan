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
