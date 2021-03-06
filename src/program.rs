//! Program space and execution.

use std::io::{Read, BufRead, BufReader, Write};
use consts;
use pc::Pc;

/// Program space, counter and stack.
///
/// # Example
///
/// ```
/// use std::io;
/// use befrust::program::Program;
///
/// let mut p = Program::new(io::stdin(), io::stdout());
/// p.load(b"\"!dlroW ,olleH\">:#,_@");
/// p.run();
/// ```
pub struct Program<I: Read, O: Write> {
    data: [[u8; consts::WIDTH]; consts::HEIGHT],
    pc: Pc,
    strmode: bool,
    stack: Vec<i32>,
    input: BufReader<I>,
    output: O,
}

impl<I: Read, O: Write> Program<I, O> {
    /// Creates a blank `Program` attached to input and output.
    pub fn new(input: I, output: O) -> Program<I, O> {
        Program {
            data: [[b' '; consts::WIDTH]; consts::HEIGHT],
            pc: Pc::new(),
            strmode: false,
            stack: Vec::new(),
            input: BufReader::new(input),
            output: output,
        }
    }

    /// Load a program from ASCII bytes.
    pub fn load(&mut self, source: &[u8]) {
        let lines = source
            .split(|&b| b == b'\n')
            .take(consts::HEIGHT)
            .enumerate();

        for (y, line) in lines {
            for (x, &b) in line.iter().take(consts::WIDTH).enumerate() {
                self.data[y][x] = b;
            }
        }
    }

    /// Runs the program to completion. Programs may never complete.
    pub fn run(&mut self) {
        while self.step() {}
    }

    /// Executes the next command, returning true if execution should continue.
    pub fn step(&mut self) -> bool {
        match self.data[self.pc.y][self.pc.x] {

            b'"'              => self.toggle_strmode(),
            b if self.strmode => self.push(b as i32),

            b @ b'0' ... b'9' => self.push((b - b'0') as i32),

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

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap_or(0)
    }

    fn push(&mut self, v: i32) {
        self.stack.push(v);
    }

    fn toggle_strmode(&mut self) {
        self.strmode = !self.strmode;
    }

    fn add(&mut self) {
        let s = self.pop().wrapping_add(self.pop());
        self.push(s);
    }

    fn subtract(&mut self) {
        let (b, a) = (self.pop(), self.pop());
        self.push(a.wrapping_sub(b));
    }

    fn multiply(&mut self) {
        let p = self.pop().wrapping_mul(self.pop());
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
        let v = self.pop();
        write!(&mut self.output, "{} ", v).unwrap();
    }

    fn output_ascii(&mut self) {
        let c = self.pop() as u8 as char;
        write!(&mut self.output, "{}", c).unwrap();
    }

    fn get(&mut self) {
        let (y, x) = (self.pop() as usize, self.pop() as usize);

        if x >= consts::WIDTH || y >= consts::HEIGHT {
            self.push(0);
        } else {
            let v = self.data[y][x];
            self.push(v as i32);
        }
    }

    fn put(&mut self) {
        let (y, x, v) = (self.pop() as usize, self.pop() as usize, self.pop());

        if x < consts::WIDTH && y < consts::HEIGHT {
            self.data[y][x] = v as u8;
        }
    }

    fn input_value(&mut self) {
        let mut buf = String::new();
        self.input.read_line(&mut buf).unwrap();
        self.push(buf.trim().parse().unwrap_or(0));
    }

    fn input_ascii(&mut self) {
        let c = self.input
            .by_ref()
            .bytes()
            .next()
            .map(Result::unwrap)
            .map(|b| b as i32)
            .unwrap_or(-1);
        self.push(c);
    }
}
