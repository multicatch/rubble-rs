//! Compiler for [`Template`](crate::simple::template::Template), evaluates code blocks and joins everything

use rubble_templates_core::evaluator::{Context, Evaluator};
use rubble_templates_core::template::{EvaluableMixedContent, TemplateSlice};
use rubble_templates_core::compiler::{CompilationError, Compiler};
use rubble_templates_core::units::Position;
use crate::simple::template::{Template, EvaluableMixedContentIterator, START_PATTERN, END_PATTERN};
use crate::parser::parse_ast;

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

    fn compile<C>(&self, content: C, context: Context) -> Result<String, CompilationError>
        where C: EvaluableMixedContent<Item=Self::Item, IntoIter=Self::ItemIterator> {
        let mut result = String::new();

        let mut context = context;
        let context = &mut context;

        for item in content {
            let compiled = match item {
                TemplateSlice::Text { value, .. } => value.to_string(),
                TemplateSlice::Code { value, start_position, .. } => self.engine
                    .evaluate(&parse_ast(value, START_PATTERN, END_PATTERN), context)
                    .map_err(|err| CompilationError::EvaluationFailed {
                        error: err,
                        position: Position::Absolute(start_position),
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
    use std::collections::HashMap;
    use rubble_templates_core::evaluator::{Context, SyntaxError, EvaluationError};
    use rubble_templates_core::units::Position;
    use crate::simple::template::Template;
    use crate::simple::evaluator::SimpleEvaluationEngine;
    use crate::simple::compiler::TemplateCompiler;
    use rubble_templates_core::compiler::{Compiler, CompilationError};

    #[test]
    fn should_compile_template() {
        let template = Template::from("Some simple-template. {{ variable }} - or something".to_string());
        let engine = SimpleEvaluationEngine::from(HashMap::new());
        let compiler = TemplateCompiler::new(engine);
        let mut variables = HashMap::new();
        variables.insert("variable".to_string(), "Hello world".to_string());

        let result = compiler.compile(&template, Context::with_variables(variables));

        assert_eq!(result, Ok("Some simple-template. Hello world - or something".to_string()));
    }

    #[test]
    fn should_return_error_during_evaluation() {
        let template = Template::from("Should fail. {{ variable }}".to_string());
        let engine = SimpleEvaluationEngine::from(HashMap::new());
        let compiler = TemplateCompiler::new(engine);

        let result = compiler.compile(&template, Context::empty());

        assert_eq!(result, Err(CompilationError::EvaluationFailed {
            error: SyntaxError::at_position(Position::RelativeToCodeStart(1), EvaluationError::UnknownSymbol {
                symbol: "variable".to_string()
            }
            ),
            position: Position::Absolute(13),
            source: "{{ variable }}".to_string()
        }));
    }
}