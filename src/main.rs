#![allow(dead_code)]

mod register_position {
    /// The register position of the overflow flag
    pub const OVERFLOW: usize = 15;
}

/// Defines the instruction set of the CHIP-8
trait CHIP8 {
    const STACK_SIZE: usize = 0x10;
    const MEMORY_SIZE: usize = 0x1000;
    const SYSTEM_MEMORY_START: usize = 0x100;
    const SYSTEM_MEMORY_END: usize = 0x100;

    /// Op Code: `0x8xyF`
    ///
    /// Adds the value contained in register `x` to the value contained in
    /// register `y`, storing the result in register `x`.
    fn add(&mut self, x: u8, y: u8);

    /// Op Code: `0x2nnn`
    ///
    /// Sets the program counter to `nnn`.
    fn call(&mut self, nnn: u16);

    /// Op Code: `0x00EE`
    ///
    /// Sets the program counter to the memory address of the previous CALL
    /// opcode;
    fn ret(&mut self);
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

    pub stack_pointer: u16,
    pub stack: [u16; 0x10],

    pub program_counter: u16,
    pub memory: [u8; 0x1000],

    state: CpuState,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 0x10],

            stack_pointer: 0,
            stack: [0; 0x10],

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

    pub fn step(&mut self) {
        let opcode = self.read_opcode();
        self.increment_program_counter();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        match (c, x, y, d) {
            (0, 0, 0, 0) => self.state = CpuState::Paused,
            (8, _, _, 4) => self.add(x, y),
            (2, _, _, _) => self.call({
                let n1 = (x as u16) << 8;
                let n2 = (y as u16) << 4;
                let n3 = (d as u16) << 0;
                n1 | n2 | n3
            }),
            (0, 0, 0xE, 0xE) => self.ret(),
            _ => unimplemented!("Unimplemented opcode! '{}'", opcode),
        }
    }

    pub fn load_program(&mut self, location_in_memory: usize, program: &[u8]) {
        assert!(location_in_memory > Self::SYSTEM_MEMORY_END);
        self.memory[location_in_memory..(location_in_memory + program.len())]
            .copy_from_slice(&program);
    }

    pub fn set_program_counter(&mut self, location_in_memory: usize) {
        use std::convert::TryFrom;

        self.program_counter = u16::try_from(location_in_memory).expect(
            &format!("location_in_memory too large: {}", location_in_memory),
        );
    }

    fn increment_program_counter(&mut self) {
        self.program_counter += 2;
        assert!((self.program_counter as usize) < Self::MEMORY_SIZE);
    }

    fn increment_stack_counter(&mut self) {
        self.stack_pointer += 1;
        assert!(
            (self.stack_pointer as usize) < Self::STACK_SIZE,
            "Stack overflow, SP:{:x}",
            self.stack_pointer
        );
    }

    fn decrement_stack_counter(&mut self) {
        assert!(
            self.stack_pointer != 0,
            "Stack underflow, SP:{:x}",
            self.stack_pointer
        );
        self.stack_pointer -= 1;
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

    fn call(&mut self, nnn: u16) {
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.increment_stack_counter();
        self.program_counter = nnn;
    }

    fn ret(&mut self) {
        self.decrement_stack_counter();
        self.program_counter = self.stack[self.stack_pointer as usize];
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn init() {
        let cpu = CPU::new();

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

    mod functions_as_data {
        use crate::*;

        #[test]
        fn load_add_twice_program() {
            let mut cpu = CPU::new();
            let add_twice = [
                0x80, 0x14, // add(0, 1)
                0x80, 0x14, // add(0, 1)
                0x00, 0xEE, // ret()
            ];

            cpu.load_program(0x101, &add_twice);

            assert_eq!(
                cpu.memory[0xFF..0x109],
                [
                    0x00, 0x00, // noop
                    0x80, 0x14, // add(0, 1)
                    0x80, 0x14, // add(0, 1)
                    0x00, 0xEE, // ret()
                    0x00, 0x00, // noop
                ]
            );
        }
    }

    mod call {
        use crate::*;

        #[test]
        #[should_panic]
        fn stack_underflow() {
            let mut cpu = CPU::new();
            let simply_return = [
                0x00, 0xEE, // ret()
            ];

            cpu.load_program(0x101, &simply_return);
            cpu.set_program_counter(0x101);
            cpu.run();
        }

        #[test]
        #[should_panic]
        fn stack_overflow() {
            let mut cpu = CPU::new();
            let infinite_loop = [
                0x21, 0x01, // call(0x101)
            ];

            cpu.load_program(0x101, &infinite_loop);
            cpu.set_program_counter(0x101);

            cpu.run();
        }
    }
}
