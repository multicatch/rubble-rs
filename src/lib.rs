use std::path::PathBuf;
use std::collections::HashMap;
use crate::evaluator::Function;
use crate::template::Template;
use crate::evaluator::engine::SimpleEvaluationEngine;
use crate::compiler::{TemplateCompiler, Compiler};
use std::error::Error;

pub mod template;
pub mod evaluator;
pub mod compiler;

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
    let template = Template::read_from(&file)
        .map_err(|error| Box::new(error))?;
    let engine = SimpleEvaluationEngine::from(functions);
    let compiler = TemplateCompiler::new(engine);

    compiler.compile(&template, &variables)
        .map_err(|error| Box::new(error) as Box<dyn Error>)
}

#[cfg(test)]
mod tests {
    use crate::compile_template_from_file;
    use std::path::PathBuf;
    use std::collections::HashMap;
    use crate::evaluator::{Function, Evaluator, EvaluationError};
    use crate::evaluator::ast::SyntaxNode;

    #[test]
    fn should_compile_template() {
        let file = PathBuf::from("test-assets/complex-template");
        let mut functions: HashMap<String, Box<dyn Function>> = HashMap::new();
        functions.insert("plus".to_string(), Box::new(plus_function));

        let mut variables: HashMap<String, String> = HashMap::new();
        variables.insert("hello".to_string(), "Hello world!".to_string());

        let result = compile_template_from_file(file, variables, functions);

        assert_eq!(result.ok(), Some("Some template. Hello world!.\n\nThis shows a function evaluation usage example:\n2 + 2 = 4".to_string()));
    }

    fn plus_function(evaluator: &dyn Evaluator, parameters: &Vec<SyntaxNode>, variables: &HashMap<String, String>) -> Result<String, EvaluationError> {
        Ok(
            parameters.iter()
                .map(|node|
                    evaluator.evaluate(node.clone(), variables).unwrap().parse::<i32>().unwrap()
                )
                .sum::<i32>()
                .to_string()
        )
    }
}

