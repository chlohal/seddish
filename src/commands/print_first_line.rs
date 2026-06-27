use crate::parser::{
    ParserError,
    parsecommand::{CommandResult, NoArgumentCommandFactory, SedCommand},
};

pub struct PrintFirstLineCommand;

impl SedCommand for PrintFirstLineCommand {
    fn execute<'a>(
        &'a self,
        _: crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        pattern: &'a mut String,
    ) -> CommandResult<'a> {
        let f = pattern.split_once('\n').map(|(f, _)| f).unwrap_or(pattern.as_str());
        CommandResult::Print(f)
    }
}

impl NoArgumentCommandFactory for PrintFirstLineCommand {
    fn new(&self, _: &mut crate::parser::ParserState) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(PrintFirstLineCommand))
    }
}
