#![allow(dead_code)]

mod register_position {
    /// The register position of the overflow flag
    pub const OVERFLOW: usize = 0x0F;
}

/// Defines the instruction set of the CHIP-8
trait CHIP8 {
    /// Adds the value contained in register `x` to the value contained in
    /// register `y`, storing the result in register `x`.
    fn add(&mut self, x: u8, y: u8);
}

/// Describes the State of the CPU
/// Setting the state to `Paused` will cause the run loop to exit
#[derive(Debug, PartialEq)]
enum CpuState {
    Running,
    Paused,
}

#[derive(Debug)]
struct CPU {
    pub registers: [u8; 0x10],
    pub program_counter: u16,
    pub memory: [u8; 0x1000],
    state: CpuState,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 0x10],
            program_counter: 0,
            memory: [0; 0x1000],
            state: CpuState::Paused,
        }
    }

    pub fn run(&mut self) {
        self.state = CpuState::Running;

        while self.state != CpuState::Paused {
            self.step();
        }
    }

    fn step(&mut self) {
        let opcode = self.read_opcode();
        self.advance_program_counter();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        match (c, x, y, d) {
            (0, 0, 0, 0) => self.state = CpuState::Paused,
            (8, _, _, 4) => self.add(x, y),
            _ => todo!(),
        }
    }

    fn advance_program_counter(&mut self) {
        self.program_counter += 2;
    }

    fn read_opcode(&self) -> u16 {
        let pc = self.program_counter as usize;
        let op_b1 = self.memory[pc] as u16;
        let op_b2 = self.memory[pc + 1] as u16;
        op_b1 << 8 | op_b2
    }
}

impl CHIP8 for CPU {
    fn add(&mut self, x: u8, y: u8) {
        use register_position::OVERFLOW;

        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (result, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = result;

        if overflow {
            self.registers[OVERFLOW] = 1;
        } else {
            self.registers[OVERFLOW] = 0;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn init() {
        let cpu = CPU::new();
        assert_eq!(cpu.program_counter, 0);
        assert_eq!(cpu.registers[0], 0);
        assert_eq!(cpu.registers[1], 0);
        for byte in cpu.memory {
            assert_eq!(byte, 0);
        }
    }

    mod adder {
        use crate::*;

        #[test]
        fn add_second_register_to_first() {
            let mut cpu = CPU::new();

            cpu.memory[0] = 0x80;
            cpu.memory[1] = 0x14;
            cpu.registers[0] = 5;
            cpu.registers[1] = 10;
            cpu.step();

            assert_eq!(cpu.registers[0], 15);
            assert_eq!(cpu.registers[1], 10);
        }

        #[test]
        fn add_first_register_to_second() {
            let mut cpu = CPU::new();

            cpu.memory[0] = 0x81;
            cpu.memory[1] = 0x04;
            cpu.registers[0] = 5;
            cpu.registers[1] = 10;
            cpu.step();

            assert_eq!(cpu.registers[0], 5);
            assert_eq!(cpu.registers[1], 15);
        }

        #[test]
        fn overflows_set_overflow_flag() {
            let mut cpu = CPU::new();

            cpu.memory[0] = 0x80;
            cpu.memory[1] = 0x14;
            cpu.registers[0] = u8::MAX;
            cpu.registers[1] = 1;
            cpu.step();

            assert_eq!(cpu.registers[0], 0);
            assert_eq!(cpu.registers[1], 1);
            assert_eq!(cpu.registers[0xF], 1);
        }
    }

    mod run_loop {
        use crate::*;

        #[test]
        fn pauses_on_0000() {
            let mut cpu = CPU::new();
            cpu.run();
            assert_eq!(cpu.program_counter, 2);
        }
    }
}
