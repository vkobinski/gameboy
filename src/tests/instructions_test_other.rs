#[cfg(test)]
mod other_tests {
    use crate::architecture::cpu::{Cpu, Flag, Reg16, Reg8};

    #[test]
    fn inc_hl_ind() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bus.mem.store_byte(0x1234, 0x0F);
        cpu.inc_hl_ind();

        assert_eq!(cpu.bus.mem.read_byte_u8(0x1234), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn dec_hl_ind() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bus.mem.store_byte(0x1234, 0x10);
        cpu.dec_hl_ind();

        assert_eq!(cpu.bus.mem.read_byte_u8(0x1234), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn dec_bc() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::BC, 0x0100);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.dec_bc();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::BC), 0x00FF);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
    }

    #[test]
    fn inc_de() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::DE, 0x00FF);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.inc_de();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::DE), 0x0100);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
    }

    #[test]
    fn dec_de() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::DE, 0x0100);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.dec_de();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::DE), 0x00FF);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
    }

    #[test]
    fn inc_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x00FF);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.inc_hl();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::HL), 0x0100);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
    }

    #[test]
    fn dec_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x0100);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.dec_hl();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::HL), 0x00FF);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
    }

    #[test]
    fn inc_sp() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0x00FF);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.inc_sp();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0x0100);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
    }

    #[test]
    fn dec_sp() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0x0100);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.dec_sp();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0x00FF);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
    }

    #[test]
    fn inc_c() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::C, 0x0F);
        cpu.inc_c();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::C), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn dec_c() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::C, 0x10);
        cpu.dec_c();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::C), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn inc_d() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::D, 0x0F);
        cpu.inc_d();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::D), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn dec_d() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::D, 0x10);
        cpu.dec_d();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::D), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn inc_e() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::E, 0x0F);
        cpu.inc_e();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::E), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn dec_e() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::E, 0x10);
        cpu.dec_e();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::E), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn inc_h() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::H, 0x0F);
        cpu.inc_h();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::H), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn dec_h() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::H, 0x10);
        cpu.dec_h();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::H), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn inc_l() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::L, 0x0F);
        cpu.inc_l();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::L), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn dec_l() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::L, 0x10);
        cpu.dec_l();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::L), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn inc_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x0F);
        cpu.inc_a();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn dec_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.dec_a();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
    }

    #[test]
    fn ld_c_d8() {
        let mut cpu = Cpu::new();
        cpu.bus.mem.store_byte(0, 0x5A);
        cpu.ld_c_d8();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::C), 0x5A);
    }

    #[test]
    fn ld_d_d8() {
        let mut cpu = Cpu::new();
        cpu.bus.mem.store_byte(0, 0x5A);
        cpu.ld_d_d8();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::D), 0x5A);
    }

    #[test]
    fn ld_e_d8() {
        let mut cpu = Cpu::new();
        cpu.bus.mem.store_byte(0, 0x5A);
        cpu.ld_e_d8();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::E), 0x5A);
    }

    #[test]
    fn ld_h_d8() {
        let mut cpu = Cpu::new();
        cpu.bus.mem.store_byte(0, 0x5A);
        cpu.ld_h_d8();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::H), 0x5A);
    }

    #[test]
    fn ld_l_d8() {
        let mut cpu = Cpu::new();
        cpu.bus.mem.store_byte(0, 0x5A);
        cpu.ld_l_d8();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::L), 0x5A);
    }

    #[test]
    fn ld_a_d8() {
        let mut cpu = Cpu::new();
        cpu.bus.mem.store_byte(0, 0x5A);
        cpu.ld_a_d8();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x5A);
    }

    #[test]
    fn ld_hl_ind_d8() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bus.mem.store_byte(0, 0x5A);
        cpu.ld_hl_ind_d8();

        assert_eq!(cpu.bus.mem.read_byte_u8(0x1234), 0x5A);
    }

    #[test]
    fn ld_de_d16() {
        let mut cpu = Cpu::new();
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.ld_de_d16();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::DE), 0x1234);
    }

    #[test]
    fn ld_hl_d16() {
        let mut cpu = Cpu::new();
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.ld_hl_d16();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::HL), 0x1234);
    }

    #[test]
    fn ld_sp_d16() {
        let mut cpu = Cpu::new();
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.ld_sp_d16();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0x1234);
    }

    #[test]
    fn ld_de_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::DE, 0x1000);
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x42);
        cpu.ld_de_a();

        assert_eq!(cpu.bus.mem.read_byte_u8(0x1000), 0x42);
    }

    #[test]
    fn ld_a_de() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::DE, 0x1000);
        cpu.bus.mem.store_byte(0x1000, 0x42);
        cpu.ld_a_de();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x42);
    }

    #[test]
    fn ld_hli_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1000);
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x42);
        cpu.ld_hli_a();

        assert_eq!(cpu.bus.mem.read_byte_u8(0x1000), 0x42);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::HL), 0x1001);
    }

    #[test]
    fn ld_a_hli() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1000);
        cpu.bus.mem.store_byte(0x1000, 0x42);
        cpu.ld_a_hli();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x42);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::HL), 0x1001);
    }

    #[test]
    fn ld_hld_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1000);
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x42);
        cpu.ld_hld_a();

        assert_eq!(cpu.bus.mem.read_byte_u8(0x1000), 0x42);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::HL), 0x0FFF);
    }

    #[test]
    fn ld_a_hld() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1000);
        cpu.bus.mem.store_byte(0x1000, 0x42);
        cpu.ld_a_hld();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x42);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::HL), 0x0FFF);
    }

    #[test]
    fn ld_a16_sp() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xABCD);
        cpu.bus.mem.store_byte(0, 0x00);
        cpu.bus.mem.store_byte(1, 0x20);
        cpu.ld_a16_sp();

        assert_eq!(cpu.bus.mem.read_byte_u16(0x2000), 0xABCD);
    }

    #[test]
    fn ldh_a8_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x42);
        cpu.bus.mem.store_byte(0, 0x10);
        cpu.ldh_a8_a();

        assert_eq!(cpu.bus.mem.read_byte_u8(0xFF10), 0x42);
    }

    #[test]
    fn ldh_a_a8() {
        let mut cpu = Cpu::new();
        cpu.bus.mem.store_byte(0xFF10, 0x42);
        cpu.bus.mem.store_byte(0, 0x10);
        cpu.ldh_a_a8();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x42);
    }

    #[test]
    fn ld_c_ind_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::C, 0x10);
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x42);
        cpu.ld_c_ind_a();

        assert_eq!(cpu.bus.mem.read_byte_u8(0xFF10), 0x42);
    }

    #[test]
    fn ld_a_c_ind() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::C, 0x10);
        cpu.bus.mem.store_byte(0xFF10, 0x42);
        cpu.ld_a_c_ind();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x42);
    }

    #[test]
    fn ld_a16_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x42);
        cpu.bus.mem.store_byte(0, 0x00);
        cpu.bus.mem.store_byte(1, 0x20);
        cpu.ld_a16_a();

        assert_eq!(cpu.bus.mem.read_byte_u8(0x2000), 0x42);
    }

    #[test]
    fn ld_a_a16() {
        let mut cpu = Cpu::new();
        cpu.bus.mem.store_byte(0x2000, 0x42);
        cpu.bus.mem.store_byte(0, 0x00);
        cpu.bus.mem.store_byte(1, 0x20);
        cpu.ld_a_a16();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x42);
    }

    #[test]
    fn rlca() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x96);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.rlca();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x2D);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
    }

    #[test]
    fn rrca() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x96);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.rrca();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x4B);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
    }

    #[test]
    fn rla() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x96);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.rla();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x2D);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
    }

    #[test]
    fn rra() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x96);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.rra();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0xCB);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
    }

    #[test]
    fn cpl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0xB0);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bank.unset_flag(Flag::ZERO);
        cpu.cpl();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x4F);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
    }

    #[test]
    fn scf() {
        let mut cpu = Cpu::new();
        cpu.bank.unset_flag(Flag::CARRY);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.scf();

        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
    }

    #[test]
    fn ccf_from_set() {
        let mut cpu = Cpu::new();
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.ccf();

        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
    }

    #[test]
    fn ccf_from_unset() {
        let mut cpu = Cpu::new();
        cpu.bank.unset_flag(Flag::CARRY);
        cpu.ccf();

        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 1);
    }

    #[test]
    fn daa_after_add_low_nibble() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x0A);
        cpu.bank.unset_flag(Flag::SUB);
        cpu.bank.unset_flag(Flag::HALFCARRY);
        cpu.bank.unset_flag(Flag::CARRY);
        cpu.daa();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn daa_after_add_overflow() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x9A);
        cpu.bank.unset_flag(Flag::SUB);
        cpu.bank.unset_flag(Flag::HALFCARRY);
        cpu.bank.unset_flag(Flag::CARRY);
        cpu.daa();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x00);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 1);
    }

    #[test]
    fn halt() {
        let mut cpu = Cpu::new();
        let a_before = cpu.bank.get_8_bit_reg(&Reg8::A);
        cpu.halt();

        // no IME/halted flag is exposed on Cpu yet; this only guards against
        // accidental register corruption until that state exists.
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), a_before);
    }

    #[test]
    fn stop() {
        let mut cpu = Cpu::new();
        let a_before = cpu.bank.get_8_bit_reg(&Reg8::A);
        cpu.stop();

        // no IME/halted flag is exposed on Cpu yet; this only guards against
        // accidental register corruption until that state exists.
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), a_before);
    }

    #[test]
    fn di() {
        let mut cpu = Cpu::new();
        let a_before = cpu.bank.get_8_bit_reg(&Reg8::A);
        cpu.di();

        // no IME/halted flag is exposed on Cpu yet; this only guards against
        // accidental register corruption until that state exists.
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), a_before);
    }

    #[test]
    fn ei() {
        let mut cpu = Cpu::new();
        let a_before = cpu.bank.get_8_bit_reg(&Reg8::A);
        cpu.ei();

        // no IME/halted flag is exposed on Cpu yet; this only guards against
        // accidental register corruption until that state exists.
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), a_before);
    }

    #[test]
    fn jr_r8() {
        let mut cpu = Cpu::new();
        cpu.bus.mem.store_byte(0, 0x05);
        cpu.jr_r8();

        assert_eq!(cpu.bank.pc.get_value(), 6);
    }

    #[test]
    fn jr_nz_r8_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.unset_flag(Flag::ZERO);
        cpu.bus.mem.store_byte(0, 0x05);
        cpu.jr_nz_r8();

        assert_eq!(cpu.bank.pc.get_value(), 6);
    }

    #[test]
    fn jr_nz_r8_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_flag(Flag::ZERO);
        cpu.bus.mem.store_byte(0, 0x05);
        cpu.jr_nz_r8();

        assert_eq!(cpu.bank.pc.get_value(), 1);
    }

    #[test]
    fn jr_z_r8_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_flag(Flag::ZERO);
        cpu.bus.mem.store_byte(0, 0x05);
        cpu.jr_z_r8();

        assert_eq!(cpu.bank.pc.get_value(), 6);
    }

    #[test]
    fn jr_z_r8_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.unset_flag(Flag::ZERO);
        cpu.bus.mem.store_byte(0, 0x05);
        cpu.jr_z_r8();

        assert_eq!(cpu.bank.pc.get_value(), 1);
    }

    #[test]
    fn jr_nc_r8_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.unset_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0, 0x05);
        cpu.jr_nc_r8();

        assert_eq!(cpu.bank.pc.get_value(), 6);
    }

    #[test]
    fn jr_nc_r8_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0, 0x05);
        cpu.jr_nc_r8();

        assert_eq!(cpu.bank.pc.get_value(), 1);
    }

    #[test]
    fn jr_c_r8_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0, 0x05);
        cpu.jr_c_r8();

        assert_eq!(cpu.bank.pc.get_value(), 6);
    }

    #[test]
    fn jr_c_r8_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.unset_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0, 0x05);
        cpu.jr_c_r8();

        assert_eq!(cpu.bank.pc.get_value(), 1);
    }

    #[test]
    fn jp_a16() {
        let mut cpu = Cpu::new();
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.jp_a16();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
    }

    #[test]
    fn jp_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x9ABC);
        cpu.jp_hl();

        assert_eq!(cpu.bank.pc.get_value(), 0x9ABC);
    }

    #[test]
    fn jp_nz_a16_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.unset_flag(Flag::ZERO);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.jp_nz_a16();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
    }

    #[test]
    fn jp_nz_a16_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_flag(Flag::ZERO);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.jp_nz_a16();

        assert_eq!(cpu.bank.pc.get_value(), 1);
    }

    #[test]
    fn jp_z_a16_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_flag(Flag::ZERO);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.jp_z_a16();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
    }

    #[test]
    fn jp_z_a16_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.unset_flag(Flag::ZERO);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.jp_z_a16();

        assert_eq!(cpu.bank.pc.get_value(), 1);
    }

    #[test]
    fn jp_nc_a16_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.unset_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.jp_nc_a16();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
    }

    #[test]
    fn jp_nc_a16_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.jp_nc_a16();

        assert_eq!(cpu.bank.pc.get_value(), 1);
    }

    #[test]
    fn jp_c_a16_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.jp_c_a16();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
    }

    #[test]
    fn jp_c_a16_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.unset_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.jp_c_a16();

        assert_eq!(cpu.bank.pc.get_value(), 1);
    }

    #[test]
    fn call_a16() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.call_a16();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 1);
    }

    #[test]
    fn call_nz_a16_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.bank.unset_flag(Flag::ZERO);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.call_nz_a16();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 1);
    }

    #[test]
    fn call_nz_a16_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.call_nz_a16();

        assert_eq!(cpu.bank.pc.get_value(), 1);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFE);
    }

    #[test]
    fn call_z_a16_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.call_z_a16();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 1);
    }

    #[test]
    fn call_z_a16_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.bank.unset_flag(Flag::ZERO);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.call_z_a16();

        assert_eq!(cpu.bank.pc.get_value(), 1);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFE);
    }

    #[test]
    fn call_nc_a16_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.bank.unset_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.call_nc_a16();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 1);
    }

    #[test]
    fn call_nc_a16_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.call_nc_a16();

        assert_eq!(cpu.bank.pc.get_value(), 1);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFE);
    }

    #[test]
    fn call_c_a16_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.call_c_a16();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 1);
    }

    #[test]
    fn call_c_a16_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.bank.unset_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0, 0x34);
        cpu.bus.mem.store_byte(1, 0x12);
        cpu.call_c_a16();

        assert_eq!(cpu.bank.pc.get_value(), 1);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFE);
    }

    #[test]
    fn ret() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFC);
        cpu.bus.mem.store_byte(0xFFFC, 0x34);
        cpu.bus.mem.store_byte(0xFFFD, 0x12);
        cpu.ret();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFE);
    }

    #[test]
    fn reti() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFC);
        cpu.bus.mem.store_byte(0xFFFC, 0x34);
        cpu.bus.mem.store_byte(0xFFFD, 0x12);
        cpu.reti();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFE);
    }

    #[test]
    fn ret_nz_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFC);
        cpu.bank.unset_flag(Flag::ZERO);
        cpu.bus.mem.store_byte(0xFFFC, 0x34);
        cpu.bus.mem.store_byte(0xFFFD, 0x12);
        cpu.ret_nz();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFE);
    }

    #[test]
    fn ret_nz_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFC);
        cpu.bank.pc.set_reg(0x0005);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.ret_nz();

        assert_eq!(cpu.bank.pc.get_value(), 0x0005);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
    }

    #[test]
    fn ret_z_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFC);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.bus.mem.store_byte(0xFFFC, 0x34);
        cpu.bus.mem.store_byte(0xFFFD, 0x12);
        cpu.ret_z();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFE);
    }

    #[test]
    fn ret_z_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFC);
        cpu.bank.pc.set_reg(0x0005);
        cpu.bank.unset_flag(Flag::ZERO);
        cpu.ret_z();

        assert_eq!(cpu.bank.pc.get_value(), 0x0005);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
    }

    #[test]
    fn ret_nc_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFC);
        cpu.bank.unset_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0xFFFC, 0x34);
        cpu.bus.mem.store_byte(0xFFFD, 0x12);
        cpu.ret_nc();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFE);
    }

    #[test]
    fn ret_nc_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFC);
        cpu.bank.pc.set_reg(0x0005);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.ret_nc();

        assert_eq!(cpu.bank.pc.get_value(), 0x0005);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
    }

    #[test]
    fn ret_c_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFC);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0xFFFC, 0x34);
        cpu.bus.mem.store_byte(0xFFFD, 0x12);
        cpu.ret_c();

        assert_eq!(cpu.bank.pc.get_value(), 0x1234);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFE);
    }

    #[test]
    fn ret_c_not_taken() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFC);
        cpu.bank.pc.set_reg(0x0005);
        cpu.bank.unset_flag(Flag::CARRY);
        cpu.ret_c();

        assert_eq!(cpu.bank.pc.get_value(), 0x0005);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
    }

    #[test]
    fn rst_00() {
        let mut cpu = Cpu::new();
        cpu.bank.pc.set_reg(0x1050);
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.rst_00();

        assert_eq!(cpu.bank.pc.get_value(), 0x00);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 0x1050);
    }

    #[test]
    fn rst_08() {
        let mut cpu = Cpu::new();
        cpu.bank.pc.set_reg(0x1050);
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.rst_08();

        assert_eq!(cpu.bank.pc.get_value(), 0x08);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 0x1050);
    }

    #[test]
    fn rst_10() {
        let mut cpu = Cpu::new();
        cpu.bank.pc.set_reg(0x1050);
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.rst_10();

        assert_eq!(cpu.bank.pc.get_value(), 0x10);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 0x1050);
    }

    #[test]
    fn rst_18() {
        let mut cpu = Cpu::new();
        cpu.bank.pc.set_reg(0x1050);
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.rst_18();

        assert_eq!(cpu.bank.pc.get_value(), 0x18);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 0x1050);
    }

    #[test]
    fn rst_20() {
        let mut cpu = Cpu::new();
        cpu.bank.pc.set_reg(0x1050);
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.rst_20();

        assert_eq!(cpu.bank.pc.get_value(), 0x20);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 0x1050);
    }

    #[test]
    fn rst_28() {
        let mut cpu = Cpu::new();
        cpu.bank.pc.set_reg(0x1050);
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.rst_28();

        assert_eq!(cpu.bank.pc.get_value(), 0x28);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 0x1050);
    }

    #[test]
    fn rst_30() {
        let mut cpu = Cpu::new();
        cpu.bank.pc.set_reg(0x1050);
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.rst_30();

        assert_eq!(cpu.bank.pc.get_value(), 0x30);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 0x1050);
    }

    #[test]
    fn rst_38() {
        let mut cpu = Cpu::new();
        cpu.bank.pc.set_reg(0x1050);
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.rst_38();

        assert_eq!(cpu.bank.pc.get_value(), 0x38);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 0x1050);
    }

    #[test]
    fn push_bc() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.bank.set_16_bit_reg(&Reg16::BC, 0x1234);
        cpu.push_bc();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 0x1234);
    }

    #[test]
    fn push_de() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.bank.set_16_bit_reg(&Reg16::DE, 0x1234);
        cpu.push_de();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 0x1234);
    }

    #[test]
    fn push_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.push_hl();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u16(0xFFFC), 0x1234);
    }

    #[test]
    fn push_af() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFE);
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x12);
        cpu.bank.set_flag(Flag::ZERO);
        cpu.push_af();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFC);
        assert_eq!(cpu.bus.mem.read_byte_u8(0xFFFD), 0x12);
    }

    #[test]
    fn pop_bc() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFC);
        cpu.bus.mem.store_byte(0xFFFC, 0x34);
        cpu.bus.mem.store_byte(0xFFFD, 0x12);
        cpu.pop_bc();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::BC), 0x1234);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFE);
    }

    #[test]
    fn pop_de() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFC);
        cpu.bus.mem.store_byte(0xFFFC, 0x34);
        cpu.bus.mem.store_byte(0xFFFD, 0x12);
        cpu.pop_de();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::DE), 0x1234);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFE);
    }

    #[test]
    fn pop_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFC);
        cpu.bus.mem.store_byte(0xFFFC, 0x34);
        cpu.bus.mem.store_byte(0xFFFD, 0x12);
        cpu.pop_hl();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::HL), 0x1234);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFE);
    }

    #[test]
    fn pop_af() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0xFFFC);
        cpu.bus.mem.store_byte(0xFFFC, 0x0F);
        cpu.bus.mem.store_byte(0xFFFD, 0x12);
        cpu.pop_af();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x12);
        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0xFFFE);
    }

    #[test]
    fn ld_hl_sp_r8() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::SP, 0x2424);
        cpu.bus.mem.store_byte(0, (-50i8) as u8);
        cpu.ld_hl_sp_r8();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::HL), 0x23F2);
    }

    #[test]
    fn ld_sp_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x9ABC);
        cpu.ld_sp_hl();

        assert_eq!(cpu.bank.get_16_bit_reg(&Reg16::SP), 0x9ABC);
    }

    #[test]
    fn sub_a_d8() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bus.mem.store_byte(0, 0x01);
        cpu.sub_a_d8();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sbc_a_d8() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bus.mem.store_byte(0, 0x00);
        cpu.sbc_a_d8();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn xor_a_d8() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bus.mem.store_byte(0, 0b0110);
        cpu.xor_a_d8();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1100);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn or_a_d8() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bus.mem.store_byte(0, 0b0110);
        cpu.or_a_d8();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1110);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn cp_a_d8() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bus.mem.store_byte(0, 0x01);
        cpu.cp_a_d8();

        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

}
