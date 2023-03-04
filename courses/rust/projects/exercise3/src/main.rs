use data::{Move, MoveArray};
use std::io::{BufWriter, Write};
mod data;
fn main() {
    let file_path = "./out.bson";
    let values = init_values();
    let file_handle = std::fs::File::create(file_path).unwrap();
    write_bson_documents_to_file(values, &file_handle);
    let reader = std::fs::File::open(file_path).unwrap();
    let mut buf_reader = std::io::BufReader::new(reader);
    read_bson_documents_from_file(&mut buf_reader);
}

fn write_bson_documents_to_file(move_array: [Move; 1000], file_handle: &std::fs::File) {
    let mut writer = BufWriter::new(file_handle);
    let move_arr: MoveArray<1000> = MoveArray { array: move_array };
    let doc = bson::to_document(&move_arr).unwrap();
    doc.to_writer(&mut writer).unwrap();
    writer.flush().unwrap();
}

fn read_bson_documents_from_file(reader: &mut impl std::io::Read) {
    let parsed_doc = bson::document::Document::from_reader(reader).unwrap();
    let move_arr: MoveArray<1000> = bson::from_bson(bson::Bson::Document(parsed_doc)).unwrap();
    println!("Length of move array: {}", move_arr.array.len());
}

fn init_values() -> [Move; 1000] {
    let arr = [Move::Left { steps: 10 }; 1000];
    arr
}
