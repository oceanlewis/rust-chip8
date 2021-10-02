#![allow(dead_code)]

trait InstructionSet {
    fn add(&mut self, x: u8, y: u8);
}

struct CPU {
    pub current_operation: u16,
    pub registers: [u8; 2],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            current_operation: 0,
            registers: [0; 2],
        }
    }

    fn step(&mut self) {
        let opcode = self.read_opcode();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        match (c, x, y, d) {
            (8, _, _, 4) => self.add(x, y),
            _ => todo!(),
        }
    }

    fn read_opcode(&self) -> u16 {
        self.current_operation
    }
}

impl InstructionSet for CPU {
    fn add(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
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
        fn add_second_register_to_first() {
            let mut cpu = CPU::new();

            cpu.current_operation = 0x8014;
            cpu.registers[0] = 5;
            cpu.registers[1] = 10;
            cpu.step();

            assert_eq!(cpu.registers[0], 15);
            assert_eq!(cpu.registers[1], 10);
        }

        #[test]
        fn add_first_register_to_second() {
            let mut cpu = CPU::new();

            cpu.current_operation = 0x8104;
            cpu.registers[0] = 5;
            cpu.registers[1] = 10;
            cpu.step();

            assert_eq!(cpu.registers[0], 5);
            assert_eq!(cpu.registers[1], 15);
        }
    }
}
