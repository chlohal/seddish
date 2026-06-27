use regex::{Captures, Regex, Replacer};

use crate::parser::{
    ParserError,
    parsecommand::{
        CommandResult, SedCommand, SubstitutionLikeCommandFactory,
    },
};

pub struct SubstituteCommand {
    from: Regex,
    to: SubstitutionTo,
    all: bool,
}

#[derive(Debug)]
struct SubstutionNode {
    insert_at: usize,
    capture_group: usize,
}
#[derive(Debug)]
struct SubstitutionTo(String, Box<[SubstutionNode]>);

impl<'a> Replacer for &'a SubstitutionTo {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let mut last_noninserted_idx = 0;
        for snode in &self.1 {
            dst.push_str(&self.0[last_noninserted_idx..snode.insert_at]);
            if let Some(grp) = caps.get(snode.capture_group) {
                dst.push_str(grp.as_str());
            }
            last_noninserted_idx = snode.insert_at;
        }
        dst.push_str(&self.0[last_noninserted_idx..]);
    }
    fn no_expansion<'r>(&'r mut self) -> Option<std::borrow::Cow<'r, str>> {
        self.1.is_empty().then(|| std::borrow::Cow::Borrowed(self.0.as_str()))
    }
}

impl SedCommand for SubstituteCommand {
    fn execute<'a>(
        &'a self,
        linestate: crate::program::SedLineState,
        _: &crate::program::SedLineInfo,
        _: &mut String,
        pattern: &'a mut String,
    ) -> CommandResult<'a> {
        let limit = if self.all { 0 } else { 1 };

        

        let replaced = self.from.replacen(pattern.as_str(), limit, &self.to);

        //If the cow is borrowed, then the pattern hasn't changed
        if let std::borrow::Cow::Owned(pat) = replaced {
            *pattern = pat;
            *linestate.substitution_successful = true;
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

        if from.is_empty() {
            return Err(ParserError::EmptyRegularExpression);
        }

        let from = regex::RegexBuilder::new(&from)
        .case_insensitive(flags.contains('i'))
        .build().map_err(ParserError::RegexError)?;

        

        let all = flags.contains('g');

        Ok(Box::new(SubstituteCommand {
            from,
            to: parse_sed_style_replacement_string(to.as_ref()),
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
    
    fn command_name(&self) -> &'static str {
        "substitution"
    }
    
}


fn parse_sed_style_replacement_string(s: &str) -> SubstitutionTo {
    let mut src = String::with_capacity(s.len());
    let mut sub_points = Vec::new();
    let mut chars = s.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        //escaped digits become a capture group in Sed's syntax
        if c == '\\' && chars.peek().is_some_and(|(_, next)| next.is_ascii_digit()) {
            let (_, capture_group_char) = chars.next().unwrap();
            let capture_group_i = (capture_group_char as u8) - b'0';
            sub_points.push(SubstutionNode { insert_at: i, capture_group: capture_group_i as usize });
        } else {
            src.push(c);
        }
    }

    SubstitutionTo(src, sub_points.into_boxed_slice())
}