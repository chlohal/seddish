use crate::parser::Parser;

pub(self) fn test_from_busybox_sed(expected: &str, input: &str, source: &str) {
    let sed = Parser::new().parse(source).unwrap();
    let mut doc = sed.document();
    let mut lines  = input.lines().peekable();

    let mut printed = String::new();

    while let Some(l) = lines.next() {
        let is_last = lines.peek().is_none();
        
        let mut line = doc.line(l.to_string(), is_last);

        while let Some(eff) = line.next_effect() {
            match eff {
                crate::program::SedEffect::Error(err) => panic!("{err}"),
                crate::program::SedEffect::Quit => break,
                crate::program::SedEffect::Print(p) => {
                    printed += &p;
                },
                crate::program::SedEffect::WriteFile(_, _) => panic!("Can't write in test"),
                crate::program::SedEffect::RequestReadFileAppend(_) => panic!("Can't read in test"),
                crate::program::SedEffect::RequestNextLineAppended => if let Some(nl) = lines.next() {
                    line.append_next_line(nl)
                },
                crate::program::SedEffect::RequestNextLine => if let Some(nl) = lines.next() {
                    line.replace_with_next_line(nl.to_string())
                },
            }
        }
        let final_pat = line.pattern();
        if !final_pat.trim().is_empty() {
            printed += &final_pat;
        }
    }

    assert_eq!(printed, expected);
}

mod busybox;