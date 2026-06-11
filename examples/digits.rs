use base88::{decode_digits, encode_digits};

fn main() {
    let bit_mask =
        "753260148214738650860154732325081476087546321146372805578623014602417583431805267";
    let original_len = bit_mask.len();

    println!("Original string: {}", bit_mask);
    println!("Original length: {}", original_len);

    let encoded_str = encode_digits(bit_mask);
    println!("Encoded string: {}", encoded_str);
    println!("Final length in characters: {}", encoded_str.len());

    let decoded_str = decode_digits(&encoded_str);
    println!("Decoded string: {}", decoded_str);

    assert_eq!(
        decoded_str, bit_mask,
        "got: {} expected: {}",
        decoded_str, bit_mask
    );
}
