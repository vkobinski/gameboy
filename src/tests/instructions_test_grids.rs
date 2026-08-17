#[cfg(test)]
mod grid_tests {
    use crate::architecture::cpu::{Cpu, Flag, Reg16, Reg8};

    #[test]
    fn ld_b_b() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x5A);
        cpu.ld_b_b();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x5A);
    }

    #[test]
    fn ld_b_c() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::C, 0x5A);
        cpu.ld_b_c();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x5A);
    }

    #[test]
    fn ld_b_d() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::D, 0x5A);
        cpu.ld_b_d();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x5A);
    }

    #[test]
    fn ld_b_e() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::E, 0x5A);
        cpu.ld_b_e();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x5A);
    }

    #[test]
    fn ld_b_h() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::H, 0x5A);
        cpu.ld_b_h();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x5A);
    }

    #[test]
    fn ld_b_l() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::L, 0x5A);
        cpu.ld_b_l();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x5A);
    }

    #[test]
    fn ld_b_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bus.mem.store_byte(0x1234, 0x5A);
        cpu.ld_b_hl();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x5A);
    }

    #[test]
    fn ld_b_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x5A);
        cpu.ld_b_a();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::B), 0x5A);
    }

    #[test]
    fn ld_c_b() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x5A);
        cpu.ld_c_b();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::C), 0x5A);
    }

    #[test]
    fn ld_c_c() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::C, 0x5A);
        cpu.ld_c_c();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::C), 0x5A);
    }

    #[test]
    fn ld_c_d() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::D, 0x5A);
        cpu.ld_c_d();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::C), 0x5A);
    }

    #[test]
    fn ld_c_e() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::E, 0x5A);
        cpu.ld_c_e();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::C), 0x5A);
    }

    #[test]
    fn ld_c_h() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::H, 0x5A);
        cpu.ld_c_h();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::C), 0x5A);
    }

    #[test]
    fn ld_c_l() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::L, 0x5A);
        cpu.ld_c_l();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::C), 0x5A);
    }

    #[test]
    fn ld_c_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bus.mem.store_byte(0x1234, 0x5A);
        cpu.ld_c_hl();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::C), 0x5A);
    }

    #[test]
    fn ld_c_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x5A);
        cpu.ld_c_a();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::C), 0x5A);
    }

    #[test]
    fn ld_d_b() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x5A);
        cpu.ld_d_b();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::D), 0x5A);
    }

    #[test]
    fn ld_d_c() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::C, 0x5A);
        cpu.ld_d_c();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::D), 0x5A);
    }

    #[test]
    fn ld_d_d() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::D, 0x5A);
        cpu.ld_d_d();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::D), 0x5A);
    }

    #[test]
    fn ld_d_e() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::E, 0x5A);
        cpu.ld_d_e();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::D), 0x5A);
    }

    #[test]
    fn ld_d_h() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::H, 0x5A);
        cpu.ld_d_h();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::D), 0x5A);
    }

    #[test]
    fn ld_d_l() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::L, 0x5A);
        cpu.ld_d_l();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::D), 0x5A);
    }

    #[test]
    fn ld_d_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bus.mem.store_byte(0x1234, 0x5A);
        cpu.ld_d_hl();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::D), 0x5A);
    }

    #[test]
    fn ld_d_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x5A);
        cpu.ld_d_a();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::D), 0x5A);
    }

    #[test]
    fn ld_e_b() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x5A);
        cpu.ld_e_b();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::E), 0x5A);
    }

    #[test]
    fn ld_e_c() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::C, 0x5A);
        cpu.ld_e_c();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::E), 0x5A);
    }

    #[test]
    fn ld_e_d() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::D, 0x5A);
        cpu.ld_e_d();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::E), 0x5A);
    }

    #[test]
    fn ld_e_e() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::E, 0x5A);
        cpu.ld_e_e();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::E), 0x5A);
    }

    #[test]
    fn ld_e_h() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::H, 0x5A);
        cpu.ld_e_h();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::E), 0x5A);
    }

    #[test]
    fn ld_e_l() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::L, 0x5A);
        cpu.ld_e_l();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::E), 0x5A);
    }

    #[test]
    fn ld_e_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bus.mem.store_byte(0x1234, 0x5A);
        cpu.ld_e_hl();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::E), 0x5A);
    }

    #[test]
    fn ld_e_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x5A);
        cpu.ld_e_a();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::E), 0x5A);
    }

    #[test]
    fn ld_h_b() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x5A);
        cpu.ld_h_b();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::H), 0x5A);
    }

    #[test]
    fn ld_h_c() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::C, 0x5A);
        cpu.ld_h_c();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::H), 0x5A);
    }

    #[test]
    fn ld_h_d() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::D, 0x5A);
        cpu.ld_h_d();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::H), 0x5A);
    }

    #[test]
    fn ld_h_e() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::E, 0x5A);
        cpu.ld_h_e();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::H), 0x5A);
    }

    #[test]
    fn ld_h_h() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::H, 0x5A);
        cpu.ld_h_h();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::H), 0x5A);
    }

    #[test]
    fn ld_h_l() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::L, 0x5A);
        cpu.ld_h_l();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::H), 0x5A);
    }

    #[test]
    fn ld_h_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bus.mem.store_byte(0x1234, 0x5A);
        cpu.ld_h_hl();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::H), 0x5A);
    }

    #[test]
    fn ld_h_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x5A);
        cpu.ld_h_a();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::H), 0x5A);
    }

    #[test]
    fn ld_l_b() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x5A);
        cpu.ld_l_b();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::L), 0x5A);
    }

    #[test]
    fn ld_l_c() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::C, 0x5A);
        cpu.ld_l_c();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::L), 0x5A);
    }

    #[test]
    fn ld_l_d() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::D, 0x5A);
        cpu.ld_l_d();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::L), 0x5A);
    }

    #[test]
    fn ld_l_e() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::E, 0x5A);
        cpu.ld_l_e();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::L), 0x5A);
    }

    #[test]
    fn ld_l_h() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::H, 0x5A);
        cpu.ld_l_h();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::L), 0x5A);
    }

    #[test]
    fn ld_l_l() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::L, 0x5A);
        cpu.ld_l_l();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::L), 0x5A);
    }

    #[test]
    fn ld_l_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bus.mem.store_byte(0x1234, 0x5A);
        cpu.ld_l_hl();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::L), 0x5A);
    }

    #[test]
    fn ld_l_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x5A);
        cpu.ld_l_a();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::L), 0x5A);
    }

    #[test]
    fn ld_hl_ind_b() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x5A);
        cpu.ld_hl_ind_b();
        
        assert_eq!(cpu.bus.mem.read_byte_u8(0x1234), 0x5A);
    }

    #[test]
    fn ld_hl_ind_c() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bank.set_8_bit_reg(&Reg8::C, 0x5A);
        cpu.ld_hl_ind_c();
        
        assert_eq!(cpu.bus.mem.read_byte_u8(0x1234), 0x5A);
    }

    #[test]
    fn ld_hl_ind_d() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bank.set_8_bit_reg(&Reg8::D, 0x5A);
        cpu.ld_hl_ind_d();
        
        assert_eq!(cpu.bus.mem.read_byte_u8(0x1234), 0x5A);
    }

    #[test]
    fn ld_hl_ind_e() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bank.set_8_bit_reg(&Reg8::E, 0x5A);
        cpu.ld_hl_ind_e();
        
        assert_eq!(cpu.bus.mem.read_byte_u8(0x1234), 0x5A);
    }

    #[test]
    fn ld_hl_ind_h() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.ld_hl_ind_h();
        
        assert_eq!(cpu.bus.mem.read_byte_u8(0x1234), 0x12);
    }

    #[test]
    fn ld_hl_ind_l() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.ld_hl_ind_l();
        
        assert_eq!(cpu.bus.mem.read_byte_u8(0x1234), 0x34);
    }

    #[test]
    fn ld_hl_ind_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x5A);
        cpu.ld_hl_ind_a();
        
        assert_eq!(cpu.bus.mem.read_byte_u8(0x1234), 0x5A);
    }

    #[test]
    fn ld_a_b() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x5A);
        cpu.ld_a_b();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x5A);
    }

    #[test]
    fn ld_a_c() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::C, 0x5A);
        cpu.ld_a_c();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x5A);
    }

    #[test]
    fn ld_a_d() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::D, 0x5A);
        cpu.ld_a_d();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x5A);
    }

    #[test]
    fn ld_a_e() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::E, 0x5A);
        cpu.ld_a_e();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x5A);
    }

    #[test]
    fn ld_a_h() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::H, 0x5A);
        cpu.ld_a_h();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x5A);
    }

    #[test]
    fn ld_a_l() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::L, 0x5A);
        cpu.ld_a_l();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x5A);
    }

    #[test]
    fn ld_a_hl_ind() {
        let mut cpu = Cpu::new();
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bus.mem.store_byte(0x1234, 0x5A);
        cpu.ld_a_hl_ind();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x5A);
    }

    #[test]
    fn ld_a_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x5A);
        cpu.ld_a_a();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x5A);
    }

    #[test]
    fn sub_a_b() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x01);
        cpu.sub_a_b();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sub_a_c() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_8_bit_reg(&Reg8::C, 0x01);
        cpu.sub_a_c();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sub_a_d() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_8_bit_reg(&Reg8::D, 0x01);
        cpu.sub_a_d();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sub_a_e() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_8_bit_reg(&Reg8::E, 0x01);
        cpu.sub_a_e();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sub_a_h() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_8_bit_reg(&Reg8::H, 0x01);
        cpu.sub_a_h();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sub_a_l() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_8_bit_reg(&Reg8::L, 0x01);
        cpu.sub_a_l();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sub_a_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bus.mem.store_byte(0x1234, 0x01);
        cpu.sub_a_hl();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sub_a_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.sub_a_a();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x00);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sbc_a_b() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x00);
        cpu.sbc_a_b();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sbc_a_c() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bank.set_8_bit_reg(&Reg8::C, 0x00);
        cpu.sbc_a_c();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sbc_a_d() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bank.set_8_bit_reg(&Reg8::D, 0x00);
        cpu.sbc_a_d();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sbc_a_e() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bank.set_8_bit_reg(&Reg8::E, 0x00);
        cpu.sbc_a_e();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sbc_a_h() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bank.set_8_bit_reg(&Reg8::H, 0x00);
        cpu.sbc_a_h();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sbc_a_l() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bank.set_8_bit_reg(&Reg8::L, 0x00);
        cpu.sbc_a_l();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sbc_a_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bus.mem.store_byte(0x1234, 0x00);
        cpu.sbc_a_hl();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0F);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn sbc_a_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_flag(Flag::CARRY);
        cpu.sbc_a_a();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0xFF);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 1);
    }

    #[test]
    fn xor_a_b() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bank.set_8_bit_reg(&Reg8::B, 0b0110);
        cpu.xor_a_b();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1100);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn xor_a_c() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bank.set_8_bit_reg(&Reg8::C, 0b0110);
        cpu.xor_a_c();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1100);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn xor_a_d() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bank.set_8_bit_reg(&Reg8::D, 0b0110);
        cpu.xor_a_d();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1100);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn xor_a_e() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bank.set_8_bit_reg(&Reg8::E, 0b0110);
        cpu.xor_a_e();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1100);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn xor_a_h() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bank.set_8_bit_reg(&Reg8::H, 0b0110);
        cpu.xor_a_h();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1100);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn xor_a_l() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bank.set_8_bit_reg(&Reg8::L, 0b0110);
        cpu.xor_a_l();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1100);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn xor_a_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bus.mem.store_byte(0x1234, 0b0110);
        cpu.xor_a_hl();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1100);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn xor_a_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.xor_a_a();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x00);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn or_a_b() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bank.set_8_bit_reg(&Reg8::B, 0b0110);
        cpu.or_a_b();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1110);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn or_a_c() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bank.set_8_bit_reg(&Reg8::C, 0b0110);
        cpu.or_a_c();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1110);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn or_a_d() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bank.set_8_bit_reg(&Reg8::D, 0b0110);
        cpu.or_a_d();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1110);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn or_a_e() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bank.set_8_bit_reg(&Reg8::E, 0b0110);
        cpu.or_a_e();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1110);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn or_a_h() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bank.set_8_bit_reg(&Reg8::H, 0b0110);
        cpu.or_a_h();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1110);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn or_a_l() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bank.set_8_bit_reg(&Reg8::L, 0b0110);
        cpu.or_a_l();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1110);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn or_a_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0b1010);
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bus.mem.store_byte(0x1234, 0b0110);
        cpu.or_a_hl();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0b1110);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn or_a_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x0A);
        cpu.or_a_a();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x0A);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 0);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn cp_a_b() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_8_bit_reg(&Reg8::B, 0x01);
        cpu.cp_a_b();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn cp_a_c() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_8_bit_reg(&Reg8::C, 0x01);
        cpu.cp_a_c();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn cp_a_d() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_8_bit_reg(&Reg8::D, 0x01);
        cpu.cp_a_d();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn cp_a_e() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_8_bit_reg(&Reg8::E, 0x01);
        cpu.cp_a_e();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn cp_a_h() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_8_bit_reg(&Reg8::H, 0x01);
        cpu.cp_a_h();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn cp_a_l() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_8_bit_reg(&Reg8::L, 0x01);
        cpu.cp_a_l();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn cp_a_hl() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.bank.set_16_bit_reg(&Reg16::HL, 0x1234);
        cpu.bus.mem.store_byte(0x1234, 0x01);
        cpu.cp_a_hl();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 0);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 1);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

    #[test]
    fn cp_a_a() {
        let mut cpu = Cpu::new();
        cpu.bank.set_8_bit_reg(&Reg8::A, 0x10);
        cpu.cp_a_a();
        
        assert_eq!(cpu.bank.get_8_bit_reg(&Reg8::A), 0x10);
        assert_eq!(cpu.bank.get_flag(Flag::ZERO), 1);
        assert_eq!(cpu.bank.get_flag(Flag::SUB), 1);
        assert_eq!(cpu.bank.get_flag(Flag::HALFCARRY), 0);
        assert_eq!(cpu.bank.get_flag(Flag::CARRY), 0);
    }

}
