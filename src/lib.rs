use crate::{
    bus::{Bus, memory::Memory},
    cpu::{Cpu, CpuEvent},
    display::*,
};

use std::{rc::Rc, cell::RefCell};

pub use clap::Parser;
pub use minifb::{Key, Window, WindowOptions};

mod bus;
mod cpu;
mod display;

#[derive(Parser, Debug, Default)]
#[command(version, about)]
pub struct Args {
    /// Path to Chip-8 ROM
    rom_path: String,
}

pub struct Core {
    cpu: Cpu,
    display: Rc<RefCell<Display>>,
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

        // TODO: load Chip-8 font into RAM

        let display = Rc::new(RefCell::new(Display::default()));

        Self {
            cpu: Cpu::new(Bus::new(mem), display.clone()),
            display,
        }
    }

    pub fn run(&mut self) {
        let mut window = Window::new(
            "myuchip",
            Display::WIDTH,
            Display::HEIGHT,
            WindowOptions { borderless: false, title: true, resize: false, scale: minifb::Scale::X8, scale_mode: minifb::ScaleMode::Center, topmost: true, transparency: false, none: false },
        ).unwrap();

        window.set_target_fps(60);

        while window.is_open() && !window.is_key_down(Key::Escape) {
            'step_cpu: for _ in 0..Cpu::STEPS {
                if let Some(event) = self.cpu.step() {
                    match event {
                        CpuEvent::Draw => break 'step_cpu,
                        _ => {},
                    }
                }
            }

            self.cpu.tick();

            window.update_with_buffer((self.display.borrow()).as_slice(), Display::WIDTH, Display::HEIGHT).unwrap();
        }
    }
}
