pub mod register_position {
    /// The register position of the overflow flag
    pub const OVERFLOW: usize = 15;
}

/// Defines the instruction set of the CHIP-8
pub trait CHIP8 {
    const STACK_SIZE: usize = 0x10;
    const MEMORY_SIZE: usize = 0x1000;
    const SYSTEM_MEMORY_START: usize = 0x0;
    const SYSTEM_MEMORY_END: usize = 0x1FF;
    const PROGRAM_MEMORY_START: usize = 0x200;

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
