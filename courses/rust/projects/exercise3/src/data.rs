use serde::ser::SerializeStructVariant;
use serde::Serializer;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Debug, Deserialize, Clone, Copy)]
pub(crate) enum Move {
    Left { steps: u8 },
    Right { steps: u8 },
    Up { steps: u8 },
    Down { steps: u8 },
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct MoveArray<const N: usize> {
    #[serde_as(as = "[_; N]")]
    pub(crate) array: [Move; N],
}

impl Serialize for Move {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let (mut state_variant, steps) = match *self {
            Move::Left { steps } => (
                serializer.serialize_struct_variant("Move", 0, "Left", 1)?,
                steps,
            ),
            Move::Right { steps } => (
                serializer.serialize_struct_variant("Move", 1, "Right", 1)?,
                steps,
            ),
            Move::Up { steps } => (
                serializer.serialize_struct_variant("Move", 2, "Up", 1)?,
                steps,
            ),
            Move::Down { steps } => (
                serializer.serialize_struct_variant("Move", 3, "Down", 1)?,
                steps,
            ),
        };

        state_variant.serialize_field("steps", &steps)?;
        state_variant.end()
    }
}
