use base88::{decode, encode};
use std::fs;
use std::time::Instant;

const ITERS: usize = 1000;

fn main() {
    let data = fs::read("tests/test-font.ttf").expect("Put the file in tests/test-font.ttf");

    let start = Instant::now();

    for _ in 0..ITERS {
        std::hint::black_box(encode(&data));
    }

    let elapsed = start.elapsed();
    let total_ms = elapsed.as_secs_f64() * 1000.0;
    let avg_ms = total_ms / ITERS as f64;

    println!("encode: avg={:.4}ms/op", avg_ms,);

    let encoded = encode(&data);
    let start = Instant::now();

    for _ in 0..ITERS {
        std::hint::black_box(decode(&encoded));
    }

    let elapsed = start.elapsed();
    let total_ms = elapsed.as_secs_f64() * 1000.0;
    let avg_ms = total_ms / ITERS as f64;

    println!("decode: avg={:.4}ms/op", avg_ms,);
}
