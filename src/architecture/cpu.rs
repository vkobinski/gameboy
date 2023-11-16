use std::{thread, time::Duration};

use super::bus::Bus;

pub enum RegPos {
    HIGH,
    LOW,
}

pub struct Register {
    high: u8,
    low: u8,
}

impl Register {
    pub fn new() -> Self {
        Self { high: 0, low: 0 }
    }

    pub fn reg_part_and(&mut self, part: RegPos, val: u8) {
        match part {
            RegPos::HIGH => self.high &= val,
            RegPos::LOW => self.low &= val,
        }
    }

    pub fn reg_part_or(&mut self, part: RegPos, val: u8) {
        match part {
            RegPos::HIGH => self.high |= val,
            RegPos::LOW => self.low |= val,
        }
    }

    pub fn set_part(&mut self, part: RegPos, val: u8) {
        match part {
            RegPos::HIGH => self.high = val,
            RegPos::LOW => self.low = val,
        }
    }

    pub fn get_part(&self, part: RegPos) -> u8 {
        match part {
            RegPos::HIGH => self.high,
            RegPos::LOW => self.low,
        }
    }

    pub fn get_value(&self) -> u16 {
        let higher: u16 = u16::from(self.high) << 8;
        let entire: u16 = u16::from(self.low) | higher;
        entire
    }

    pub fn set_reg(&mut self, val: u16) {
        let higher = val >> 8;
        let lower = val & 0b11111111;
        self.high = higher as u8;
        self.low = lower as u8;
    }
}

#[derive(Clone)]
pub enum Flag {
    ZERO = 0,
    SUB = 1,
    HALFCARRY = 2,
    CARRY = 3,
}

pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum Reg16 {
    BC,
    DE,
    HL,
}

pub struct RegBank {
    pub af: Register,
    pub bc: Register,
    pub de: Register,
    pub hl: Register,
    pub sp: Register,
    pub pc: Register,
}

impl RegBank {
    pub fn new() -> Self {
        RegBank {
            af: (Register::new()),
            bc: (Register::new()),
            de: (Register::new()),
            hl: (Register::new()),
            sp: (Register::new()),
            pc: (Register::new()),
        }
    }

    pub fn get_flag(&self, pos: Flag) -> u8 {
        let res = self.af.get_part(RegPos::LOW) & 1 << (pos.clone() as u8);
        res >> pos as u8
    }

    pub fn set_flag(&mut self, pos: Flag) {
        let mask = 1 << (pos as u8);
        self.af.reg_part_or(RegPos::LOW, mask)
    }

    pub fn unset_flag(&mut self, pos: Flag) {
        let mask = !(1 << (pos as u8));
        self.af.reg_part_and(RegPos::LOW, mask)
    }

    // TODO(#2): The carry check should be before the reg value is changed
    pub fn check_all_8_sum_flags(&mut self, reg: &Reg8, reg_before: u8, val: u8) {
        self.check_zero_8(reg);
        self.check_half_carry_8(reg, val);
        self.check_carry_8(reg_before, val);
    }

    pub fn check_zero_8(&mut self, reg: &Reg8) {
        if self.get_8_bit_reg(reg) == 0 {
            self.set_flag(Flag::ZERO);
        }
    }

    pub fn check_half_carry_8(&mut self, reg: &Reg8, _val: u8) {
        let res = self.get_8_bit_reg(reg);

        if (res & 0x10) != 0 {
            self.set_flag(Flag::HALFCARRY);
            return;
        }
        self.unset_flag(Flag::HALFCARRY);
    }


    // TODO(#1): Check if carry checking is correct
    pub fn check_carry_8(&mut self, reg_before: u8, val: u8) {
        let sum = reg_before as u16 + val as u16 + self.get_flag(Flag::CARRY) as u16;

        if (sum & 0x100) != 0 {
            self.set_flag(Flag::CARRY);
            return;
        }
        self.unset_flag(Flag::CARRY);
    }


    pub fn check_all_16_sum_flags(&mut self, reg: &Reg16, reg_before: u16, val: u16) {
        self.check_zero_16(reg);
        self.check_half_carry_16(reg, val);
        self.check_carry_16(reg_before, val);
    }

    pub fn check_zero_16(&mut self, reg: &Reg16) {
        if self.get_16_bit_reg(reg) == 0 {
            self.set_flag(Flag::ZERO);
        }
    }

    pub fn check_half_carry_16(&mut self, reg: &Reg16, _val: u16) {
        let res = self.get_16_bit_reg(reg);

        if (res & 0x1000) != 0 {
            self.set_flag(Flag::HALFCARRY);
            return;
        }
        self.unset_flag(Flag::HALFCARRY);
    }


    pub fn check_carry_16(&mut self, reg_before: u16, val: u16) {
        let sum = reg_before as u32 + val as u32 + self.get_flag(Flag::CARRY) as u32;

        if (sum & 0x1000) != 0 {
            self.set_flag(Flag::CARRY);
            return;
        }
        self.unset_flag(Flag::CARRY);
    }



    pub fn get_8_bit_reg(&self, reg: &Reg8) -> u8 {
        match reg {
            Reg8::A => self.af.get_part(RegPos::HIGH),
            Reg8::B => self.bc.get_part(RegPos::HIGH),
            Reg8::C => self.bc.get_part(RegPos::LOW),
            Reg8::D => self.de.get_part(RegPos::HIGH),
            Reg8::E => self.de.get_part(RegPos::LOW),
            Reg8::H => self.hl.get_part(RegPos::HIGH),
            Reg8::L => self.hl.get_part(RegPos::LOW),
        }
    }

    pub fn set_8_bit_reg(&mut self, reg: &Reg8, val: u8) {
        match reg {
            Reg8::A => self.af.set_part(RegPos::HIGH, val),
            Reg8::B => self.bc.set_part(RegPos::HIGH, val),
            Reg8::C => self.bc.set_part(RegPos::LOW, val),
            Reg8::D => self.de.set_part(RegPos::HIGH, val),
            Reg8::E => self.de.set_part(RegPos::LOW, val),
            Reg8::H => self.hl.set_part(RegPos::HIGH, val),
            Reg8::L => self.hl.set_part(RegPos::LOW, val),
        }
    }


    pub fn get_16_bit_reg(&self, reg: &Reg16) -> u16 {
        match reg {
            Reg16::BC => self.bc.get_value(),
            Reg16::DE => self.de.get_value(),
            Reg16::HL => self.hl.get_value(),
        }
    }

    pub fn set_16_bit_reg(&mut self, reg: &Reg16, val: u16) {
        match reg {
            Reg16::BC => self.bc.set_reg(val),
            Reg16::DE => self.de.set_reg(val),
            Reg16::HL => self.hl.set_reg(val),
        }
    }
}

pub struct Cpu {
    pub bank: RegBank,
    pub bus: Bus,
    clocks: u64,
}

impl Cpu {
    pub fn new() -> Self {

        let bus = Bus::new();

        Self {
            bank: RegBank::new(),
            clocks: 0,
            bus,
        }
    }

    pub fn tick(&mut self, cycles: u8) {
        let clock_frequency = 4.194304e6; // 4.194304 MHz
        let sleep_time = Duration::from_secs_f64(1.0 / clock_frequency);

        for _ in 0..cycles {
            thread::sleep(sleep_time);
        }

        self.clocks += u64::from(cycles);
    }
}
