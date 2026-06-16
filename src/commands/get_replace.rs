use crate::parser::{
    ParserError,
    parsecommand::{CommandResult, NoArgumentCommandFactory, SedCommand},
};

pub struct GetReplaceCommand;

impl SedCommand for GetReplaceCommand {
    fn execute(
        &self,
        _: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        hold: &mut String,
        pattern: &mut String,
    ) -> CommandResult<'_> {
        pattern.clear();
        pattern.push_str(&hold);
        CommandResult::Nothing
    }
}

impl NoArgumentCommandFactory for GetReplaceCommand {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(GetReplaceCommand))
    }
}
