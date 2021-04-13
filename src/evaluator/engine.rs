use std::collections::HashMap;
use crate::evaluator::ast::SyntaxNode;
use crate::evaluator::{Function, EvaluationError, Evaluator};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::evaluator::engine::{EvaluationError, SimpleEvaluationEngine};
    use crate::evaluator::ast::SyntaxNode;
    use crate::evaluator::Evaluator;

    #[test]
    fn should_evaluate_variable() {
        let mut variables = HashMap::new();
        variables.insert("variable".to_string(), "1234".to_string());
        let engine = SimpleEvaluationEngine::from(HashMap::new());

        // variable subsitution
        let result = engine.evaluate(node_of("variable"), &variables);
        assert_eq!(result.ok(), Some("1234".to_string()));

        // number literal
        let result = engine.evaluate(node_of("-12.2"), &variables);
        assert_eq!(result.ok(), Some("-12.2".to_string()));

        // string literal
        let result = engine.evaluate(node_of("\"test\""), &variables);
        assert_eq!(result.ok(), Some("test".to_string()));
    }

    #[test]
    fn should_fail_evaluation() {
        let engine = SimpleEvaluationEngine::from(HashMap::new());
        let variables = HashMap::new();

        let result = engine.evaluate(node_of("unknown"), &variables);
        assert_eq!(result.err(), Some(EvaluationError::UnknownSymbol {
            symbol: "unknown".to_string()
        }));
    }

    fn node_of(identifier: &str) -> SyntaxNode {
        SyntaxNode::AnonymousNode {
            children: vec![
                SyntaxNode::NamedNode {
                    identifier: identifier.to_string(),
                    children: vec![],
                }
            ],
        }
    }
}

pub struct SimpleEvaluationEngine {
    functions: HashMap<String, Box<dyn Function>>
}

impl SimpleEvaluationEngine {
    pub fn from(functions: HashMap<String, Box<dyn Function>>) -> SimpleEvaluationEngine {
        SimpleEvaluationEngine {
            functions
        }
    }

    fn evaluate_symbol(&self, variables: &HashMap<String, String>, identifier: &str, parameters: Vec<SyntaxNode>) -> Result<String, EvaluationError> {
        match variables.get(identifier).cloned() {
            Some(result) => Ok(result),
            None => self.evaluate_function_or_literal(identifier, parameters),
        }
    }

    fn evaluate_function_or_literal(&self, identifier: &str, parameters: Vec<SyntaxNode>) -> Result<String, EvaluationError> {
        self.evaluate_function(identifier, parameters)
            .unwrap_or_else(|| {
                extract_literal(identifier)
                    .map(|it| it.to_string())
                    .ok_or(EvaluationError::UnknownSymbol {
                        symbol: identifier.to_string(),
                    })
            })
    }

    fn evaluate_function(&self, identifier: &str, parameters: Vec<SyntaxNode>) -> Option<Result<String, EvaluationError>> {
        Some(self.functions.get(identifier)?.evaluate(self as &dyn Evaluator, &parameters))
    }
}

impl Evaluator for SimpleEvaluationEngine {
    fn evaluate(&self, syntax_node: SyntaxNode, variables: &HashMap<String, String>) -> Result<String, EvaluationError> {
        evaluate(syntax_node, |identifier, parameters|
            self.evaluate_symbol(variables, identifier, parameters)
        )
    }
}

fn evaluate<E>(syntax_node: SyntaxNode, evaluate_symbol: E) -> Result<String, EvaluationError>
    where E: Fn(&str, Vec<SyntaxNode>) -> Result<String, EvaluationError> {
    match syntax_node {
        SyntaxNode::NamedNode { identifier, children } =>
            evaluate_symbol(identifier.as_str(), children),

        SyntaxNode::AnonymousNode { children } =>
            evaluate_nested(children, evaluate_symbol),
    }
}

fn evaluate_nested<E>(children: Vec<SyntaxNode>, evaluate_symbol: E) -> Result<String, EvaluationError>
    where E: Fn(&str, Vec<SyntaxNode>) -> Result<String, EvaluationError> {
    let children = children.as_slice();
    if children.is_empty() {
        return Result::Ok("".to_string());
    }

    let child = children[0].clone();
    if children.len() > 1 {
        Result::Err(EvaluationError::UnexpectedElements {
            last_expected: child,
            unexpected_elements: children[1..].to_vec(),
        })
    } else {
        if let SyntaxNode::NamedNode { identifier, children } = child {
            evaluate_symbol(identifier.as_str(), children)
        } else {
            evaluate(child, evaluate_symbol)
        }
    }
}

fn extract_literal(source: &str) -> Option<&str> {
    if source.parse::<f64>().is_ok() {
        Some(source)
    } else if source.starts_with("\"") && source.ends_with("\"") {
        Some(&source[1..(source.len() - 1)])
    } else {
        None
    }
}
