use crate::{parser::{
    ParserError,
    parsecommand::{CommandResult, MultiLineArgumentCommandFactory, SedCommand},
}};

pub struct AppendCommand(String);

impl SedCommand for AppendCommand {
    fn execute(
        &self,
        _: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        pattern_space: &mut String,
    ) -> CommandResult<'_> {
        pattern_space.push_str(self.0.as_str());
        CommandResult::Nothing
    }
}

pub struct AppendCommandFactory;

impl MultiLineArgumentCommandFactory for AppendCommandFactory {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
        argument: String,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(AppendCommand(argument)))
    }
}
