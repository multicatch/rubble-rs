use std::path::PathBuf;
use std::collections::HashMap;
use crate::evaluator::Function;
use crate::template::Template;
use crate::evaluator::engine::SimpleEvaluationEngine;
use crate::compiler::{TemplateCompiler, Compiler, CompilationError};
use std::error::Error;

pub mod template;
pub mod evaluator;
pub mod compiler;
pub mod functions;

/// Compiles template from file.
///
/// This function reads a file and uses supplied variables and functions to compile a template.
/// It is a quick way to get a compiled template, as it initializes Engine and Compiler with each invocation.
///
/// For some special cases consider using SimpleEvaluationEngine, TemplateCompiler or other specific
/// Evaluator and Compiler traits implementations.
///
/// Template can look like the following: `Some template {{ variable }} - or something`
/// Code that will be evaluated should be put between `{{` and `}}`.
pub fn compile_template_from_file(file: PathBuf, variables: HashMap<String, String>, functions: HashMap<String, Box<dyn Function>>) -> Result<String, Box<dyn Error>> {
    let template = Template::read_from(&file)?;

    compile_template_from(template, variables, functions)
        .map_err(|error| Box::new(error) as Box<dyn Error>)
}

/// Compiles template from String.
///
/// It creates a Template instance on the fly and then compiles it.
///
/// For some special cases consider using SimpleEvaluationEngine, TemplateCompiler or other specific
/// Evaluator and Compiler traits implementations.
///
/// Template can look like the following: `Some template {{ variable }} - or something`
/// Code that will be evaluated should be put between `{{` and `}}`.
pub fn compile_template_from_string(template: String, variables: HashMap<String, String>, functions: HashMap<String, Box<dyn Function>>) -> Result<String, CompilationError> {
    compile_template_from(Template::from(template), variables, functions)
}

/// Compiles template from Template.
///
/// For some special cases consider using SimpleEvaluationEngine, TemplateCompiler or other specific
/// Evaluator and Compiler traits implementations.
///
/// Template can look like the following: `Some template {{ variable }} - or something`
/// Code that will be evaluated should be put between `{{` and `}}`.
pub fn compile_template_from(template: Template, variables: HashMap<String, String>, functions: HashMap<String, Box<dyn Function>>) -> Result<String, CompilationError> {
    let engine = SimpleEvaluationEngine::from(functions);
    let compiler = TemplateCompiler::new(engine);

    compiler.compile(&template, &variables)
}

#[cfg(test)]
mod tests {
    use crate::compile_template_from_file;
    use std::path::PathBuf;
    use std::collections::HashMap;
    use crate::evaluator::Function;
    use crate::evaluator::functions::SimpleFunction;

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

