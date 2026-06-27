use crate::parser::{
    ParserError,
    parsecommand::{CommandResult, SedCommand, SubstitutionLikeCommandFactory},
};

pub struct TransliterateCommand {
    map: Vec<(char, char)>,
    all_ascii: bool,
}

impl SedCommand for TransliterateCommand {
    fn execute<'a>(
        &'a self,
        _: crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        pattern: &'a mut String,
    ) -> CommandResult<'a> {
        if self.all_ascii {
            //we can work on the bytes if all characters addressed are ASCII
            unsafe {
                for c in pattern.as_bytes_mut() {
                    if let Some((_, t)) = self.map.iter().find(|(f, _)| *f as u8 == *c) {
                        *c = *t as u8;
                    }
                }
            }
        } else {
            *pattern = pattern
                .chars()
                .map(|c| {
                    if let Some((_, t)) = self.map.iter().find(|(f, _)| *f == c) {
                        *t
                    } else {
                        c
                    }
                })
                .collect();
        }

        CommandResult::Nothing
    }
}

pub struct TransliterateCommandFactory;

impl SubstitutionLikeCommandFactory for TransliterateCommandFactory {
    fn new(
        &self,
        _current_parser: &mut crate::parser::ParserState,
        mut arguments: Vec<String>,
        _flags: String,
    ) -> Result<Box<dyn SedCommand>, ParserError> {
        if arguments.len() != 2 {
            return Err(ParserError::IncorrectArgumentCount);
        }

        if arguments[0].len() != arguments[1].len() {
            return Err(ParserError::InvalidArguments);
        }

        let to = arguments.pop().unwrap();
        let from = arguments.pop().unwrap();

        let all_ascii = to.is_ascii() && from.is_ascii();

        Ok(Box::new(TransliterateCommand {
            all_ascii,
            map: from.chars().zip(to.chars()).collect(),
        }))
    }

    fn check_flag(&self, _flag: char) -> bool {
        false
    }

    fn field_count(&self) -> usize {
        2
    }

    fn command_name(&self) -> &'static str {
        "transliteration"
    }
}
