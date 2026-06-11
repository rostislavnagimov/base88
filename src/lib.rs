const ALPHABET: &[u8; 88] =
    b" !#$%()*+,-./0123456789:;=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]^_abcdefghijklmnopqrstuvwxyz{|}~";
const TAIL_CHARS: [usize; 19] = [
    0, 2, 3, 4, 5, 7, 8, 9, 10, 12, 13, 14, 15, 17, 18, 19, 20, 21, 23,
];
const INDEX: [u8; 128] = make_index_table();

const fn make_index_table() -> [u8; 128] {
    let mut table = [0u8; 128];
    let mut i = 0;
    while i < ALPHABET.len() {
        table[ALPHABET[i] as usize] = i as u8;
        i += 1
    }

    table
}

struct U152 {
    hi: u32,
    lo: u128,
}

impl U152 {
    #[inline(always)]
    fn from_be_bytes(b: &[u8]) -> Self {
        let hi = (b[0] as u32) << 16 | (b[1] as u32) << 8 | b[2] as u32;
        let lo = u128::from_be_bytes(b[3..19].try_into().unwrap());
        Self { hi, lo }
    }

    #[inline(always)]
    fn div_rem_88(self) -> (Self, u8) {
        let mut parts = [
            self.hi,
            (self.lo >> 96) as u32,
            (self.lo >> 64) as u32,
            (self.lo >> 32) as u32,
            self.lo as u32,
        ];

        let mut rem = 0u64;
        for p in parts.iter_mut() {
            let cur = rem * (1u64 << 32) + *p as u64;
            *p = (cur / 88) as u32;
            rem = cur % 88;
        }

        let new_hi = parts[0];
        let new_lo = (parts[1] as u128) << 96
            | (parts[2] as u128) << 64
            | (parts[3] as u128) << 32
            | parts[4] as u128;

        (
            Self {
                hi: new_hi,
                lo: new_lo,
            },
            rem as u8,
        )
    }

    #[inline(always)]
    fn mul_add(self, factor: u64, add: u8) -> Self {
        let mut parts = [
            self.hi,
            (self.lo >> 96) as u32,
            (self.lo >> 64) as u32,
            (self.lo >> 32) as u32,
            self.lo as u32,
        ];

        let mut carry = add as u64;
        for p in parts.iter_mut().rev() {
            let cur = *p as u64 * factor + carry;
            *p = cur as u32;
            carry = cur >> 32;
        }

        Self {
            hi: parts[0],
            lo: (parts[1] as u128) << 96
                | (parts[2] as u128) << 64
                | (parts[3] as u128) << 32
                | parts[4] as u128,
        }
    }
}

#[inline(always)]
fn decode_block(chars: &[u8]) -> [u8; 19] {
    let mut n = U152 { hi: 0, lo: 0 };
    for &c in chars {
        n = n.mul_add(88, INDEX[c as usize]);
    }
    let mut bytes = [0u8; 19];
    bytes[0] = (n.hi >> 16) as u8;
    bytes[1] = (n.hi >> 8) as u8;
    bytes[2] = n.hi as u8;
    bytes[3..19].copy_from_slice(&n.lo.to_be_bytes());
    bytes
}

#[inline(always)]
fn decode_block_tail(chars: &[u8]) -> [u8; 19] {
    let mut n = U152 { hi: 0, lo: 0 };
    for &c in chars {
        n = n.mul_add(88, INDEX[c as usize]);
    }
    let mut bytes = [0u8; 19];
    bytes[0] = (n.hi >> 16) as u8;
    bytes[1] = (n.hi >> 8) as u8;
    bytes[2] = n.hi as u8;
    bytes[3..19].copy_from_slice(&n.lo.to_be_bytes());
    bytes
}

