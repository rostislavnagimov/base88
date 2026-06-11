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

Measured on an 854 KB TTF file, release build (Apple M1):

| | encode | decode | output size | overhead |
|:---|:---:|:---:|:---:|:---:|
| base88 (current) | **3.87ms** | **1.54ms** | 1078 KB | 26.3% |
| base64 | 0.73ms | 0.71ms | 1138 KB | 33.3% |

For typical web payloads (fonts, images, configuration blobs) these absolute times are negligible — e.g., encoding a 1 MB file takes ~4 ms, decoding ~1.5 ms.

## Utility Functions

### Digit‑String Compression

```rust
pub fn encode_digits(digits: &str) -> String
pub fn decode_digits(encoded: &str) -> String
```

Compresses strings that consist only of decimal digits (0‑9) by packing two digits into one base‑88 character (when possible). Single digits `0` and `9` stay as‑is for correct round‑tripping. To achieve the best compression (2x) you can adjust your data to 0-87 zone, avoiding 88, 89, 9 and 0.

Example:
```
"753260148214738650860154732325081476087546321146372805578623014602417583431805267" (81 digits) → "rEc1y1p}W} 2TEEWxTc~[f81fow]ueC1c;4aGDwYj" (41 chars)
```

Benchmark (for a random 10_000‑digit string):
- `encode_digits`: **0.0190 ms/op**
- `decode_digits`: **0.0101 ms/op**

### Bit‑String Compression

```rust
pub fn encode_bit_string(string: &str) -> String
pub fn decode_bit_string(string: &str) -> String
```

Accepts a string of `'0'` and `'1'`, packs it into bytes (big‑endian, left‑aligned), then applies base88 encoding. Useful for compressing binary flags or boolean arrays.

Warning: if your data length is not divisible by 8 (e.g. 81) you will get extra zeros to fit the byte size at the end of the decoded string.

Example:
```
"001100101001001001010010011001001011110001100010011000100001010011011100111101000" (81 digits) → "$6Ni^zLc(Ho0D;" (14 chars)
```

## Usage

```rust
let binary = b"Hello, world!";
let encoded = base88::encode(binary);
let decoded = base88::decode(&encoded);
assert_eq!(decoded, binary);

let digits = "12345678901234567890";
let compact = base88::encode_digits(digits);
let restored = base88::decode_digits(&compact);
assert_eq!(restored, digits);

let bits = "101100101010";
let packed = base88::encode_bit_string(bits);
let unpacked = base88::decode_bit_string(&packed);
assert_eq!(unpacked, bits);
```