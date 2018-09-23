use std::env;
use std::fs::File;

mod gameboy;
mod mmu;
mod cpu;
mod mbc;
mod registers;

use gameboy::Gameboy;

fn main() {
    let rom_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Please provide a rom file. Exiting program...");
            std::process::exit(1);
        }
    };

    let rom = match File::open(&rom_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Error opening file at {}. Exiting program...", rom_path);
            std::process::exit(1);
        }
    };

    let mut game_boy = Gameboy::new(rom);
    game_boy.start();
}
