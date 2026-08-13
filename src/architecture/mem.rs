pub struct Memory {
    data: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Self {
        Self { data: [0; 0xFFFF] }
    }

    pub fn read_byte_u8(&self, addr: u16) -> u8 {
        *self.data.get(addr as usize).unwrap_or(&0)
    }

    pub fn read_byte_i8(&self, addr: u16) -> i8 {
        *self.data.get(addr as usize).unwrap_or(&0) as i8
    }

    pub fn read_byte_u16(&self, addr: u16) -> u16 {
        let low = *self.data.get(addr as usize).unwrap_or(&0) as u16;
        let high = *self.data.get((addr as usize) + 1).unwrap_or(&0) as u16;
        (high << 8) | low
    }

    pub fn read_byte_i16(&self, addr: u16) -> i16 {
        let low = *self.data.get(addr as usize).unwrap_or(&0) as u16;
        let high = *self.data.get((addr as usize) + 1).unwrap_or(&0) as u16;
        ((high << 8) | low) as i16
    }

    pub fn read_byte_reg(&self, high: u8, low: u8) -> u8 {
        let addr: u16;

        let higher = (high as u16) & 0xFF00;
        let lower = low as u16;
        addr = higher | lower;

        *self.data.get(addr as usize).unwrap_or(&0)
    }

    pub fn store_byte(&mut self, addr: u16, val: u8) {
        self.data[addr as usize] = val;
    }
}
