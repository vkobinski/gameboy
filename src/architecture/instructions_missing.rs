use crate::architecture::cpu::Reg8;

use super::cpu;

fn pass() {}

impl cpu::Cpu {
    pub fn ld_bc_d16(&mut self) {
        let arg = self.fetch_argument_u16();

        self.bank.set_8_bit_reg(&Reg8::B, (arg >> 8) as u8);
        self.bank.set_8_bit_reg(&Reg8::C, (arg) as u8);
    }

    pub fn ld_bc_a(&mut self) {
        let addr = self.bank.get_16_bit_reg(&cpu::Reg16::BC);
        let val = self.bank.get_8_bit_reg(&cpu::Reg8::A);

        self.bus.mem.store_byte(addr, val);
    }

    pub fn inc_bc(&mut self) {
        let val = self.bank.get_16_bit_reg(&cpu::Reg16::BC);
        self.bank.set_16_bit_reg(&cpu::Reg16::BC, val + 1);
    }

    pub fn inc_b(&mut self) {
        let old_val = self.bank.get_8_bit_reg(&cpu::Reg8::B);
        let val = old_val.wrapping_add(1);
        self.bank.set_8_bit_reg(&cpu::Reg8::B, val);

        self.bank.check_inc_flags_8(&cpu::Reg8::B, old_val);
    }

    pub fn dec_b(&mut self) {
        let old_val = self.bank.get_8_bit_reg(&cpu::Reg8::B);
        let val = old_val.wrapping_sub(1);
        self.bank.set_8_bit_reg(&cpu::Reg8::B, val);

        self.bank.check_dec_flags_8(&cpu::Reg8::B, old_val);
    }

    pub fn ld_b_d8(&mut self) {
        let arg = self.fetch_argument_u8();

        self.bank.set_8_bit_reg(&cpu::Reg8::B, arg);
    }

    pub fn rlca(&mut self) {
        pass();
    } // 0x07

    pub fn ld_a16_sp(&mut self) {
        pass();
    } // 0x08

    pub fn ld_a_bc(&mut self) {
        let addr = self.bank.get_16_bit_reg(&cpu::Reg16::BC);
        let val = self.bus.mem.read_byte_u8(addr);

        self.bank.set_8_bit_reg(&cpu::Reg8::A, val);
    }

    pub fn dec_bc(&mut self) {
        let old_val = self.bank.get_16_bit_reg(&cpu::Reg16::BC);
        let val = old_val.wrapping_sub(1);
        self.bank.set_16_bit_reg(&cpu::Reg16::BC, val);
    }

    pub fn inc_c(&mut self) {
        let old_val = self.bank.get_8_bit_reg(&cpu::Reg8::C);
        let val = old_val.wrapping_add(1);
        self.bank.set_8_bit_reg(&cpu::Reg8::C, val);
    }

    pub fn dec_c(&mut self) {
        let old_val = self.bank.get_8_bit_reg(&cpu::Reg8::C);
        let val = old_val.wrapping_sub(1);
        self.bank.set_8_bit_reg(&cpu::Reg8::C, val);
    }

    pub fn ld_c_d8(&mut self) {
        let val = self.fetch_argument_u8();
        self.bank.set_8_bit_reg(&cpu::Reg8::C, val);
    }

    pub fn rrca(&mut self) {
        pass();
    }

    pub fn stop(&mut self) {
        pass();
    }

    pub fn ld_de_d16(&mut self) {
        let val = self.fetch_argument_u16();
        self.bank.set_16_bit_reg(&cpu::Reg16::DE, val);
    }

    pub fn ld_de_a(&mut self) {
        let addr = self.bank.get_16_bit_reg(&cpu::Reg16::DE);
        let val = self.bus.mem.read_byte_u8(addr);

        self.bank.set_8_bit_reg(&cpu::Reg8::A, val);
    }

    pub fn inc_de(&mut self) {
        let old_val = self.bank.get_16_bit_reg(&cpu::Reg16::DE);
        let val = old_val.wrapping_add(1);
        self.bank.set_16_bit_reg(&cpu::Reg16::DE, val);
    }

    pub fn inc_d(&mut self) {
        let old_val = self.bank.get_8_bit_reg(&cpu::Reg8::D);
        let val = old_val.wrapping_add(1);
        self.bank.set_8_bit_reg(&cpu::Reg8::D, val);
    }

