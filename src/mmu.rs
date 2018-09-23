use mbc;
use mbc::MBC;

pub struct MMU {
    ram: [u8; 8192],
    mbc: Box<MBC>,
}

impl MMU {
    pub fn new(rom: Vec<u8>) -> Self {
        MMU {
            ram: [0; 8192],
            mbc: Box::new(mbc::get_mbc(rom)),
        }
    }

    pub fn rb(&mut self, a: u16) -> u8 {
        match a {
            0x0000 ... 0x7FFF => { self.mbc.rb(a) }
            _ =>  {
                println!("Unrecognized address {:#X}", a);
                0
            }
        }
    }

    pub fn rw(&mut self, a: u16) -> u16 {
        self.mbc.rw(a)
    }
}