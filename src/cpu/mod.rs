use crate::bus::{Address, Bus};

type OpcodePattern = u16;
type OpcodeMask = u16;
type OpcodeHandler = fn(&mut Cpu, u16);

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
        Self { registered_opcodes: Vec::new(), }
    }
}

pub struct Cpu {
    bus: Bus,
    matcher: OpcodeMatcher,
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        Self {
            bus,
            matcher: OpcodeMatcher::default(),
        }
    }

    /// Executes a single Chip-8 instruction
    pub fn step(&mut self) {
        let opcode = self.bus.read_word(Address::new(0x200));

        self.matcher.match_opcode(opcode)(self, opcode);
    }
}
