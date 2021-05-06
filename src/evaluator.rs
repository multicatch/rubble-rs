use std::collections::HashMap;
use ast::SyntaxNode;

pub mod ast;
pub mod engine;

/// Trait that describes an ability to evaluate code in template.
///
/// Any struct that implements this trait should be able to interpret a given AST and evaluate it.
/// AST are not constructed by the Evaluator, they are consumed by it.
///
pub trait Evaluator {
    fn evaluate(&self, syntax_node: &SyntaxNode, variables: &HashMap<String, String>) -> Result<String, SyntaxError>;
}

/// An error that can happen during evaluation with full info about where and what happened.
/// 
/// Contains additional fields that describe EvaluationError.
/// This is a wrapper of EvaluationError that provides more info about the error so the user can see
/// where the error is and how they can fix it.
/// 
/// Unlike EvaluationError, it can be created where a much broader context is available
/// and when additional info can be supplied (eg. position of currently evaluated block).
///
#[derive(Debug, PartialEq)]
pub struct SyntaxError {
    pub relative_pos: usize,
    pub description: EvaluationError,
}

/// An error that can happen during evaluation.
///
/// Used by an Evaluator or Function to indicate that something bad happened.
/// This is a very short description of what is happening - it can be created in a much narrow context,
/// and then wrapped by an SyntaxError in a context where more info can be supplied.
/// 
/// It is intended to be used in cases when you don't want to pass all unused data just in case
/// to be able to create an SyntaxError if it happens. It contains only the most necessary
/// info about the error.
///
#[derive(Debug, PartialEq)]
pub enum EvaluationError {
    UnexpectedElements {
        last_expected: Option<SyntaxNode>,
        unexpected_elements: Vec<SyntaxNode>,
    },
    UnknownSymbol {
        symbol: String,
    },
    InvalidArguments {
        description: Option<String>,
        arguments: Vec<SyntaxNode>,
    },
}

/// A function that can be used to add features to the template.
///
/// Any struct that implements this trait adds additional features that can additional *function*
/// which will be available to anyone using given Evaluator that has it installed.
///
/// For example, you might implement custom function for date parsing etc etc.
///
/// During evaluation, an original Evaluator will be supplied to enable parameter evaluation.
/// Parameter evaluation with a supplied Evaluator is optional and a given Function can evaluate
/// them independently.
pub trait Function {
    fn evaluate(&self, evaluator: &dyn Evaluator, parameters: &[SyntaxNode], variables: &HashMap<String, String>, offset: usize) -> Result<String, SyntaxError>;
}

impl<F> Function for F where F: Fn(&dyn Evaluator, &[SyntaxNode], &HashMap<String, String>, usize) -> Result<String, SyntaxError> {
    fn evaluate(&self, evaluator: &dyn Evaluator, parameters: &[SyntaxNode], variables: &HashMap<String, String>, offset: usize) -> Result<String, SyntaxError> {
        let vec = parameters.to_vec();
        self(evaluator, &vec, variables, offset)
    }
}