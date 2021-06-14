//! An API for creating custom functions that extend the features of an evaluator.
//!
//! Evaluator can be extended with functions by using custom `Function` trait implementations.
//! To make this process easier, there is `evaluator::functions` module that contains structs that can be used with static functions and lambdas:
//! * [`SimpleFunction`] - Use this when you want to implement a simple function without any side effects.
//! * [`FunctionWithContext`] - Use this when you want to use pre-evaluated parameters, but you still need variables.
//!   Side effects can cause errors indicated by SyntaxError.
//! * [`FunctionWithAst`] - Gives full access to `SyntaxNode`s of parameters and `Evaluator`.
//!   Allows evaluating additional expressions, manipulating the AST or introducing DSL (domain-specific language).
//!
//! Mind you, [`Context`] is a struct that holds variables and states that can be shared between function invocations.
//! You can use it to store some properties.
//!
//! Examples for each struct are provided in the documentation. Refer to the generated docs for more specific info.
//!
//! Basic example:
//! ```rust
//! use std::collections::HashMap;
//! use rubble_templates_core::functions::SimpleFunction;
//! use rubble_templates_core::evaluator::Function;
//!
//! fn plus_function(parameters: &[String]) -> String {
//!     parameters.iter()
//!          .map(|param|
//!              param.parse::<i32>().unwrap()
//!          )
//!          .sum::<i32>()
//!          .to_string()
//! }
//!
//! let mut functions: HashMap<String, Box<dyn Function>> = HashMap::new();
//! functions.insert("plus".to_string(), SimpleFunction::new(plus_function)); // will be treated as Box<dyn Function>
//!
//! // Now functions can be used with any Evaluator that supports custom functions.
//! // SimpleEvaluationEngine is the default and it supports such extensions.
//! ```

use crate::evaluator::{Function, Evaluator, SyntaxError, Context};
use crate::ast::SyntaxNode;

/// A wrapper for a `Fn(&[String]) -> String`, to be used in [Evaluator].
///
///
/// Example:
/// ```
/// use rubble_templates_core::evaluator::{Evaluator, Function, SyntaxError};
/// use std::collections::HashMap;
/// use rubble_templates_core::functions::SimpleFunction;
///
/// fn plus_function(parameters: &[String]) -> String {
///     parameters.iter()
///             .map(|param|
///                 param.parse::<i32>().unwrap()
///             )
///             .sum::<i32>()
///             .to_string()
/// }
///
/// let mut functions: HashMap<String, Box<dyn Function>> = HashMap::new();
/// functions.insert("plus".to_string(), SimpleFunction::new(plus_function)); // will be treated as Box<dyn Function>
/// ```
pub struct SimpleFunction<F> where F: Fn(&[String]) -> String {
    function: F
}

impl<F> SimpleFunction<F> where F: Fn(&[String]) -> String {
    pub fn new(function: F) -> Box<SimpleFunction<F>> {
        Box::new(SimpleFunction { function })
    }
}

impl<F> Function for SimpleFunction<F> where F: Fn(&[String]) -> String {
    fn evaluate(&self, evaluator: &dyn Evaluator, parameters: &[SyntaxNode], context: &mut Context) -> Result<String, SyntaxError> {
        let parameters = resolve_params(evaluator, parameters, context);
        match parameters {
            Ok(parameters) => Ok((self.function)(&parameters)),
            Err(err) => Err(err)
        }
    }
}


/// A wrapper for a `Fn(&dyn Evaluator, &[String], &mut Context) -> Result<String, SyntaxError>`, to be used in [Evaluator].
///
/// Example:
/// ```
/// use std::collections::HashMap;
/// use rubble_templates_core::evaluator::{Context, SyntaxError, Function};
/// use rubble_templates_core::functions::FunctionWithContext;
///
/// fn plus_function(parameters: &[String], _context: &mut Context) -> Result<String, SyntaxError> {
///     Ok(
///         parameters.iter()
///             .map(|param|
///                 param.parse::<i32>().unwrap()
///             )
///             .sum::<i32>()
///             .to_string()
///     )
/// }
///
/// let mut functions: HashMap<String, Box<dyn Function>> = HashMap::new();
/// functions.insert("plus".to_string(), FunctionWithContext::new(plus_function)); // will be treated as Box<dyn Function>
///
/// ```
pub struct FunctionWithContext<F> where F: Fn(&[String], &mut Context) -> Result<String, SyntaxError> {
    function: F
}

impl<F> FunctionWithContext<F> where F: Fn(&[String], &mut Context) -> Result<String, SyntaxError> {
    pub fn new(function: F) -> Box<FunctionWithContext<F>> {
        Box::new(FunctionWithContext { function })
    }
}

impl<F> Function for FunctionWithContext<F> where F: Fn(&[String], &mut Context) -> Result<String, SyntaxError> {
    fn evaluate(&self, evaluator: &dyn Evaluator, parameters: &[SyntaxNode], context: &mut Context) -> Result<String, SyntaxError> {
        let parameters = resolve_params(evaluator, parameters, context);
        match parameters {
            Ok(parameters) => (self.function)(&parameters, context),
            Err(err) => Err(err)
        }
    }
}


/// A wrapper for a `Fn(&dyn Evaluator, &[SyntaxNode], &mut Context) -> Result<String, SyntaxError>`, to be used in [Evaluator].
///
/// Example:
/// ```
/// use rubble_templates_core::evaluator::{Evaluator, Function, SyntaxError, Context};
/// use std::collections::HashMap;
/// use rubble_templates_core::ast::SyntaxNode;
/// use rubble_templates_core::functions::FunctionWithAst;
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
/// functions.insert("plus".to_string(), FunctionWithAst::new(plus_function)); // will be treated as Box<dyn Function>
///
/// ```
pub struct FunctionWithAst<F> where F: Fn(&dyn Evaluator, &[SyntaxNode], &mut Context) -> Result<String, SyntaxError> {
    function: F
}

impl<F> FunctionWithAst<F> where F: Fn(&dyn Evaluator, &[SyntaxNode], &mut Context) -> Result<String, SyntaxError> {
    pub fn new(function: F) -> Box<FunctionWithAst<F>> {
        Box::new(FunctionWithAst { function })
    }
}

impl<F> Function for FunctionWithAst<F> where F: Fn(&dyn Evaluator, &[SyntaxNode], &mut Context) -> Result<String, SyntaxError> {
    fn evaluate(&self, evaluator: &dyn Evaluator, parameters: &[SyntaxNode], context: &mut Context) -> Result<String, SyntaxError> {
        (self.function)(evaluator, &parameters, context)
    }
}

/// Resolves a slice of [SyntaxNode]s to a `Vec` of strings.
///
/// Invokes Evaluator on each [SyntaxNode] and returns a `Result` containing a `Vec` of strings (baked parameters ready to use)
/// or a [SyntaxError] if any parameter evaluation fails.
pub fn resolve_params(evaluator: &dyn Evaluator, parameters: &[SyntaxNode], context: &mut Context) -> Result<Vec<String>, SyntaxError> {
    parameters.iter()
        .map(|parameter| {
            evaluator.evaluate(parameter, context)
        })
        .collect::<Result<Vec<String>, SyntaxError>>()
}