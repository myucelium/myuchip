#[derive(Clone, Copy)]
pub struct Opcode(u16);

impl Opcode {
    pub fn new(opcode: u16) -> Self {
        Self(opcode)
    }

    pub fn raw(&self) -> u16 {
        self.0
    }

    pub fn kk(&self) -> u8 {
        (self.raw() & 0xFF) as u8
    }

    pub fn n(&self) -> usize {
        (self.raw() & 0xF) as usize
    }

    pub fn nnn(&self) -> u16 {
        self.raw() & 0xFFF
    }

    pub fn x(&self) -> usize {
        (self.raw().wrapping_shr(8) & 0xF) as usize
    }

    pub fn y(&self) -> usize {
        (self.raw().wrapping_shr(4) & 0xF) as usize
    }
}
