//! Program space and execution.

use std::io::{self, Read};
use consts;
use pc::Pc;

/// Program space, counter and stack.
///
/// # Example
///
/// ```
/// use befrust::program::Program;
///
/// let mut p = Program::with_bytes(b"\"!dlroW ,olleH\">:#,_@").unwrap();
/// p.run();
/// ```
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
            data: [[b' '; consts::HEIGHT]; consts::WIDTH],
            pc: Pc::new(),
            strmode: false,
            stack: Vec::new(),
        }
    }

    /// Creates a new Program from a slice of ASCII bytes.
    pub fn with_bytes(source: &[u8]) -> Result<Program, String> {
        let mut program = Program::new();

        for (y, line) in source.split(|&b| b == b'\n').enumerate() {
            if y == consts::HEIGHT {
                return Err("Too many rows".to_string());
            }
            if line.len() >= consts::WIDTH {
                return Err(format!("Line {} is too long", y + 1));
            }

            // FIXME: Better way of doing this? Would like to just copy one buffer to another,
            // which would require [y][x].
            for (x, &b) in line.iter().enumerate() {
                program.data[x][y] = b;
            }
        }

        Ok(program)
    }

    /// Runs the program to completion. Programs may never complete.
    pub fn run(&mut self) {
        loop {
            if !self.step() { break }
        }
    }

    /// Executes the next command, returning true if execution should continue.
    pub fn step(&mut self) -> bool {
        match self.data[self.pc.x][self.pc.y] {

            b'"'              => self.toggle_strmode(),
            b if self.strmode => self.push(b),

            b'+' => self.add(),
            b'-' => self.subtract(),
            b'*' => self.multiply(),
            b'/' => self.divide(),
            b'%' => self.modulo(),

            b'!' => self.not(),
            b'`' => self.greater(),

            b'>' => self.pc.right(),
            b'<' => self.pc.left(),
            b'^' => self.pc.up(),
            b'v' => self.pc.down(),
            b'?' => self.pc.random(),

            b'_' => self.horizontal_if(),
            b'|' => self.vertical_if(),

            b':'  => self.dup(),
            b'\\' => self.swap(),
            b'$'  => { self.stack.pop(); },

            b'.' => self.output_value(),
            b',' => self.output_ascii(),

            b'#' => self.pc.step(),

            b'g' => self.get(),
            b'p' => self.put(),

            b'&' => self.input_value(),
            b'~' => self.input_ascii(),

            b'@' => return false,

            _ => (),
        }

        self.pc.step();

        true
    }

    fn pop(&mut self) -> u8 {
        self.stack.pop().unwrap_or(0)
    }

    fn push(&mut self, b: u8) {
        self.stack.push(b);
    }

    fn toggle_strmode(&mut self) {
        self.strmode = !self.strmode;
    }

    fn add(&mut self) {
        let s = self.pop() + self.pop();
        self.push(s);
    }

    fn subtract(&mut self) {
        let (b, a) = (self.pop(), self.pop());
        self.push(a - b);
    }

    fn multiply(&mut self) {
        let p = self.pop() * self.pop();
        self.push(p);
    }

    fn divide(&mut self) {
        let (b, a) = (self.pop(), self.pop());
        self.push(a / b);
    }

    fn modulo(&mut self) {
        let (b, a) = (self.pop(), self.pop());
        self.push(a % b);
    }

    fn not(&mut self) {
        let t = self.pop();
        self.push(if t == 0 { 1 } else { 0 });
    }

    fn greater(&mut self) {
        let (b, a) = (self.pop(), self.pop());
        self.push(if a > b { 1 } else { 0 });
    }

    fn horizontal_if(&mut self) {
        if self.pop() == 0 {
            self.pc.right();
        } else {
            self.pc.left();
        }
    }

    fn vertical_if(&mut self) {
        if self.pop() == 0 {
            self.pc.down();
        } else {
            self.pc.up();
        }
    }

    fn dup(&mut self) {
        let t = self.stack.last().cloned().unwrap_or(0);
        self.push(t);
    }

    fn swap(&mut self) {
        let (a, b) = (self.pop(), self.pop());
        self.push(a);
        self.push(b);
    }

    fn output_value(&mut self) {
        print!("{} ", self.pop());
    }

    fn output_ascii(&mut self) {
        print!("{}", self.pop() as char);
    }

    fn get(&mut self) {
        let (y, x) = (self.pop(), self.pop());
        let v = self.data[x as usize][y as usize];
        self.push(v);
    }

    fn put(&mut self) {
        let (y, x, v) = (self.pop(), self.pop(), self.pop());
        self.data[x as usize][y as usize] = v;
    }

    fn input_value(&mut self) {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        self.push(buf.parse().unwrap_or(0));
    }

    fn input_ascii(&mut self) {
        let v = io::stdin().bytes().next().unwrap().unwrap();
        self.push(v);
    }
}
