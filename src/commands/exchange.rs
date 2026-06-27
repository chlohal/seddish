use crate::parser::{
    ParserError,
    parsecommand::{CommandResult, NoArgumentCommandFactory, SedCommand},
};

pub struct ExchangeCommand;

impl SedCommand for ExchangeCommand {
    fn execute(
        &self,
        _: crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        hold: &mut String,
        pattern: &mut String,
    ) -> CommandResult<'_> {
        std::mem::swap(hold, pattern);
        CommandResult::Nothing
    }
}

impl NoArgumentCommandFactory for ExchangeCommand {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(ExchangeCommand))
    }
}
