use crate::{
    bus::{Bus, memory::Memory},
    cpu::{Cpu, CpuEvent},
    display::*,
    keypad::Keypad,
};

use std::{rc::Rc, cell::RefCell};

pub use clap::Parser;
pub use minifb::{Key, KeyRepeat, Window, WindowOptions};

mod bus;
mod cpu;
mod display;
mod keypad;

#[derive(Parser, Debug, Default)]
#[command(version, about)]
pub struct Args {
    /// Path to Chip-8 ROM
    rom_path: String,
}

pub struct Core {
    cpu: Cpu,
    display: Rc<RefCell<Display>>,
    keypad: Rc<RefCell<Keypad>>,
}

impl Core {
    const SPRITES_START: usize = 0x50;
    const ROM_START: usize = 0x200;

    const SPRITE_SIZE: usize = 5;
    const NUM_SPRITES: usize = 16;

    const SPRITES_SIZE: usize = Self::SPRITE_SIZE * Self::NUM_SPRITES;
    const MAX_ROM_SIZE: usize = Memory::SIZE - Self::ROM_START;

    const SPRITES: [u8; Self::SPRITES_SIZE] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0,
        0x20, 0x60, 0x20, 0x20, 0x70,
        0xF0, 0x10, 0xF0, 0x80, 0xF0,
        0xF0, 0x10, 0xF0, 0x10, 0xF0,
        0x90, 0x90, 0xF0, 0x10, 0x10,
        0xF0, 0x80, 0xF0, 0x10, 0xF0,
        0xF0, 0x80, 0xF0, 0x90, 0xF0,
        0xF0, 0x10, 0x20, 0x40, 0x40,
        0xF0, 0x90, 0xF0, 0x90, 0xF0,
        0xF0, 0x90, 0xF0, 0x10, 0xF0,
        0xF0, 0x90, 0xF0, 0x90, 0x90,
        0xE0, 0x90, 0xE0, 0x90, 0xE0,
        0xF0, 0x80, 0x80, 0x80, 0xF0,
        0xE0, 0x90, 0x90, 0x90, 0xE0,
        0xF0, 0x80, 0xF0, 0x80, 0xF0,
        0xF0, 0x80, 0xF0, 0x80, 0x80,
    ];

    pub fn new(args: Args) -> Self {
        // Load ROM
        let mut mem = Memory::default();

        let rom = std::fs::read(args.rom_path).expect("Failed to read ROM");
        let len = usize::min(rom.len(), Self::MAX_ROM_SIZE);

        mem[Self::SPRITES_START..Self::SPRITES_START + Self::SPRITES_SIZE].copy_from_slice(&Self::SPRITES[..]);
        mem[Self::ROM_START..Self::ROM_START + len].copy_from_slice(&rom[..len]);

        let display = Rc::new(RefCell::new(Display::default()));
        let keypad = Rc::new(RefCell::new(Keypad::default()));

        Self {
            cpu: Cpu::new(Bus::new(mem), display.clone(), keypad.clone()),
            display,
            keypad,
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
            self.keypad.borrow_mut().update_state(window.get_keys());

            self.cpu.tick();

            'step_cpu: for _ in 0..Cpu::STEPS {
                if let Some(event) = self.cpu.step() {
                    match event {
                        CpuEvent::Draw | CpuEvent::WaitForKey => break 'step_cpu,
                    }
                }
            }

            window.update_with_buffer(self.display.borrow().as_slice(), Display::WIDTH, Display::HEIGHT).unwrap();
        }
    }
}
