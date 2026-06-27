use std::{collections::BTreeMap, iter::Peekable};

use regex::Regex;

use crate::{
    address_range::{Address, AddressRange},
    parser::{
        CommandParsingSpec, Parser, ParserError, ParserState,
        parsecommand::{SedCommand, SubstitutionLikeCommandFactory},
    },
    program::SedProgram,
};

impl Parser {
    pub fn parse(&self, src: impl AsRef<str>) -> Result<SedProgram, ParserError> {
        let mut reader = src.as_ref().chars().peekable();
        parse_script(&mut reader, &self.commands)
    }
}

fn parse_script(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    commands: &BTreeMap<char, CommandParsingSpec>,
) -> Result<SedProgram, ParserError> {
    let mut parser_state = ParserState {
        labels: Default::default(),
        stmts: vec![],
    };
    parse_block(chars, commands, &mut parser_state)?;

    if !chars.peek().is_none() {
        return Err(ParserError::UnmatchedBracket);
    }

    Ok(parser_state.into_program())
}

fn parse_address_range(
    chars: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<AddressRange, ParserError> {
    let Some(addr) = parse_address(chars)? else {
        return Ok(AddressRange::All);
    };

    if chars.next_if_eq(&',').is_none() {
        let negated = chars.next_if_eq(&'!').is_some();

        return Ok(AddressRange::Single { addr, negated });
    }

    let Some(addr_end) = parse_address(chars)? else {
        return Err(ParserError::IncompleteAddressRange);
    };

    let negated = chars.next_if_eq(&'!').is_some();

    Ok(AddressRange::Range {
        start: addr,
        end: addr_end,
        negated,
    })
}

fn parse_address(
    chars: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<Option<Address>, ParserError> {
    match chars.peek() {
        Some(c) if c.is_ascii_digit() => {
            let n = parse_usize(chars)?;
            if chars.next_if_eq(&'~').is_some() {
                let step = parse_usize(chars)?;
                Ok(Some(Address::Step { first: n, step }))
            } else {
                Ok(Some(Address::LineNumber(n)))
            }
        }
        Some('$') => {
            chars.next();
            Ok(Some(Address::Last))
        }
        Some('/') => {
            chars.next();
            Ok(Some(Address::Regex(parse_regex_with_fences(chars, '/')?)))
        }
        Some('\\') => {
            chars.next();
            let delim = chars.next().ok_or_else(|| ParserError::UnexpectedEof)?;
            Ok(Some(Address::Regex(parse_regex_with_fences(chars, delim)?)))
        }
        _ => Ok(None),
    }
}

/// Parses a string, understanding escape sequences and translating them, until
/// reaching a character for which is_end(c) returns true. The end character _will_
/// be consumed. 
/// 
/// The first character for which is_end(c) returns true is NOT NECESSARILY where 
/// this function will return: if the character was escaped, then it will continue
/// regardless.
/// 
/// If & only if EOF is reached before is_end returns true, then it will
/// return Err((src, ParserError::UnexpectedEOF)), where src contains the entire
/// content up to that point.
/// 
fn parse_string_until_end(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    mut is_end: impl FnMut(char) -> bool,
) -> Result<String, (String, ParserError)> {
    let mut src = String::new();
    let mut escaped = false;
    
    loop {
        match chars.next() {
            //Sed doesn't like newlines in regexes, so escaping them is valid.
            Some('n' | '\n') if escaped => {
                src.push('\n');
            }
            Some('r') if escaped => {
                src.push('\r');
            }
            Some('t') if escaped => {
                src.push('\t');
            }
            Some(c) if escaped && is_end(c) => {
                src.push(c);
            }
            Some(c) if escaped => {
                //In 99% of cases, we want to make the underlying command aware that
                //there was a backslash here in case they want to use it for some kind of
                //syntax. We will ALWAYS put the backslash in the source code UNLESS it was escaping a
                //special character (\n, \r, or \t); or if it was escaping a fence: all of these cases
                //are covered above, so if this is reached, ALWAYS include the backslash.
                src.push('\\');
                src.push(c);
            }
            Some(c) if is_end(c) => {
                break;
            }
            Some('\\') => {
                escaped = true;
                continue;
            }
            Some(c) => {
                src.push(c);
            }
            None => {
                return Err((src, ParserError::UnexpectedEof));
            }
        }
        escaped = false;
    }

    Ok(src)
}

fn parse_substitution_style_argument(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    fence: char,
) -> Result<String, ParserError> {
    if fence == '\\' {
        return Err(ParserError::BackslashFence);
    }
    if fence.is_ascii_whitespace() {
        return Err(ParserError::WhitespaceFence);
    }
    return parse_string_until_end(chars, |c| c == fence).map_err(|(_, e)| e)
}

fn parse_regex_with_fences(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    fence: char,
) -> Result<Regex, ParserError> {
    let src = parse_substitution_style_argument(chars, fence)?;
    if src.is_empty() {
        return Err(ParserError::EmptyRegularExpression);
    }
    Regex::new(&src).map_err(ParserError::RegexError)
}

fn parse_usize(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<usize, ParserError> {
    if !chars.peek().is_some_and(char::is_ascii_digit) {
        return Err(ParserError::BadNumberLiteral);
    }

    let mut n = 0;
    while let Some(digit) = chars.next_if(char::is_ascii_digit) {
        n *= 10;
        n += (digit as u8 - b'0') as usize;
    }

    Ok(n)
}

fn parse_block(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    commands: &BTreeMap<char, CommandParsingSpec>,
    parse_state: &mut ParserState,
) -> Result<(), ParserError> {
    loop {
        skip_ws_semicolons(chars);
        if chars.peek().is_none() {
            break;
        }
        let addr = parse_address_range(chars)?;

        skip_inline_ws(chars);

        match chars.peek() {
            Some('{') => {
                chars.next();

                //if there's some kind of filter on it, then ensure it's preserved by inserting an
                //inverted jump that we'll fixup to point to the end of the block after.
                //If it is All, then no need to have a guard at all, since it's semantically equivalent
                //to a bunch of unblocked commands.
                if matches!(addr, AddressRange::All) {
                    parse_block(chars, commands, parse_state)?;
                } else {
                    parse_state.guard_block(addr.inverted().unwrap(), |parse_state| {
                        parse_block(chars, commands, parse_state)
                    })?;
                }

                if chars.next_if_eq(&'}').is_none() {
                    return Err(ParserError::UnexpectedEof);
                }
            }
            Some('#') => skip_line(chars),
            Some('}') => break,
            Some(c) if commands.contains_key(c) => {
                let cmd = match commands.get(&chars.next().unwrap()).unwrap() {
                    CommandParsingSpec::SubstitutionLike(fac) => {
                        parse_substitution(fac, chars, parse_state)?
                    }
                    CommandParsingSpec::SingleLineArgument(fac) => {
                        fac.new(parse_state, parse_singleline_string(chars))?
                    }
                    CommandParsingSpec::MultilineArgument(fac) => {
                        fac.new(parse_state, parse_multiline_string(chars))?
                    }
                    CommandParsingSpec::NoArgument(fac) => fac.new(parse_state)?,
                };

                parse_state.push(cmd, addr);
            }
            Some(c) => return Err(ParserError::UnknownCommand(*c)),
            None => break,
        }
    }
    Ok(())
}

fn parse_substitution(
    fac: &Box<dyn SubstitutionLikeCommandFactory>,
    chars: &mut Peekable<impl Iterator<Item = char>>,
    parse_state: &mut ParserState,
) -> Result<Box<dyn SedCommand>, ParserError> {
    let fence = chars.next().ok_or(ParserError::UnexpectedEof)?;

    let mut flags = String::new();

    let argc = fac.field_count();

    let mut fields = vec![];

    for _ in 0..argc {
        fields.push(parse_substitution_style_argument(chars, fence)?);
    }

    while let Some(flag) = chars.peek() {
        if *flag == ';' || flag.is_ascii_whitespace() {
            break;
        } else if fac.check_flag(*flag) {
            flags.push(*flag);
            chars.next();
        } else {
            return Err(ParserError::UnknownFlag(
                fac.command_name().to_string(),
                *flag,
            ));
        }
    }

    fac.new(parse_state, fields, flags)
}

fn parse_singleline_string(chars: &mut Peekable<impl Iterator<Item = char>>) -> String {
    skip_inline_ws(chars);

    match parse_string_until_end(chars, |c| c == '\n' || c == ';' || c == '}') {
        Ok(s) => s,
        Err((s, _)) => s,
    }
}

fn parse_multiline_string(chars: &mut Peekable<impl Iterator<Item = char>>) -> String {
    //swallow the heading characters
    skip_inline_ws(chars);

    match parse_string_until_end(chars, |c| c == '\n') {
        Ok(s) => s,
        Err((s, _)) => s,
    }
}

fn skip_ws_semicolons(chars: &mut Peekable<impl Iterator<Item = char>>) {
    loop {
        match chars.peek() {
            Some(' ' | '\t' | '\n' | '\r' | ';') => {
                chars.next();
            }
            Some('#') => skip_line(chars),
            _ => break,
        }
    }
}

fn skip_inline_ws(chars: &mut Peekable<impl Iterator<Item = char>>) {
    loop {
        match chars.peek() {
            Some(' ' | '\t' | '\n' | '\r') => {
                chars.next();
            }
            _ => break,
        }
    }
}

fn skip_line(chars: &mut Peekable<impl Iterator<Item = char>>) {
    loop {
        match chars.peek() {
            Some('\n') => {
                chars.next();
            }
            _ => break,
        }
    }
}
