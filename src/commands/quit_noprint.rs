use crate::parser::{
    ParserError,
    parsecommand::{CommandResult, NoArgumentCommandFactory, SedCommand},
};

pub struct QuitNoPrintCommand;

impl SedCommand for QuitNoPrintCommand {
    fn execute<'a>(
        &'a self,
        _: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        pattern: &'a mut String,
    ) -> CommandResult<'a> {
        pattern.clear();
        CommandResult::QuitScript
    }
}

impl NoArgumentCommandFactory for QuitNoPrintCommand {
    fn new(&self, _: &mut crate::parser::ParserState) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(QuitNoPrintCommand))
    }
}
