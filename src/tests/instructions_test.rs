#[cfg(test)]
mod ins_tests {
    use crate::architecture::{cpu::{Flag, Cpu, Reg8, Reg16}, mem::Memory};

    #[test]
    fn adc_a_r8_no_carry() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0xFF);
        cpu.adc_a_r8(Reg8::B);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0xFF);
    }

    #[test]
    fn adc_a_r8_with_carry_after() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0xFF);
        cpu.bank.set_8_bit_reg(&Reg8::A, 0xFF);
        cpu.adc_a_r8(Reg8::B);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0xFF - 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 1);

    }

    #[test]
    fn adc_a_r8_with_carry_before() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x01);
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x02);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.adc_a_r8(Reg8::B);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), (0x01 + 0x02) + 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 1);

    }


    #[test]
    fn adc_a_r8_arbitrary() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0xBC);
        cpu.bank.set_8_bit_reg(&Reg8::A, 0xAD);
        cpu.adc_a_r8(Reg8::B);

        let op1 : u8 = 0xBC;
        let op2 : u8 = 0xAD;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 1);
    }

    #[test]
    fn adc_a_n8() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0xAD);
        cpu.adc_a_n8(0xBC);

        let op1 : u8 = 0xBC;
        let op2 : u8 = 0xAD;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 1);
    }

    #[test]
    fn adc_a_n8_no_carry() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x01);
        cpu.adc_a_n8(0xBC);

        let op1 : u8 = 0x01;
        let op2 : u8 = 0xBC;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 1);
    }




    #[test]
    fn adc_a_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x01AA);

        cpu.bus.mem.store_byte(cpu.bank.get_16_bit_reg(&Reg16::HL), 128);

        cpu.adc_a_hl();

        let val = cpu.bank.get_8_bit_reg(&Reg8::A);

        assert_eq!(val, 128);
    }

}
