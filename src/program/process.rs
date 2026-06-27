use std::mem::{replace, take};

use crate::{
    address_range::Address,
    parser::parsecommand::CommandResult,
    program::{SedLineState, SedProgram},
};

impl SedProgram {
    ///
    /// Evaluate the program with the given string as input.
    /// Warning: this does NO line splitting. If you want to do classic Sed-style
    /// line-by-line processing, then use the document() API to process line-by-line.
    pub fn eval(&self, src: String, mut result: impl std::fmt::Write) -> Result<(), EvalError> {
        let mut doc = self.document(true);

        let mut line = doc.line(src, true);

        while let Some(eff) = line.next_effect() {
            match eff {
                SedEffect::Error(e) => return Err(e),
                SedEffect::Quit => break,
                SedEffect::Print(s) => {
                    result
                        .write_fmt(format_args!("{}", s))
                        .map_err(|_| EvalError::Utf8Error)?;
                }
                SedEffect::WriteFile(_pat, _txt) => {
                    //unimplemented
                }
                SedEffect::RequestReadFileAppend(_pat) => {
                    //unimplemented
                }
                SedEffect::NextLineKeepingStateState | SedEffect::RequestNextLineAppended => {
                    //no further lines, entire document is one line
                }
            }
        }

        Ok(())
    }
    pub fn document(&self, implicit_print_at_end: bool) -> DocumentEval<'_> {
        DocumentEval {
            implicit_print_at_end,
            //ranges starting with 0 will begin as entered (unless inverted, ofc), since line indexes start at 1.
            ranges_entered: self
                .commands
                .iter()
                .map(|x| match &x.filter {
                    crate::address_range::AddressRange::Range {
                        start: Address::LineNumber(0),
                        end: _,
                        negated: _,
                    } => RangeState::Entered,
                    _ => RangeState::NotEntered,
                })
                .collect(),
            prog: self,
            hold: String::new(),
            //this gets pre-incremented before each line is evaluated, so we need to initialize it at 0
            line_index: 0,
        }
    }
}

pub struct DocumentEval<'d> {
    prog: &'d SedProgram,
    hold: String,
    ranges_entered: Box<[RangeState]>,
    line_index: usize,
    implicit_print_at_end: bool,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(self) enum RangeState {
    NotEntered = 0,
    Entered = 1,
    Exited = 2,
}

impl<'p> DocumentEval<'p> {
    ///
    /// Process a line in the context of the given document. This will properly increment line counts
    /// in order to support advanced scripts.
    /// The given string should be a **single line**, with no newline at the end. In debug builds, this will panic if
    /// there is a newline.
    ///
    pub fn line<'d>(&'d mut self, src: String, is_last: bool) -> LineEval<'d, 'p>
    where
        'p: 'd,
    {
        //Shouldn't end with newline.
        debug_assert_ne!(src.bytes().last(), Some(b'\n'));
        self.line_index += 1;
        LineEval {
            //copying so each new line gets its own flag; important because the lines will overwrite this
            //to signify that they've already implicitly printed
            implicit_print_at_end: self.implicit_print_at_end,
            doc: self,
            pc: 0,
            holdover_effect: None,
            pattern: src,
            subst_successful: false,
            is_last,
        }
    }
}

pub struct LineEval<'d, 'p> {
    doc: &'d mut DocumentEval<'p>,
    pc: usize,
    holdover_effect: Option<SedEffect>,
    pattern: String,
    is_last: bool,
    pub(super) subst_successful: bool,
    pub(super) implicit_print_at_end: bool,
}

impl LineEval<'_, '_> {
    ///
    /// Implements the default behaviour of the 'n' command: that is,
    ///
    /// > If auto-print is not disabled, print the pattern space, then,
    /// > regardless, replace the pattern space with the next line of input.
    /// > If there is no more input then ‘sed’ exits without processing any
    /// > more commands.
    /// > ~ Sed infopage
    ///
    /// If line is None (indicating that there is no more input), then the next
    /// value of next_effect() or next_effect_limited() will be (possibly a Print, depending on
    /// whether implicit printing is enabled, and then) Ok(None)
    ///
    /// This function will affect the next few results returned from next_effect(); that is,
    /// the next few effects will not necessarily be from the Sed script, since replacing does
    /// do more than one thing. If this function is called repeatedly without invoking next_effect(),
    /// then the results may be inconsistent.
    ///
    pub fn replace_with_next_line(&mut self, line: Option<impl AsRef<str>>) {
        if self.implicit_print_at_end {
            self.holdover_effect
                .get_or_insert(SedEffect::Print(take(&mut self.pattern)));
        }
        self.pattern.clear();
        if let Some(line) = line {
            self.pattern += line.as_ref();
        } else {
            //Branching to end without processing any more commands
            self.pc = self.doc.prog.commands.len();
            self.implicit_print_at_end = false;
        }
    }

