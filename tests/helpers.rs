extern crate befrust;
extern crate timebomb;

use std::io;
use self::timebomb::timeout_ms;
use self::befrust::program::Program;

pub fn test_io(source: &'static [u8], input: &'static [u8], expected: &'static [u8]) {
    timeout_ms(move || {

        let mut inp = io::BufReader::new(input);
        let mut out = Vec::<u8>::new();

        {
            let mut p = Program::new(&mut inp, &mut out);
            p.load(source);
            p.run();
        }

        assert_eq!(expected, &out[..]);

    }, 1000);
}
