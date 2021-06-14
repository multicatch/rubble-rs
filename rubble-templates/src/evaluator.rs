use std::collections::HashMap;
use ast::SyntaxNode;
use std::any::{Any, TypeId};
use crate::evaluator::ast::Position;

pub mod ast;
pub mod engine;
pub mod functions;

/// Trait that describes an ability to evaluate code in template.
///
/// Any struct that implements this trait should be able to interpret a given AST and evaluate it.
/// AST are not constructed by the [Evaluator], they are consumed by it.
///
pub trait Evaluator {
    fn evaluate(&self, syntax_node: &SyntaxNode, context: &mut Context) -> Result<String, SyntaxError>;
}

/// Context that is passed while evaluating an AST by an [Evaluator].
///
/// Contains all variables and states that can be shared between functions during evaluations.
/// Functions should be free to store any state that will be shared between function invocations.
///
/// Variables are stored in a map, keys are Strings and values are also Strings.
/// State is stored in a heterogeneous container, which means that is accepts any struct.
/// Structs in this store are identifier by their [TypeId].
///
pub struct Context {
    variables: HashMap<String, String>,
    states: HashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub fn empty() -> Context {
        Context {
            variables: HashMap::new(),
            states: HashMap::new(),
        }
    }

    pub fn with_variables(variables: HashMap<String, String>) -> Context {
        Context {
            variables,
            states: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: &str, value: &str) {
        self.variables.insert(name.to_string(), value.to_string());
    }

    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }

    pub fn save_state<T: Any>(&mut self, state: T) {
        let key = TypeId::of::<T>();
        self.states.insert(key, Box::new(state));
    }

    pub fn get_state<T: Any>(&self) -> Option<&T> {
        let key = TypeId::of::<T>();
        self.states.get(&key)
            .map(|it|
                it.downcast_ref::<T>()
                    .unwrap()
            )
    }
}

/// An error that can happen during evaluation with full info about where and what happened.
/// 
/// Contains additional fields that describe [EvaluationError].
/// This is a wrapper of [EvaluationError] that provides more info about the error so the user can see
/// where the error is and how they can fix it.
/// 
/// Unlike [EvaluationError], it can be created where a much broader context is available
/// and when additional info can be supplied (eg. position of currently evaluated block).
///
#[derive(Debug, PartialEq)]
pub struct SyntaxError {
    pub relative_pos: Position,
    pub invocation_pos: Position,
    pub description: EvaluationError,
}

impl SyntaxError {
    /// Creates new [SyntaxError] with given [EvaluationError].
    ///
    /// Preferred way to create [SyntaxError] when the position of error is unknown.
    /// When the relative position is known, please construct this using `at_pos`.
    pub fn new(error: EvaluationError) -> SyntaxError {
        SyntaxError {
            relative_pos: Position::Unknown,
            invocation_pos: Position::Unknown,
            description: error,
        }
    }

    /// Creates new [SyntaxError] with given [EvaluationError] at known relative position..
    pub fn at_position(position: Position, error: EvaluationError) -> SyntaxError {
        SyntaxError {
            relative_pos: position,
            invocation_pos: Position::Unknown,
            description: error
        }
    }
}

/// An error that can happen during evaluation.
///
/// Used by an [Evaluator] or [Function] to indicate that something bad happened.
/// This is a very short description of what is happening - it can be created in a much narrow context,
/// and then wrapped by an [SyntaxError] in a context where more info can be supplied.
/// 
/// It is intended to be used in cases when you don't want to pass all unused data just in case
/// to be able to create an [SyntaxError] if it happens. It contains only the most necessary
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
/// Parameter evaluation with a supplied Evaluator is optional and a given [Function] can evaluate
/// them independently.
pub trait Function {
    fn evaluate(&self, evaluator: &dyn Evaluator, parameters: &[SyntaxNode], context: &mut Context) -> Result<String, SyntaxError>;
}

/// Impl for [Function] that allows to use lambda as a function in [Evaluator].
///
/// Allows to use `Fn(&dyn Evaluator, &[SyntaxNode], &mut Context) -> Result<String, SyntaxError>` as [Function] in [Evaluator].
/// For other implementations, see [functions].
///
/// Example:
/// ```
/// use rubble_templates::evaluator::{Evaluator, Function, SyntaxError, Context};
/// use rubble_templates::evaluator::ast::SyntaxNode;
/// use rubble_templates::template::Template;
/// use rubble_templates::compile_template_from_string;
/// use std::collections::HashMap;
///
/// fn plus_function(evaluator: &dyn Evaluator, parameters: &[SyntaxNode], context: &mut Context) -> Result<String, SyntaxError> {
///     Ok(
///         parameters.iter()
///             .map(|node|
///                 evaluator.evaluate(node, context).unwrap().parse::<i32>().unwrap()
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
impl<F> Function for F where F: Fn(&dyn Evaluator, &[SyntaxNode], &mut Context) -> Result<String, SyntaxError> {
    fn evaluate(&self, evaluator: &dyn Evaluator, parameters: &[SyntaxNode], context: &mut Context) -> Result<String, SyntaxError> {
        self(evaluator, &parameters, context)
    }
}