    pub fn dec_d(&mut self) {
        let old_val = self.bank.get_8_bit_reg(&cpu::Reg8::D);
        let val = old_val.wrapping_sub(1);
        self.bank.set_8_bit_reg(&cpu::Reg8::D, val);
    }

    pub fn ld_d_d8(&mut self) {
        let val = self.fetch_argument_u8();
        self.bank.set_8_bit_reg(&cpu::Reg8::D, val);
    }

    pub fn rla(&mut self) {
        pass();
    } // 0x17

    pub fn jr_r8(&mut self) {
        let val = self.fetch_argument_i8();
        let addr = self.bank.pc.get_value();

        self.bank.pc.set_reg(addr.wrapping_add_signed(val as i16));
    }

    pub fn ld_a_de(&mut self) {
        pass();
    } // 0x1A  LD A,(DE)

    pub fn dec_de(&mut self) {
        pass();
    } // 0x1B

    pub fn inc_e(&mut self) {
        pass();
    } // 0x1C

    pub fn dec_e(&mut self) {
        pass();
    } // 0x1D

    pub fn ld_e_d8(&mut self) {
        pass();
    } // 0x1E

    pub fn rra(&mut self) {
        pass();
    } // 0x1F

    pub fn jr_nz_r8(&mut self) {
        pass();
    } // 0x20

    pub fn ld_hl_d16(&mut self) {
        pass();
    } // 0x21

    pub fn ld_hli_a(&mut self) {
        pass();
    } // 0x22  LD (HL+),A

    pub fn inc_hl(&mut self) {
        pass();
    } // 0x23

    pub fn inc_h(&mut self) {
        pass();
    } // 0x24

    pub fn dec_h(&mut self) {
        pass();
    } // 0x25

    pub fn ld_h_d8(&mut self) {
        pass();
    } // 0x26

    pub fn daa(&mut self) {
        pass();
    } // 0x27

    pub fn jr_z_r8(&mut self) {
        pass();
    } // 0x28

    pub fn ld_a_hli(&mut self) {
        pass();
    } // 0x2A  LD A,(HL+)

    pub fn dec_hl(&mut self) {
        pass();
    } // 0x2B

    pub fn inc_l(&mut self) {
        pass();
    } // 0x2C

    pub fn dec_l(&mut self) {
        pass();
    } // 0x2D

    pub fn ld_l_d8(&mut self) {
        pass();
    } // 0x2E

    pub fn cpl(&mut self) {
        pass();
    } // 0x2F

    pub fn jr_nc_r8(&mut self) {
        pass();
    } // 0x30

    pub fn ld_sp_d16(&mut self) {
        pass();
    } // 0x31

    pub fn ld_hld_a(&mut self) {
        pass();
    } // 0x32  LD (HL-),A

    pub fn inc_sp(&mut self) {
        pass();
    } // 0x33

    pub fn inc_hl_ind(&mut self) {
        pass();
    } // 0x34  INC (HL)

    pub fn dec_hl_ind(&mut self) {
        pass();
    } // 0x35  DEC (HL)

    pub fn ld_hl_ind_d8(&mut self) {
        pass();
    } // 0x36  LD (HL),d8

    pub fn scf(&mut self) {
        pass();
    } // 0x37

    pub fn jr_c_r8(&mut self) {
        pass();
    } // 0x38

    pub fn ld_a_hld(&mut self) {
        pass();
    } // 0x3A  LD A,(HL-)

    pub fn dec_sp(&mut self) {
        pass();
    } // 0x3B

    pub fn inc_a(&mut self) {
        pass();
    } // 0x3C

    pub fn dec_a(&mut self) {
        pass();
    } // 0x3D

    pub fn ld_a_d8(&mut self) {
        pass();
    } // 0x3E

    pub fn ccf(&mut self) {
        pass();
    } // 0x3F

    // 0x40-0x7F: LD r8,r8 / LD r8,(HL) / LD (HL),r8 grid, minus 0x76 = HALT
    pub fn ld_b_b(&mut self) {
        pass();
    } // 0x40

    pub fn ld_b_c(&mut self) {
        pass();
    } // 0x41

