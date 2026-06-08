# base88

Binary-to-text encoding designed for the web. Encodes arbitrary binary data into a string that is valid in JSON, HTML attributes, and JavaScript string literals — without any escaping.

## Why

Existing encodings don't simultaneously satisfy all three web contexts:

| Encoding | Overhead | JSON safe | HTML safe | JS strings safe | Fixed block size |
|:---|:---:|:---:|:---:|:---:|:---:|
| Base64 | +33.3% | ✗ | ✗ | ✗ | ✓ |
| Base85 / Ascii85 | +25.0% | ✗ | ✗ | ✗ | ✓ |
| basE91 | +14–23% | ✗ | ✗ | ✗ | ✗ |
| **Base88** | **+26.3%** | **✓** | **✓** | **✓** | **✓** |

Base88 trades a few percent of density versus basE91 to get safety guarantees across all web contexts and a fixed block size (no branching in the decoder's main loop).

## Alphabet

88 printable ASCII characters — all 95 printable ASCII chars minus 7 that are unsafe in web contexts:

| Removed | Reason |
|:---:|:---|
| `"` | JSON and HTML string delimiter |
| `&` | HTML entity trigger |
| `'` | JS and HTML string delimiter |
| `<` | HTML tag open |
| `>` | HTML tag close |
| `\` | JS/JSON escape character |
| `` ` `` | JS template literal delimiter |

## How it works

19 bytes are treated as a single 152-bit big-endian integer and converted to 24 base-88 digits — exactly like converting a number to a different base. Tail blocks (< 19 bytes) use fewer output characters with no padding.

```
88^24 = 2^155 > 2^152 = 256^19  ✓
overhead = 24/19 - 1 ≈ 26.3%
```

## Performance

Measured on an 854 KB TTF file, release build:

| | encode | decode | output size | overhead |
|:---|:---:|:---:|:---:|:---:|
| base88 | 14.93ms | 7.12ms | 1078 KB | 26.3% |
| base64 | 0.73ms | 0.71ms | 1138 KB | 33.3% |

Base88 is ~20x slower than base64 on encode due to repeated 152-bit division. This is expected — base64 uses only bitshifts while base88 performs bignum arithmetic per block. For typical use cases (fonts, images, config blobs) the absolute times are well within acceptable range. But still will be sped up in upcoming releases.

## Usage

```rust
let encoded = base88::encode(&data);
let decoded = base88::decode(&encoded);
assert_eq!(decoded, data);
```