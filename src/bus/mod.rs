use crate::bus::memory::Memory;

pub mod memory;

/// Chip-8 address (12-bit)
#[derive(Clone, Copy)]
pub struct Address(u16);

impl Address {
    const MAX: u16 = (Memory::SIZE - 1) as u16;

    pub fn new(addr: u16) -> Self {
        Self(addr)
    }

    /// Masked address for byte accesses
    pub fn masked_address(&self) -> usize {
        (self.0 & Self::MAX) as usize
    }

    /// Masked next address for word accesses
    pub fn masked_next_address(&self) -> usize {
        (self.0.wrapping_add(1) & Self::MAX) as usize
    }
}

pub struct Bus {
    mem: Memory,
}

impl Bus {
    pub fn new(mem: Memory) -> Self {
        Self { mem }
    }

    pub fn read_byte(&self, addr: Address) -> u8 {
        self.mem[addr.masked_address()]
    }

    pub fn read_word(&self, addr: Address) -> u16 {
        u16::from_be_bytes([self.mem[addr.masked_address()], self.mem[addr.masked_next_address()]])
    }

    pub fn write_byte(&mut self, addr: Address, data: u8) {
        self.mem[addr.masked_address()] = data;
    }
}
