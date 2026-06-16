use std::{env::args, io::{BufRead, BufReader, stdin}};

use seddish::program::{SedProgram, SedEffect};

pub fn main() {
    let stdin = BufReader::new(stdin());

    let script = args().nth(1).expect("Usage: seddish <script>");


    let mut sed: SedProgram = script.parse().expect("Error parsing script");

    let mut stdin_sed = sed.document();

    let mut lines = stdin.lines().peekable();

    while let Some(mut line) = lines.next().and_then(Result::ok) {
        let is_last = lines.peek().is_none();

        line.push('\n');

        let mut line = stdin_sed.line(line, is_last);

        while let Some(eff) = line.next() {
            match eff {
                SedEffect::LabelNotFound(e) => {
                    panic!("Label not found: {e}")
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