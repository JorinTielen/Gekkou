use mbc::MBC;

pub struct MBC0 {
    rom: Vec<u8>,
}

impl MBC0 {
    pub fn new(data: Vec<u8>) -> MBC0 {
        MBC0 {
            rom: data,
        }
    }
}

impl MBC for MBC0 {
    fn rb(&self, a: u16) -> u8 {
        println!("rb on mbc0");
        0
    }
}

