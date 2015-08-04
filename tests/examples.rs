mod helpers;
use helpers::test_io;

#[test]
fn hello_world() {
    test_io(b"\"!dlroW ,olleH\">:#,_@", &[], b"Hello, World!");
}

#[test]
fn cat() {
    test_io(b"~:1+!#@_,", b"cat", b"cat");
}

#[test]
fn factorial() {
    // &>:1-:v v *_$.@
    //  ^    _$>\:^
    test_io(b"&>:1-:v v *_$.@\n ^    _$>\\:^", b"5\n", b"120 ");
}

#[test]
fn quine() {
    let quine = b"01->1# +# :# 0# g# ,# :# 5# 8# *# 4# +# -# _@";
    test_io(quine, &[], quine);
}
