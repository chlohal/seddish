mod parse;
pub mod parsecommand;

use std::collections::BTreeMap;

use crate::{
    address_range::AddressRange,
    commands::{
        append::AppendCommandFactory, branch::BranchCommandFactory,
        branch_if_not_sub::BranchIfNotSubSuccessfulCommandFactory,
        branch_if_sub::BranchIfSubSuccessfulCommandFactory, change::ChangeCommandFactory,
        delete::DeleteCommand, delete_first_line::DeleteFirstLineCommand,
        exchange::ExchangeCommand, get_append::GetAppendCommand, get_replace::GetReplaceCommand,
        hold_append::HoldAppendCommand, hold_replace::HoldReplaceCommand,
        insert::InsertCommandFactory, label::LabelCommand, next::NextCommand,
        next_append::NextAppendCommand, print::PrintCommand,
        print_first_line::PrintFirstLineCommand, print_line_number::PrintLineNumberCommand,
        quit::QuitCommand, quit_noprint::QuitNoPrintCommand, read_file::ReadFileCommandFactory,
        substitute::SubstituteCommandFactory, transliterate::TransliterateCommandFactory,
        write_file::WriteFileCommandFactory, write_first_line::WriteFileFirstLineCommandFactory,
    },
    parser::parsecommand::{
        MultiLineArgumentCommandFactory, NoArgumentCommandFactory,
        SingleLineArgumentCommandFactory, SubstitutionLikeCommandFactory,
    },
    program::{SedProgram, SedProgramBlock},
};

pub struct Parser {
    commands: BTreeMap<char, CommandParsingSpec>,
}

pub struct ParserState {
    labels: BTreeMap<String, usize>,
    stmts: Vec<SedProgramBlock>,
}

#[derive(Debug)]
pub enum ParserError {
    DuplicateLabel(String),
    IncorrectArgumentCount,
    InvalidArguments,
    RegexError(regex::Error),
    BadAddressNegation,
    BadNumberLiteral,
    UnexpectedEof,
    IncompleteAddressRange,
    UnknownCommand(char),
    UnmatchedBracket,
    BackslashFence,
    UnknownFlag(String, char),
    WhitespaceFence,
    EmptyRegularExpression,
}

impl ParserState {
    pub fn define_label(&mut self, label: String) -> Result<(), ParserError> {
        if self.labels.contains_key(&label) {
            return Err(ParserError::DuplicateLabel(label));
        } else {
            self.labels.insert(label, self.stmts.len());
            Ok(())
        }
    }

    fn into_program(self) -> SedProgram {
        SedProgram {
            commands: self.stmts,
            labels: self.labels,
        }
    }

    fn guard_block<E>(
        &mut self,
        filter: AddressRange,
        mut create_block: impl FnMut(&mut ParserState) -> Result<(), E>,
    ) -> Result<(), E> {
        self.stmts.push(SedProgramBlock {
            command: crate::program::BlockType::BlockBranch(0),
            filter,
        });
        let jump_idx = self.stmts.len() - 1;

        create_block(self)?;

        self.stmts[jump_idx].command = crate::program::BlockType::BlockBranch(self.stmts.len());

        Ok(())
    }

    fn push(&mut self, cmd: Box<dyn parsecommand::SedCommand>, addr: AddressRange) {
        self.stmts.push(SedProgramBlock {
            command: crate::program::BlockType::SingleCommand(cmd),
            filter: addr,
        });
    }
}

macro_rules! cmd_list {
    ( $( $ch:literal => $case:ident $fac:expr  $(,)?)* ) => {
        [$(
            ($ch, CommandParsingSpec::$case(Box::new($fac))),
        )*]
    };
}

