use std::num::NonZero;

use crate::{
    parser::parsecommand::CommandResult,
    program::{SedLineState, SedProgram},
};

impl SedProgram {
    ///
    /// Evaluate the program with the given string as input.
    /// Warning: this does NO line splitting. If you want to do classic Sed-style
    /// line-by-line processing, then use the document() API to process line-by-line.
    pub fn eval(&self, src: String) -> Result<String, EvalError> {
        let mut doc = self.document();

        let mut result = String::new();

        let mut line = doc.line(src, true);

        while let Some(eff) = line.next_effect() {
            match eff {
                SedEffect::Error(e) => return Err(e),
                SedEffect::Quit => break,
                SedEffect::Print(s) => {
                    result += s.as_str();
                    result += "\n";
                }
                SedEffect::WriteFile(_pat, _txt) => {
                    //unimplemented
                }
                SedEffect::RequestReadFileAppend(_pat) => {
                    //unimplemented
                }
                SedEffect::RequestNextLine | SedEffect::RequestNextLineAppended => {
                    //no further lines, entire document is one line
                }
            }
        }

        Ok(result)
    }
    pub fn document(&self) -> DocumentEval<'_> {
        DocumentEval {
            ranges_entered: vec![false; self.commands.len()],
            prog: self,
            hold: String::new(),
            line_index: 0,
        }
    }
}

pub struct DocumentEval<'d> {
    prog: &'d SedProgram,
    hold: String,
    ranges_entered: Vec<bool>,
    line_index: usize,
}

impl<'p> DocumentEval<'p> {
    ///
    /// Process a line in the context of the given document. This will properly increment line counts
    /// in order to support advanced scripts.
    /// The given string should be a **single line**, with no newline at the end. Sed scripts expect a 
    /// trailing newline, but Rust's typical line-splitting APIs don't provide one: this API conforms to 
    /// Rust's style, and will internally append a newline to give sed scripts what they expect. If you 
    /// append a newline externally, then it'll be duplicated!
    /// 
    pub fn line<'d>(&'d mut self, mut src: String, is_last: bool) -> LineEval<'d, 'p>
    where
        'p: 'd,
    {
        src.push('\n');
        LineEval {
            doc: self,
            pc: 0,
            pattern: src,
            subst_successful: false,
            is_last,
        }
    }
}

pub struct LineEval<'d, 'p> {
    doc: &'d mut DocumentEval<'p>,
    pc: usize,
    pattern: String,
    is_last: bool,
    subst_successful: bool,
}

impl LineEval<'_, '_> {
    pub fn pattern(self) -> String {
        self.pattern
    }
    pub fn replace_with_next_line(&mut self, mut line: String) {
        line.push('\n');
        self.pattern = line;
    }
    pub fn append_next_line(&mut self, line: &str) {
        self.pattern += &line;
        self.pattern += "\n";
    }
    ///
    /// Progress the engine to the next external effect, with a limited amount of internal iterations.
    /// This prevents scripts from falling into infinite loops without a way to stop. 
    /// next_effect_limited(0) will do the same as next_effect(); i.e. run with an infinite amount of iterations.
    pub fn next_effect_limited(
        &mut self,
        mut num_allowed_instructions: usize,
    ) -> Result<Option<SedEffect>, EvalError> {
        let mut st = SedLineState {
            substitution_successful: self.subst_successful,
        };

        let info = super::SedLineInfo {
            index: self.doc.line_index,
        };

        if num_allowed_instructions > 0 {
            num_allowed_instructions = num_allowed_instructions.saturating_add(1);
        }

        loop {
            //We've reached the end
            if self.pc >= self.doc.prog.commands.len() {
                return Ok(None);
            }

            if num_allowed_instructions == 1 {
                return Err(EvalError::InfiniteLoop);
            }
            num_allowed_instructions = num_allowed_instructions.saturating_sub(1);
            

            let prev_pc = self.pc;
            self.pc += 1;
            if current_is_active(self, prev_pc) {
                let cmd = &self.doc.prog.commands[prev_pc];
                let cmd_result = match &cmd.command {
                    super::BlockType::BlockBranch(new_pc) => {
                        self.pc = *new_pc;
                        continue;
                    }
                    super::BlockType::SingleCommand(sed_command) => {
                        sed_command.execute(&mut st, &info, &mut self.doc.hold, &mut self.pattern)
                    }
                };
                match cmd_result {
                    CommandResult::BranchToLabel(l) => match self.doc.prog.labels.get(l) {
                        Some(new_pc) => self.pc = *new_pc,
                        None => {
                            return Ok(Some(SedEffect::Error(EvalError::LabelNotFound(
                                l.to_string(),
                            ))));
                        }
                    },
                    CommandResult::BranchToStart => {
                        self.pc = 0;
                    }
                    CommandResult::BranchToEnd => {
                        self.pc = self.doc.prog.commands.len();
                    }
                    CommandResult::QuitScript => {
                        return Ok(Some(SedEffect::Quit));
                    }
                    CommandResult::Print(s) => return Ok(Some(SedEffect::Print(s.into()))),
                    CommandResult::PrintLineNumber => {
                        return Ok(Some(SedEffect::Print(self.doc.line_index.to_string())));
                    }
                    CommandResult::ReadNextLine => return Ok(Some(SedEffect::RequestNextLine)),
                    CommandResult::ReadNextLineAppend => {
                        return Ok(Some(SedEffect::RequestNextLineAppended));
                    }
                    CommandResult::ReadFileAppend(path) => {
                        return Ok(Some(SedEffect::RequestReadFileAppend(path.to_path_buf())));
                    }
                    CommandResult::WriteFile(path, s) => {
                        return Ok(Some(SedEffect::WriteFile(
                            path.to_path_buf(),
                            s.to_string(),
                        )));
                    }
                    CommandResult::Nothing => continue,
                }
            }
        }
    }
    pub fn next_effect(&mut self) -> Option<SedEffect> {
        self.next_effect_limited(0)
            .transpose()
            .map(|x| match x {
                Ok(ef) => ef,
                Err(er) => SedEffect::Error(er),
            })
    }
}

fn current_is_active(state: &mut LineEval<'_, '_>, pc: usize) -> bool {
    match &state.doc.prog.commands[pc].filter {
        crate::address_range::AddressRange::All => true,
        crate::address_range::AddressRange::Single { addr, negated } => {
            let r = addr.matches(state.doc.line_index, &state.pattern, state.is_last);
            if *negated { !r } else { r }
        }
        crate::address_range::AddressRange::Range {
            start,
            end,
            negated,
        } => {
            let in_range = if state.doc.ranges_entered[state.pc] {
                if end.matches(state.doc.line_index, &state.pattern, state.is_last) {
                    state.doc.ranges_entered[state.pc] = false;
                }
                true
            } else {
                if start.matches(state.doc.line_index, &state.pattern, state.is_last) {
                    state.doc.ranges_entered[state.pc] = true;
                    true
                } else {
                    false
                }
            };

            if *negated { !in_range } else { in_range }
        }
    }
}

#[derive(Debug)]
pub enum SedEffect {
    Error(EvalError),
    Quit,
    Print(String),
    WriteFile(std::path::PathBuf, String),
    RequestReadFileAppend(std::path::PathBuf),
    RequestNextLineAppended,
    RequestNextLine,
}

#[derive(Debug)]
pub enum EvalError {
    LabelNotFound(String),
    InfiniteLoop,
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::LabelNotFound(l) => write!(f, "Label not found: '{l}'"),
            EvalError::InfiniteLoop => f.write_str("Infinite loop limit triggered"),
        }
    }
}