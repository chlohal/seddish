use crate::tests::test_from_busybox_sed;

/// Taken from https://github.com/mirror/busybox/blob/master/testsuite/sed.tests

///Automatically transformed from Busybox sed test sed handles empty lines
#[test]
fn busybox_test_3() {
    let expected = "@\n";
    let input = "";
    let source = "s/$/@/";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed accepts blanks before command
#[test]
fn busybox_test_5() {
    let expected = "";
    let input = "hi";
    let source = "1 d";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed accepts newlines in -e
#[test]
fn busybox_test_6() {
    let expected = "1\n2\n3\n";
    let input = "2";
    let source = "i1\na3";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed with empty match
#[test]
fn busybox_test_7() {
    let expected = "string\n";
    let input = "string";
    let source = "s/z*//g";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed s//g (exhaustive)
#[test]
fn busybox_test_9() {
    let expected = ",1,2,3,4,5,\n";
    let input = "12345";
    let source = "s/[[:space:]]*/,/g";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed s arbitrary delimiter
#[test]
fn busybox_test_10() {
    let expected = "boing\n";
    let input = "woo";
    let source = "s.woo.boing.";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed s chains
#[test]
fn busybox_test_11() {
    let expected = "baz\n";
    let input = "foo";
    let source = "s/foo/bar/; s/bar/baz/";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed s chains2
#[test]
fn busybox_test_12() {
    let expected = "bar\n";
    let input = "foo";
    let source = "s/foo/bar/; s/baz/nee/";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed s with 	 (GNU ext)
#[test]
fn busybox_test_14() {
    let expected = "one two";
    let input = "one\ttwo";
    let source = "s/\t/ /";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed b (branch)
#[test]
fn busybox_test_15() {
    let expected = "foo\n";
    let input = "foo";
    let source = "b one;p;: one";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed b (branch with no label jumps to end)
#[test]
fn busybox_test_16() {
    let expected = "foo\n";
    let input = "foo";
    let source = "b;p";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed t (test/branch)
#[test]
fn busybox_test_17() {
    let expected = "1\n1\nb\nb\nb\nc\nc\nc\n";
    let input = ["a", "b", "c"];
    let source = "s/a/1/;t one;p;: one;p";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed t (test/branch clears test bit)
#[test]
fn busybox_test_18() {
    let expected = "b\nb\nc\n";
    let input = ["a", "b", "c"];
    let source = "s/a/b/;:loop;t loop";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed T (!test/branch)
#[test]
fn busybox_test_19() {
    let expected = "1\n1\n1\nb\nb\nc\nc\n";
    let input = ["a", "b", "c"];
    let source = "s/a/1/;T notone;p;: notone;p";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed n (flushes pattern space, terminates early)
#[test]
fn busybox_test_20() {
    let expected = "a\nb\nb\nc\n";
    let input = ["a", "b", "c"];
    let source = "n;p";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed N (flushes pattern space (GNU behavior))
#[test]
fn busybox_test_21() {
    let expected = "a\nb\na\nb\nc\n";
    let input = ["a", "b", "c"];
    let source = "N;p; $d";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed N test2
#[test]
fn busybox_test_22() {
    let expected = "a b c\n";
    let input = ["a", "b", "c"];
    let source = ":a;N;s/\n/ /;ta";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed N test3
#[test]
fn busybox_test_23() {
    let expected = "a b\nc\n";
    let input = ["a", "b", "c"];
    let source = "N;s/\n/ /";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed address match newline
#[test]
fn busybox_test_24() {
    let expected = "a\nwoo\nb\nc\nd\n";
    let input = ["a", "b", "c", "d"];
    let source = "/b/N;/b\\nc/i woo";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed N (stops at end of input) and P (prints to first newline only)
#[test]
fn busybox_test_25() {
    let expected = "a\na\nb\na\nb\nc\n";
    let input = ["a", "b", "c"];
    let source = "N;P;p";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed G (append hold space to pattern space)
#[test]
fn busybox_test_26() {
    let expected = "a\n\nb\n\nc\n\n";
    let input = ["a", "b", "c"];
    let source = "G";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed d ends script iteration
#[test]
fn busybox_test_27() {
    let expected = "";
    let input = "ook";
    let source = "/ook/d;s/ook/ping/; p;i woot";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed embedded NUL
#[test]
fn busybox_test_28() {
    let expected = "\0bang\0woo\0";
    let input = "\0woo\0woo\0";
    let source = "s/woo/bang/";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed embedded NUL g
#[test]
fn busybox_test_29() {
    let expected = "bang\0bang\0";
    let input = "woo\0woo\0";
    let source = "s/woo/bang/g";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed append autoinserts newline
#[test]
fn busybox_test_30() {
    let expected = "woot\nwoo\n";
    let input = "woot";
    let source = "/woot/a woo";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed append autoinserts newline 2
#[test]
fn busybox_test_31() {
    let expected = "boot\nwoo\nwoot\nwoo\n";
    let input = ["boot", "woot"];
    let source = "/oot/a woo";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed append autoinserts newline 3
#[test]
fn busybox_test_32() {
    let expected = "boot\nwoo\n";
    let input = "boot";
    let source = "/oot/a woo";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed insert doesn't autoinsert newline
#[test]
fn busybox_test_33() {
    let expected = "woo\nwoot";
    let input = "woot";
    let source = "/woot/i woo";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed print autoinsert newlines
#[test]
fn busybox_test_34() {
    let expected = "one\none";
    let input = "one";
    let source = "p";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed print autoinsert newlines 'two files'
#[test]
fn busybox_test_35() {
    let expected = "one\none\ntwo\ntwo";
    let input = ["one", "two"];
    let source = "p";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed trailing NUL
#[test]
fn busybox_test_36() {
    let expected = "a\0b\0\nc";
    let input = ["a\0b\0", "c"];
    let source = "s/i/z/";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed escaped newline in command
#[test]
fn busybox_test_37() {
    let expected = "z\nz";
    let input = "a";
    let source = "s/a/z\\\nz/";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed match EOF
#[test]
fn busybox_test_38() {
    let expected = "hello\nthere\nthere";
    let input = ["hello", "there"];
    let source = "$p";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed s/xxx/[/
#[test]
fn busybox_test_41() {
    let expected = "[\n";
    let input = "xxx";
    let source = "s/xxx/[/";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed n command must reset 'substituted' bit
#[test]
fn busybox_test_42() {
    let expected = "0\nx\n2\ny\n";
    let input = ["0", "1", "2", "3"];
    let source = "s/1/x/;T;n;: next;s/3/y/;t quit;n;b next;: quit;q";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed d does not break n,m matching
#[test]
fn busybox_test_43() {
    let expected = "second\nsecond\nthird\nthird\n";
    let input = ["first", "second", "third", "fourth"];
    let source = "1d;1,3p;4d";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed d does not break n,regex matching
#[test]
fn busybox_test_44() {
    let expected = "second\nsecond\nthird\nthird\n";
    let input = ["first", "second", "third", "fourth"];
    let source = "1d;1,/hir/p; 4d";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed beginning (^) matches only once
#[test]
fn busybox_test_45() {
    let expected = ">/usr</>lib<\n";
    let input = "/usr/lib";
    let source = "s,(^/|)[^/][^/]*,>\\0<,g";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed c
#[test]
fn busybox_test_46() {
    let expected = "repl\nrepl\n";
    let input = ["first"," second"];
    let source = "crepl";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed nested {}s
#[test]
fn busybox_test_47() {
    let expected = "qwe\nasd\nacd\nacd\n";
    let input = ["qwe", "asd", "zxc"];
    let source = "/asd/ { p; /s/ { s/s/c/ }; p; q }";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed a cmd understands \r, \n
#[test]
fn busybox_test_48() {
    let expected = "line1\n\t\rzero\rone\ntwo\nthree";
    let input = "line1";
    let source = "/1/a\\t\\rzero\\rone\\ntwo\\\nthree";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed with N skipping lines past ranges on next cmds
#[test]
fn busybox_test_50() {
    let expected = "4\n4\n";
    let input = ["1", "2", "3", "4"];
    let source = "1{N;N;d};1p;2,3p;3p;4p";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed understands
#[test]
fn busybox_test_51() {
    let expected = "\rrr\n";
    let input = "rrr";
    let source = "s/r/\r/";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed zero chars match/replace advances correctly 1
#[test]
fn busybox_test_52() {
    let expected = "@h@e@o@\n";
    let input = "helllo";
    let source = "s/l*/@/g";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed zero chars match/replace advances correctly 2
#[test]
fn busybox_test_53() {
    let expected = "x.x.x";
    let input = ".a.b";
    let source = "s/[^ .]*/x/g";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed zero chars match/replace logic must not falsely trigger here 1
#[test]
fn busybox_test_54() {
    let expected = "_AAA1AA\n";
    let input = "_aaa1aa";
    let source = "s/a/A/g";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed zero chars match/replace logic must not falsely trigger here 2
#[test]
fn busybox_test_55() {
    let expected = "qwerty_\n";
    let input = "qwerty";
    let source = "s/ *$/_/g";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed special char as s/// delimiter, in pattern
#[test]
fn busybox_test_56() {
    let expected = "X+8=17\n";
    let input = "9+8=17";
    let source = "s+9\\++X+";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed special char as s/// delimiter, in replacement 1
#[test]
fn busybox_test_57() {
    let expected = "X&+8=17\n";
    let input = "9+8=17";
    let source = "s&9&X\\&&";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed special char as s/// delimiter, in replacement 2
#[test]
fn busybox_test_58() {
    let expected = "X1+8=17\n";
    let input = "9+8=17";
    let source = "s1(9)1X\\11";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed /regex/,N{...} addresses work
#[test]
fn busybox_test_61() {
    let expected = "1\n3\n4\n5\n";
    let input = ["1", "2", "3", "4", "5"];
    let source = "/^2/,2{d}";
    test_from_busybox_sed(expected, input, source);
}

///Automatically transformed from Busybox sed test sed ^ OR not^
#[test]
fn busybox_test_66() {
    let expected = "ca";
    let input = "abca";
    let source = "s/^a|b//g";
    test_from_busybox_sed(expected, input, source);
}
