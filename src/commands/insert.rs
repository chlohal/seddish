use crate::{parser::{
    ParserError,
    parsecommand::{CommandResult, MultiLineArgumentCommandFactory, SedCommand},
}};

pub struct InsertCommand(String);

impl SedCommand for InsertCommand {
    fn execute(
        &self,
        _: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        pattern_space: &mut String,
    ) -> CommandResult<'_> {
        *pattern_space = format!("{}{}", self.0, pattern_space);
        CommandResult::Nothing
    }
}

pub struct InsertCommandFactory;

impl MultiLineArgumentCommandFactory for InsertCommandFactory {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
        argument: String,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(InsertCommand(argument)))
    }
}
