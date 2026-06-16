use crate::{parser::{
    ParserError,
    parsecommand::{CommandResult, SedCommand, SingleLineArgumentCommandFactory},
}};

pub struct WriteFileFirstLineCommand(String);

impl SedCommand for WriteFileFirstLineCommand {
    fn execute<'a>(
        &'a self,
        _: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        pattern: &'a mut String,
    ) -> CommandResult<'a> {
        let f = pattern.split('\n').next().unwrap_or(pattern.as_str());
        CommandResult::WriteFile(&self.0.as_ref(), f)
    }
}

pub struct WriteFileFirstLineCommandFactory;

impl SingleLineArgumentCommandFactory for WriteFileFirstLineCommandFactory {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
        argument: String,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(WriteFileFirstLineCommand(argument)))
    }
}
