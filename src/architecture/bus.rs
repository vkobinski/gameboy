use super::{super::architecture::cpu::Cpu, super::architecture::mem::Memory};

pub struct Bus {
    pub cpu: Cpu,
    pub mem: Memory,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            mem: Memory::new(),
        }
    }
}
