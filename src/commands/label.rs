use crate::parser::{
    ParserError,
    parsecommand::{CommandResult, SedCommand, SingleLineArgumentCommandFactory},
};

pub struct LabelCommand;

impl SedCommand for LabelCommand {
    fn execute(
        &'_ self,
        _: crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        _: &mut String,
    ) -> CommandResult<'_> {
        //no-op at runtime
        CommandResult::Nothing
    }
}

impl SingleLineArgumentCommandFactory for LabelCommand {
    fn new(
        &self,
        current_parser: &mut crate::parser::ParserState,
        argument: String,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        current_parser.define_label(argument)?;
        Ok(Box::new(LabelCommand))
    }
}
