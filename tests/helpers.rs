extern crate befrust;
extern crate timebomb;

use self::timebomb::timeout_ms;
use self::befrust::program::Program;

pub fn test_io(source: &'static [u8], input: &'static [u8], expected: &'static [u8]) {
    timeout_ms(move || {

        let mut out = Vec::<u8>::new();

        {
            let mut p = Program::new(input, &mut out);
            p.load(source);
            p.run();
        }

        assert_eq!(expected, &out[..]);

    }, 1000);
}
