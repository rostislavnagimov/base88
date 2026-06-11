use base88::{decode_bit_string, encode_bit_string};

fn main() {
    let bit_mask =
        "001100101001001001010010011001001011110001100010011000100001010011011100111101000";
    let original_len = bit_mask.len();
    println!("Original string: {}", bit_mask);
    println!("Length before packing: {} bits", original_len);

    let padded_zeros = 8 - (original_len % 8);
    let padded_bit_mask = format!("{}{}", bit_mask, "0".repeat(padded_zeros));

    let packed = encode_bit_string(bit_mask);
    println!("Length after packing: {} bytes", packed.len());
    println!("Encoded string: {}", packed);

    let restored = decode_bit_string(&packed);
    assert_eq!(padded_bit_mask, restored);
    println!("Success! String fully restored.");
}
