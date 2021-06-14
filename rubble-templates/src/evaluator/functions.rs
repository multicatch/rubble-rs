use rubble_templates_core::evaluator::{Function, Evaluator, SyntaxError, Context};
use rubble_templates_core::ast::SyntaxNode;

/// A wrapper for a `Fn(&[String]) -> String`, to be used in [Evaluator].
///
///
/// Example:
/// ```
/// use rubble_templates_core::evaluator::{Evaluator, Function, SyntaxError};
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
    fn evaluate(&self, evaluator: &dyn Evaluator, parameters: &[SyntaxNode], context: &mut Context) -> Result<String, SyntaxError> {
        let parameters = resolve_params(evaluator, parameters, context);
        match parameters {
            Ok(parameters) => Ok((self.function)(&parameters)),
            Err(err) => Err(err)
        }
    }
}


/// A wrapper for a `Fn(&dyn Evaluator, &[String], &HashMap<String, String>) -> Result<String, SyntaxError>`, to be used in [Evaluator].
///
/// Example:
/// ```
/// use std::collections::HashMap;
/// use rubble_templates::template::Template;
/// use rubble_templates::compile_template_from_string;
/// use rubble_templates::evaluator::functions::FunctionWithContext;
/// use rubble_templates_core::evaluator::{Context, SyntaxError, Function};
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
/// let variables: HashMap<String, String> = HashMap::new();
///
/// let result = compile_template_from_string("2 + 2 = {{ plus 2 2 }}".to_string(), variables, functions);
/// assert_eq!(result.ok(), Some("2 + 2 = 4".to_string()));
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


/// A wrapper for a `Fn(&dyn Evaluator, &[SyntaxNode], &HashMap<String, String>) -> Result<String, SyntaxError>`, to be used in [Evaluator].
///
/// Example:
/// ```
/// use rubble_templates_core::evaluator::{Evaluator, Function, SyntaxError, Context};
/// use std::collections::HashMap;
/// use rubble_templates::template::Template;
/// use rubble_templates::compile_template_from_string;
/// use rubble_templates::evaluator::functions::FunctionWithAst;
/// use rubble_templates_core::ast::SyntaxNode;
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
/// let variables: HashMap<String, String> = HashMap::new();
///
/// let result = compile_template_from_string("2 + 2 = {{ plus 2 2 }}".to_string(), variables, functions);
/// assert_eq!(result.ok(), Some("2 + 2 = 4".to_string()));
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