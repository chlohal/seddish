use regex::Regex;

use crate::parser::{
    ParserError,
    parsecommand::{
        CommandResult, SedCommand, SubstitutionLikeCommandFactory,
    },
};

pub struct SubstituteCommand {
    from: Regex,
    to: String,
    has_expansion_groups: bool,
    all: bool,
}

impl SedCommand for SubstituteCommand {
    fn execute<'a>(
        &'a self,
        linestate: &mut crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        pattern: &'a mut String,
    ) -> CommandResult<'a> {
        let limit = if self.all { 0 } else { 1 };

        let replaced = if self.has_expansion_groups {
            self.from.replacen(pattern.as_str(), limit, &self.to)
        } else {
            self.from
                .replacen(pattern.as_str(), limit, regex::NoExpand(&self.to))
        };

        //If the cow is borrowed, then the pattern hasn't changed
        if let std::borrow::Cow::Owned(pat) = replaced {
            *pattern = pat;
            linestate.substitution_successful = true;
        }

        CommandResult::Nothing
    }
}

pub struct SubstituteCommandFactory;

impl SubstitutionLikeCommandFactory for SubstituteCommandFactory {
    fn new(
        &self,
        _current_parser: &mut crate::parser::ParserState,
        mut arguments: Vec<String>,
        flags: String,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        if arguments.len() != 2 {
            return Err(ParserError::IncorrectArgumentCount);
        }
        let to = arguments.pop().unwrap();
        let from = arguments.pop().unwrap();
        

        let from = regex::RegexBuilder::new(&from)
        .case_insensitive(flags.contains('i'))
        .build().map_err(ParserError::RegexError)?;

        let has_expansion_groups = to.contains('$');

        let all = flags.contains('g');

        Ok(Box::new(SubstituteCommand {
            from,
            to,
            has_expansion_groups,
            all,
        }))
        
    }

    fn check_flag(&self, flag: char) -> bool {
        match flag {
            'i' | 'g' => true,
            _ => false,
        }
    }

    fn field_count(&self) -> usize {
        2
    }
}
