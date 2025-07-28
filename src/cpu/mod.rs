use crate::{
    bus::{Address, Bus},
    cpu::{opcode::Opcode, regfile::RegFile},
    display::Display,
};

use std::{rc::Rc, cell::RefCell};

mod opcode;
mod regfile;

type OpcodePattern = u16;
type OpcodeMask = u16;
type OpcodeHandler = fn(&mut Cpu, Opcode);

/// Opcode descriptor (opcode pattern, mask, handler)
#[derive(Clone, Copy)]
struct OpcodeDesc(OpcodePattern, OpcodeMask, OpcodeHandler);

struct OpcodeMatcher {
    registered_opcodes: Vec<OpcodeDesc>,
}

impl OpcodeMatcher {
    pub fn register(&mut self, desc: OpcodeDesc) {
        self.registered_opcodes.push(desc);
    }

    /// Matches opcode against registered opcodes and returns corresponding opcode handler
    pub fn match_opcode(&self, opcode: u16) -> OpcodeHandler {
        for desc in self.registered_opcodes.iter() {
            let OpcodeDesc(pattern, mask, handler) = *desc;

            let masked_opcode = opcode & mask;

            if masked_opcode == pattern {
                return handler;
            }
        }

        panic!("No match for opcode {:04X}", opcode);
    }
}

impl Default for OpcodeMatcher {
    fn default() -> Self {
        Self { registered_opcodes: Vec::new() }
    }
}

pub struct Cpu {
    bus: Bus,
    display: Rc<RefCell<Display>>,
    matcher: OpcodeMatcher,
    regfile: RegFile,
}

impl Cpu {
    pub const STEPS: usize = 256;

    pub fn new(bus: Bus, display: Rc<RefCell<Display>>) -> Self {
        // Populate matcher with descriptors
        const OPCODE_DESCS: [OpcodeDesc; 6] = [
            OpcodeDesc(0x00E0, 0xFFFF, Cpu::cls),
            OpcodeDesc(0x1000, 0xF000, Cpu::jp),
            OpcodeDesc(0x6000, 0xF000, Cpu::ldv),
            OpcodeDesc(0x7000, 0xF000, Cpu::add_imm),
            OpcodeDesc(0xA000, 0xF000, Cpu::ldi),
            OpcodeDesc(0xD000, 0xF000, Cpu::drw),
        ];

        let mut matcher = OpcodeMatcher::default();

        for desc in OPCODE_DESCS {
            matcher.register(desc);
        }

        Self {
            bus,
            display,
            matcher,
            regfile: RegFile::default(),
        }
    }

    /// Executes a single Chip-8 instruction
    pub fn step(&mut self) {
        let opcode = Opcode::new(self.bus.read_word(Address::new(self.regfile.pc)));

        self.regfile.advance_pc();

        self.matcher.match_opcode(opcode.raw())(self, opcode);
    }

    // --- Opcode handlers

    /// Adds immediate
    fn add_imm(&mut self, opcode: Opcode) {
        let x = opcode.x();

        self.regfile.gprs[x] = self.regfile.gprs[x].wrapping_add(opcode.kk());
    }

    /// Clears screen
    fn cls(&mut self, _opcode: Opcode) {
        self.display.borrow_mut().as_mut_slice().fill(0);
    }

    /// Draws sprite
    fn drw(&mut self, opcode: Opcode) {
        let mut display = self.display.borrow_mut();

        let (x, y) = (self.regfile.gprs[opcode.x()] as usize, self.regfile.gprs[opcode.y()] as usize);

        // TODO: set VF

        for n in 0..opcode.n() {
            // Get next row of pixels
            let pixels = self.bus.read_byte(Address::new(self.regfile.index.wrapping_add(n as u16))).reverse_bits();

            // Draw every individual pixel as either white or black
            for i in 0..8 {
                // 1 == white
                let pixel = (pixels.wrapping_shr(i) & 1) as u32;

                display[Display::WIDTH * (y + n) + x + (i as usize)] = Display::COLOR_WHITE * pixel;
            }
        }
    }

    /// Jumps to other location in program
    fn jp(&mut self, opcode: Opcode) {
        self.regfile.pc = opcode.nnn();
    }

    /// Loads index register
    fn ldi(&mut self, opcode: Opcode) {
        self.regfile.index = opcode.nnn();
    }
    
    /// Loads GPR with immediate
    fn ldv(&mut self, opcode: Opcode) {
        self.regfile.gprs[opcode.x()] = opcode.kk();
    }
}
