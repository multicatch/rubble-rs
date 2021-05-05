use crate::template::content::{EvaluableMixedContent, EvaluableMixedContentIterator, TemplateSlice};
use std::collections::HashMap;
use crate::evaluator::{Evaluator, SyntaxError, EvaluationError};
use crate::template::Template;
use crate::evaluator::ast::parse_ast;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Describes a struct that is able to compile a template.
///
/// Any implementation of this trait should be able to compile a template from specified input Iterator and Items.
/// For example, you may specify a custom iterator and custom items that are supported by your Compiler.
///
/// The compiler should return the resulting String compiled from all items that the iterator returned.
pub trait Compiler<T> {
    type Item;
    type ItemIterator: Iterator<Item = Self::Item>;

    fn compile<C>(&self, content: C, variables: &HashMap<String, String>) -> Result<String, CompilationError>
        where C: EvaluableMixedContent<T, Item=Self::Item, IntoIter=Self::ItemIterator>;
}

#[derive(Debug, PartialEq)]
pub enum CompilationError {
    EvaluationFailed {
        error: SyntaxError,
        position: usize,
        source: String,
    },
}

impl Error for CompilationError {}

impl Display for CompilationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct TemplateCompiler<E: Evaluator> {
    engine: E
}

impl<E> TemplateCompiler<E> where E: Evaluator {
    pub fn new(engine: E) -> TemplateCompiler<E> {
        TemplateCompiler {
            engine
        }
    }
}

impl<'a, E> Compiler<&'a Template> for TemplateCompiler<E> where E: Evaluator {
    type Item = TemplateSlice<'a>;
    type ItemIterator = EvaluableMixedContentIterator<'a, Template>;

    fn compile<C>(&self, content: C, variables: &HashMap<String, String>) -> Result<String, CompilationError>
        where C: EvaluableMixedContent<&'a Template, Item=Self::Item, IntoIter=Self::ItemIterator> {
        let mut result = String::new();

        for item in content {
            let compiled = match item {
                TemplateSlice::Text { value, .. } => value.to_string(),
                TemplateSlice::Code { value, start_position, .. } => self.engine
                    .evaluate(parse_ast(value), variables)
                    .map_err(|err| CompilationError::EvaluationFailed {
                        error: err,
                        position: start_position,
                        source: value.to_string(),
                    })?,
            };

            result.push_str(compiled.as_str());
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::template::Template;
    use crate::evaluator::engine::SimpleEvaluationEngine;
    use std::collections::HashMap;
    use crate::compiler::{TemplateCompiler, Compiler, CompilationError};
    use crate::evaluator::{SyntaxError, EvaluationError};

    #[test]
    fn should_compile_template() {
        let template = Template::from("Some simple-template. {{ variable }} - or something".to_string());
        let engine = SimpleEvaluationEngine::from(HashMap::new());
        let compiler = TemplateCompiler::new(engine);
        let mut variables = HashMap::new();
        variables.insert("variable".to_string(), "Hello world".to_string());

        let result = compiler.compile(&template, &variables);

        assert_eq!(result, Ok("Some simple-template. Hello world - or something".to_string()));
    }

    #[test]
    fn should_return_error_during_evaluation() {
        let template = Template::from("Should fail. {{ variable }}".to_string());
        let engine = SimpleEvaluationEngine::from(HashMap::new());
        let compiler = TemplateCompiler::new(engine);
        let variables = HashMap::new();

        let result = compiler.compile(&template, &variables);

        assert_eq!(result, Err(CompilationError::EvaluationFailed {
            error: SyntaxError {
                relative_pos: 1,
                description: EvaluationError::UnknownSymbol {
                    symbol: "variable".to_string()
                }
            },
            position: 13,
            source: "{{ variable }}".to_string()
        }));
    }
}