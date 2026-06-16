use crate::{
    parser::{ParserError, ParserState},
    program::{SedLineInfo, SedLineState},
};

pub trait SubstitutionLikeCommandFactory: 'static {
    fn new(&self, current_parser: &mut ParserState, arguments: Vec<String>, flags: String) -> Result<Box<dyn SedCommand>, ParserError>;
    fn check_flag(&self, flag: char) -> bool;
    fn field_count(&self) -> usize;
}

pub trait SingleLineArgumentCommandFactory: 'static {
    fn new(&self, current_parser: &mut ParserState, argument: String) -> Result<Box<dyn SedCommand>, ParserError>;
}

pub trait MultiLineArgumentCommandFactory: 'static {
    fn new(&self, current_parser: &mut ParserState, argument: String) -> Result<Box<dyn SedCommand>, ParserError>;
}

pub trait NoArgumentCommandFactory: 'static {
    fn new(&self, current_parser: &mut ParserState) -> Result<Box<dyn SedCommand>, ParserError>;
}

pub trait SedCommand: 'static {
    fn execute<'a>(
        &'a self,
        line_state: &'a mut SedLineState,
        line_details: &'a SedLineInfo,
        hold_space: &'a mut String,
        pattern_space: &'a mut String,
    ) -> CommandResult<'a>;
}

pub enum CommandResult<'a> {
    BranchToLabel(&'a str),
    BranchToStart,
    BranchToEnd,
    Print(&'a str),
    PrintLineNumber,
    QuitScript,
    ReadNextLine,
    ReadNextLineAppend,
    ReadFileAppend(&'a std::path::Path),
    Nothing,
    WriteFile(&'a std::path::Path, &'a str),
}
