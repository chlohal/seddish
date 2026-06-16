use crate::parser::{
    ParserError,
    parsecommand::{CommandResult, NoArgumentCommandFactory, SedCommand},
};

pub struct QuitCommand;

impl SedCommand for QuitCommand {
    fn execute<'a>(
        &'a self,
        _: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        _: &'a mut String,
    ) -> CommandResult<'a> {
        CommandResult::QuitScript
    }
}

impl NoArgumentCommandFactory for QuitCommand {
    fn new(&self, _: &mut crate::parser::ParserState) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(QuitCommand))
    }
}
