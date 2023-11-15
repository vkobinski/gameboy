mod architecture;
mod tests;

use architecture::cpu::{Cpu, Reg8};

fn main() {
    let mut cpu = Cpu::new();
    cpu.bank.set_8_bit_reg(&Reg8::B, 0xFF);
    cpu.adc_a_r8(Reg8::B);
}
