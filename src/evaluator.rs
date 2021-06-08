use std::collections::HashMap;
use ast::SyntaxNode;

pub mod ast;
pub mod engine;
pub mod functions;

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
    InvalidValues {
        description: Option<String>,
        values: Vec<String>,
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

/// Impl for Function that allows to use lambda as a function in Evaluator
///
/// Allows to use `Fn(&dyn Evaluator, &[SyntaxNode], &HashMap<String, String>, usize) -> Result<String, SyntaxError>` as `Function` in Evaluator.
/// For other implementations, see [functions].
///
/// Example:
/// ```
/// use rubble_templates::evaluator::{Evaluator, Function, SyntaxError};
/// use rubble_templates::evaluator::ast::SyntaxNode;
/// use std::collections::HashMap;
/// use rubble_templates::template::Template;
/// use rubble_templates::compile_template_from_string;
///
/// fn plus_function(evaluator: &dyn Evaluator, parameters: &[SyntaxNode], variables: &HashMap<String, String>, _offset: usize) -> Result<String, SyntaxError> {
///     Ok(
///         parameters.iter()
///             .map(|node|
///                 evaluator.evaluate(node, variables).unwrap().parse::<i32>().unwrap()
///             )
///             .sum::<i32>()
///             .to_string()
///     )
/// }
///
/// let mut functions: HashMap<String, Box<dyn Function>> = HashMap::new();
/// functions.insert("plus".to_string(), Box::new(plus_function)); // will be treated as Box<dyn Function>
///
/// let variables: HashMap<String, String> = HashMap::new();
///
/// let result = compile_template_from_string("2 + 2 = {{ plus 2 2 }}".to_string(), variables, functions);
/// assert_eq!(result.ok(), Some("2 + 2 = 4".to_string()));
/// ```
impl<F> Function for F where F: Fn(&dyn Evaluator, &[SyntaxNode], &HashMap<String, String>, usize) -> Result<String, SyntaxError> {
    fn evaluate(&self, evaluator: &dyn Evaluator, parameters: &[SyntaxNode], variables: &HashMap<String, String>, offset: usize) -> Result<String, SyntaxError> {
        self(evaluator, &parameters, variables, offset)
    }
}