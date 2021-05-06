use std::collections::HashMap;
use crate::evaluator::ast::SyntaxNode;
use crate::evaluator::{Function, SyntaxError, Evaluator, EvaluationError};

/// Simple evaluation engine providing basic features like variable and function evaluation.
///
/// This engine evaluates are variables, functions and literals in a code fragment and returns
/// a resulting String. It might also return an error when encounters unexpected identifier or
/// a variable with parameters. When it encounters a function, it tries to evaluate it and
/// propagates any error that it returns.
///
/// This engine always uses the same Functions for evaluations, but can be supplied with different
/// parameters during every evaluation.
pub struct SimpleEvaluationEngine {
    functions: HashMap<String, Box<dyn Function>>
}

impl SimpleEvaluationEngine {
    pub fn from(functions: HashMap<String, Box<dyn Function>>) -> SimpleEvaluationEngine {
        SimpleEvaluationEngine {
            functions
        }
    }

    fn evaluate_symbol(&self, variables: &HashMap<String, String>, identifier: &str, offset: usize, parameters: &[SyntaxNode]) -> Result<String, SyntaxError> {
        match variables.get(identifier).cloned() {
            Some(result) => Ok(result),
            None => self.evaluate_function_or_literal(identifier, offset, parameters, variables),
        }
    }

    fn evaluate_function_or_literal(&self, identifier: &str, offset: usize, parameters: &[SyntaxNode], variables: &HashMap<String, String>) -> Result<String, SyntaxError> {
        self.evaluate_function(identifier, offset, parameters, variables)
            .unwrap_or_else(|| {
                extract_literal(identifier)
                    .map(|it| it.to_string())
                    .ok_or(SyntaxError {
                        relative_pos: offset,
                        description: EvaluationError::UnknownSymbol {
                            symbol: identifier.to_string(),
                        },
                    })
            })
    }

    fn evaluate_function(&self, identifier: &str, offset: usize, parameters: &[SyntaxNode], variables: &HashMap<String, String>) -> Option<Result<String, SyntaxError>> {
        Some(self.functions.get(identifier)?.evaluate(self as &dyn Evaluator, &parameters, variables, offset))
    }
}

impl Evaluator for SimpleEvaluationEngine {
    fn evaluate(&self, syntax_node: &SyntaxNode, variables: &HashMap<String, String>) -> Result<String, SyntaxError> {
        evaluate(syntax_node, |identifier, offset, parameters|
            self.evaluate_symbol(variables, identifier, offset, parameters),
        )
    }
}

fn evaluate<E>(syntax_node: &SyntaxNode, evaluate_symbol: E) -> Result<String, SyntaxError>
    where E: Fn(&str, usize, &[SyntaxNode]) -> Result<String, SyntaxError> {
    match syntax_node {
        SyntaxNode::NamedNode { identifier, children, starts_at } =>
            evaluate_symbol(identifier.as_str(), *starts_at, children),

        SyntaxNode::AnonymousNode { children, starts_at } =>
            evaluate_nested(*starts_at, children, evaluate_symbol),
    }
}

fn evaluate_nested<E>(offset: usize, children: &[SyntaxNode], evaluate_symbol: E) -> Result<String, SyntaxError>
    where E: Fn(&str, usize, &[SyntaxNode]) -> Result<String, SyntaxError> {
    if children.is_empty() {
        return Result::Ok("".to_string());
    }

    let child = children[0].clone();
    if children.len() > 1 {
        Result::Err(SyntaxError {
            relative_pos: offset,
            description: EvaluationError::UnexpectedElements {
                last_expected: Some(child),
                unexpected_elements: children[1..].to_vec(),
            },
        })
    } else if let SyntaxNode::NamedNode { identifier, children, .. } = child {
        evaluate_symbol(identifier.as_str(), offset, children.as_slice())
    } else {
        evaluate(&child, evaluate_symbol)
    }
}

fn extract_literal(source: &str) -> Option<&str> {
    if source.parse::<f64>().is_ok() {
        Some(source)
    } else if source.starts_with('"') && source.ends_with('"') && source.len() > 2 {
        Some(&source[1..(source.len() - 1)])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::evaluator::engine::{SyntaxError, SimpleEvaluationEngine};
    use crate::evaluator::ast::SyntaxNode;
    use crate::evaluator::{Evaluator, Function, EvaluationError};

    #[test]
    fn should_evaluate_variable() {
        let mut variables = HashMap::new();
        variables.insert("variable".to_string(), "1234".to_string());
        let engine = SimpleEvaluationEngine::from(HashMap::new());

        // variable subsitution
        let result = engine.evaluate(&node_of("variable"), &variables);
        assert_eq!(result.ok(), Some("1234".to_string()));

        // number literal
        let result = engine.evaluate(&node_of("-12.2"), &variables);
        assert_eq!(result.ok(), Some("-12.2".to_string()));

        // string literal
        let result = engine.evaluate(&node_of("\"test\""), &variables);
        assert_eq!(result.ok(), Some("test".to_string()));
    }

    #[test]
    fn should_evaluate_function() {
        let variables = HashMap::new();
        let mut functions = HashMap::new();

        // using closure as function in the evaluation engine
        let function =
            |_evaluator: &dyn Evaluator, parameters: &[SyntaxNode], _variables: &HashMap<String, String>, offset: usize| {
                if let Some(SyntaxNode::NamedNode { identifier, .. }) = parameters.get(0) {
                    Result::Ok(identifier.clone())
                } else {
                    Result::Err(SyntaxError {
                        relative_pos: offset,
                        description: EvaluationError::InvalidArguments {
                            description: None,
                            arguments: parameters.to_vec(),
                        },
                    })
                }
            };

        functions.insert("our_function".to_string(), Box::new(function) as Box<dyn Function>);
        let engine = SimpleEvaluationEngine::from(functions);

        // our function call
        let node = SyntaxNode::NamedNode {
            identifier: "our_function".to_string(),
            starts_at: 0,
            children: vec![
                SyntaxNode::NamedNode {
                    identifier: "param".to_string(),
                    starts_at: 0,
                    children: vec![],
                },
            ],
        };

        // correct function call
        let result = engine.evaluate(&node, &variables);
        assert_eq!(result.ok(), Some("param".to_string()));

        // incorrect function call
        let result = engine.evaluate(&node_of("our_function"), &variables);
        assert_eq!(result.err(), Some(SyntaxError {
            relative_pos: 0,
            description: EvaluationError::InvalidArguments {
                description: None,
                arguments: vec![],
            },
        }))
    }

    #[test]
    fn should_fail_evaluation() {
        let engine = SimpleEvaluationEngine::from(HashMap::new());
        let variables = HashMap::new();

        let result = engine.evaluate(&node_of("unknown"), &variables);
        assert_eq!(result.err(), Some(SyntaxError {
            relative_pos: 0,
            description: EvaluationError::UnknownSymbol {
                symbol: "unknown".to_string()
            },
        }));
    }

    fn node_of(identifier: &str) -> SyntaxNode {
        SyntaxNode::AnonymousNode {
            starts_at: 0,
            children: vec![
                SyntaxNode::NamedNode {
                    identifier: identifier.to_string(),
                    starts_at: 0,
                    children: vec![],
                }
            ],
        }
    }
}