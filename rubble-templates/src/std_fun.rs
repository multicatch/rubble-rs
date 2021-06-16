//! This module contains standard functions for rubble-templates.
//! Use [std_functions] to get a copy of map with those functions.
//!
//! ```rust
//! use std::collections::HashMap;
//! use rubble_templates::std_fun::std_functions;
//! use rubble_templates_evaluators::simple::template::Template;
//! use rubble_templates_evaluators::simple::evaluator::SimpleEvaluationEngine;
//! use rubble_templates_evaluators::simple::compiler::TemplateCompiler;
//! use rubble_templates_core::compiler::Compiler;
//! use rubble_templates_core::evaluator::Context;
//!
//! let raw_input = "2 + 3 = {{ + 2 3 }}".to_string();
//!
//! // prepare compilation evironment
//! let template = Template::from(raw_input);
//! let engine = SimpleEvaluationEngine::from(std_functions());
//! let compiler = TemplateCompiler::new(engine);
//!
//! let result = compiler.compile(&template, Context::empty());
//!
//! assert_eq!(result.unwrap(), "2 + 3 = 5".to_string());
//! ```
//!
//! See [rubble_templates_core::functions] for more info on how to implement custom [Function]s.

pub mod math;
pub mod strings;

use rubble_templates_core::evaluator::Function;
use std::collections::HashMap;
use rubble_templates_core::functions::SimpleFunction;
use crate::std_fun::math::math_functions;
use crate::std_fun::strings::EMPTY_STRING;

/// Provides a set of standard functions.
///
/// This is a cumulative set of the following functions:
/// * [`math_functions`](math_functions) - All math-related functions.
/// * and the following functions.
///
/// Available functions:
/// * [`concat`](concat_function) - Concatenates parameters.
pub fn std_functions() -> HashMap<String, Box<dyn Function>> {
    let mut functions: HashMap<String, Box<dyn Function>> = HashMap::new();
    functions.extend(math_functions());
    functions.insert("concat".to_string(), SimpleFunction::new(concat_function));
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

#[cfg(test)]
mod tests {
    use crate::compile_template_from_file;
    use std::path::PathBuf;
    use std::collections::HashMap;
    use crate::std_fun::std_functions;
    use std::fs;

    #[test]
    fn should_compile_template() {
        let expected = fs::read_to_string(PathBuf::from("test-assets/stdlib-template-expected")).unwrap();
        let file = PathBuf::from("test-assets/stdlib-template");
        let functions = std_functions();
        let variables: HashMap<String, String> = HashMap::new();

        let result = compile_template_from_file(file, variables, functions);

        assert_eq!(result.ok(), Some(expected));
    }
}

