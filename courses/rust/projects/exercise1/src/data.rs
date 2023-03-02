use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Move {
    Left { steps: u8 },
    Right { steps: u8 },
    Up { steps: u8 },
    Down { steps: u8 },
}
