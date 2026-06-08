use base64::Engine;
use std::fs;
use std::time::Instant;

fn report(name: &str, original: &[u8], encoded: &str, encode_ms: f64, decode_ms: f64) {
    let overhead = (encoded.len() as f64 / original.len() as f64 - 1.0) * 100.0;
    println!(
        "{name}: encode={encode_ms:.2}ms decode={decode_ms:.2}ms \
         original={}KB encoded={}KB overhead={overhead:.1}%",
        original.len() / 1024,
        encoded.len() / 1024,
    );
}

#[test]
fn file_roundtrip() {
    let data = fs::read("tests/test-font.ttf").expect("положи файл в tests/test-font.ttf");

    // --- base88 ---
    let t = Instant::now();
    let b88_encoded = base88::encode(&data);
    let encode_ms = t.elapsed().as_secs_f64() * 1000.0;

    let t = Instant::now();
    let b88_decoded = base88::decode(&b88_encoded);
    let decode_ms = t.elapsed().as_secs_f64() * 1000.0;

    assert_eq!(b88_decoded, data, "base88 roundtrip failed");
    report("base88", &data, &b88_encoded, encode_ms, decode_ms);

    // --- base64 ---
    let engine = base64::engine::general_purpose::STANDARD;

    let t = Instant::now();
    let b64_encoded = engine.encode(&data);
    let encode_ms = t.elapsed().as_secs_f64() * 1000.0;

    let t = Instant::now();
    let b64_decoded = engine.decode(&b64_encoded).unwrap();
    let decode_ms = t.elapsed().as_secs_f64() * 1000.0;

    assert_eq!(b64_decoded, data, "base64 roundtrip failed");
    report("base64", &data, &b64_encoded, encode_ms, decode_ms);
}
