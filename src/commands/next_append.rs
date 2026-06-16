use crate::parser::{
    ParserError,
    parsecommand::{CommandResult, NoArgumentCommandFactory, SedCommand},
};

pub struct NextAppendCommand;

impl SedCommand for NextAppendCommand {
    fn execute(
        &self,
        _: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        _: &mut String,
    ) -> CommandResult<'_> {
        CommandResult::ReadNextLineAppend
    }
}

impl NoArgumentCommandFactory for NextAppendCommand {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(NextAppendCommand))
    }
}
