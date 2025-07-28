use crate::{bus::{Bus, memory::Memory}, cpu::Cpu};

pub use clap::Parser;

mod bus;
mod cpu;

#[derive(Parser, Debug, Default)]
#[command(version, about)]
pub struct Args {
    /// Path to Chip-8 ROM
    rom_path: String,
}

pub struct Core {
    cpu: Cpu,
}

impl Core {
    const ROM_START: usize = 0x200;
    const MAX_ROM_SIZE: usize = Memory::SIZE - Self::ROM_START;

    pub fn new(args: Args) -> Self {
        // Load ROM
        let mut mem = Memory::default();

        let rom = std::fs::read(args.rom_path).expect("Failed to read ROM");
        let len = usize::min(rom.len(), Self::MAX_ROM_SIZE);

        mem[Self::ROM_START..Self::ROM_START + len].copy_from_slice(&rom[..len]);

        Self {
            cpu: Cpu::new(Bus::new(mem)),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step();
        }
    }
}
