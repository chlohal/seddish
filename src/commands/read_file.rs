use crate::{parser::{
    ParserError,
    parsecommand::{CommandResult, SedCommand, SingleLineArgumentCommandFactory},
}};

pub struct ReadFileCommand(String);

impl SedCommand for ReadFileCommand {
    fn execute<'a>(
        &'a self,
        _: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        _: &'a mut String,
    ) -> CommandResult<'a> {
        CommandResult::ReadFileAppend(self.0.as_ref())
    }
}

pub struct ReadFileCommandFactory;

impl SingleLineArgumentCommandFactory for ReadFileCommandFactory {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
        argument: String,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(ReadFileCommand(argument)))
    }
}
