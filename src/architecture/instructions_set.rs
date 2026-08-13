use num_traits::ops::bytes;

use super::{
    super::architecture::cpu,
    cpu::{Reg16, Reg8},
};

impl cpu::Cpu {
    pub fn step(&mut self) {
        let addr = self.current_pc();
        let opcode = self.bus.mem.read_byte_u8(addr);
        self.parse_instruction(opcode);
    }

    pub fn fetch_argument_u8(&mut self) -> u8 {
        let addr = self.current_pc();
        let arg = self.bus.mem.read_byte_u8(addr);
        return arg;
    }

    pub fn fetch_argument_i8(&mut self) -> i8 {
        let addr = self.current_pc();
        let arg = self.bus.mem.read_byte_i8(addr);
        return arg;
    }

    pub fn fetch_argument_u16(&mut self) -> u16 {
        let addr = self.current_pc();
        let arg = self.bus.mem.read_byte_u16(addr);
        return arg;
    }

    pub fn parse_instruction(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.nop(),
            0x01 => self.ld_bc_d16(),
            0x02 => self.ld_bc_a(),
            0x03 => self.inc_bc(),
            0x04 => self.inc_b(),
            0x05 => self.dec_b(),
            0x06 => self.ld_b_d8(),
            0x07 => self.rlca(),
            0x08 => self.ld_a16_sp(),
            0x09 => self.add_hl_r16(&Reg16::BC),
            0x0A => self.ld_a_bc(),
            0x0B => self.dec_bc(),
            0x0C => self.inc_c(),
            0x0D => self.dec_c(),
            0x0E => self.ld_c_d8(),
            0x0F => self.rrca(),

            0x10 => self.stop(),
            0x11 => self.ld_de_d16(),
            0x12 => self.ld_de_a(),
            0x13 => self.inc_de(),
            0x14 => self.inc_d(),
            0x15 => self.dec_d(),
            0x16 => self.ld_d_d8(),
            0x17 => self.rla(),
            0x18 => self.jr_r8(),
            0x19 => self.add_hl_r16(&Reg16::DE),
            0x1A => self.ld_a_de(),
            0x1B => self.dec_de(),
            0x1C => self.inc_e(),
            0x1D => self.dec_e(),
            0x1E => self.ld_e_d8(),
            0x1F => self.rra(),

            0x20 => self.jr_nz_r8(),
            0x21 => self.ld_hl_d16(),
            0x22 => self.ld_hli_a(),
            0x23 => self.inc_hl(),
            0x24 => self.inc_h(),
            0x25 => self.dec_h(),
            0x26 => self.ld_h_d8(),
            0x27 => self.daa(),
            0x28 => self.jr_z_r8(),
            0x29 => self.add_hl_r16(&Reg16::HL),
            0x2A => self.ld_a_hli(),
            0x2B => self.dec_hl(),
            0x2C => self.inc_l(),
            0x2D => self.dec_l(),
            0x2E => self.ld_l_d8(),
            0x2F => self.cpl(),

            0x30 => self.jr_nc_r8(),
            0x31 => self.ld_sp_d16(),
            0x32 => self.ld_hld_a(),
            0x33 => self.inc_sp(),
            0x34 => self.inc_hl_ind(),
            0x35 => self.dec_hl_ind(),
            0x36 => self.ld_hl_ind_d8(),
            0x37 => self.scf(),
            0x38 => self.jr_c_r8(),
            0x39 => self.add_hl_sp(),
            0x3A => self.ld_a_hld(),
            0x3B => self.dec_sp(),
            0x3C => self.inc_a(),
            0x3D => self.dec_a(),
            0x3E => self.ld_a_d8(),
            0x3F => self.ccf(),

            0x40 => self.ld_b_b(),
            0x41 => self.ld_b_c(),
            0x42 => self.ld_b_d(),
            0x43 => self.ld_b_e(),
            0x44 => self.ld_b_h(),
            0x45 => self.ld_b_l(),
            0x46 => self.ld_b_hl(),
            0x47 => self.ld_b_a(),
            0x48 => self.ld_c_b(),
            0x49 => self.ld_c_c(),
            0x4A => self.ld_c_d(),
            0x4B => self.ld_c_e(),
            0x4C => self.ld_c_h(),
            0x4D => self.ld_c_l(),
            0x4E => self.ld_c_hl(),
            0x4F => self.ld_c_a(),

            0x50 => self.ld_d_b(),
            0x51 => self.ld_d_c(),
            0x52 => self.ld_d_d(),
            0x53 => self.ld_d_e(),
            0x54 => self.ld_d_h(),
            0x55 => self.ld_d_l(),
            0x56 => self.ld_d_hl(),
            0x57 => self.ld_d_a(),
            0x58 => self.ld_e_b(),
            0x59 => self.ld_e_c(),
            0x5A => self.ld_e_d(),
            0x5B => self.ld_e_e(),
            0x5C => self.ld_e_h(),
            0x5D => self.ld_e_l(),
            0x5E => self.ld_e_hl(),
            0x5F => self.ld_e_a(),

            0x60 => self.ld_h_b(),
            0x61 => self.ld_h_c(),
            0x62 => self.ld_h_d(),
            0x63 => self.ld_h_e(),
            0x64 => self.ld_h_h(),
            0x65 => self.ld_h_l(),
            0x66 => self.ld_h_hl(),
            0x67 => self.ld_h_a(),
            0x68 => self.ld_l_b(),
            0x69 => self.ld_l_c(),
            0x6A => self.ld_l_d(),
            0x6B => self.ld_l_e(),
            0x6C => self.ld_l_h(),
            0x6D => self.ld_l_l(),
            0x6E => self.ld_l_hl(),
            0x6F => self.ld_l_a(),

            0x70 => self.ld_hl_ind_b(),
            0x71 => self.ld_hl_ind_c(),
            0x72 => self.ld_hl_ind_d(),
            0x73 => self.ld_hl_ind_e(),
            0x74 => self.ld_hl_ind_h(),
            0x75 => self.ld_hl_ind_l(),
            0x76 => self.halt(),
            0x77 => self.ld_hl_ind_a(),
            0x78 => self.ld_a_b(),
            0x79 => self.ld_a_c(),
            0x7A => self.ld_a_d(),
            0x7B => self.ld_a_e(),
            0x7C => self.ld_a_h(),
            0x7D => self.ld_a_l(),
            0x7E => self.ld_a_hl_ind(),
            0x7F => self.ld_a_a(),

            0x80 => self.add_a_r8(Reg8::B),
            0x81 => self.add_a_r8(Reg8::C),
            0x82 => self.add_a_r8(Reg8::D),
            0x83 => self.add_a_r8(Reg8::E),
            0x84 => self.add_a_r8(Reg8::H),
            0x85 => self.add_a_r8(Reg8::L),
            0x86 => self.add_a_hl(),
            0x87 => self.add_a_r8(Reg8::A),
            0x88 => self.adc_a_r8(Reg8::B),
            0x89 => self.adc_a_r8(Reg8::C),
            0x8A => self.adc_a_r8(Reg8::D),
            0x8B => self.adc_a_r8(Reg8::E),
            0x8C => self.adc_a_r8(Reg8::H),
            0x8D => self.adc_a_r8(Reg8::L),
            0x8E => self.adc_a_hl(),
            0x8F => self.adc_a_r8(Reg8::A),

            0x90 => self.sub_a_b(),
            0x91 => self.sub_a_c(),
            0x92 => self.sub_a_d(),
            0x93 => self.sub_a_e(),
            0x94 => self.sub_a_h(),
            0x95 => self.sub_a_l(),
            0x96 => self.sub_a_hl(),
            0x97 => self.sub_a_a(),
            0x98 => self.sbc_a_b(),
            0x99 => self.sbc_a_c(),
            0x9A => self.sbc_a_d(),
            0x9B => self.sbc_a_e(),
            0x9C => self.sbc_a_h(),
            0x9D => self.sbc_a_l(),
            0x9E => self.sbc_a_hl(),
            0x9F => self.sbc_a_a(),

            0xA0 => self.and_a_r8(&Reg8::B),
            0xA1 => self.and_a_r8(&Reg8::C),
            0xA2 => self.and_a_r8(&Reg8::D),
            0xA3 => self.and_a_r8(&Reg8::E),
            0xA4 => self.and_a_r8(&Reg8::H),
            0xA5 => self.and_a_r8(&Reg8::L),
            0xA6 => self.and_a_hl(),
            0xA7 => self.and_a_r8(&Reg8::A),
            0xA8 => self.xor_a_b(),
            0xA9 => self.xor_a_c(),
            0xAA => self.xor_a_d(),
            0xAB => self.xor_a_e(),
            0xAC => self.xor_a_h(),
            0xAD => self.xor_a_l(),
            0xAE => self.xor_a_hl(),
            0xAF => self.xor_a_a(),

            0xB0 => self.or_a_b(),
            0xB1 => self.or_a_c(),
            0xB2 => self.or_a_d(),
            0xB3 => self.or_a_e(),
            0xB4 => self.or_a_h(),
            0xB5 => self.or_a_l(),
            0xB6 => self.or_a_hl(),
            0xB7 => self.or_a_a(),
            0xB8 => self.cp_a_b(),
            0xB9 => self.cp_a_c(),
            0xBA => self.cp_a_d(),
            0xBB => self.cp_a_e(),
            0xBC => self.cp_a_h(),
            0xBD => self.cp_a_l(),
            0xBE => self.cp_a_hl(),
            0xBF => self.cp_a_a(),

            0xC0 => self.ret_nz(),
            0xC1 => self.pop_bc(),
            0xC2 => self.jp_nz_a16(),
            0xC3 => self.jp_a16(),
            0xC4 => self.call_nz_a16(),
            0xC5 => self.push_bc(),
            0xC6 => self.add_a_n8(),
            0xC7 => self.rst_00(),
            0xC8 => self.ret_z(),
            0xC9 => self.ret(),
            0xCA => self.jp_z_a16(),
            0xCB => self.cb_prefixed(),
            0xCC => self.call_z_a16(),
            0xCD => self.call_a16(),
            0xCE => self.adc_a_n8(),
            0xCF => self.rst_08(),

            0xD0 => self.ret_nc(),
            0xD1 => self.pop_de(),
            0xD2 => self.jp_nc_a16(),
            0xD4 => self.call_nc_a16(),
            0xD5 => self.push_de(),
            0xD6 => self.sub_a_d8(),
            0xD7 => self.rst_10(),
            0xD8 => self.ret_c(),
            0xD9 => self.reti(),
            0xDA => self.jp_c_a16(),
            0xDC => self.call_c_a16(),
            0xDE => self.sbc_a_d8(),
            0xDF => self.rst_18(),

            0xE0 => self.ldh_a8_a(),
            0xE1 => self.pop_hl(),
            0xE2 => self.ld_c_ind_a(),
            0xE5 => self.push_hl(),
            0xE6 => self.and_a_n8(),
            0xE7 => self.rst_20(),
            0xE8 => self.add_sp_e8(),
            0xE9 => self.jp_hl(),
            0xEA => self.ld_a16_a(),
            0xEE => self.xor_a_d8(),
            0xEF => self.rst_28(),

            0xF0 => self.ldh_a_a8(),
            0xF1 => self.pop_af(),
            0xF2 => self.ld_a_c_ind(),
            0xF3 => self.di(),
            0xF5 => self.push_af(),
            0xF6 => self.or_a_d8(),
            0xF7 => self.rst_30(),
            0xF8 => self.ld_hl_sp_r8(),
            0xF9 => self.ld_sp_hl(),
            0xFA => self.ld_a_a16(),
            0xFB => self.ei(),
            0xFE => self.cp_a_d8(),
            0xFF => self.rst_38(),

            _ => panic!("Could not parse opcode: {:X}", opcode),
        }
    }

    pub fn current_pc(&mut self) -> u16 {
        let cur = self.bank.pc.get_value();
        self.bank.pc.set_reg(cur + 0x1);
        cur
    }

    pub fn nop(&mut self) {
        self.tick(1);
    }

    pub fn adc_a_r8(&mut self, reg: Reg8) {
        let a_val = self.bank.get_8_bit_reg(&Reg8::A);
        let r_val = self.bank.get_8_bit_reg(&reg);

        let carry = self.bank.get_flag(cpu::Flag::CARRY);

        let sum = a_val.wrapping_add(r_val).wrapping_add(carry);

        self.bank.set_8_bit_reg(&Reg8::A, sum);
        self.bank.check_all_8_sum_flags(&Reg8::A, a_val, r_val);

        self.tick(1);
    }

    pub fn adc_a_hl(&mut self) {
        let a_val = self.bank.get_8_bit_reg(&Reg8::A);
        let r_val = self
            .bus
            .mem
            .read_byte_u8(self.bank.get_16_bit_reg(&cpu::Reg16::HL));

        let carry = self.bank.get_flag(cpu::Flag::CARRY);

        let sum = a_val.wrapping_add(r_val).wrapping_add(carry);

        self.bank.set_8_bit_reg(&Reg8::A, sum);
        self.bank.check_all_8_sum_flags(&Reg8::A, a_val, r_val);

        self.tick(2);
    }

    pub fn adc_a_n8(&mut self) {
        let val = self.fetch_argument_u8();

        let a_val = self.bank.get_8_bit_reg(&Reg8::A);

        let carry = self.bank.get_flag(cpu::Flag::CARRY);

        let sum = a_val.wrapping_add(val).wrapping_add(carry);

        self.bank.set_8_bit_reg(&Reg8::A, sum);
        self.bank.check_all_8_sum_flags(&Reg8::A, a_val, val);

        self.tick(2);
    }

    pub fn add_a_r8(&mut self, reg: Reg8) {
        let a_val = self.bank.get_8_bit_reg(&Reg8::A);
        let r_val = self.bank.get_8_bit_reg(&reg);

        let sum = a_val.wrapping_add(r_val);

        self.bank.set_8_bit_reg(&Reg8::A, sum);
        self.bank.check_all_8_sum_flags(&Reg8::A, a_val, r_val);

        self.tick(1);
    }

    pub fn add_a_hl(&mut self) {
        let a_val = self.bank.get_8_bit_reg(&Reg8::A);
        let r_val = self
            .bus
            .mem
            .read_byte_u8(self.bank.get_16_bit_reg(&cpu::Reg16::HL));

        let sum = a_val.wrapping_add(r_val);

        self.bank.set_8_bit_reg(&Reg8::A, sum);
        self.bank.check_all_8_sum_flags(&Reg8::A, a_val, r_val);

        self.tick(2);
    }

    pub fn add_a_n8(&mut self) {
        let val = self.fetch_argument_u8();
        let a_val = self.bank.get_8_bit_reg(&Reg8::A);
        let sum = a_val.wrapping_add(val);

        self.bank.set_8_bit_reg(&Reg8::A, sum);
        self.bank.check_all_8_sum_flags(&Reg8::A, a_val, val);

        self.tick(2);
    }

    pub fn add_hl_r16(&mut self, val: &Reg16) {
        let a_val = self.bank.get_16_bit_reg(&Reg16::HL);
        let r_val = self.bank.get_16_bit_reg(val);
        let sum = a_val.wrapping_add(r_val);

        self.bank.set_16_bit_reg(&Reg16::HL, sum);
        self.bank.check_all_16_sum_flags(&Reg16::HL, a_val, r_val);

        self.tick(2);
    }

    pub fn add_hl_sp(&mut self) {
        let a_val = self.bank.get_16_bit_reg(&Reg16::HL);
        let r_val = self.bank.get_16_bit_reg(&Reg16::SP);

        let sum = a_val.wrapping_add(r_val);

        self.bank.set_16_bit_reg(&Reg16::HL, sum);
        self.bank.check_all_16_sum_flags(&Reg16::HL, a_val, r_val);

        self.tick(2);
    }

    pub fn add_sp_e8(&mut self) {
        let val = self.fetch_argument_i8();

        let a_val = self.bank.get_16_bit_reg(&Reg16::SP);

        let sum = a_val.wrapping_add_signed(val as i16);

        self.bank.set_16_bit_reg(&Reg16::SP, sum);
        self.bank
            .check_all_16_sum_flags(&Reg16::SP, a_val, val as u16);

        self.tick(4);
    }

    pub fn and_a_r8(&mut self, reg: &Reg8) {
        let a_val = self.bank.get_8_bit_reg(&Reg8::A);
        let r_val = self.bank.get_8_bit_reg(reg);

        let res = a_val & r_val;

        self.bank.set_8_bit_reg(&Reg8::A, res);
        self.bank.set_flags_and(res);

        self.tick(1);
    }

    pub fn and_a_hl(&mut self) {
        let a_val = self.bank.get_8_bit_reg(&Reg8::A);
        let r_val = self
            .bus
            .mem
            .read_byte_u8(self.bank.get_16_bit_reg(&cpu::Reg16::HL));

        let res = a_val & r_val;

        self.bank.set_8_bit_reg(&Reg8::A, res);
        self.bank.set_flags_and(res);

        self.tick(2);
    }

    pub fn and_a_n8(&mut self) {
        let val = self.fetch_argument_u8();
        let a_val = self.bank.get_8_bit_reg(&Reg8::A);

        let res = a_val & val;

        self.bank.set_8_bit_reg(&Reg8::A, res);
        self.bank.set_flags_and(res);

        self.tick(2);
    }
}
