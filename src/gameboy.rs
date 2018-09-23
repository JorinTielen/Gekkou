use std;
use std::fs::File;
use std::io::Read;

use cpu::CPU;

pub struct Gameboy {
    cpu: CPU,
}

impl Gameboy {
    pub fn new(rom: File) -> Self {
        Gameboy {
            cpu: CPU::new(load_rom(rom)),
        }
    }

    pub fn start(&mut self) {
        loop {
            self.cpu.cycle();
        }
    }
}

fn load_rom(rom: File) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

    for (_, byte) in rom.bytes().enumerate() {
        let byte = match byte {
            Ok(byte) => byte,
            Err(msg) => {
                println!("Error loading ROM: {}", msg);
                std::process::exit(1);
            }
        };
        data.push(byte);
    }

    data
}