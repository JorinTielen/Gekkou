use mbc::MBC;

pub struct MBC0 {
    rom: Vec<u8>,
}

impl MBC0 {
    pub fn new(data: Vec<u8>) -> Self {
        MBC0 {
            rom: data,
        }
    }
}

impl MBC for MBC0 {
    fn rb(&self, a: u16) -> u8 {
        self.rom[a as usize]
    }

    fn rw(&self, a: u16) -> u16 {
        (self.rb(a) as u16) | ((self.rb(a + 1) as u16) << 8)
    }
}

