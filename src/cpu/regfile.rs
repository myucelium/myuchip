use std::{mem::size_of, ops::{Index, IndexMut}};

pub const NUM_GPRS: usize = 16;

pub const VF: usize = 15;

/// Chip-8 general-purpose registers V0-VF
pub struct Gprs([u8; NUM_GPRS]);

impl Default for Gprs {
    fn default() -> Self {
        Self([0; NUM_GPRS])
    }
}

impl Index<usize> for Gprs {
    type Output = u8;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Gprs {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/// 8-bit downcounter
pub struct Timer(u8);

impl Timer {
    pub fn counter(&mut self) -> &mut u8 {
        &mut self.0
    }

    /// Decrements counter only if current counter is not 0
    pub fn decrement(&mut self) {
        if self.0 > 0 {
            self.0 = self.0.wrapping_sub(1);
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self(u8::MAX)
    }
}

/// Chip-8 register file
pub struct RegFile {
    /// 12-bit program counter
    pub pc: u16,

    /// 16 8-bit general purpose registers, V0-VF
    pub gprs: Gprs,

    /// 16-bit index register
    pub index: u16,

    /// Delay and sound timers
    pub delay_timer: Timer,
    pub sound_timer: Timer,
}

impl RegFile {
    pub fn advance_pc(&mut self) {
        self.pc = self.pc.wrapping_add(size_of::<u16>() as u16);
    }

    pub fn rewind_pc(&mut self) {
        self.pc = self.pc.wrapping_sub(size_of::<u16>() as u16);
    }
}

impl Default for RegFile {
    fn default() -> Self {
        Self {
            pc: crate::Core::ROM_START as u16,
            gprs: Gprs::default(),
            index: 0,
            delay_timer: Timer::default(),
            sound_timer: Timer::default(),
        }
    }
}
