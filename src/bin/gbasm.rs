use std::env;
use std::fs;
use std::process::ExitCode;

use gameboy::architecture::assembler::assemble;
use gameboy::architecture::cpu::{Cpu, Flag, Reg16, Reg8};

fn main() -> ExitCode {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: gbasm <path-to-asm-file>");
            return ExitCode::FAILURE;
        }
    };

    let source = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("could not read {}: {}", path, e);
            return ExitCode::FAILURE;
        }
    };

    let program = match assemble(&source) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("assembly error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let mut cpu = Cpu::new();

    for (i, byte) in program.iter().enumerate() {
        cpu.bus.mem.store_byte(i as u16, *byte);
    }

    while (cpu.bank.pc.get_value() as usize) < program.len() {
        cpu.step();
    }

    print_registers(&cpu);

    ExitCode::SUCCESS
}

fn print_registers(cpu: &Cpu) {
    println!("A  = {:#04X}", cpu.bank.get_8_bit_reg(&Reg8::A));
    println!("B  = {:#04X}", cpu.bank.get_8_bit_reg(&Reg8::B));
    println!("C  = {:#04X}", cpu.bank.get_8_bit_reg(&Reg8::C));
    println!("D  = {:#04X}", cpu.bank.get_8_bit_reg(&Reg8::D));
    println!("E  = {:#04X}", cpu.bank.get_8_bit_reg(&Reg8::E));
    println!("H  = {:#04X}", cpu.bank.get_8_bit_reg(&Reg8::H));
    println!("L  = {:#04X}", cpu.bank.get_8_bit_reg(&Reg8::L));
    println!("BC = {:#06X}", cpu.bank.get_16_bit_reg(&Reg16::BC));
    println!("DE = {:#06X}", cpu.bank.get_16_bit_reg(&Reg16::DE));
    println!("HL = {:#06X}", cpu.bank.get_16_bit_reg(&Reg16::HL));
    println!("SP = {:#06X}", cpu.bank.get_16_bit_reg(&Reg16::SP));
    println!("PC = {:#06X}", cpu.bank.pc.get_value());
    println!(
        "flags: Z={} N={} H={} C={}",
        cpu.bank.get_flag(Flag::ZERO),
        cpu.bank.get_flag(Flag::SUB),
        cpu.bank.get_flag(Flag::HALFCARRY),
        cpu.bank.get_flag(Flag::CARRY),
    );
}
