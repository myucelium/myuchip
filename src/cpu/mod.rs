use crate::{
    bus::{Address, Bus},
    cpu::{opcode::Opcode, regfile::{RegFile, VF}},
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

        Cpu::dummy
    }
}

impl Default for OpcodeMatcher {
    fn default() -> Self {
        Self { registered_opcodes: Vec::new() }
    }
}

pub struct Stack {
    stack: Vec<u16>,
}

impl Stack {
    const MAX_DEPTH: usize = 16;

    pub fn pop(&mut self) -> u16 {
        assert!(self.stack.len() > 0);

        self.stack.pop().unwrap()
    }

    pub fn push(&mut self, data: u16) {
        assert!(self.stack.len() < Self::MAX_DEPTH);

        self.stack.push(data);
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self { stack: Vec::new() }
    }
}

pub struct Cpu {
    bus: Bus,
    display: Rc<RefCell<Display>>,
    matcher: OpcodeMatcher,
    regfile: RegFile,
    stack: Stack,
}

impl Cpu {
    pub const STEPS: usize = 256;

    pub fn new(bus: Bus, display: Rc<RefCell<Display>>) -> Self {
        // Populate matcher with descriptors
        const OPCODE_DESCS: [OpcodeDesc; 25] = [
            OpcodeDesc(0x00E0, 0xFFFF, Cpu::cls),
            OpcodeDesc(0x00EE, 0xFFFF, Cpu::ret),
            OpcodeDesc(0x1000, 0xF000, Cpu::jp),
            OpcodeDesc(0x2000, 0xF000, Cpu::call),
            OpcodeDesc(0x3000, 0xF000, Cpu::se_imm),
            OpcodeDesc(0x4000, 0xF000, Cpu::sne_imm),
            OpcodeDesc(0x5000, 0xF00F, Cpu::se_reg),
            OpcodeDesc(0x6000, 0xF000, Cpu::ldv_imm),
            OpcodeDesc(0x7000, 0xF000, Cpu::add_imm),
            OpcodeDesc(0x8000, 0xF00F, Cpu::ldv_reg),
            OpcodeDesc(0x8001, 0xF00F, Cpu::or),
            OpcodeDesc(0x8002, 0xF00F, Cpu::and),
            OpcodeDesc(0x8003, 0xF00F, Cpu::xor),
            OpcodeDesc(0x8004, 0xF00F, Cpu::add_reg),
            OpcodeDesc(0x8005, 0xF00F, Cpu::sub),
            OpcodeDesc(0x8006, 0xF00F, Cpu::shr),
            OpcodeDesc(0x8007, 0xF00F, Cpu::subn),
            OpcodeDesc(0x800E, 0xF00F, Cpu::shl),
            OpcodeDesc(0x9000, 0xF00F, Cpu::sne_reg),
            OpcodeDesc(0xA000, 0xF000, Cpu::ldi_imm),
            OpcodeDesc(0xD000, 0xF000, Cpu::drw),
            OpcodeDesc(0xF01E, 0xF0FF, Cpu::addi),
            OpcodeDesc(0xF033, 0xF0FF, Cpu::ldb),
            OpcodeDesc(0xF055, 0xF0FF, Cpu::ldi_mem),
            OpcodeDesc(0xF065, 0xF0FF, Cpu::ldv_mem),
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
            stack: Stack::default(),
        }
    }

    /// Executes a single Chip-8 instruction
    pub fn step(&mut self) {
        let pc = *self.pc();

        let opcode = Opcode::new(self.bus.read_word(Address::new(pc)));

        self.regfile.advance_pc();

        self.matcher.match_opcode(opcode.raw())(self, opcode);
    }

    /// Returns a mutable reference to PC
    fn pc(&mut self) -> &mut u16 {
        &mut self.regfile.pc
    }

    /// Returns a mutable reference to the index register
    fn i(&mut self) -> &mut u16 {
        &mut self.regfile.index
    }

    /// Returns a mutable reference to a GPR
    fn v(&mut self, idx: usize) -> &mut u8 {
        &mut self.regfile.gprs[idx]
    }

    // --- Opcode handlers

    fn dummy(&mut self, opcode: Opcode) {
        panic!("No handler for opcode {:04X}", opcode.raw());
    }

    /// Vx += kk
    fn add_imm(&mut self, opcode: Opcode) {
        let x = opcode.x();

        *self.v(x) = self.v(x).wrapping_add(opcode.kk());
    }
    
    /// Vx += Vy, VF = carry
    fn add_reg(&mut self, opcode: Opcode) {
        let x = opcode.x();

        let (result, has_overflowed) = self.v(x).overflowing_add(*self.v(opcode.y()));

        (*self.v(x), *self.v(VF)) = (result, has_overflowed as u8);
    }

    /// I += Vx
    fn addi(&mut self, opcode: Opcode) {
        *self.i() = self.i().wrapping_add(*self.v(opcode.x()) as u16);
    }
    
    /// Vx = Vx AND Vy
    fn and(&mut self, opcode: Opcode) {
        *self.v(opcode.x()) &= *self.v(opcode.y());
    }

    /// Call subroutine
    fn call(&mut self, opcode: Opcode) {
        let return_addr = *self.pc();

        self.stack.push(return_addr);

        *self.pc() = opcode.nnn();
    }

    /// Clear screen
    fn cls(&mut self, _opcode: Opcode) {
        self.display.borrow_mut().as_mut_slice().fill(0);
    }

