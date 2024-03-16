#[cfg(test)]
mod rom_size_tests {
    use crate::cartridge::rom_size::{self, ByteRomSize};

    #[test]
    fn test_size_in_bytes_32() {
        let size = ByteRomSize::from_code(0x00).unwrap();
        assert_eq!(size.size_in_bytes(), 0x8000);
    }

    #[test]
    fn test_size_in_bytes_512() {
        let size = ByteRomSize::from_code(0x04).unwrap();
        assert_eq!(size.size_in_bytes(), 0x80000);
    }

    #[test]
    fn test_size_in_bytes_1m() {
        let size = ByteRomSize::from_code(0x05).unwrap();
        assert_eq!(size.size_in_bytes(), 0x100000);
    }


}
