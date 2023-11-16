use super::{super::architecture::cpu::Cpu, super::architecture::mem::Memory};

pub struct Bus {
    pub mem: Memory,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            mem: Memory::new(),
        }
    }
}