macro_rules! command_add_replace_method {
    (
        fn+ $fn_name:ident impl $enum_name:ident for $trait_name:ident;
    ) => {
        pub fn $fn_name<C: $trait_name>(
            &mut self,
            command_letter: char,
            command: C,
        ) -> Result<(), C> {
            if self.commands.contains_key(&command_letter) {
                return Err(command);
            } else {
                debug_assert!(
                    self.commands
                        .insert(
                            command_letter,
                            CommandParsingSpec::$enum_name(Box::new(command))
                        )
                        .is_none()
                );
                Ok(())
            }
        }
    };
    (
        fn= $fn_name:ident impl $enum_name:ident for $trait_name:ident;
    ) => {
        pub fn $fn_name<C: $trait_name>(
            &mut self,
            command_letter: char,
            command: C,
        ) -> Result<(), C> {
            if !self.commands.contains_key(&command_letter) {
                return Err(command);
            } else {
                debug_assert!(
                    self.commands
                        .insert(
                            command_letter,
                            CommandParsingSpec::$enum_name(Box::new(command))
                        )
                        .is_some()
                );
                Ok(())
            }
        }
    };
}

impl Parser {
    pub fn empty() -> Self {
        Self {
            commands: BTreeMap::new(),
        }
    }
    pub fn new() -> Self {
        Self::empty().with_commands(
            cmd_list!(
                ':' => SingleLineArgument LabelCommand,
                '=' => NoArgument PrintLineNumberCommand,
                'a' => MultilineArgument AppendCommandFactory,
                'i' => MultilineArgument InsertCommandFactory,
                'q' => NoArgument QuitCommand,
                'Q' => NoArgument QuitNoPrintCommand,
                'r' => SingleLineArgument ReadFileCommandFactory,
                'b' => SingleLineArgument BranchCommandFactory,
                'c' => MultilineArgument ChangeCommandFactory,
                'd' => NoArgument DeleteCommand,
                'D' => NoArgument DeleteFirstLineCommand,
                'h' => NoArgument HoldReplaceCommand,
                'H' => NoArgument HoldAppendCommand,
                'g' => NoArgument GetReplaceCommand,
                'G' => NoArgument GetAppendCommand,
                'n' => NoArgument NextCommand,
                'N' => NoArgument NextAppendCommand,
                'p' => NoArgument PrintCommand,
                'P' => NoArgument PrintFirstLineCommand,
                's' => SubstitutionLike SubstituteCommandFactory,
                't' => SingleLineArgument BranchIfSubSuccessfulCommandFactory,
                'T' => SingleLineArgument BranchIfNotSubSuccessfulCommandFactory,
                'w' => SingleLineArgument WriteFileCommandFactory,
                'W' => SingleLineArgument WriteFileFirstLineCommandFactory,
                'x' => NoArgument ExchangeCommand,
                'y' => SubstitutionLike TransliterateCommandFactory,
            )
            .into_iter(),
        )
    }

    fn with_commands(mut self, commands: impl Iterator<Item = (char, CommandParsingSpec)>) -> Self {
        self.commands = commands.collect();
        self
    }

    command_add_replace_method! { fn+ add_substitution_command impl SubstitutionLike for SubstitutionLikeCommandFactory; }
    command_add_replace_method! { fn= replace_substitution_command impl SubstitutionLike for SubstitutionLikeCommandFactory; }

    command_add_replace_method! { fn+ add_single_line_command impl SingleLineArgument for SingleLineArgumentCommandFactory; }
    command_add_replace_method! { fn= replace_single_line_command impl SingleLineArgument for SingleLineArgumentCommandFactory; }

    command_add_replace_method! { fn+ add_multi_line_command impl MultilineArgument for MultiLineArgumentCommandFactory; }
    command_add_replace_method! { fn= replace_multi_line_command impl MultilineArgument for MultiLineArgumentCommandFactory; }

    command_add_replace_method! { fn+ add_no_argument_command impl NoArgument for NoArgumentCommandFactory; }
    command_add_replace_method! { fn= replace_no_argument_command impl NoArgument for NoArgumentCommandFactory; }
}

pub(self) enum CommandParsingSpec {
    SubstitutionLike(Box<dyn SubstitutionLikeCommandFactory>),
    SingleLineArgument(Box<dyn SingleLineArgumentCommandFactory>),
    MultilineArgument(Box<dyn MultiLineArgumentCommandFactory>),
    NoArgument(Box<dyn NoArgumentCommandFactory>),
}
