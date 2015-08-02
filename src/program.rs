//! Program space and execution.

use consts;
use pc::{Pc, Direction};

/// Program space, counter and stack.
pub struct Program {
    data: [[u8; consts::HEIGHT]; consts::WIDTH],
    pc: Pc,
    strmode: bool,
    stack: Vec<u8>,
}

impl Program {
    /// Creates a new Program, filled with spaces.
    pub fn new() -> Program {
        Program {
            data: [[32; consts::HEIGHT]; consts::WIDTH],
            pc: Pc::new(),
            strmode: false,
            stack: Vec::new(),
        }
    }
}
