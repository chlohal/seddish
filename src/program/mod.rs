mod process;

use std::{collections::BTreeMap, str::FromStr};

use crate::{address_range::AddressRange, parser::{Parser, ParserError, parsecommand::SedCommand}};

pub use process::LineEval;
pub use process::DocumentEval;
pub use process::EvalError;
pub use process::SedEffect;

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

pub struct SedProgramBlock {
    pub(crate) command: BlockType,
    pub(crate) filter: AddressRange
}

pub enum BlockType {
    SingleCommand(Box<dyn SedCommand>),
    BlockBranch(usize)
}

pub struct SedLineInfo {
    pub index: usize,
}
pub struct SedLineState {
    pub substitution_successful: bool,
}