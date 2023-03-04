/// - **Exercise: Serialize and deserialize a data structure to a buffer with
///    `serde` (RON)**.
///
///   Do the same as above, except this time, instead of serializing to a `File`,
///   serialize to a `Vec<u8>` buffer, and after that try using [RON] instead of
///   JSON as the format. Are there any differences in serialization to a `Vec`
///   instead of a `File`? What about in using the RON crate vs the JSON crate?
/// 
///   Convert the `Vec<u8>` to `String` with [`str::from_utf8`], unwrapping the
///   result, then print that serialized string representation to see what `Move`
///   looks like serialized to RON.

use std::io::Write;

use serde::Serialize;

mod data;
fn main() {
    pub(crate) use crate::data::Move;
    let a = Move::Left { steps: 9 };
    let mut buffer: Vec<u8> = vec![];
    //  Serialize to buffer : Vec<u8>
    a.serialize(&mut ron::ser::Serializer::new(&mut buffer, None).unwrap()).unwrap();

    println!("Move value a bytes len: {:?}", buffer.len());
    let buffer_str = String::from_utf8(buffer.clone()).unwrap();
    dbg!(&buffer_str); // &buffer_str = "Left(steps:9)"
    
}
