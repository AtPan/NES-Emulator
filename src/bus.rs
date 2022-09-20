use crate::memory::{Memory, Endian};

#[derive(Default)]
pub struct Bus {
    address: u16,
    data: u8,
    pub read: bool,
    memory: Memory,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            address: 0,
            data: 0,
            read: true,
            memory: Memory::new(0x800, Endian::Little),
        }
    }

    #[inline(always)]
    pub fn set_address(&mut self, addr: u16) {
        self.address = addr & 0x07ff;
    }

    #[inline(always)]
    pub fn set_data(&mut self, dat: u8) {
        self.data = dat;
    }

    #[inline(always)]
    pub fn read_address(&self) -> u16 {
        self.address
    }

    #[inline(always)]
    pub fn read_data(&self) -> u8 {
        self.data
    }

    pub fn interact_mem_byte(&mut self) {
        match self.read {
            true => self.data = self.memory.read_byte(self.address as usize),
            false => self.memory.write_byte(self.address as usize, self.data),
        }
    }
}