    pub fn ld_b_d(&mut self) {
        pass();
    } // 0x42

    pub fn ld_b_e(&mut self) {
        pass();
    } // 0x43

    pub fn ld_b_h(&mut self) {
        pass();
    } // 0x44

    pub fn ld_b_l(&mut self) {
        pass();
    } // 0x45

    pub fn ld_b_hl(&mut self) {
        pass();
    } // 0x46  LD B,(HL)

    pub fn ld_b_a(&mut self) {
        pass();
    } // 0x47

    pub fn ld_c_b(&mut self) {
        pass();
    } // 0x48

    pub fn ld_c_c(&mut self) {
        pass();
    } // 0x49

    pub fn ld_c_d(&mut self) {
        pass();
    } // 0x4A

    pub fn ld_c_e(&mut self) {
        pass();
    } // 0x4B

    pub fn ld_c_h(&mut self) {
        pass();
    } // 0x4C

    pub fn ld_c_l(&mut self) {
        pass();
    } // 0x4D

    pub fn ld_c_hl(&mut self) {
        pass();
    } // 0x4E  LD C,(HL)

    pub fn ld_c_a(&mut self) {
        pass();
    } // 0x4F

    pub fn ld_d_b(&mut self) {
        pass();
    } // 0x50

    pub fn ld_d_c(&mut self) {
        pass();
    } // 0x51

    pub fn ld_d_d(&mut self) {
        pass();
    } // 0x52

    pub fn ld_d_e(&mut self) {
        pass();
    } // 0x53

    pub fn ld_d_h(&mut self) {
        pass();
    } // 0x54

    pub fn ld_d_l(&mut self) {
        pass();
    } // 0x55

    pub fn ld_d_hl(&mut self) {
        pass();
    } // 0x56  LD D,(HL)

    pub fn ld_d_a(&mut self) {
        pass();
    } // 0x57

    pub fn ld_e_b(&mut self) {
        pass();
    } // 0x58

    pub fn ld_e_c(&mut self) {
        pass();
    } // 0x59

    pub fn ld_e_d(&mut self) {
        pass();
    } // 0x5A

    pub fn ld_e_e(&mut self) {
        pass();
    } // 0x5B

    pub fn ld_e_h(&mut self) {
        pass();
    } // 0x5C

    pub fn ld_e_l(&mut self) {
        pass();
    } // 0x5D

    pub fn ld_e_hl(&mut self) {
        pass();
    } // 0x5E  LD E,(HL)

    pub fn ld_e_a(&mut self) {
        pass();
    } // 0x5F

    pub fn ld_h_b(&mut self) {
        pass();
    } // 0x60

    pub fn ld_h_c(&mut self) {
        pass();
    } // 0x61

    pub fn ld_h_d(&mut self) {
        pass();
    } // 0x62

    pub fn ld_h_e(&mut self) {
        pass();
    } // 0x63

    pub fn ld_h_h(&mut self) {
        pass();
    } // 0x64

    pub fn ld_h_l(&mut self) {
        pass();
    } // 0x65

    pub fn ld_h_hl(&mut self) {
        pass();
    } // 0x66  LD H,(HL)

    pub fn ld_h_a(&mut self) {
        pass();
    } // 0x67

    pub fn ld_l_b(&mut self) {
        pass();
    } // 0x68

    pub fn ld_l_c(&mut self) {
        pass();
    } // 0x69

    pub fn ld_l_d(&mut self) {
        pass();
    } // 0x6A

    pub fn ld_l_e(&mut self) {
        pass();
    } // 0x6B

    pub fn ld_l_h(&mut self) {
        pass();
    } // 0x6C

    pub fn ld_l_l(&mut self) {
        pass();
    } // 0x6D

    pub fn ld_l_hl(&mut self) {
        pass();
    } // 0x6E  LD L,(HL)

    pub fn ld_l_a(&mut self) {
        pass();
    } // 0x6F

    pub fn ld_hl_ind_b(&mut self) {
        pass();
    } // 0x70  LD (HL),B

    pub fn ld_hl_ind_c(&mut self) {
        pass();
    } // 0x71

    pub fn ld_hl_ind_d(&mut self) {
        pass();
    } // 0x72

    pub fn ld_hl_ind_e(&mut self) {
        pass();
    } // 0x73

