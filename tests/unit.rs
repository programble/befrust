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

#[test]
fn add() {
    test_io(b"23+.@", &[], b"5 ");
}

#[test]
fn subtract() {
    test_io(b"53-.@", &[], b"2 ");
}

#[test]
fn multiply() {
    test_io(b"23*.@", &[], b"6 ");
}

#[test]
fn divide() {
    test_io(b"63/.@", &[], b"2 ");
}

#[test]
fn modulo() {
    test_io(b"54%.@", &[], b"1 ");
}

#[test]
fn add_wrap() {
    test_io(b"255**1+5*1+.@", &[], b"0 ");
}

#[test]
fn subtract_wrap() {
    test_io(b"01-.@", &[], b"255 ");
}

#[test]
fn multiply_wrap() {
    test_io(b"4444***.@", &[], b"0 ");
}

#[test]
fn not() {
    test_io(b"5!.@", &[], b"0 ");
    test_io(b"0!.@", &[], b"1 ");
}

#[test]
fn greater() {
    test_io(b"43`.@", &[], b"1 ");
    test_io(b"34`.@", &[], b"0 ");
    test_io(b"33`.@", &[], b"0 ");
}

#[test]
fn horizontal_if() {
    test_io(b"0_1.@", &[], b"1 ");
    test_io(b"5_@.1", &[], b"1 ");
}

#[test]
fn vertical_if() {
    test_io(b"0|\n 1\n .\n @", &[], b"1 ");
    test_io(b"5|\n @\n .\n 1", &[], b"1 ");
}

#[test]
fn dup() {
    test_io(b"5:..@", &[], b"5 5 ");
}

#[test]
fn swap() {
    test_io(b"34\\..@", &[], b"3 4 ");
}

#[test]
fn pop() {
    test_io(b"34$.@", &[], b"3 ");
}