    ///
    /// Implements the default behaviour of the 'N' command: that is,
    ///
    /// > Add a newline to the pattern space, then append the next line of
    /// > input to the pattern space.  If there is no more input then ‘sed’
    /// > exits without processing any more commands.
    /// > ~ Sed infopage
    ///
    /// If line is None (indicating that there is no more input), then the next
    /// value of next_effect() or next_effect_limited() will be Ok(None)
    ///
    pub fn append_next_line(&mut self, line: Option<impl AsRef<str>>) {
        if let Some(line) = line {
            self.pattern += "\n";
            self.pattern += &line.as_ref();
        } else {
            self.pc = self.doc.prog.commands.len();
        }
    }
    ///
    /// Progress the engine to the next external effect, with a limited amount of internal iterations.
    /// This prevents scripts from falling into infinite loops without a way to stop.
    /// next_effect_limited(0) will do the same as next_effect(); i.e. run with an infinite amount of iterations.
    pub fn next_effect_limited(
        &mut self,
        mut num_allowed_instructions: usize,
    ) -> Result<Option<SedEffect>, EvalError> {
        let info = super::SedLineInfo {
            index: self.doc.line_index,
        };

        if num_allowed_instructions > 0 {
            num_allowed_instructions = num_allowed_instructions.saturating_add(1);
        }

        if let Some(holdover) = self.holdover_effect.take() {
            return Ok(Some(holdover));
        }

        loop {
            //We've reached the end
            if self.pc >= self.doc.prog.commands.len() {
                //if we should implicitly print, then print and remove the implicit printing flag.
                if take(&mut self.implicit_print_at_end) {
                    return Ok(Some(SedEffect::Print(take(&mut self.pattern))));
                } else {
                    return Ok(None);
                }
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
                    super::BlockType::SingleCommand(sed_command) => sed_command.execute(
                        SedLineState {
                            substitution_successful: &mut self.subst_successful,
                            implicit_print_at_end: &mut self.implicit_print_at_end,
                        },
                        &info,
                        &mut self.doc.hold,
                        &mut self.pattern,
                    ),
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
                        //Quitting is the end of a line, so make sure to print if it's desired!
                        //Commands that want to quit without printing can set implicit_print_at_end to false
                        //before returning if they desire.
                        if take(&mut self.implicit_print_at_end) {
                            self.holdover_effect = Some(SedEffect::Quit);
                            return Ok(Some(SedEffect::Print(take(&mut self.pattern))));
                        } else {
                            return Ok(Some(SedEffect::Quit));
                        }
                    }
                    CommandResult::Print(s) => return Ok(Some(SedEffect::Print(s.into()))),
                    CommandResult::PrintLineNumber => {
                        return Ok(Some(SedEffect::Print(self.doc.line_index.to_string())));
                    }
                    CommandResult::ReadNextLine => {
                        return Ok(Some(SedEffect::NextLineKeepingStateState));
                    }
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
        self.next_effect_limited(0).transpose().map(|x| match x {
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
            //If we've entered, no reason to check the start. Just check the end, but ensure that
            //regardless of the result, we return true to in_range: ranges are inclusive on both
            //ends.

            let in_range = if range_entered_or_stateless_entered(
                start,
                state.doc.line_index,
                state.doc.ranges_entered[pc],
            ) {
                if end.matches(state.doc.line_index, &state.pattern, state.is_last) {
                    state.doc.ranges_entered[pc] = RangeState::Exited;
                }
                true
            } else {
                if start.matches(state.doc.line_index, &state.pattern, state.is_last) {
                    //If the start and end match on the same line, then we should _return_ true but
                    //_set the state_ to exited, since it will only be "entered" for this single line.
                    if end.matches(state.doc.line_index, &state.pattern, state.is_last) {
                        state.doc.ranges_entered[pc] = RangeState::Exited;
                    } else {
                        state.doc.ranges_entered[pc] = RangeState::Entered;
                    }
                    true
                } else {
                    false
                }
            };

            if *negated { !in_range } else { in_range }
        }
    }
}

fn range_entered_or_stateless_entered(
    start: &Address,
    line_index: usize,
    current_recorded_state: RangeState,
) -> bool {
    return current_recorded_state == RangeState::Entered
    //Since some commands skip the rest of the script for the input line,
    //we need to specifically check if that line _should have_ entered this range.
    //This is only relevant for line-number-based ranges, since the standard doesn't
    //dictate we need to reevaluate every regex for every line (thankfully !!!)
        || (current_recorded_state == RangeState::NotEntered
            && match start {
                Address::LineNumber(l) => line_index >= *l,
                _ => false,
            });
}

#[derive(Debug)]
pub enum SedEffect {
    Error(EvalError),
    Quit,
    Print(String),
    WriteFile(std::path::PathBuf, String),
    RequestReadFileAppend(std::path::PathBuf),
    RequestNextLineAppended,
    NextLineKeepingStateState,
}

#[derive(Debug)]
pub enum EvalError {
    LabelNotFound(String),
    InfiniteLoop,
    IoError(std::io::Error),
    Utf8Error,
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::LabelNotFound(l) => write!(f, "Label not found: '{l}'"),
            EvalError::InfiniteLoop => f.write_str("Infinite loop limit triggered"),
            EvalError::IoError(io) => f.write_fmt(format_args!("OS I/O error: {io}")),
            EvalError::Utf8Error => f.write_str("Error with UTF-8 string formatting"),
        }
    }
}
