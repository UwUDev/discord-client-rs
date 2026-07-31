const LENGTH_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LENGTH_EXTRA: [u32; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u32; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];

const HASH_MASK: usize = 8191;
const WINDOW_MASK: usize = 32767;
const MAX_MATCH: usize = 258;
const MAX_DISTANCE: usize = 32768;
const MAX_CHAIN: u32 = 2;
const MIN_COMPRESS_SIZE: usize = 128;

pub struct Compressor {
    key: Vec<u8>,
}

impl Compressor {
    pub fn new(key: Vec<u8>) -> Self {
        Self { key }
    }

    pub fn compress(&self, input: &str) -> String {
        let raw = input.as_bytes();

        let mut body = raw.to_vec();
        let mut compressed = 0u8;
        if raw.len() >= MIN_COMPRESS_SIZE {
            let deflated = deflate_fixed(raw);
            if deflated.len() < raw.len() {
                body = deflated;
                compressed = 1;
            }
        }

        let mut framed = Vec::with_capacity(body.len() + 3);
        framed.extend_from_slice(&[0xFD, 0x01, compressed]);
        framed.extend_from_slice(&body);

        self.scramble(&mut framed);
        self.encode(&framed)
    }

    fn scramble(&self, data: &mut [u8]) {
        let mut state = fnv1a(&self.key);
        for (i, byte) in data.iter_mut().enumerate() {
            state = xorshift32(state);
            *byte ^= (state >> 24) as u8 ^ self.key[i % 64];
        }
    }

    fn encode(&self, data: &[u8]) -> String {
        let rest = data.len() % 3;
        let full = data.len() - rest;
        let mut out = String::with_capacity(full / 3 * 4 + if rest > 0 { rest + 1 } else { 0 });

        let symbol = |n: u32| self.key[n as usize] as char;

        for chunk in data[..full].chunks_exact(3) {
            let n = (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8 | chunk[2] as u32;
            out.push(symbol(n >> 18 & 63));
            out.push(symbol(n >> 12 & 63));
            out.push(symbol(n >> 6 & 63));
            out.push(symbol(n & 63));
        }

        if rest == 1 {
            let n = (data[full] as u32) << 16;
            out.push(symbol(n >> 18 & 63));
            out.push(symbol(n >> 12 & 63));
        } else if rest == 2 {
            let n = (data[full] as u32) << 16 | (data[full + 1] as u32) << 8;
            out.push(symbol(n >> 18 & 63));
            out.push(symbol(n >> 12 & 63));
            out.push(symbol(n >> 6 & 63));
        }

        out
    }
}

fn fnv1a(data: &[u8]) -> u32 {
    let mut hash = 2166136261u32;
    for byte in data {
        hash ^= *byte as u32;
        hash = hash.wrapping_mul(16777619);
    }
    if hash == 0 { 2779062077 } else { hash }
}

fn xorshift32(mut state: u32) -> u32 {
    state ^= state << 13;
    state ^= state >> 17;
    state ^= state << 5;
    state
}

struct BitWriter {
    out: Vec<u8>,
    bits: u32,
    count: u32,
}

impl BitWriter {
    fn new(capacity: usize) -> Self {
        Self {
            out: Vec::with_capacity(capacity),
            bits: 0,
            count: 0,
        }
    }

    fn write(&mut self, value: u32, length: u32) {
        self.bits |= value << self.count;
        self.count += length;
        while self.count >= 8 {
            self.out.push(self.bits as u8);
            self.bits >>= 8;
            self.count -= 8;
        }
    }

