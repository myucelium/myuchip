use std::ops::{Index, IndexMut, Range};

/// Chip-8 RAM
pub struct Memory([u8; Self::SIZE]);

impl Memory {
    pub const SIZE: usize = 0x1000;
}

impl Default for Memory {
    fn default() -> Self {
        Self([0; Self::SIZE])
    }
}

impl Index<Range<usize>> for Memory {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index.start..index.end]
    }
}

impl Index<usize> for Memory {
    type Output = u8;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<Range<usize>> for Memory {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.0[index.start..index.end]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