pub fn encode(data: &[u8]) -> String {
    let len = data.len();
    let full_blocks = len / 19;
    let tail_bytes = len % 19;
    let total_chars = full_blocks * 24
        + if tail_bytes > 0 {
            TAIL_CHARS[tail_bytes]
        } else {
            0
        };

    let mut buf = vec![0u8; total_chars];

    let mut offset = 0;

    for block in data.chunks(19) {
        let len = block.len();
        if len == 19 {
            let mut n = U152::from_be_bytes(block);

            for j in (0..24).rev() {
                let (new_n, rem) = n.div_rem_88();
                buf[offset + j] = ALPHABET[rem as usize];
                n = new_n;
            }

            offset += 24;
        } else {
            let n_chars = TAIL_CHARS[len];
            let mut padded = [0u8; 19];
            padded[19 - len..].copy_from_slice(block);

            let mut n = U152::from_be_bytes(&padded);

            for j in (0..n_chars).rev() {
                let (new_n, rem) = n.div_rem_88();
                buf[offset + j] = ALPHABET[rem as usize];
                n = new_n;
            }
            offset += n_chars;
        }
    }

    String::from_utf8(buf).unwrap()
}

pub fn decode(s: &str) -> Vec<u8> {
    let s = s.as_bytes();
    let full_blocks = s.len() / 24;
    let tail_chars = s.len() % 24;

    let tail_bytes = if tail_chars > 0 {
        TAIL_CHARS.iter().position(|&c| c == tail_chars).unwrap()
    } else {
        0
    };

    let mut result = vec![0u8; full_blocks * 19 + tail_bytes];

    for i in 0..full_blocks {
        let block = decode_block(&s[i * 24..(i + 1) * 24]);
        result[i * 19..(i + 1) * 19].copy_from_slice(&block);
    }

    if tail_bytes > 0 {
        let block = decode_block_tail(&s[full_blocks * 24..]);
        result[full_blocks * 19..].copy_from_slice(&block[19 - tail_bytes..]);
    }

    result
}

pub fn encode_digits(digits: &str) -> String {
    let len = digits.len();
    let bytes = digits.as_bytes();

    let mut result = String::with_capacity(len);

    let mut i = 0;

    while i < len {
        let number = bytes[i] - b'0';
        if number == 9 || number == 0 {
            result.push(ALPHABET[number as usize] as char);
            i += 1;
            continue;
        }

        if i + 1 < len {
            let next_number = bytes[i + 1] - b'0';
            if number == 8 {
                if next_number > 7 {
                    result.push(ALPHABET[8] as char);
                    i += 1;
                    continue;
                }

                let val = 80 + next_number;

                result.push(ALPHABET[val as usize] as char);
                i += 2;
                continue;
            }

            let val = (number * 10) + next_number;

            result.push(ALPHABET[val as usize] as char);
            i += 2;
            continue;
        } else {
            result.push(ALPHABET[number as usize] as char);
            i += 1;
            continue;
        }
    }

    result
}

pub fn decode_digits(encoded_digits: &str) -> String {
    let bytes = encoded_digits.as_bytes();
    let mut result = String::with_capacity(bytes.len() * 2);

    for &b in bytes {
        let val = INDEX[b as usize];
        if val < 10 {
            result.push((b'0' + val) as char);
        } else {
            result.push((b'0' + (val / 10)) as char);
            result.push((b'0' + (val % 10)) as char);
        }
    }
    result
}

fn pack_bit_string(bit_str: &str) -> Vec<u8> {
    let bytes_len = bit_str.len().div_ceil(8);
    let mut result = Vec::with_capacity(bytes_len);

    for chunk in bit_str.as_bytes().chunks(8) {
        let mut current_byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            if bit == b'1' {
                current_byte |= 1 << (7 - i);
            }
        }
        result.push(current_byte);
    }
    result
}

fn unpack_to_bit_string(bytes: &[u8]) -> String {
    let mut bit_str = String::with_capacity(bytes.len() * 8);
    for &byte in bytes {
        for bit_idx in 0..8 {
            if (byte & (1 << (7 - bit_idx))) != 0 {
                bit_str.push('1');
            } else {
                bit_str.push('0');
            }
        }
    }
    bit_str
}

pub fn encode_bit_string(string: &str) -> String {
    let packed = pack_bit_string(string);
    encode(&packed)
}

pub fn decode_bit_string(string: &str) -> String {
    let decoded = decode(string);
    unpack_to_bit_string(&decoded)
}
