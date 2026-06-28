use crate::{
    parser::{ParserError, ParserState},
    program::{SedLineInfo, SedLineState},
};

pub trait SubstitutionLikeCommandFactory: 'static + Send {
    fn new(&self, current_parser: &mut ParserState, arguments: Vec<String>, flags: String) -> Result<Box<dyn SedCommand>, ParserError>;
    fn check_flag(&self, flag: char) -> bool;
    fn field_count(&self) -> usize;
    fn command_name(&self) -> &'static str;
}

impl<F: Fn(&mut ParserState, String) -> Result<Box<dyn SedCommand>, ParserError> + 'static + Send> SingleLineArgumentCommandFactory for F {
    fn new(&self, current_parser: &mut ParserState, argument: String) -> Result<Box<dyn SedCommand>, ParserError> {
        (self)(current_parser, argument)
    }
}

pub trait SingleLineArgumentCommandFactory: 'static + Send {
    fn new(&self, current_parser: &mut ParserState, argument: String) -> Result<Box<dyn SedCommand>, ParserError>;
}

impl<F: Fn(&mut ParserState, String) -> Result<Box<dyn SedCommand>, ParserError> + 'static + Send> MultiLineArgumentCommandFactory for F {
    fn new(&self, current_parser: &mut ParserState, argument: String) -> Result<Box<dyn SedCommand>, ParserError> {
        (self)(current_parser, argument)
    }
}

pub trait MultiLineArgumentCommandFactory: 'static + Send {
    fn new(&self, current_parser: &mut ParserState, argument: String) -> Result<Box<dyn SedCommand>, ParserError>;
}

impl<F: Fn(&mut ParserState) -> Result<Box<dyn SedCommand>, ParserError> + 'static + Send> NoArgumentCommandFactory for F {
    fn new(&self, current_parser: &mut ParserState) -> Result<Box<dyn SedCommand>, ParserError> {
        (self)(current_parser)
    }
}

impl NoArgumentCommandFactory for fn() -> CommandResult<'static> {
    fn new(&self, _: &mut ParserState) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(*self))
    }
}

impl SedCommand for fn() -> CommandResult<'static> {
    fn execute<'a>(
        &'a self,
        _: SedLineState,
        _: &'a SedLineInfo,
        _: &'a mut String,
        _: &'a mut String,
    ) -> CommandResult<'a> {
        self()
    }
}

pub trait NoArgumentCommandFactory: 'static + Send {
    fn new(&self, current_parser: &mut ParserState) -> Result<Box<dyn SedCommand>, ParserError>;
}

pub trait SedCommand: 'static + Send + Sync {
    fn execute<'a>(
        &'a self,
        line_state: SedLineState,
        line_details: &'a SedLineInfo,
        hold_space: &'a mut String,
        pattern_space: &'a mut String,
    ) -> CommandResult<'a>;
}

#[derive(Debug)]
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
