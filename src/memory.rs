use std::fs::File;
use std::io::Read;

pub enum Endian {
    Big,
    Little,
}

pub struct Memory {
    buf: Vec<u8>,
    endian: Endian,
}

impl Memory {
    pub fn new(size: u32, endian: Endian) -> Memory {
        Memory {
            buf: vec![0; size as usize],
            endian,
        }
    }

    pub fn as_mut(&mut self, addr: usize) -> &mut u8 {
        &mut self.buf[addr]
    }

    pub fn read_byte(&self, addr: usize) -> u8 {
        self.buf[addr]
    }

    pub fn read_word(&self, addr: usize) -> u16 {
        let (shift1, shift2) = match self.endian {
            Endian::Big => (8, 0),
            Endian::Little => (0, 8),
        };
        ((self.buf[addr + 1] as u16) << shift1) + ((self.buf[addr] as u16) << shift2)
    }

    pub fn write_byte(&mut self, addr: usize, byte: u8) {
        self.buf[addr] = byte;
    }

    pub fn write_word(&mut self, addr: usize, word: u16) {
        let (shift1, shift2) = match self.endian {
            Endian::Big => (8, 0),
            Endian::Little => (0, 8),
        };
        self.buf[addr + 1] = (word >> shift1) as u8;
        self.buf[addr] = (word >> shift2) as u8;
    }

    pub fn load(&mut self, file: &str, addr: usize) -> std::io::Result<()> {
        let mut file = File::open(file)?;
        let buf = &mut self.buf[addr..];
        file.read_exact(buf)?;
        Ok(())
    }
}
