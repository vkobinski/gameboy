mod architecture;
mod cartridge;
mod tests;

use architecture::cpu::{Reg8, Cpu};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main()  {
    let mut cpu = Cpu::new();

    cpu.bank.set_8_bit_reg(&Reg8::B, 0xFF);
    cpu.adc_a_r8(Reg8::B);

    let file = File::open("./games/tetris.gb");

    let mut read = match file {
        Ok(f) => {
            println!("{:?}", f);
            f
        }
        Err(e) => {
            eprintln!("{}", e);
            panic!("Could not open rom.");
        }
    };

    let mut buffer = [0; 0x14F];
    let mut buf_reader = BufReader::new(read);
    let readed = buf_reader.read_exact(&mut buffer).expect("Could not read Rom.");

    for byte in &buffer[0x0134..0x0143] {
        print!("{} ", *byte as char);

    }

    for (i, byte) in buffer.into_iter().enumerate() {
        print!("{:02x} ", byte);
        if i % 20 == 0 {
            println!()
        }
    }


}
