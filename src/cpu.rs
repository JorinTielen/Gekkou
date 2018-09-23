use std::process;

use mmu::MMU;
use registers::Registers;
use registers::Flag::{C, H, N, Z};

#[derive(Default)]
struct Clock {
    m: usize,
    t: usize,
}

pub struct CPU {
    clock: Clock,
    regs: Registers,

    mmu: MMU,
}

impl CPU {
    pub fn new(rom: Vec<u8>) -> Self {
        CPU {
            clock: Clock::default(),
            regs: Registers::new(),

            mmu: MMU::new(rom),
        }
    }

    pub fn cycle(&mut self) -> usize {
        let op = self.next_byte();
        println!("op code: {:#X}", op);

        // Example operation template:
        // (opcode) (params)
        // (bytes)  (cycle)
        //
        // opcode: The opcode name
        // params: Any parameters needed
        // bytes: How many bytes the operation takes
        // cycle: how many machine cycles
        //        (at 1Mz) the operation takes
        let cycles = match op {
            0xC3 => {
                // JP nn
                // 3  4
                println!("Executing Opcode {:#X}", op);
                self.regs.pc = self.next_word();
                4
            },
            0xAF => {
                // XOR n
                // 1   1
                let a = self.regs.a ^ self.regs.a;
                self.regs.set_flag(Z, a == 0);
                self.regs.set_flag(N, false);
                self.regs.set_flag(H, false);
                self.regs.set_flag(C, false);
                self.regs.a = a;
                1
            }
            _ => {
                println!("Unrecognized Operation {:#X}! Exiting program...", op);
                process::exit(1);
            },
        };

        cycles
    }

    fn next_byte(&mut self) -> u8 {
        let b = self.mmu.rb(self.regs.pc);
        self.regs.pc += 1;
        b
    }

    fn next_word(&mut self) -> u16 {
        let w = self.mmu.rw(self.regs.pc);
        self.regs.pc += 2;
        w
    }
}