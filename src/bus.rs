#[derive(Default)]
pub struct Bus {
    mem: Vec<u8>,
    pub len: u32,
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

    pub fn as_mut_slice(&mut self, start: u32) -> &mut [u8] {
        &mut self.mem[(start as usize)..]
    }

    pub fn read_byte(&self, addr: u32) -> u8 {
        self.mem[(addr as usize & 0x07ff)]
    }

    pub fn write_byte(&mut self, addr: u32, val: u8) {
        self.mem[(addr as usize & 0x07ff)] = val;
    }

    pub fn read_word(&self, addr: u32) -> u16 {
        let addr = addr & 0x07ff;
        (self.mem[addr as usize] as u16) | ((self.mem[(addr as usize + 1)]) << 8) as u16
    }

    pub fn write_word(&mut self, addr: u32, val: u16) {
        let addr = addr & 0x07ff;
        self.mem[addr as usize] = (val >> 8) as u8;
        self.mem[(addr as usize + 1)] = (val & 0xff) as u8;
    }
}
