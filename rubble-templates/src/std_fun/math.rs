//! Provides standard math functions.
//!
//! See [`std_fun`](rubble-templates::std_fun) or [`math_functions`] for more info.
//!
use std::collections::HashMap;
use rubble_templates_core::evaluator::{Function, Context, SyntaxError, EvaluationError};
use rubble_templates_core::functions::{SimpleFunction, FunctionWithContext};
use std::num::ParseFloatError;
use crate::std_fun::strings::EMPTY_STRING;

/// Provides a set of math functions.
///
/// Available functions:
/// * [`+`](plus_function) - Adds the parameters or concatenates them if it fails.
/// * [`-`](minus_function) - Subtracts the parameters.
/// * [`*`](multiply_function) - Multiplies the parameters.
/// * [`/`](divide_function) - Divides the parameters.
/// * [`mod`](modulo_function) - Calculates the remainder.
pub fn math_functions() -> HashMap<String, Box<dyn Function>> {
    let mut functions: HashMap<String, Box<dyn Function>> = HashMap::new();
    functions.insert("+".to_string(), SimpleFunction::new(plus_function));
    functions.insert("-".to_string(), FunctionWithContext::new(minus_function));
    functions.insert("*".to_string(), FunctionWithContext::new(multiply_function));
    functions.insert("/".to_string(), FunctionWithContext::new(divide_function));
    functions.insert("mod".to_string(), FunctionWithContext::new(modulo_function));
    functions
}

/// Adds (or concatenates) values.
/// If any of the parameters is not convertible to a number, then the rest will be concatenated.
///
/// Eg.
/// ```text
/// + 1 2 3.3
/// + 1 2 "hello" 3.3
/// ```
/// Expected output:
/// ```text
/// 6.3
/// 3hello3.3
/// ```
pub fn plus_function(parameters: &[String]) -> String {
    let mut result: String = EMPTY_STRING.to_string();
    let mut floating_result: Option<f64> = None;

    parameters.iter().for_each(|param| {
        if result.is_empty() {
            if let Result::Ok(value) = param.parse::<f64>() {
                floating_result = Some(floating_result.unwrap_or(0 as f64) + value);
            } else {
                if let Some(number) = floating_result
                    .map(|number| number.to_string()) {
                    result += &number
                }

                result.push_str(param);
            }
        } else {
            result.push_str(param);
        };
    });

    if result.is_empty() && floating_result.is_some() {
        floating_result.map(|number| number.to_string()).unwrap()
    } else {
        result
    }
}

/// Subtracts values.
/// If any of the parameters is not convertible to a number, then an error will be emitted with the invalid value.
///
/// Eg.
/// ```text
/// - 8.3 1 1.4
/// ```
/// Expected output:
/// ```text
/// 5.9
/// ```
pub fn minus_function(parameters: &[String], _context: &mut Context) -> Result<String, SyntaxError> {
    reduce_numbers(parameters, |a, b| a - b)
}

/// Multiplies values.
/// If any of the parameters is not convertible to a number, then an error will be emitted with the invalid value.
///
/// Eg.
/// ```text
/// * 4 2 7.2
/// ```
/// Expected output:
/// ```text
/// 57.6
/// ```
pub fn multiply_function(parameters: &[String], _context: &mut Context) -> Result<String, SyntaxError> {
    reduce_numbers(parameters, |a, b| a * b)
}

/// Divides values.
/// If any of the parameters is not convertible to a number, then an error will be emitted with the invalid value.
///
/// Eg.
/// ```text
/// / 1440 8 6 4
/// ```
/// Expected output:
/// ```text
/// 7.5
/// ```
pub fn divide_function(parameters: &[String], _context: &mut Context) -> Result<String, SyntaxError> {
    reduce_numbers(parameters, |a, b| a / b)
}

/// Calculates the remainder (modulo).
/// If any of the parameters is not convertible to a number, then an error will be emitted with the invalid value.
///
/// Eg.
/// ```text
/// mod 7 4
/// ```
/// Expected output:
/// ```text
/// 3
/// ```
pub fn modulo_function(parameters: &[String], _context: &mut Context) -> Result<String, SyntaxError> {
    reduce_numbers(parameters, |a, b| a % b)
}

/// Converts parameters to f64 and applies given reduce function
pub fn reduce_numbers<F>(parameters: &[String], f: F) -> Result<String, SyntaxError>
    where F: Fn(f64, f64) -> f64 {
    let (index, numbers) = as_numbers(parameters);

    if let Result::Err(error) = numbers {
        Err(SyntaxError::new(EvaluationError::InvalidValues {
            description: Some(error.to_string()),
            values: vec![parameters[index - 1].clone()],
        })
        )
    } else {
        Ok(numbers.unwrap()
            .into_iter()
            .reduce(f)
            .unwrap_or(0 as f64)
            .to_string()
        )
    }
}

/// Converts parameters to f64 and returns index of inconvertible parameter and result of conversion
pub fn as_numbers(parameters: &[String]) -> (usize, Result<Vec<f64>, ParseFloatError>) {
    let mut index: usize = 0;
    let numbers: Result<Vec<f64>, ParseFloatError> = parameters.iter()
        .map(|number| {
            index += 1;
            number.parse::<f64>()
        })
        .collect();

    (index, numbers)
}