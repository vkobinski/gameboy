mod architecture;
mod cartridge;
mod tests;

use architecture::cpu::{Reg8, Cpu};
use std::io::SeekFrom;
use cartridge::header::Header;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::cartridge::cartridge::Cartridge;

fn main()  {
    let mut cpu = Cpu::new();

    cpu.bank.set_8_bit_reg(&Reg8::B, 0xFF);
    cpu.adc_a_r8(Reg8::B);

    let file = File::open("./games/megaman.gb");

    let  read = match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            panic!("Could not open rom.");
        }
    };

    let mut buffer = [0; 0x15F];
    let mut buf_reader = BufReader::new(read);
    let readed = buf_reader.read_exact(&mut buffer).expect("Could not read Rom.");

    buf_reader.seek(SeekFrom::Start(0)).unwrap();

    let header = Header::from_bytes(&buffer).unwrap();

    let cartridge = Cartridge::new(header, &mut buf_reader);

    println!("{:?}", cartridge.unwrap().header);

}
