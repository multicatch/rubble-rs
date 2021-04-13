use std::collections::HashMap;
use ast::SyntaxNode;

mod ast;
mod engine;

/// Trait that describes an ability to evaluate code in template.
///
/// Any struct that implements this trait should be able to interpret a given AST and evaluate it.
/// AST are not constructed by the Evaluator, they are consumed by it.
///
pub trait Evaluator {
    fn evaluate(&self, syntax_node: SyntaxNode, variables: &HashMap<String, String>) -> Result<String, EvaluationError>;
}

/// An error that can happen during evaluation.
///
/// Used by an Evaluator or Function to indicate that something bad happened.
///
#[derive(Debug, PartialEq)]
pub enum EvaluationError {
    UnexpectedElements {
        last_expected: SyntaxNode,
        unexpected_elements: Vec<SyntaxNode>,
    },
    UnknownSymbol {
        symbol: String
    },
    InvalidArgument {
        argument: SyntaxNode
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
    fn evaluate(&self, evaluator: &dyn Evaluator, parameters: &Vec<SyntaxNode>) -> Result<String, EvaluationError>;
}