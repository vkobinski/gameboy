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

    pub fn reg_part_and(&mut self, part: RegPos,val: u8) {
        match part {
            RegPos::HIGH => self.high &= val,
            RegPos::LOW => self.low &= val

        }
    }

    pub fn reg_part_or(&mut self, part: RegPos,val: u8) {
        println!("{}", val);
        match part {
            RegPos::HIGH => self.high |= val,
            RegPos::LOW => self.low |= val
        }
    }

    pub fn set_part(&mut self, part: RegPos, val: u8) {
        match part {
            RegPos::HIGH => self.high = val,
            RegPos::LOW => self.low = val
        }

    }

    pub fn get_part(&self, part: RegPos) -> u8 {
        match part {
            RegPos::HIGH => self.high,
            RegPos::LOW => self.low
        }
    }

    pub fn get_value(&self) -> u16 {
        let higher : u16 = u16::from(self.high) << 8;
        let entire: u16 = u16::from(self.low) | higher;
        entire
    }
}

#[derive(Clone)]
pub enum Flag {
    ZERO = 1,
    SUB = 2,
    HALFCARRY = 3,
    CARRY = 4,
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
        let carry = 1 << (pos as u8);
        self.af.reg_part_or(RegPos::LOW, carry)
    }

    pub fn unset_flag(&mut self, pos: Flag) {
        let carry = 0 << (pos as u8);
        self.af.reg_part_or(RegPos::LOW, carry)
    }

}

pub struct Cpu {
    registers: RegBank,
}

