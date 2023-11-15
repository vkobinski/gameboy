struct Memory {
    data: [u8; 0xFFFF],
}

impl Memory {
    pub fn new(&self) -> Self {
        Self {
            data: [0; 0xFFFF],
        }
    }

}
