use crate::{parser::{
    ParserError,
    parsecommand::{CommandResult, SedCommand, SingleLineArgumentCommandFactory},
}, program::SedLineState};

pub struct BranchIfSubSuccessfulCommand(String);

impl SedCommand for BranchIfSubSuccessfulCommand {
    fn execute(
        &self,
        SedLineState { substitution_successful, .. }: crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        _: &mut String,
    ) -> CommandResult<'_> {
        if *substitution_successful {
            *substitution_successful = false;
            if self.0.is_empty() {
                CommandResult::BranchToEnd
            } else {
                CommandResult::BranchToLabel(&self.0)
            }
        } else {
            CommandResult::Nothing
        }
    }
}

pub struct BranchIfSubSuccessfulCommandFactory;

impl SingleLineArgumentCommandFactory for BranchIfSubSuccessfulCommandFactory {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
        argument: String,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(BranchIfSubSuccessfulCommand(argument.trim().to_owned())))
    }
}
