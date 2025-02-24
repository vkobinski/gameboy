mod architecture;
mod cartridge;
mod tests;

use architecture::cpu::{Cpu, Reg8};
use cartridge::header::Header;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;

use crate::cartridge::cartridge::Cartridge;

fn main() {
    let mut cpu = Cpu::new();

    let file = File::open("./games/tetris.gb");

    let read = match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            panic!("Could not open rom.");
        }
    };

    let mut buffer = [0; 0x15F];
    let mut buf_reader = BufReader::new(read);
    let _ = buf_reader
        .read_exact(&mut buffer)
        .expect("Could not read Rom.");

    buf_reader.seek(SeekFrom::Start(0)).unwrap();

    let header = Header::from_bytes(&buffer).unwrap();

    let cartridge = Cartridge::new(header, &mut buf_reader);

    println!("{:?}", cartridge.unwrap().header);

    // test ci
}
