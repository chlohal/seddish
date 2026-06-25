mod process;

use std::{any::Any, collections::BTreeMap, str::FromStr};

use crate::{address_range::AddressRange, parser::{Parser, ParserError, parsecommand::SedCommand}};

pub use process::LineEval;
pub use process::DocumentEval;
pub use process::EvalError;
pub use process::SedEffect;

#[derive(Debug)]
pub struct SedProgram {
    pub(crate) commands: Vec<SedProgramBlock>,
    pub(crate) labels: BTreeMap<String, usize>,
}

impl FromStr for SedProgram {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Parser::new().parse(s)
    }
}

#[derive(Debug)]
pub struct SedProgramBlock {
    pub(crate) command: BlockType,
    pub(crate) filter: AddressRange
}

pub enum BlockType {
    SingleCommand(Box<dyn SedCommand>),
    BlockBranch(usize)
}

impl std::fmt::Debug for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SingleCommand(arg0) => f.debug_tuple("SingleCommand").field(&arg0.type_id()).finish(),
            Self::BlockBranch(arg0) => f.debug_tuple("BlockBranch").field(arg0).finish(),
        }
    }
}

pub struct SedLineInfo {
    pub index: usize,
}
pub struct SedLineState {
    pub substitution_successful: bool,
}