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
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x1);
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x2);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.adc_a_r8(Reg8::B);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), (0x1 + 0x2) + 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn adc_a_r8_arbitrary() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0xC);
        cpu.bank.set_8_bit_reg(&Reg8::A, 0xD);
        cpu.adc_a_r8(Reg8::B);

        let op1 : u8 = 0xC;
        let op2 : u8 = 0xD;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn adc_a_n8() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0xD);
        cpu.adc_a_n8(0xC);

        let op1 : u8 = 0xC;
        let op2 : u8 = 0xD;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn adc_a_n8_decimal() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 120);
        cpu.adc_a_n8(40);

        let op1 : u8 = 120;
        let op2 : u8 = 40;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }


    #[test]
    fn adc_a_n8_no_carry() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x1);
        cpu.adc_a_n8(0x2);

        let op1 : u8 = 0x1;
        let op2 : u8 = 0x2;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
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

    #[test]
    fn adc_a_hl_carry_before() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x01AA);
        cpu.bank.set_flag(Flag::CARRY);

        cpu.bus.mem.store_byte(cpu.bank.get_16_bit_reg(&Reg16::HL), 128);

        cpu.adc_a_hl();

        let val = cpu.bank.get_8_bit_reg(&Reg8::A);

        assert_eq!(val, 129);
    }

    #[test]
    fn add_a_r8_with_carry_after() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0xFF);
        cpu.bank.set_8_bit_reg(&Reg8::A, 0xFF);
        cpu.add_a_r8(Reg8::B);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0xFF - 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 1);

    }

    #[test]
    fn add_a_r8_with_carry_before() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x1);
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x2);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.add_a_r8(Reg8::B);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), (0x1 + 0x2));
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);

    }

    #[test]
    fn add_a_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x01AA);

        cpu.bus.mem.store_byte(cpu.bank.get_16_bit_reg(&Reg16::HL), 128);

        cpu.add_a_hl();

        let val = cpu.bank.get_8_bit_reg(&Reg8::A);

        assert_eq!(val, 128);
    }

    #[test]
    fn add_a_hl_carry_before() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x01AA);
        cpu.bank.set_flag(Flag::CARRY);

        cpu.bus.mem.store_byte(cpu.bank.get_16_bit_reg(&Reg16::HL), 128);

        cpu.add_a_hl();

        let val = cpu.bank.get_8_bit_reg(&Reg8::A);

        assert_eq!(val, 128);
    }

    #[test]
    fn add_a_n8() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0xAD);
        cpu.add_a_n8(0xBC);

        let op1 : u8 = 0xBC;
        let op2 : u8 = 0xAD;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 1);
    }

    #[test]
    fn add_a_n8_carry_before() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x01);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.add_a_n8(0x02);

        let op1 : u8 = 0x01;
        let op2 : u8 = 0x02;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
    }

    #[test]

    fn add_hl_r16() {
       let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x12);
        cpu.bank.set_16_bit_reg(&Reg16::BC, 0x24);
        cpu.add_hl_r16(&Reg16::BC);

        let op1 : u16 = 0x12;
        let op2 : u16 = 0x24;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::HL), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn add_hl_r16_with_carry_before() {
       let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x12);
        cpu.bank.set_16_bit_reg(&Reg16::BC, 0x24);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.add_hl_r16(&Reg16::BC);

        let op1 : u16 = 0x12;
        let op2 : u16 = 0x24;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::HL), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn add_hl_sp() {
       let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x12);
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0x24);
        cpu.add_hl_sp();

        let op1 : u16 = 0x12;
        let op2 : u16 = 0x24;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::HL), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn add_sp_e8() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0x2424);
        cpu.add_sp_e8(-50);

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0x23F2);
    }

    #[test]
    fn and_a_r8() {
       let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x12);
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x24);
        cpu.and_a_r8(&Reg8::B);

        let op1 : u8 = 0x12;
        let op2 : u8 = 0x24;

        let sum = op1 & op2;

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn and_a_r8_with_no_zero_flag() {
       let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b0010);
        cpu.bank.set_8_bit_reg(&Reg8::B, 0b0011);
        cpu.and_a_r8(&Reg8::B);

        let op1 : u8 = 0b0010;
        let op2 : u8 = 0b0011;

        let sum = op1 & op2;

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }
}
