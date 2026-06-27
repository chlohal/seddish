use crate::parser::{
    ParserError,
    parsecommand::{CommandResult, NoArgumentCommandFactory, SedCommand},
};

pub struct NextCommand;

impl SedCommand for NextCommand {
    fn execute(
        &self,
        st: crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        _: &mut String,
    ) -> CommandResult<'_> {
        *st.substitution_successful = false;
        CommandResult::ReadNextLine
    }
}

impl NoArgumentCommandFactory for NextCommand {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(NextCommand))
    }
}