    pub fn ld_hl_ind_h(&mut self) {
        pass();
    } // 0x74

    pub fn ld_hl_ind_l(&mut self) {
        pass();
    } // 0x75

    pub fn halt(&mut self) {
        pass();
    } // 0x76

    pub fn ld_hl_ind_a(&mut self) {
        pass();
    } // 0x77  LD (HL),A

    pub fn ld_a_b(&mut self) {
        pass();
    } // 0x78

    pub fn ld_a_c(&mut self) {
        pass();
    } // 0x79

    pub fn ld_a_d(&mut self) {
        pass();
    } // 0x7A

    pub fn ld_a_e(&mut self) {
        pass();
    } // 0x7B

    pub fn ld_a_h(&mut self) {
        pass();
    } // 0x7C

    pub fn ld_a_l(&mut self) {
        pass();
    } // 0x7D

    pub fn ld_a_hl_ind(&mut self) {
        pass();
    } // 0x7E  LD A,(HL)

    pub fn ld_a_a(&mut self) {
        pass();
    } // 0x7F

    pub fn sub_a_b(&mut self) {
        pass();
    } // 0x90

    pub fn sub_a_c(&mut self) {
        pass();
    } // 0x91

    pub fn sub_a_d(&mut self) {
        pass();
    } // 0x92

    pub fn sub_a_e(&mut self) {
        pass();
    } // 0x93

    pub fn sub_a_h(&mut self) {
        pass();
    } // 0x94

    pub fn sub_a_l(&mut self) {
        pass();
    } // 0x95

    pub fn sub_a_hl(&mut self) {
        pass();
    } // 0x96

    pub fn sub_a_a(&mut self) {
        pass();
    } // 0x97

    pub fn sbc_a_b(&mut self) {
        pass();
    } // 0x98

    pub fn sbc_a_c(&mut self) {
        pass();
    } // 0x99

    pub fn sbc_a_d(&mut self) {
        pass();
    } // 0x9A

    pub fn sbc_a_e(&mut self) {
        pass();
    } // 0x9B

    pub fn sbc_a_h(&mut self) {
        pass();
    } // 0x9C

    pub fn sbc_a_l(&mut self) {
        pass();
    } // 0x9D

    pub fn sbc_a_hl(&mut self) {
        pass();
    } // 0x9E

    pub fn sbc_a_a(&mut self) {
        pass();
    } // 0x9F

    pub fn xor_a_b(&mut self) {
        pass();
    } // 0xA8

    pub fn xor_a_c(&mut self) {
        pass();
    } // 0xA9

    pub fn xor_a_d(&mut self) {
        pass();
    } // 0xAA

    pub fn xor_a_e(&mut self) {
        pass();
    } // 0xAB

    pub fn xor_a_h(&mut self) {
        pass();
    } // 0xAC

    pub fn xor_a_l(&mut self) {
        pass();
    } // 0xAD

    pub fn xor_a_hl(&mut self) {
        pass();
    } // 0xAE

    pub fn xor_a_a(&mut self) {
        pass();
    } // 0xAF

    pub fn or_a_b(&mut self) {
        pass();
    } // 0xB0

    pub fn or_a_c(&mut self) {
        pass();
    } // 0xB1

    pub fn or_a_d(&mut self) {
        pass();
    } // 0xB2

    pub fn or_a_e(&mut self) {
        pass();
    } // 0xB3

    pub fn or_a_h(&mut self) {
        pass();
    } // 0xB4

    pub fn or_a_l(&mut self) {
        pass();
    } // 0xB5

    pub fn or_a_hl(&mut self) {
        pass();
    } // 0xB6

    pub fn or_a_a(&mut self) {
        pass();
    } // 0xB7

    pub fn cp_a_b(&mut self) {
        pass();
    } // 0xB8

    pub fn cp_a_c(&mut self) {
        pass();
    } // 0xB9

    pub fn cp_a_d(&mut self) {
        pass();
    } // 0xBA

    pub fn cp_a_e(&mut self) {
        pass();
    } // 0xBB

    pub fn cp_a_h(&mut self) {
        pass();
    } // 0xBC

    pub fn cp_a_l(&mut self) {
        pass();
    } // 0xBD

    pub fn cp_a_hl(&mut self) {
        pass();
    } // 0xBE

