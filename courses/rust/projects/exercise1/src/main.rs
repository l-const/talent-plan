//! Exercise: Serialize and deserialize a data structure with serde (JSON).
//! This exercise and the next two will introduce basic serialization and deserialization with serde. serde serializes data quickly and is easy to use, while also being extensible and expressive.
//! For your serializable data structure, imagine a flat game-playing surface covered in a grid of squares, like a chess board. Imagine you have a game character that every turn may move any number of squares in a single direction. Define a type, Move that represents a single move of that character.
//! Derive the Debug trait so Move is easily printable with the {:?} format specifier.
//! Write a main function that defines a variable, a, of type Move, serializes it with serde to a File, then deserializes it back again to a variable, b, also of type Move.
//! Use JSON as the serialization format.
//! Print a and b with println! and the {:?} format specifier to verify successful deserializat

use std::io::Write;

// use serde::Serialize;

mod data;
fn main() {
    pub(crate) use crate::data::Move;
    let file_handle = std::fs::File::create("./out.json").unwrap();
    let a = crate::data::Move::Left { steps: 7 };
    println!("Initial Move value a:, {:?}", a);
    let mut writer = std::io::BufWriter::new(&file_handle);
    writer.flush().unwrap();
    // Serialize move value to file
    serde_json::to_writer(writer, &a).unwrap();
    // a.serialize(&mut serde_json::Serializer::new(&mut writer));
    // Deserialize move value from file
    let read_fd = std::fs::File::open("./out.json").unwrap();
    let reader = std::io::BufReader::new(read_fd);
    let b: Move = serde_json::from_reader(reader).unwrap();
    println!("Move value b read from file, {:?}", b);
}
