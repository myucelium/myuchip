use std::ops::{Index, IndexMut};

pub struct Display([u32; Self::WIDTH * Self::HEIGHT]);

impl Display {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;

    pub const COLOR_WHITE: u32 = 0xFFFFFFFF;

    pub fn as_mut_slice(&mut self) -> &mut [u32] {
        &mut self.0
    }

    pub fn as_slice(&self) -> &[u32] {
        &self.0
    }
}

impl Default for Display {
    fn default() -> Self {
        Self([0; Self::WIDTH * Self::HEIGHT])
    }
}

impl Index<usize> for Display {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Display {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
