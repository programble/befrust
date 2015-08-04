mod helpers;
use helpers::test_io;

#[test]
fn end() {
    test_io(b"@", &[], &[]);
}

#[test]
fn output_value() {
    test_io(b".@", &[], b"0 ");
}

#[test]
fn left() {
    test_io(b"<@.", &[], b"0 ");
}

#[test]
fn down() {
    test_io(b"v\n.\n@", &[], b"0 ");
}

#[test]
fn up() {
    test_io(b"^\n@\n.", &[], b"0 ");
}

#[test]
fn right() {
    test_io(b"v\n>.@", &[], b"0 ");
}

#[test]
fn random() {
    test_io(b"?.@.<\n>   ^", &[], b"0 ");
}
