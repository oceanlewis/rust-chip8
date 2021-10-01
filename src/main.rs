#![allow(dead_code)]

type OpCode = u16;

struct CPU {
    pub current_operation: OpCode,
    pub registers: [u8; 2],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            current_operation: 0,
            registers: [0; 2],
        }
    }

    pub fn run(&mut self) {
        let opcode = self.read_opcode();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        match (c, x, y, d) {
            _ => todo!(),
        }
    }

    fn read_opcode(&self) -> OpCode {
        self.current_operation
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn init() {
        let cpu = CPU::new();
        assert_eq!(cpu.current_operation, 0);
        assert_eq!(cpu.registers[0], 0);
        assert_eq!(cpu.registers[1], 0);
    }

    mod adder {
        use crate::*;

        #[test]
        fn simple_add() {
            let mut cpu = CPU::new();
            cpu.current_operation = 0x8014;
            cpu.registers[0] = 5;
            cpu.registers[1] = 10;
        }
    }
}
