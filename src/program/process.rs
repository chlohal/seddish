use crate::{
    parser::parsecommand::CommandResult,
    program::{SedLineState, SedProgram},
};

impl SedProgram {
    pub fn eval(&mut self, src: String) -> Result<String, EvalError> {
        let mut doc = self.document();

        let mut result = String::new();

        let mut line = doc.line(src, true);

        while let Some(eff) = line.next() {
            match eff {
                SedEffect::LabelNotFound(s) => return Err(EvalError::LabelNotFound(s)),
                SedEffect::Quit => break,
                SedEffect::Print(s) => {
                    result += s.as_str();
                    result += "\n";
                },
                SedEffect::WriteFile(_pat, _txt) => {
                    //unimplemented
                },
                SedEffect::RequestReadFileAppend(_pat) => {
                    //unimplemented
                },
                SedEffect::RequestNextLine | SedEffect::RequestNextLineAppended => {
                    //no further lines, entire document is one line
                },
            }
        }

        Ok(result)
    }
    pub fn document(&mut self) -> DocumentEval<'_> {
        DocumentEval {
            ranges_entered: vec![false; self.commands.len()],
            prog: self,
            hold: String::new(),
            line_index: 0,
        }
    }
}

pub struct DocumentEval<'d> {
    prog: &'d mut SedProgram,
    hold: String,
    ranges_entered: Vec<bool>,
    line_index: usize,
}

impl<'p> DocumentEval<'p> {
    pub fn line<'d>(&'d mut self, src: String, is_last: bool) -> LineEval<'d, 'p>
    where
        'p: 'd,
    {
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
}

impl Iterator for LineEval<'_, '_> {
    type Item = SedEffect;

    fn next(&mut self) -> Option<Self::Item> {
        let mut st = SedLineState {
            substitution_successful: self.subst_successful,
        };

        let info = super::SedLineInfo {
            index: self.doc.line_index,
        };

        loop {
            //We've reached the end
            if self.pc >= self.doc.prog.commands.len() {
                return None;
            }

            if current_is_active(self) {
                let cmd = &self.doc.prog.commands[self.pc];
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
                        None => return Some(SedEffect::LabelNotFound(l.to_string())),
                    },
                    CommandResult::BranchToStart => {
                        self.pc = 0;
                    }
                    CommandResult::BranchToEnd => {
                        self.pc = self.doc.prog.commands.len();
                    }
                    CommandResult::QuitScript => {
                        return Some(SedEffect::Quit);
                    }
                    pc_inc_result => {
                        self.pc += 1;
                        match pc_inc_result {
                            CommandResult::Print(s) => return Some(SedEffect::Print(s.into())),
                            CommandResult::PrintLineNumber => {
                                return Some(SedEffect::Print(self.doc.line_index.to_string()));
                            }
                            CommandResult::ReadNextLine => return Some(SedEffect::RequestNextLine),
                            CommandResult::ReadNextLineAppend => {
                                return Some(SedEffect::RequestNextLineAppended);
                            }
                            CommandResult::ReadFileAppend(path) => {
                                return Some(SedEffect::RequestReadFileAppend(path.to_path_buf()));
                            }
                            CommandResult::WriteFile(path, s) => {
                                return Some(SedEffect::WriteFile(
                                    path.to_path_buf(),
                                    s.to_string(),
                                ));
                            }
                            CommandResult::Nothing => continue,

                            CommandResult::BranchToLabel(_)
                            | CommandResult::BranchToStart
                            | CommandResult::BranchToEnd
                            | CommandResult::QuitScript => unreachable!(),
                        }
                    }
                }
            } else {
                self.pc += 1;
            }
        }
    }
}

fn current_is_active(state: &mut LineEval<'_, '_>) -> bool {
    let pc = state.pc;
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

pub enum SedEffect {
    LabelNotFound(String),
    Quit,
    Print(String),
    WriteFile(std::path::PathBuf, String),
    RequestReadFileAppend(std::path::PathBuf),
    RequestNextLineAppended,
    RequestNextLine,
}

pub enum EvalError {
    LabelNotFound(String)
}