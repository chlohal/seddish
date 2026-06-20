use std::{collections::BTreeMap, iter::Peekable};

use regex::Regex;

use crate::{
    address_range::{Address, AddressRange},
    parser::{
        CommandParsingSpec, Parser, ParserError, ParserState,
        parsecommand::{SedCommand, SubstitutionLikeCommandFactory},
    },
    program::{BlockType, SedProgram, SedProgramBlock},
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
        current_stmt_count: 0,
    };
    let mut script = vec![];
    parse_block(chars, &mut script, commands, &mut parser_state)?;

    if !chars.peek().is_none() {
        return Err(ParserError::UnmatchedBracket);
    }

    Ok(SedProgram {
        commands: script,
        labels: parser_state.labels,
    })
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

fn parse_string_with_fence(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    fence: char,
) -> Result<String, ParserError> {
    let mut src = String::new();
    while let Some(s) = chars.next_if(|c| *c != fence) {
        src.push(s);
    }
    let end_fence = chars.next();
    if end_fence.is_none() {
        return Err(ParserError::UnexpectedEof);
    }

    debug_assert_eq!(end_fence, Some(fence));
    Ok(src)
}

fn parse_regex_with_fences(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    fence: char,
) -> Result<Regex, ParserError> {
    Regex::new(&parse_string_with_fence(chars, fence)?).map_err(ParserError::RegexError)
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
    script: &mut Vec<SedProgramBlock>,
    commands: &BTreeMap<char, CommandParsingSpec>,
    parse_state: &mut ParserState,
) -> Result<(), ParserError> {
    loop {
        skip_ws_semicolons(chars);
        if chars.peek().is_none() {
            break;
        }
        let addr = parse_address_range(chars)?;

        match chars.peek() {
            Some('{') => {
                chars.next();

                //if there's some kind of filter on it, then ensure it's preserved by inserting an
                //inverted jump that we'll fixup to point to the end of the block after.
                //If it is All, then no need to have a guard at all, since it's semantically equivalent
                //to a bunch of unblocked commands.
                let mut guard_jump_index = None;
                if !matches!(addr, AddressRange::All) {
                    guard_jump_index = Some(commands.len());
                    script.push(SedProgramBlock {
                        command: BlockType::BlockBranch(0),
                        filter: addr.inverted().unwrap(),
                    });
                }

                parse_block(chars, script, commands, parse_state)?;

                if let Some(guard_jump_index) = guard_jump_index {
                    script[guard_jump_index].command = BlockType::BlockBranch(commands.len());
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

                script.push(SedProgramBlock {
                    command: BlockType::SingleCommand(cmd),
                    filter: addr,
                });
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
        fields.push(parse_string_with_fence(chars, fence)?);
    }

    while let Some(flag) = chars.peek() {
        if fac.check_flag(*flag) {
            flags.push(*flag);
            chars.next();
        } else {
            break;
        }
    }

    fac.new(parse_state, fields, flags)
}

fn parse_singleline_string(chars: &mut Peekable<impl Iterator<Item = char>>) -> String {
    chars.next_if_eq(&' ');

    let mut s = String::new();
    while let Some(c) = chars.next_if(|c| *c != '\n' && *c != ';') {
        s.push(c);
    }
    s
}

fn parse_multiline_string(chars: &mut Peekable<impl Iterator<Item = char>>) -> String {
    //swallow the heading characters
    if chars.next_if_eq(&'\\').is_some() && chars.next_if_eq(&' ').is_none() {
        chars.next_if_eq(&'\n');
    };

    let mut s = String::new();
    loop {
        match chars.next() {
            None => return s,
            Some('\n') => {
                // Check for backslash continuation
                if s.ends_with('\\') {
                    s.pop();
                    s.push('\n');
                } else {
                    return s;
                }
            }
            Some(c) => {
                s.push(c);
            }
        }
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
