use super::{Readable,Writable,Endianness};

pub struct Bus {
    mem: Vec<u8>,
    pub len: u32,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            mem: vec![0; 0x1f400],
            len: 0x1f400,
        }
    }

    pub fn as_mut(&mut self, addr: u32) -> &mut u8 {
        &mut self.mem[addr as usize]
    }

    pub fn as_slice(&self, start: u32) -> &[u8] {
        &self.mem[(start as usize)..]
    }
}

impl Readable<u32> for Bus {
    fn has_endian(&self) -> Endianness {
        Endianness::Little
    }

    fn read_byte(&self, addr: u32) -> u8 {
        self.mem[(addr as usize) % self.len as usize]
    }
}

impl Writable<u32> for Bus {
    fn as_mut_slice(&mut self, addr: u32) -> &mut [u8] {
        &mut self.mem[(addr as usize)..]
    }

    fn has_endian(&self) -> Endianness {
        Endianness::Little
    }

    fn write_byte(&mut self, addr: u32, byte: u8) {
        self.mem[(addr as usize) % self.len as usize] = byte;
    }
}
