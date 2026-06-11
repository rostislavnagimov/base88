use base88::{decode_digits, encode_digits};
use std::fs;
use std::time::Instant;

const ITERS: usize = 1000;

fn main() {
    let data = fs::read_to_string("tests/test-digits.txt")
        .expect("Put the file with digits in tests/test-digits.txt");

    let data = data.trim().to_string();

    let start = Instant::now();

    for _ in 0..ITERS {
        std::hint::black_box(encode_digits(&data));
    }

    let elapsed = start.elapsed();
    let total_ms = elapsed.as_secs_f64() * 1000.0;
    let avg_ms = total_ms / ITERS as f64;

    println!("encode_digits: avg={:.4}ms/op", avg_ms);

    let encoded = encode_digits(&data);

    let start = Instant::now();

    for _ in 0..ITERS {
        std::hint::black_box(decode_digits(&encoded));
    }

    let elapsed = start.elapsed();
    let total_ms = elapsed.as_secs_f64() * 1000.0;
    let avg_ms = total_ms / ITERS as f64;

    println!("decode_digits: avg={:.4}ms/op", avg_ms);

    let decoded = decode_digits(&encoded);
    assert_eq!(decoded, data, "Data mismatch after roundtrip!");
}
