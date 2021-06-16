//! Provides standard string manipulation and utility functions.
//!
//! See [`std_fun`](rubble-templates::std_fun) or [`string_functions`] for more info.
//!
use std::collections::HashMap;
use rubble_templates_core::evaluator::Function;
use rubble_templates_core::functions::SimpleFunction;

pub const EMPTY_STRING: &str = "";

/// Provides a set of string related functions.
///
/// Available functions:
/// * [`concat`](concat_function) - Concatenates parameters.
/// * [`trim`](trim_function) - Trims parameters.
/// * [`$}`](right_brackets_function) - Inserts "}}".
/// * [`$quote`](right_brackets_function) - Inserts double qoutes (").
pub fn string_functions() -> HashMap<String, Box<dyn Function>> {
    let mut functions: HashMap<String, Box<dyn Function>> = HashMap::new();
    functions.insert("concat".to_string(), SimpleFunction::new(concat_function));
    functions.insert("trim".to_string(), SimpleFunction::new(trim_function));
    functions.insert("$}".to_string(), SimpleFunction::new(right_brackets_function));
    functions.insert("$quote".to_string(), SimpleFunction::new(quotes_function));
    functions
}

/// Concatenates the parameters.
///
/// Eg.
/// ```text
/// concat 1 "hello" " " 3.14 "world!"
/// ```
/// Expected output:
/// ```text
/// 1hello 3.14world!
/// ```
pub fn concat_function(parameters: &[String]) -> String {
    let mut result = EMPTY_STRING.to_string();
    parameters.iter().for_each(|param| {
        result.push_str(param);
    });
    result
}

/// Trims the parameters.
/// If there is more than one parameter, then concatenates them.
///
/// Eg.
/// ```text
/// trim "  hello"
/// trim " hello " "   world" "text  "
/// ```
/// Expected output:
/// ```text
/// hello
/// helloworldtext
/// ```
pub fn trim_function(parameters: &[String]) -> String {
    let mut result = EMPTY_STRING.to_string();
    parameters.iter().for_each(|param| {
        result.push_str(param.trim());
    });
    result
}

/// Inserts "}}". Ignores the parameters.
///
/// Eg.
/// ```text
/// $}}
/// $}} " hello " "   world" "text  "
/// ```
/// Expected output:
/// ```text
/// }}
/// }}
/// ```
pub fn right_brackets_function(_: &[String]) -> String {
    "}}".to_string()
}

/// Inserts ". Ignores the parameters.
///
/// Eg.
/// ```text
/// $quote
/// $quote " hello " "   world" "text  "
/// ```
/// Expected output:
/// ```text
/// "
/// "
/// ```
pub fn quotes_function(_: &[String]) -> String {
    "\"".to_string()
}