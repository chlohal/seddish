use crate::parser::{
    ParserError,
    parsecommand::{CommandResult, NoArgumentCommandFactory, SedCommand},
};

pub struct HoldReplaceCommand;

impl SedCommand for HoldReplaceCommand {
    fn execute(
        &self,
        _: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        hold: &mut String,
        pattern: &mut String,
    ) -> CommandResult<'_> {
        hold.clear();
        hold.push_str(&pattern);
        CommandResult::Nothing
    }
}

impl NoArgumentCommandFactory for HoldReplaceCommand {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(HoldReplaceCommand))
    }
}
