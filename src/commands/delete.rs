use crate::parser::{
    ParserError,
    parsecommand::{CommandResult, NoArgumentCommandFactory, SedCommand},
};

pub struct DeleteCommand;

impl SedCommand for DeleteCommand {
    fn execute(
        &self,
        st: crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        pattern: &mut String,
    ) -> CommandResult<'_> {
        pattern.clear();
        *st.implicit_print_at_end = false;
        CommandResult::BranchToEnd
    }
}

impl NoArgumentCommandFactory for DeleteCommand {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(DeleteCommand))
    }
}
