use crate::evaluator::{Function, Evaluator, SyntaxError};
use crate::evaluator::ast::SyntaxNode;
use std::collections::HashMap;

/// A wrapper for a `Fn(&[String]) -> String`, to be used in Evaluator.
///
///
/// Example:
/// ```
/// use rubble_templates::evaluator::{Evaluator, Function, SyntaxError};
/// use rubble_templates::evaluator::ast::SyntaxNode;
/// use std::collections::HashMap;
/// use rubble_templates::template::Template;
/// use rubble_templates::compile_template_from_string;
/// use rubble_templates::evaluator::functions::SimpleFunction;
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
///
/// let variables: HashMap<String, String> = HashMap::new();
///
/// let result = compile_template_from_string("2 + 2 = {{ plus 2 2 }}".to_string(), variables, functions);
/// assert_eq!(result.ok(), Some("2 + 2 = 4".to_string()));
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
    fn evaluate(&self, evaluator: &dyn Evaluator, parameters: &[SyntaxNode], variables: &HashMap<String, String>, offset: usize) -> Result<String, SyntaxError> {
        let parameters = resolve_params(evaluator, parameters, variables, offset);
        match parameters {
            Ok(parameters) => Ok((self.function)(&parameters)),
            Err(err) => Err(err)
        }
    }
}


/// A wrapper for a `Fn(&dyn Evaluator, &[String], &HashMap<String, String>, usize) -> Result<String, SyntaxError>`, to be used in Evaluator.
///
/// Example:
/// ```
/// use rubble_templates::evaluator::{Evaluator, Function, SyntaxError};
/// use rubble_templates::evaluator::ast::SyntaxNode;
/// use std::collections::HashMap;
/// use rubble_templates::template::Template;
/// use rubble_templates::compile_template_from_string;
/// use rubble_templates::evaluator::functions::FunctionWithEvaluator;
///
/// fn plus_function(parameters: &[String], variables: &HashMap<String, String>, _offset: usize) -> Result<String, SyntaxError> {
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
/// functions.insert("plus".to_string(), FunctionWithEvaluator::new(plus_function)); // will be treated as Box<dyn Function>
///
/// let variables: HashMap<String, String> = HashMap::new();
///
/// let result = compile_template_from_string("2 + 2 = {{ plus 2 2 }}".to_string(), variables, functions);
/// assert_eq!(result.ok(), Some("2 + 2 = 4".to_string()));
/// ```
pub struct FunctionWithEvaluator<F> where F: Fn(&[String], &HashMap<String, String>, usize) -> Result<String, SyntaxError> {
    function: F
}

impl<F> FunctionWithEvaluator<F> where F: Fn(&[String], &HashMap<String, String>, usize) -> Result<String, SyntaxError> {
    pub fn new(function: F) -> Box<FunctionWithEvaluator<F>> {
        Box::new(FunctionWithEvaluator { function })
    }
}

impl<F> Function for FunctionWithEvaluator<F> where F: Fn(&[String], &HashMap<String, String>, usize) -> Result<String, SyntaxError> {
    fn evaluate(&self, evaluator: &dyn Evaluator, parameters: &[SyntaxNode], variables: &HashMap<String, String>, offset: usize) -> Result<String, SyntaxError> {
        let parameters = resolve_params(evaluator, parameters, variables, offset);
        match parameters {
            Ok(parameters) => (self.function)(&parameters, variables, offset),
            Err(err) => Err(err)
        }
    }
}


/// A wrapper for a `Fn(&dyn Evaluator, &[SyntaxNode], &HashMap<String, String>, usize) -> Result<String, SyntaxError>`, to be used in Evaluator.
///
/// Example:
/// ```
/// use rubble_templates::evaluator::{Evaluator, Function, SyntaxError};
/// use rubble_templates::evaluator::ast::SyntaxNode;
/// use std::collections::HashMap;
/// use rubble_templates::template::Template;
/// use rubble_templates::compile_template_from_string;
/// use rubble_templates::evaluator::functions::FunctionWithAst;
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
/// functions.insert("plus".to_string(), FunctionWithAst::new(plus_function)); // will be treated as Box<dyn Function>
///
/// let variables: HashMap<String, String> = HashMap::new();
///
/// let result = compile_template_from_string("2 + 2 = {{ plus 2 2 }}".to_string(), variables, functions);
/// assert_eq!(result.ok(), Some("2 + 2 = 4".to_string()));
/// ```
pub struct FunctionWithAst<F> where F: Fn(&dyn Evaluator, &[SyntaxNode], &HashMap<String, String>, usize) -> Result<String, SyntaxError> {
    function: F
}

impl<F> FunctionWithAst<F> where F: Fn(&dyn Evaluator, &[SyntaxNode], &HashMap<String, String>, usize) -> Result<String, SyntaxError> {
    pub fn new(function: F) -> Box<FunctionWithAst<F>> {
        Box::new(FunctionWithAst { function })
    }
}

impl<F> Function for FunctionWithAst<F> where F: Fn(&dyn Evaluator, &[SyntaxNode], &HashMap<String, String>, usize) -> Result<String, SyntaxError> {
    fn evaluate(&self, evaluator: &dyn Evaluator, parameters: &[SyntaxNode], variables: &HashMap<String, String>, offset: usize) -> Result<String, SyntaxError> {
        (self.function)(evaluator, &parameters, variables, offset)
    }
}

/// Resolves a slice of `SyntaxNode`s to a `Vec` of strings.
///
/// Invokes Evaluator on each `SyntaxNode` and returns a `Result` containing a `Vec` of strings (baked parameters ready to use)
/// or a `SyntaxError` if any parameter evaluation fails.
pub fn resolve_params(evaluator: &dyn Evaluator, parameters: &[SyntaxNode], variables: &HashMap<String, String>, offset: usize) -> Result<Vec<String>, SyntaxError> {
    parameters.iter()
        .map(|parameter| {
            evaluator.evaluate(parameter, variables)
        })
        .collect::<Result<Vec<String>, SyntaxError>>()
        .map_err(|err| with_relative_pos(err, offset))
}

fn with_relative_pos(mut err: SyntaxError, offset: usize) -> SyntaxError {
    err.relative_pos += offset;
    err
}