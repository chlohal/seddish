use crate::parser::{
    ParserError,
    parsecommand::{CommandResult, NoArgumentCommandFactory, SedCommand},
};

pub struct DeleteFirstLineCommand;

impl SedCommand for DeleteFirstLineCommand {
    fn execute(
        &self,
        st: crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        pattern: &mut String,
    ) -> CommandResult<'_> {
        if let Some((_pre, suf)) = pattern.split_once("\n") {
            *pattern = suf.to_string();
            CommandResult::BranchToStart
        } else {
            pattern.clear();
            *st.implicit_print_at_end = false;
            CommandResult::BranchToEnd
        }
        
    }
}

impl NoArgumentCommandFactory for DeleteFirstLineCommand {
    fn new(
        &self,
        _: &mut crate::parser::ParserState,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        Ok(Box::new(DeleteFirstLineCommand))
    }
}
