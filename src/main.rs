mod architecture;
mod tests;

use architecture::cpu::Reg8;
use architecture::bus::Bus;

fn main() {
    let mut bus = Bus::new();
    bus.cpu.bank.set_8_bit_reg(&Reg8::B, 0xFF);
    bus.cpu.adc_a_r8(Reg8::B);
}
