//! An API for compilers that use a selected parser and evaluator, and compile output text.

use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::evaluator::{SyntaxError, Context};
use crate::template::EvaluableMixedContent;
use crate::units::Position;

/// Describes a struct that is able to compile a template.
///
/// Any implementation of this trait should be able to compile a template from specified input Iterator and Items.
/// For example, you may specify a custom iterator and custom items that are supported by your Compiler.
///
/// The compiler should return the resulting String compiled from all items that the iterator returned.
pub trait Compiler<T> {
    /// Type of a single template part that can be compiled/evaluated/parsed.
    ///
    /// Those parts will be compiled into a template
    type Item;
    /// Iterator that can provide template parts that need to be compiled.
    type ItemIterator: Iterator<Item = Self::Item>;

    fn compile<C>(&self, content: C, context: Context) -> Result<String, CompilationError>
        where C: EvaluableMixedContent<Item=Self::Item, IntoIter=Self::ItemIterator>;
}

#[derive(Debug, PartialEq)]
pub enum CompilationError {
    EvaluationFailed {
        error: SyntaxError,
        position: Position,
        source: String,
    },
}

impl Error for CompilationError {}

impl Display for CompilationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
