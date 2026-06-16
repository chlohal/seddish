use crate::parser::{
    ParserError,
    parsecommand::{CommandResult, NoArgumentCommandFactory, SedCommand},
};

pub struct DeleteCommand;

impl SedCommand for DeleteCommand {
    fn execute(
        &self,
        _: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        pattern: &mut String,
    ) -> CommandResult<'_> {
        pattern.clear();
        CommandResult::BranchToEnd
    }
}

impl NoArgumentCommandFactory for DeleteCommand {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(DeleteCommand))
    }
}
