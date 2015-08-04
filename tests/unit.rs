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

#[test]
fn push() {
    test_io(b"0.@", &[], b"0 ");
    test_io(b"1.@", &[], b"1 ");
    test_io(b"2.@", &[], b"2 ");
    test_io(b"3.@", &[], b"3 ");
    test_io(b"4.@", &[], b"4 ");
    test_io(b"5.@", &[], b"5 ");
    test_io(b"6.@", &[], b"6 ");
    test_io(b"7.@", &[], b"7 ");
    test_io(b"8.@", &[], b"8 ");
    test_io(b"9.@", &[], b"9 ");
}

#[test]
fn strmode() {
    test_io(b"\"@\".@", &[], b"64 ");
}
