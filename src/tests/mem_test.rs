#[cfg(test)]
mod mem_tests {
    use crate::architecture::{cpu::{Flag, Cpu, Reg8, RegPos, Reg16}, mem::Memory};

    #[test]
    fn write_byte_and_read() {
        let mut mem = Memory::new();
        mem.store_byte(0x02, 128);
        let val = mem.read_byte(0x02);

        assert_eq!(val, 128);
    }

    #[test]
    fn write_byte_and_read_with_reg() {
        let mut cpu = Cpu::new();

        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x01AA);

        let mut mem = Memory::new();
        mem.store_byte(cpu.bank.get_16_bit_reg(&Reg16::HL), 128);
        let val = mem.read_byte(cpu.bank.get_16_bit_reg(&Reg16::HL));

        assert_eq!(val, 128);
    }

}
