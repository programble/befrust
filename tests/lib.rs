extern crate befrust;
extern crate timebomb;

use std::io;
use timebomb::timeout_ms;
use befrust::program::Program;

fn test_io(source: &'static [u8], input: &'static [u8], expected: &'static [u8]) {
    timeout_ms(move || {

        let mut inp = io::BufReader::new(input);
        let mut out = Vec::<u8>::new();

        {
            let mut p = Program::new(&mut inp, &mut out);
            p.load(source);
            p.run();
        }

        assert_eq!(out, expected);

    }, 1000);
}

#[test]
fn hello_world() {
    test_io(b"\"!dlroW ,olleH\">:#,_@", &[], b"Hello, World!");
}