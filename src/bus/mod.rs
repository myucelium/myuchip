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

    /// Unchecked mask for byte accesses
    pub fn mask_address(&self) -> u16 {
        self.0 & Self::MAX
    }

    /// Returns masked address if 16-bit access won't go out of bounds,
    /// None otherwise
    pub fn checked_mask_address(&self) -> Option<u16> {
        let masked_addr = self.0 & Self::MAX;

        if masked_addr == Self::MAX {
            None
        } else {
            Some(masked_addr)
        }
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
        let masked_addr = addr.mask_address() as usize;

        self.mem[masked_addr]
    }

    pub fn read_word(&self, addr: Address) -> u16 {
        let masked_addr = addr.checked_mask_address().expect("Word address out of bounds") as usize;

        u16::from_be_bytes([self.mem[masked_addr], self.mem[masked_addr + 1]])
    }

    pub fn write_byte(&mut self, addr: Address, data: u8) {
        let masked_addr = addr.mask_address() as usize;

        self.mem[masked_addr] = data;
    }
}
