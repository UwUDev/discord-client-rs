use lz_str::IntoWideIter;

pub struct Compressor {
    key: Vec<u8>,
}

impl Compressor {
    pub fn new(key: Vec<u8>) -> Self {
        Self { key }
    }

    pub fn compress(&self, input: &str) -> String {
        let data: Vec<u16> = input.into_wide_iter().collect();
        let compressed = lz_str::compress_internal(&data, 6, |k| self.key[k as usize] as u16);

        String::from_utf16(&compressed).expect("compression failed")
    }

    #[allow(dead_code)]
    pub fn decompress(&self, input: &str) -> String {
        let compressed: Option<Vec<u16>> = input
            .encode_utf16()
            .flat_map(|c| {
                self.key
                    .iter()
                    .position(|k| u8::try_from(c) == Ok(*k))
                    .map(|n| u16::try_from(n).ok())
            })
            .collect();

        let decompressed = lz_str::decompress_internal(compressed.unwrap().into_wide_iter(), 6)
            .expect("decompression failed");
        String::from_utf16(&decompressed).expect("utf16 string decompression failed")
    }
}
