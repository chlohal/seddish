use regex::Regex;

#[derive(Debug)]
pub enum AddressRange {
    All,
    Single {
        addr: Address,
        negated: bool,
    },
    Range {
        start: Address,
        end: Address,
        negated: bool,
    },
}
impl AddressRange {
    pub(crate) fn inverted(mut self) -> Option<AddressRange> {
        match &mut self {
            AddressRange::All => return None,
            AddressRange::Single { negated, .. }
            | AddressRange::Range { negated, .. } => *negated = !*negated,
        }
        Some(self)
    }
}

#[derive(Debug)]
pub enum Address {
    LineNumber(usize),
    Last,
    Regex(Regex),
    Step { first: usize, step: usize },
}
impl Address {
    pub fn matches(&self, line_index: usize, pattern: &str, is_last: bool) -> bool {
        match self {
            Address::LineNumber(l) => *l == line_index,
            Address::Last => is_last,
            Address::Regex(regex) => regex.is_match(pattern),
            Address::Step { first, step } => {
                ((line_index).saturating_sub(*first) % step) == 0
            },
        }
    }
}
