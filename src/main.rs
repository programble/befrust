#![cfg_attr(test, allow(dead_code, unused_imports))]

extern crate befrust;

use std::{env, io, fs};
use std::io::Read;
use befrust::program::Program;

fn main() {
    let path = env::args().skip(1).next().unwrap();
    let mut file = fs::File::open(path).unwrap();
    let mut source = Vec::<u8>::new();
    file.read_to_end(&mut source).unwrap();

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut program = Program::new(stdin.lock(), stdout.lock());
    program.load(&source);
    program.run();
}
