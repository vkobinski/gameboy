use super::header::Header;

use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
pub struct Cartridge {
    pub header: Header,
    pub rom: Vec<u8>,
}

impl Cartridge {
    pub fn new(header: Header, reader: &mut BufReader<File>) -> Option<Self> {
        let rom_spec = header.rom_size.clone();
        let rom_size: usize = rom_spec.size_in_bytes();

        let mut rom: Vec<u8> = vec![0; rom_size];

        reader.read_exact(&mut rom).unwrap();

        let cartridge = Self { header, rom };

        cartridge.global_checksum_validation();

        Some(cartridge)
    }

    pub fn global_checksum_validation(&self) {
        let mut sum: u16 = 0;
        for (index, byte) in self.rom.iter().enumerate() {
            // The checksum validation should not count
            // the bytes of the header global checksum itself.
            if index == 0x14E || index == 0x14F {
                continue;
            }

            sum = sum.wrapping_add(u16::from(*byte));
        }

        if sum != self.header.global_checksum {
            panic!("Global checksum validation failed.");
        }
    }
}
