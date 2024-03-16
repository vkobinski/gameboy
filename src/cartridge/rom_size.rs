use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct ByteRomSize {
    pub code: u8,
    pub banks: usize,
    pub size_kb: usize,
}

impl ByteRomSize {
    pub fn from_code(code: u8) -> Option<ByteRomSize> {
        match code {
            0x00 => Some(ByteRomSize { code, banks: 0, size_kb: 32 }),
            0x01 => Some(ByteRomSize { code, banks: 4, size_kb: 64 }),
            0x02 => Some(ByteRomSize { code, banks: 8, size_kb: 128 }),
            0x03 => Some(ByteRomSize { code, banks: 16, size_kb: 256 }),
            0x04 => Some(ByteRomSize { code, banks: 32, size_kb: 512 }),
            0x05 => Some(ByteRomSize { code, banks: 64, size_kb: 1024 }),
            0x06 => Some(ByteRomSize { code, banks: 128, size_kb: 2048 }),
            0x07 => Some(ByteRomSize { code, banks: 256, size_kb: 4096 }),
            0x08 => Some(ByteRomSize { code, banks: 512, size_kb: 8192 }),
            _ => None,
        }
    }
}

impl ByteRomSize {

    pub fn size_in_bytes(&self) -> usize {
        self.size_kb * 1024
    }

}
