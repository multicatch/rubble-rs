//! General purpose API for rubble-templates.
//!
//! Allows to quickly compile a text output of out a template.
//!
//! ### Syntax
//!
//! By default, rubble-templates while parsing a template looks for all blocks starting `{{` and ending with `}}` those are marked as an evaluation spots with code that can be evaluated by an `Evaluator`.
//!
//! Given the following example:
//! ```text
//! Hello there, {{ name }}!
//! ```
//!
//! This template contains three parts:
//! * `Hello there, ` - raw text
//! * `{{ name }}` - code
//! * `!` - raw text
//!
//! The second part will be passed to a given `Evaluator` by the `Compiler` in order to get a String output.
//! The code fragment will be substituted with the output.
//!
//! The rubble-templates library can also evaluate more sophisticated code.
//! You can pass your own functions that can enrich the template.
//!
//! This library uses Lisp-like syntax during evaluation.
//! All function calls look like the following:
//! ```text
//! function_name arg0 arg1 arg2
//! ```
//!
//! To call a `plus` function (that can be hypothetically implemented) to calculate the result of 2 + 2 you will need to write
//! ```text
//! The result is: {{ plus 2 2 }}
//! ```
//!
//! The parameters can also be grouped using parenthesis. This can be helpful in certain cases.
//! For example, given `plus` and `multiply` functions, you will need to use the following code to calculate (1 + 2) * 3:
//!
//! ```text
//! The result is: {{ multiply (plus 1 2) 3 }}
//! ```
//!
//! ### Sample
//! ```rust
//! use rubble_templates::std_fun::std_functions;
//! use std::collections::HashMap;
//!
//! use rubble_templates::compile_template_from_string;
//!
//! let template = "{{ hello }}. 2 + 2 = {{ + 2 2 }}".to_string();
//!
//! let mut variables: HashMap<String, String> = HashMap::new();
//! variables.insert("hello".to_string(), "Hello world!".to_string());
//!
//! let result = compile_template_from_string(template, variables, std_functions());
//!
//! assert_eq!(
//!     result.ok(),
//!     Some("Hello world!. 2 + 2 = 4".to_string())
//! );
//! ```
//!
//! ### Customizing
//!
//! Compilation contains three phases:
//! * `parsing` - done by the template iterator, which can extract template parts (eg. raw text, code, etc) that can be further interpreted,
//! * `evaluation` - done by the [`Evaluator`](rubble_templates_core::evaluator::Evaluator), which evaluates all code found by the iterator,
//! * `compiling` - done by the [`Compiler`](rubble_templates_core::compiler::Compiler), which uses iterator to parse the content, feeds the [`Evaluator`](rubble_templates_core::evaluator::Evaluator) and then joins everything into output text.
//!
//! You can implement your own iterators, evaluators or compilers.
//! To modify the compilation process, you just need to use your own trait implementations instead of the default ones.
//!
//! To show you how it works, here is what [`compile_template_from_string`] does in practice:
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
//! // compile_template_from_string parameters
//! let raw_input = "2 + 3 = {{ + 2 3 }}".to_string();
//! let functions = std_functions();
//! let variables = HashMap::new();
//!
//! // compilation process
//! let template = Template::from(raw_input);
//! let engine = SimpleEvaluationEngine::from(functions);
//! let compiler = TemplateCompiler::new(engine);
//!
//! // the result
//! let result = compiler.compile(&template, Context::with_variables(variables));
//!
//! assert_eq!(result.unwrap(), "2 + 3 = 5".to_string());
//! ```

use std::path::PathBuf;
use std::collections::HashMap;
use rubble_templates_core::evaluator::{Function, Context};
use std::error::Error;
use rubble_templates_core::compiler::{CompilationError, Compiler};
use rubble_templates_evaluators::simple::template::Template;
use rubble_templates_evaluators::simple::evaluator::SimpleEvaluationEngine;
use rubble_templates_evaluators::simple::compiler::TemplateCompiler;

pub mod std_fun;

/// Compiles template from file.
///
/// This function reads a file and uses supplied variables and functions to compile a template.
/// It is a quick way to get a compiled template, as it initializes Engine and Compiler with each invocation.
///
/// For some special cases consider using [SimpleEvaluationEngine], [TemplateCompiler] or other specific
/// [Evaluator](rubble_templates_core::evaluator::Evaluator) and [Compiler] traits implementations.
///
/// Template can look like the following: `Some template {{ variable }} - or something`.
/// Code that will be evaluated should be put between `{{` and `}}`.
pub fn compile_template_from_file(file: PathBuf, variables: HashMap<String, String>, functions: HashMap<String, Box<dyn Function>>) -> Result<String, Box<dyn Error>> {
    let template = Template::read_from(&file)?;

    compile_template_from(template, variables, functions)
        .map_err(|error| Box::new(error) as Box<dyn Error>)
}

/// Compiles template from String.
///
/// It creates a [Template] instance on the fly and then compiles it.
///
/// For some special cases consider using [SimpleEvaluationEngine], [TemplateCompiler] or other specific
/// [Evaluator](rubble_templates_core::evaluator::Evaluator) and [Compiler] traits implementations.
///
/// Template can look like the following: `Some template {{ variable }} - or something`.
/// Code that will be evaluated should be put between `{{` and `}}`.
pub fn compile_template_from_string(template: String, variables: HashMap<String, String>, functions: HashMap<String, Box<dyn Function>>) -> Result<String, CompilationError> {
    compile_template_from(Template::from(template), variables, functions)
}

/// Compiles template from [Template].
///
/// For some special cases consider using [SimpleEvaluationEngine], [TemplateCompiler] or other specific
/// [Evaluator](rubble_templates_core::evaluator::Evaluator) and [Compiler] traits implementations.
///
/// Template can look like the following: `Some template {{ variable }} - or something`.
/// Code that will be evaluated should be put between `{{` and `}}`.
pub fn compile_template_from(template: Template, variables: HashMap<String, String>, functions: HashMap<String, Box<dyn Function>>) -> Result<String, CompilationError> {
    let engine = SimpleEvaluationEngine::from(functions);
    let compiler = TemplateCompiler::new(engine);

    compiler.compile(&template, Context::with_variables(variables))
}

#[cfg(test)]
mod tests {
    use crate::compile_template_from_file;
    use std::path::PathBuf;
    use std::collections::HashMap;
    use rubble_templates_core::evaluator::Function;
    use rubble_templates_core::functions::SimpleFunction;

    #[test]
    fn should_compile_template() {
        let file = PathBuf::from("test-assets/complex-template");
        let mut functions: HashMap<String, Box<dyn Function>> = HashMap::new();
        functions.insert("plus".to_string(), SimpleFunction::new(plus_function));

        let mut variables: HashMap<String, String> = HashMap::new();
        variables.insert("hello".to_string(), "Hello world!".to_string());

        let result = compile_template_from_file(file, variables, functions);

        assert_eq!(result.ok(), Some("Some template. Hello world!.\n\nThis shows a function evaluation usage example:\n2 + 2 = 4".to_string()));
    }

    fn plus_function(parameters: &[String]) -> String {
        parameters.iter()
                .map(|param|
                    param.parse::<i32>().unwrap()
                )
                .sum::<i32>()
                .to_string()
    }
}

