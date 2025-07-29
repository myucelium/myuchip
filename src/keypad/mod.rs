use minifb::Key;

struct Keymap;

impl Keymap {
    pub fn cpu_index(key_index: usize) -> u8 {
        const CPU_INDICES: [u8; Keypad::NUM] = [
            0x1, 0x2, 0x3, 0xC,
            0x4, 0x5, 0x6, 0xD,
            0x7, 0x8, 0x9, 0xE,
            0xA, 0x0, 0xB, 0xF,
        ];

        CPU_INDICES[key_index]
    }

    pub fn key_index(cpu_index: usize) -> usize {
        const KEY_INDICES: [usize; Keypad::NUM] = [
            0xD, 0x0, 0x1, 0x2,
            0x4, 0x5, 0x6, 0x8,
            0x9, 0xA, 0xC, 0xE,
            0x3, 0x7, 0xB, 0xF,
        ];

        KEY_INDICES[cpu_index]
    }

    pub fn from_key(key: &Key) -> Option<usize> {
        let key_index: usize = match key {
            Key::Key1 => 0,
            Key::Key2 => 1,
            Key::Key3 => 2,
            Key::Key4 => 3,
            Key::Q => 4,
            Key::W => 5,
            Key::E => 6,
            Key::R => 7,
            Key::A => 8,
            Key::S => 9,
            Key::D => 10,
            Key::F => 11,
            Key::Z => 12,
            Key::X => 13,
            Key::C => 14,
            Key::V => 15,
            _ => 16,
        };

        if key_index == 16 {
            None
        } else {
            Some(key_index)
        }
    }
}

type KeyState = [bool; Keypad::NUM];

pub struct Keypad {
    state: KeyState,
}

impl Keypad {
    const NUM: usize = 16;

    pub fn is_key_pressed(&self, cpu_index: usize) -> bool {
        self.state[Keymap::key_index(cpu_index)]
    }

    /// Returns Some if any key is pressed (key with the lowest index is returned)
    pub fn any_key(&self) -> Option<u8> {
        for key_index in 0..Self::NUM {
            if self.state[key_index] {
                return Some(Keymap::cpu_index(key_index));
            }
        }
    
        None
    }

    /// Fills key state from window key state
    pub fn update_state(&mut self, state: Vec<Key>) {
        self.state.fill(false);

        state.iter().for_each(|key| {
            if let Some(key_index) = Keymap::from_key(key) {
                self.state[key_index] = true;
            }
        });
    }
}

impl Default for Keypad {
    fn default() -> Self {
        Self { state: [false; Self::NUM] }
    }
}
