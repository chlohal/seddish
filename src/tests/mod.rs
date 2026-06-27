use std::fmt::Write;

use crate::parser::Parser;

trait ListOrSingle<T> {
    fn to_list(&self) -> &[T];
}

impl<T> ListOrSingle<T> for T {
    fn to_list(&self) -> &[T] {
        std::array::from_ref(self)
    }
}

impl<const N: usize, T> ListOrSingle<T> for [T; N] {
    fn to_list(&self) -> &[T] {
        self
    }
}

pub(self) fn test_from_busybox_sed(expected: &str, input_lines: impl ListOrSingle<&'static str>, source: &str) {
    let sed = Parser::new().parse(source).unwrap();
    let mut doc = sed.document(true);
    let mut lines  = input_lines.to_list().into_iter().peekable();

    let mut printed = String::new();

    'lines: while let Some(l) = lines.next() {
        let is_last = lines.peek().is_none();
        
        let mut line = doc.line(l.to_string(), is_last);

        while let Some(eff) = line.next_effect() {
            match eff {
                crate::program::SedEffect::Error(err) => panic!("{err}"),
                crate::program::SedEffect::Quit => break 'lines,
                crate::program::SedEffect::Print(p) => {
                    writeln!(&mut printed, "{p}").unwrap();
                },
                crate::program::SedEffect::WriteFile(_, _) => panic!("Can't write in test"),
                crate::program::SedEffect::RequestReadFileAppend(_) => panic!("Can't read in test"),
                crate::program::SedEffect::RequestNextLineAppended => {
                    line.append_next_line(lines.next());
                },
                crate::program::SedEffect::NextLineKeepingStateState => {
                    line.replace_with_next_line(lines.next());
                },
            }
        }
    }

    assert_eq!(printed.trim(), expected.trim(), "Expected: {:?}, actual: {:?}", expected.trim(), printed.trim());
}

mod busybox;