    /// Draw sprite
    fn drw(&mut self, opcode: Opcode) {
        let (index, x, y) = (*self.i() as u16, *self.v(opcode.x()) as usize, *self.v(opcode.y()) as usize);

        let mut has_collided = false;

        {
            let mut display = self.display.borrow_mut();

            for n in 0..opcode.n() {
                // Get next row of pixels
                let pixels = self.bus.read_byte(Address::new(index.wrapping_add(n as u16))).reverse_bits();
    
                // Draw every individual pixel as either white or black
                for i in 0..8 {
                    let display_idx = Display::WIDTH * ((y + n) % Display::HEIGHT) + ((x + i as usize) % Display::WIDTH);

                    // 1 == white
                    let (pixel, old_pixel) = (
                        Display::COLOR_WHITE * (pixels.wrapping_shr(i) & 1) as u32,
                        display[display_idx],
                    );
    
                    has_collided |= (pixel & old_pixel) == Display::COLOR_WHITE;
    
                    display[display_idx] ^= pixel;
                }
            }
        }

        *self.v(VF) = has_collided as u8;
    }

    /// Jump
    fn jp(&mut self, opcode: Opcode) {
        *self.pc() = opcode.nnn();
    }

    /// [I] = BCD(Vx)
    fn ldb(&mut self, opcode: Opcode) {
        let (index, vx) = (*self.i(), *self.v(opcode.x()));

        let digits = [vx / 100, (vx / 10) % 10, vx % 10];

        for i in 0..3 {
            self.bus.write_byte(Address::new(index.wrapping_add(i as u16)), digits[i]);
        }
    }

    /// I = nnn
    fn ldi_imm(&mut self, opcode: Opcode) {
        *self.i() = opcode.nnn();
    }

    /// [I] = V0-Vx
    fn ldi_mem(&mut self, opcode: Opcode) {
        let index = *self.i();

        for i in 0..=opcode.x() {
            let vx = *self.v(i);
    
            self.bus.write_byte(Address::new(index.wrapping_add(i as u16)), vx);
        }
    }
    
    /// Vx = kk
    fn ldv_imm(&mut self, opcode: Opcode) {
        *self.v(opcode.x()) = opcode.kk();
    }

    /// V0-Vx = [I]
    fn ldv_mem(&mut self, opcode: Opcode) {
        let index = *self.i();
    
        for i in 0..=opcode.x() {
            *self.v(i) = self.bus.read_byte(Address::new(index.wrapping_add(i as u16)));
        }
    }
    
    /// Vx = Vy
    fn ldv_reg(&mut self, opcode: Opcode) {
       *self.v(opcode.x()) = *self.v(opcode.y());
    }
    
    /// Vx = Vx OR Vy
    fn or(&mut self, opcode: Opcode) {
        *self.v(opcode.x()) |= *self.v(opcode.y());
    }

    /// Return
    fn ret(&mut self, _opcode: Opcode) {
        *self.pc() = self.stack.pop();
    }

    /// Skip if Vx == kk
    fn se_imm(&mut self, opcode: Opcode) {
        if *self.v(opcode.x()) == opcode.kk() {
            self.regfile.advance_pc();
        }
    }

    /// Skip if Vx == Vy
    fn se_reg(&mut self, opcode: Opcode) {
        if *self.v(opcode.x()) == *self.v(opcode.y()) {
            self.regfile.advance_pc();
        }
    }
    
    /// Vx <<= 1, VF = carry
    fn shl(&mut self, opcode: Opcode) {
        let (x, vx) = (opcode.x(), *self.v(opcode.x()));

        let (result, has_overflowed) = (self.v(x).unbounded_shl(1), vx.reverse_bits() & 1 != 0);

        (*self.v(x), *self.v(VF)) = (result, has_overflowed as u8);
    }
    
    /// Vx >>= 1, VF = carry
    fn shr(&mut self, opcode: Opcode) {
        let (x, vx) = (opcode.x(), *self.v(opcode.x()));

        let (result, has_overflowed) = (self.v(x).unbounded_shr(1), vx & 1 != 0);

        (*self.v(x), *self.v(VF)) = (result, has_overflowed as u8);
    }

    /// Skip if Vx != kk
    fn sne_imm(&mut self, opcode: Opcode) {
        if *self.v(opcode.x()) != opcode.kk() {
            self.regfile.advance_pc();
        }
    }

    /// Skip if Vx != Vy
    fn sne_reg(&mut self, opcode: Opcode) {
        if *self.v(opcode.x()) != *self.v(opcode.y()) {
            self.regfile.advance_pc();
        }
    }
    
    /// Vx -= Vy, VF = !borrow
    fn sub(&mut self, opcode: Opcode) {
        let x = opcode.x();

        let (result, has_overflowed) = self.v(x).overflowing_sub(*self.v(opcode.y()));

        (*self.v(x), *self.v(VF)) = (result, !has_overflowed as u8);
    }
    
    /// Vx = Vy - Vx, VF = !borrow
    fn subn(&mut self, opcode: Opcode) {
        let x = opcode.x();

        let (result, has_overflowed) = self.v(opcode.y()).overflowing_sub(*self.v(x));

        (*self.v(x), *self.v(VF)) = (result, !has_overflowed as u8);
    }
    
    /// Vx = Vx XOR Vy
    fn xor(&mut self, opcode: Opcode) {
        *self.v(opcode.x()) ^= *self.v(opcode.y());
    }
}
