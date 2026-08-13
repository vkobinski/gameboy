#[cfg(test)]
mod ins_tests {
    use crate::architecture::{
        cpu::{Cpu, Flag, Reg16, Reg8},
        mem::Memory,
    };

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

        let op1: u8 = 0xC;
        let op2: u8 = 0xD;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn adc_a_n8() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0xD);
        cpu.bus.mem.store_byte(0, 0xC);
        cpu.adc_a_n8();

        let op1: u8 = 0xC;
        let op2: u8 = 0xD;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn adc_a_n8_decimal() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 120);
        cpu.bus.mem.store_byte(0, 40);
        cpu.adc_a_n8();

        let op1: u8 = 120;
        let op2: u8 = 40;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn adc_a_n8_no_carry() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x1);
        cpu.bus.mem.store_byte(0, 0x2);
        cpu.adc_a_n8();

        let op1: u8 = 0x1;
        let op2: u8 = 0x2;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn adc_a_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x01AA);

        cpu.bus
            .mem
            .store_byte(cpu.bank.get_16_bit_reg(&Reg16::HL), 128);

        cpu.adc_a_hl();

        let val = cpu.bank.get_8_bit_reg(&Reg8::A);

        assert_eq!(val, 128);
    }

    #[test]
    fn adc_a_hl_carry_before() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x01AA);
        cpu.bank.set_flag(Flag::CARRY);

        cpu.bus
            .mem
            .store_byte(cpu.bank.get_16_bit_reg(&Reg16::HL), 128);

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

        cpu.bus
            .mem
            .store_byte(cpu.bank.get_16_bit_reg(&Reg16::HL), 128);

        cpu.add_a_hl();

        let val = cpu.bank.get_8_bit_reg(&Reg8::A);

        assert_eq!(val, 128);
    }

    #[test]
    fn add_a_hl_carry_before() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x01AA);
        cpu.bank.set_flag(Flag::CARRY);

        cpu.bus
            .mem
            .store_byte(cpu.bank.get_16_bit_reg(&Reg16::HL), 128);

        cpu.add_a_hl();

        let val = cpu.bank.get_8_bit_reg(&Reg8::A);

        assert_eq!(val, 128);
    }

    #[test]
    fn add_a_n8() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0xAD);
        cpu.bus.mem.store_byte(0, 0xBC);
        cpu.add_a_n8();

        let op1: u8 = 0xBC;
        let op2: u8 = 0xAD;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 1);
    }

    #[test]
    fn add_a_n8_carry_before() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x01);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0, 0x02);
        cpu.add_a_n8();

        let op1: u8 = 0x01;
        let op2: u8 = 0x02;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
    }

    #[test]

    fn add_hl_r16() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x12);
        cpu.bank.set_16_bit_reg(&Reg16::BC, 0x24);
        cpu.add_hl_r16(&Reg16::BC);

        let op1: u16 = 0x12;
        let op2: u16 = 0x24;

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

        let op1: u16 = 0x12;
        let op2: u16 = 0x24;

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

        let op1: u16 = 0x12;
        let op2: u16 = 0x24;

        let sum = op1.wrapping_add(op2);

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::HL), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn add_sp_e8() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0x2424);
        cpu.bus.mem.store_byte(0, (-50i8) as u8);
        cpu.add_sp_e8();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0x23F2);
    }

    #[test]
    fn and_a_r8() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x12);
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x24);
        cpu.and_a_r8(&Reg8::B);

        let op1: u8 = 0x12;
        let op2: u8 = 0x24;

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

        let op1: u8 = 0b0010;
        let op2: u8 = 0b0011;

        let sum = op1 & op2;

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), sum);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn ld_bc_d16() {
        let mut cpu = Cpu::new();
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.ld_bc_d16();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::BC), 0x1234);
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x12);
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::C), 0x34);
    }

    #[test]
    fn ld_bc_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::BC, 0x1000);
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x42);
        cpu.ld_bc_a();

        assert_eq!(cpu.bus.mem.read_byte_u8(0x1000), 0x42);
    }

    #[test]
    fn inc_bc() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::BC, 0x00FF);
        cpu.inc_bc();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::BC), 0x0100);
    }

    #[test]
    fn inc_b_basic() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x05);
        cpu.inc_b();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x06);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
    }

    #[test]
    fn inc_b_sets_zero_and_half_carry_on_wrap() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0xFF);
        cpu.inc_b();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x00);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn inc_b_sets_half_carry_on_nibble_overflow() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x0F);
        cpu.inc_b();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn inc_b_clears_stale_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.bank.set_flag(Flag::ZERO);
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x01);
        cpu.inc_b();

        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
    }

    #[test]
    fn dec_b_basic() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x05);
        cpu.dec_b();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x04);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
    }

    #[test]
    fn dec_b_sets_zero_on_one_to_zero() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x01);
        cpu.dec_b();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x00);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
    }

    #[test]
    fn dec_b_sets_half_carry_on_nibble_borrow() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x10);
        cpu.dec_b();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn ld_b_d8() {
        let mut cpu = Cpu::new();
        cpu.bus.mem.store_byte(0, 0x7A);
        cpu.ld_b_d8();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x7A);
    }

    #[test]
    fn runs_a_small_program() {
        let mut cpu = Cpu::new();

        cpu.bank.set_16_bit_reg(&Reg16::BC, 0x0502);
        cpu.bank.set_16_bit_reg(&Reg16::DE, 0x1000);
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x2000);
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0x0010);

        cpu.bus.mem.store_byte(0x2000, 0x07);

        let program: [u8; 15] = [
            0xC6, 0x0F, // ADD A, 0x0F
            0xCE, 0x01, // ADC A, 0x01
            0xE6, 0x3C, // AND A, 0x3C
            0x80, // ADD A, B
            0x89, // ADC A, C
            0xA2, // AND A, D
            0x86, // ADD A, (HL)
            0x09, // ADD HL, BC
            0x19, // ADD HL, DE
            0xE8, 0xFE, // ADD SP, -2
            0x00, // NOP
        ];

        for (i, byte) in program.iter().enumerate() {
            cpu.bus.mem.store_byte(i as u16, *byte);
        }

        let instruction_count = 11;
        for _ in 0..instruction_count {
            cpu.step();
        }

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x17);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::HL), 0x3502);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0x000E);
    }
}
