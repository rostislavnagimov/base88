use base88::{decode, encode};
use std::fs;

fn main() {
    let file_path = "tests/test-font.ttf";
    let file_data = fs::read(file_path).expect("Put the file in tests/test-font.ttf");
    let original_len = file_data.len();

    println!("Length before packing: {} bytes", original_len);

    let packed = encode(&file_data);
    println!("Length after packing: {} bytes", packed.len());

    let restored = decode(&packed);
    assert_eq!(file_data, restored);
    println!("Success! String fully restored.");
}
