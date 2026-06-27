use crate::parser::{
    ParserError,
    parsecommand::{CommandResult, NoArgumentCommandFactory, SedCommand},
};

pub struct PrintLineNumberCommand;

impl SedCommand for PrintLineNumberCommand {
    fn execute(
        &self,
        _: crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        _: &mut String,
    ) -> CommandResult<'_> {
        CommandResult::PrintLineNumber
    }
}

impl NoArgumentCommandFactory for PrintLineNumberCommand {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(PrintLineNumberCommand))
    }
}
