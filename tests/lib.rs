extern crate befrust;

use std::io;
use befrust::program::Program;

fn test_output(source: &[u8], expected: &[u8]) {
    let mut out = Vec::<u8>::new();

    {
        let mut p = Program::new(io::empty(), &mut out);
        p.load(source);
        p.run();
    }

    assert_eq!(out, expected);
}

#[test]
fn hello_world() {
    test_output(b"\"!dlroW ,olleH\">:#,_@", b"Hello, World!");
}
