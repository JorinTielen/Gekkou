pub enum Flag {
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}

#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,

    flags: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Default::default()
    }

    pub fn set_flag(&mut self, flag: Flag, set: bool) {
        let mask = flag as u8;
        match set {
            true  => self.flags |=  mask,
            false => self.flags &= !mask,
        }
        self.flags &= 0xF0;
    }
}