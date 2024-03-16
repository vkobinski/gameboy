use super::{header::Header, rom_size};

use std::io::{BufReader, Read};
use std::fs::File;
use std::alloc::{self, Layout};
use std::ptr::NonNull;

pub struct Cartridge {
    pub header: Header,
    pub rom: NonNull<u8>,
}

impl Cartridge {

    pub fn new(header: Header, reader: &mut BufReader<File>) -> Option<Self> {

        let rom_spec = header.rom_size.clone();
        let rom_size: usize = rom_spec.size_in_bytes();

        let layout = Layout::array::<u8>(rom_size).unwrap();
        let rom: NonNull<u8>;

        let mut buffer: Vec<u8> = vec![0; rom_size];

        reader.read_exact(&mut buffer).unwrap();
        
        println!("Rom size: {:02x}", rom_size);

        for (index,byte) in buffer.iter().enumerate() {
            print!("{:02x}:{:02x} ", index, byte);

            if index % 16 == 0 {
                println!();
            }

        }

        unsafe {
            rom = NonNull::new(alloc::alloc(layout)).unwrap();
            rom.as_ptr().copy_from(buffer.as_ptr(), rom_size);
        }

        Some(
            Self {
                header,
                rom
            }
        )
    }

    pub fn global_checksum_validation() {

    }
}
