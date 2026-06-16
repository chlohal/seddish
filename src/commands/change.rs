use crate::{parser::{
    ParserError,
    parsecommand::{CommandResult, MultiLineArgumentCommandFactory, SedCommand},
}};

pub struct ChangeCommand(String);

impl SedCommand for ChangeCommand {
    fn execute(
        &self,
        _: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        pattern_space: &mut String,
    ) -> CommandResult<'_> {
        pattern_space.clear();
        pattern_space.push_str(&self.0);
        CommandResult::Nothing
    }
}

pub struct ChangeCommandFactory;

impl MultiLineArgumentCommandFactory for ChangeCommandFactory {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
        argument: String,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(ChangeCommand(argument)))
    }
}
