use crate::template::content::{EvaluableMixedContent, EvaluableMixedContentIterator, TemplateSlice};
use std::collections::HashMap;
use crate::evaluator::{Evaluator, EvaluationError};
use crate::template::Template;
use crate::evaluator::ast::parse_ast;

pub trait Compiler<T> {
    type Iter;
    type Item;

    fn compile<C>(&self, content: C, variables: &HashMap<String, String>) -> Result<String, CompilationError>
        where C: EvaluableMixedContent<T, Item=Self::Item, IntoIter=Self::Iter>;
}

#[derive(Debug, PartialEq)]
pub enum CompilationError {
    EvaluationFailed { description: EvaluationError, position: usize, source: String },
}

pub struct TemplateCompiler<E: Evaluator> {
    engine: E
}

impl<E> TemplateCompiler<E> where E: Evaluator {
    fn new(engine: E) -> TemplateCompiler<E> {
        TemplateCompiler {
            engine
        }
    }
}

impl<'a, E> Compiler<&'a Template> for TemplateCompiler<E> where E: Evaluator {
    type Iter = EvaluableMixedContentIterator<'a, Template>;
    type Item = TemplateSlice<'a>;

    fn compile<C>(&self, content: C, variables: &HashMap<String, String>) -> Result<String, CompilationError>
        where C: EvaluableMixedContent<&'a Template, Item=Self::Item, IntoIter=Self::Iter> {
        let mut result = String::new();

        for item in content {
            let compiled = match item {
                TemplateSlice::Text { value, .. } => value.to_string(),
                TemplateSlice::Code { value, start_position, .. } => self.engine
                    .evaluate(parse_ast(value), variables)
                    .map_err(|err| CompilationError::EvaluationFailed {
                        description: err,
                        position: start_position,
                        source: value.to_string(),
                    })?,
            };

            result.push_str(compiled.as_str());
        }

        return Ok(result);
    }
}

#[cfg(test)]
mod tests {
    use crate::template::Template;
    use crate::evaluator::engine::SimpleEvaluationEngine;
    use std::collections::HashMap;
    use crate::compiler::{TemplateCompiler, Compiler};

    #[test]
    fn should_compile_template() {
        let template = Template::from("Some template. {{ variable }} - or something".to_string());
        let engine = SimpleEvaluationEngine::from(HashMap::new());
        let compiler = TemplateCompiler::new(engine);
        let mut variables = HashMap::new();
        variables.insert("variable".to_string(), "Hello world".to_string());

        let result = compiler.compile(&template, &variables);

        assert_eq!(result, Ok("Some template. Hello world - or something".to_string()));
    }
}