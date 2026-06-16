use crate::{parser::{
    ParserError,
    parsecommand::{CommandResult, SedCommand, SingleLineArgumentCommandFactory},
}};

pub struct WriteFileCommand(String);

impl SedCommand for WriteFileCommand {
    fn execute<'a>(
        &'a self,
        _: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        pattern: &'a mut String,
    ) -> CommandResult<'a> {
        CommandResult::WriteFile(&self.0.as_ref(), pattern.as_str())
    }
}

pub struct WriteFileCommandFactory;

impl SingleLineArgumentCommandFactory for WriteFileCommandFactory {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
        argument: String,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(WriteFileCommand(argument)))
    }
}
