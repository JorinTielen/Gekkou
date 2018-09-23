mod mbc0;

pub trait MBC {
    fn rb(&self, a: u16) -> u8;
    fn rw(&self, a: u16) -> u16;
}

pub fn get_mbc(rom: Vec<u8>) -> impl MBC {
    mbc0::MBC0::new(rom)
}