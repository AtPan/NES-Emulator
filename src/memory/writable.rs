use std::ops::Add;

pub enum Endianness {
    Big,
    Little,
}

pub trait Readable<T>
where
    T: Add<T, Output = T> + From<u8> + Copy
{
    fn has_endian(&self) -> Endianness;
    fn read_byte(&self, addr: T) -> u8;

    fn read_word(&self, addr: T) -> u16 {
        let (shift1, shift2) = match self.has_endian() {
            Endianness::Big => (8, 0),
            Endianness::Little => (0, 8),
        };

        ((self.read_byte(addr) as u16) << shift1) | ((self.read_byte(addr + T::from(1)) as u16) << shift2)
    }
}

pub trait Writable<T>
where
    T: Add<T, Output = T> + From<u8> + Copy
{
    fn as_mut_slice(&mut self, addr: T) -> &mut [u8];
    fn has_endian(&self) -> Endianness;
    fn write_byte(&mut self, addr: T, byte: u8);

    fn write_word(&mut self, addr: T, word: u16) {
        let (shift1, shift2) = match self.has_endian() {
            Endianness::Big => (8, 0),
            Endianness::Little => (0, 8),
        };

        self.write_byte(addr, (word >> shift1) as u8);
        self.write_byte(addr + T::from(1), (word >> shift2) as u8);
    }
}
