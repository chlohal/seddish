use crate::parser::{
    ParserError,
    parsecommand::{CommandResult, NoArgumentCommandFactory, SedCommand},
};

pub struct PrintCommand;

impl SedCommand for PrintCommand {
    fn execute<'a>(
        &'a self,
        _: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        pattern: &'a mut String,
    ) -> CommandResult<'a> {
        CommandResult::Print(pattern.as_str())
    }
}

impl NoArgumentCommandFactory for PrintCommand {
    fn new(&self, _: &mut crate::parser::ParserState) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(PrintCommand))
    }
}
