use std::{env::args, io::{BufRead, BufReader, stdin}};

use seddish::program::{SedProgram, SedEffect};

pub fn main() {
    let stdin = BufReader::new(stdin());

    let mut argv = args();
    let argv0 = argv.next().expect("Should have a defined argv0");
    let script = argv.next();
    let _ = assert!(script.is_some(), "{argv0}: Missing argument. Usage: {argv0} <script>");
    let _ = assert!(argv.next().is_none(), "{argv0}: Unexpected argument. Usage: {argv0} <script>");


    let sed: SedProgram = script.unwrap().parse().expect("Error parsing script");

    let mut stdin_sed = sed.document();

    let mut lines = stdin.lines().peekable();

    while let Some(line) = lines.next().and_then(Result::ok) {
        let is_last = lines.peek().is_none();

        let mut line = stdin_sed.line(line, is_last);

        while let Some(eff) = line.next_effect() {
            match eff {
                SedEffect::Error(e) => {
                    panic!("{argv0}: {e}")
                },
                SedEffect::Quit => break,
                SedEffect::Print(p) => print!("{p}"),
                SedEffect::WriteFile(_, _) => {},
                SedEffect::RequestReadFileAppend(_) => {},
                SedEffect::RequestNextLineAppended => {},
                SedEffect::RequestNextLine => {},
            }
        }

        print!("{}", line.pattern());
    }
}