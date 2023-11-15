pub struct Register {
    high: u8,
    low: u8,
}

impl Register {
    pub fn new() -> Self {
        Self { high: 0, low: 0 }
    }

    pub fn high_and(&mut self, val: u8) {
        self.high &= val;
    }

    pub fn low_and(&mut self, val: u8) {
        self.low &= val;
    }

    pub fn set_high(&mut self, val: u8) {
        self.high = val;
    }

    pub fn set_low(&mut self, val: u8) {
        self.low = val;
    }

    pub fn get_high(&self) -> u8 {
        self.high
    }

    pub fn get_low(&self) -> u8 {
        self.low
    }

    pub fn get_value(&self) -> u16 {
        let higher : u16 = u16::from(self.high) << 8;
        let entire: u16 = u16::from(self.low) | higher;
        entire
    }
}

enum Flag {
    ZERO = 7,
    SUB = 6,
    HALFCARRY = 5,
    CARRY = 4,
}

struct RegBank {
    af: Register,
    bc: Register,
    de: Register,
    hl: Register,
    sp: Register,
    pc: Register,
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
        self.af.get_low() & 1 >> pos as u8
    }

    pub fn set_flag(&mut self, pos: Flag) {
        let carry = 1 >> pos as u8;
        self.af.low_and(carry)
    }
}

pub struct Cpu {
    registers: RegBank,
}