    fn finish(mut self) -> Vec<u8> {
        if self.count > 0 {
            self.out.push(self.bits as u8);
        }
        self.out
    }
}

fn reverse_bits(mut value: u32, mut length: u32) -> u32 {
    let mut reversed = 0;
    while length > 0 {
        reversed = reversed << 1 | value & 1;
        value >>= 1;
        length -= 1;
    }
    reversed
}

fn fixed_codes() -> ([u32; 288], [u32; 288]) {
    let mut codes = [0u32; 288];
    let mut lengths = [0u32; 288];
    for symbol in 0..288u32 {
        let (code, length) = match symbol {
            0..=143 => (48 + symbol, 8),
            144..=255 => (256 + symbol, 9),
            256..=279 => (symbol - 256, 7),
            _ => (192 + symbol - 280, 8),
        };
        codes[symbol as usize] = reverse_bits(code, length);
        lengths[symbol as usize] = length;
    }
    (codes, lengths)
}

/// Raw DEFLATE, one final fixed-Huffman block, hash-chain matcher limited to two
/// candidates — a byte-for-byte port of the compressor in Cloudflare's jsd script.
fn deflate_fixed(data: &[u8]) -> Vec<u8> {
    let (codes, lengths) = fixed_codes();
    let mut writer = BitWriter::new(data.len() / 2);
    let mut head = vec![0i32; HASH_MASK + 1];
    let mut prev = vec![0i32; WINDOW_MASK + 1];

    let insert = |head: &mut Vec<i32>, prev: &mut Vec<i32>, position: usize| -> i32 {
        let hash = ((data[position] as usize) << 5
            ^ (data[position + 1] as usize) << 2
            ^ data[position + 2] as usize)
            & HASH_MASK;
        let previous = head[hash] - 1;
        prev[position & WINDOW_MASK] = previous + 1;
        head[hash] = position as i32 + 1;
        previous
    };

    writer.write(1, 1);
    writer.write(1, 2);

    let mut position = 0usize;
    while position < data.len() {
        let mut best_length = 0usize;
        let mut best_distance = 0usize;

        if position + 3 <= data.len() {
            let mut candidate = insert(&mut head, &mut prev, position);
            let mut tried = 0;
            while candidate >= 0
                && (candidate as usize) < position
                && position - candidate as usize <= MAX_DISTANCE
                && tried < MAX_CHAIN
            {
                let start = candidate as usize;
                let limit = (data.len() - position).min(MAX_MATCH);
                let mut length = 0;
                while length < limit && data[start + length] == data[position + length] {
                    length += 1;
                }
                if length > best_length && length > 2 {
                    best_length = length;
                    best_distance = position - start;
                    if length == limit {
                        tried = MAX_CHAIN;
                    }
                }
                candidate = prev[start & WINDOW_MASK] - 1;
                tried += 1;
            }
        }

        if best_length > 2 {
            let index = LENGTH_BASE
                .iter()
                .zip(LENGTH_EXTRA)
                .position(|(base, extra)| best_length <= *base as usize + (1 << extra) - 1)
                .unwrap_or(0);
            let symbol = 257 + index;
            writer.write(codes[symbol], lengths[symbol]);
            if LENGTH_EXTRA[index] > 0 {
                writer.write(
                    (best_length - LENGTH_BASE[index] as usize) as u32,
                    LENGTH_EXTRA[index],
                );
            }

            if let Some(index) = DIST_BASE
                .iter()
                .zip(DIST_EXTRA)
                .position(|(base, extra)| best_distance <= *base as usize + (1 << extra) - 1)
            {
                writer.write(reverse_bits(index as u32, 5), 5);
                if DIST_EXTRA[index] > 0 {
                    writer.write(
                        (best_distance - DIST_BASE[index] as usize) as u32,
                        DIST_EXTRA[index],
                    );
                }
            }

            let mut ahead = 1;
            while ahead < best_length && position + ahead + 3 <= data.len() {
                insert(&mut head, &mut prev, position + ahead);
                ahead += 1;
            }
            position += best_length;
        } else {
            let symbol = data[position] as usize;
            writer.write(codes[symbol], lengths[symbol]);
            position += 1;
        }
    }

    writer.write(codes[256], lengths[256]);
    writer.finish()
}