    pub fn cp_a_a(&mut self) {
        pass();
    } // 0xBF

    pub fn ret_nz(&mut self) {
        pass();
    } // 0xC0

    pub fn pop_bc(&mut self) {
        pass();
    } // 0xC1

    pub fn jp_nz_a16(&mut self) {
        pass();
    } // 0xC2

    pub fn jp_a16(&mut self) {
        pass();
    } // 0xC3

    pub fn call_nz_a16(&mut self) {
        pass();
    } // 0xC4

    pub fn push_bc(&mut self) {
        pass();
    } // 0xC5

    pub fn rst_00(&mut self) {
        pass();
    } // 0xC7

    pub fn ret_z(&mut self) {
        pass();
    } // 0xC8

    pub fn ret(&mut self) {
        pass();
    } // 0xC9

    pub fn jp_z_a16(&mut self) {
        pass();
    } // 0xCA

    pub fn cb_prefixed(&mut self) {
        pass();
    } // 0xCB — 256-opcode CB table, not scoped yet

    pub fn call_z_a16(&mut self) {
        pass();
    } // 0xCC

    pub fn call_a16(&mut self) {
        pass();
    } // 0xCD

    pub fn rst_08(&mut self) {
        pass();
    } // 0xCF

    pub fn ret_nc(&mut self) {
        pass();
    } // 0xD0

    pub fn pop_de(&mut self) {
        pass();
    } // 0xD1

    pub fn jp_nc_a16(&mut self) {
        pass();
    } // 0xD2

    pub fn call_nc_a16(&mut self) {
        pass();
    } // 0xD4

    pub fn push_de(&mut self) {
        pass();
    } // 0xD5

    pub fn sub_a_d8(&mut self) {
        pass();
    } // 0xD6

    pub fn rst_10(&mut self) {
        pass();
    } // 0xD7

    pub fn ret_c(&mut self) {
        pass();
    } // 0xD8

    pub fn reti(&mut self) {
        pass();
    } // 0xD9

    pub fn jp_c_a16(&mut self) {
        pass();
    } // 0xDA

    pub fn call_c_a16(&mut self) {
        pass();
    } // 0xDC

    pub fn sbc_a_d8(&mut self) {
        pass();
    } // 0xDE

    pub fn rst_18(&mut self) {
        pass();
    } // 0xDF

    pub fn ldh_a8_a(&mut self) {
        pass();
    } // 0xE0  LDH (a8),A

    pub fn pop_hl(&mut self) {
        pass();
    } // 0xE1

    pub fn ld_c_ind_a(&mut self) {
        pass();
    } // 0xE2  LD (C),A

    pub fn push_hl(&mut self) {
        pass();
    } // 0xE5

    pub fn rst_20(&mut self) {
        pass();
    } // 0xE7

    pub fn jp_hl(&mut self) {
        pass();
    } // 0xE9

    pub fn ld_a16_a(&mut self) {
        pass();
    } // 0xEA

    pub fn xor_a_d8(&mut self) {
        pass();
    } // 0xEE

    pub fn rst_28(&mut self) {
        pass();
    } // 0xEF

    pub fn ldh_a_a8(&mut self) {
        pass();
    } // 0xF0  LDH A,(a8)

    pub fn pop_af(&mut self) {
        pass();
    } // 0xF1

    pub fn ld_a_c_ind(&mut self) {
        pass();
    } // 0xF2  LD A,(C)

    pub fn di(&mut self) {
        pass();
    } // 0xF3

    pub fn push_af(&mut self) {
        pass();
    } // 0xF5

    pub fn or_a_d8(&mut self) {
        pass();
    } // 0xF6

    pub fn rst_30(&mut self) {
        pass();
    } // 0xF7

    pub fn ld_hl_sp_r8(&mut self) {
        pass();
    } // 0xF8  LD HL,SP+r8

    pub fn ld_sp_hl(&mut self) {
        pass();
    } // 0xF9

    pub fn ld_a_a16(&mut self) {
        pass();
    } // 0xFA

    pub fn ei(&mut self) {
        pass();
    } // 0xFB

    pub fn cp_a_d8(&mut self) {
        pass();
    } // 0xFE

    pub fn rst_38(&mut self) {
        pass();
    } // 0xFF
}
