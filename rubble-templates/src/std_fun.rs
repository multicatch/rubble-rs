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
use crate::std_fun::math::math_functions;
use crate::std_fun::strings::string_functions;

/// Provides a set of standard functions.
///
/// This is a cumulative set of the following functions:
/// * [`math_functions`](math_functions) - All math-related functions.
/// * [`string_functions`](string_functions) - All math-related functions.
///
pub fn std_functions() -> HashMap<String, Box<dyn Function>> {
    let mut functions: HashMap<String, Box<dyn Function>> = HashMap::new();
    functions.extend(math_functions());
    functions.extend(string_functions());
    functions
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

