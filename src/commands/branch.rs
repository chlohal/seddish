use crate::{parser::{
    ParserError,
    parsecommand::{CommandResult, SedCommand, SingleLineArgumentCommandFactory},
}};

pub struct BranchCommand(String);

impl SedCommand for BranchCommand {
    fn execute(
        &self,
        _: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        _: &mut String,
    ) -> CommandResult<'_> {
        CommandResult::BranchToLabel(&self.0)
    }
}

pub struct BranchCommandFactory;

impl SingleLineArgumentCommandFactory for BranchCommandFactory {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
        argument: String,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(BranchCommand(argument)))
    }
}
