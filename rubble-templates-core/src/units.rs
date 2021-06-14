//! Basic rubble-templates units.

use std::fmt::{Display, Formatter};

/// Represents a position of node/symbol in template.
#[derive(Clone, Debug, PartialEq)]
pub enum Position {
    /// Used when there is no way to calculate the exact or approximate position.
    Unknown,
    /// Used to indicate a position relative to current function invocation.
    RelativeToInvocation(usize),
    /// Used to indicate a position relative to the start of currently evaluated block of code.
    RelativeToCodeStart(usize),
    /// Used to indicate an absolute position in current template.
    Absolute(usize),
}

impl Position {
    pub fn raw_value(&self) -> Option<usize> {
        match self {
            Position::Unknown => None,
            Position::RelativeToInvocation(pos) => Some(pos),
            Position::RelativeToCodeStart(pos) => Some(pos),
            Position::Absolute(pos) => Some(pos),
        }.cloned()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}