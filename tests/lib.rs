extern crate befrust;

use std::io;
use befrust::program::Program;

#[test]
fn hello_world() {
    let mut out = Vec::<u8>::new();

    {
        let mut p = Program::new(io::empty(), &mut out);
        p.load(b"\"!dlroW ,olleH\">:#,_@");
        p.run();
    }

    let out = String::from_utf8(out).unwrap();
    assert_eq!(out, "Hello, World!");
}
