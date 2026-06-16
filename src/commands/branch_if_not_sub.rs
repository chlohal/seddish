use crate::{parser::{
    ParserError,
    parsecommand::{CommandResult, SedCommand, SingleLineArgumentCommandFactory},
}, program::SedLineState};

pub struct BranchIfNotSubSuccessfulCommand(String);

impl SedCommand for BranchIfNotSubSuccessfulCommand {
    fn execute(
        &self,
        SedLineState { substitution_successful }: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        _: &mut String,
    ) -> CommandResult<'_> {
        if *substitution_successful {
            CommandResult::Nothing
        } else {
            CommandResult::BranchToLabel(&self.0)
            
        }
    }
}

pub struct BranchIfNotSubSuccessfulCommandFactory;

impl SingleLineArgumentCommandFactory for BranchIfNotSubSuccessfulCommandFactory {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
        argument: String,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(BranchIfNotSubSuccessfulCommand(argument)))
    }
}
