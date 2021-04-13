use std::collections::HashMap;
use ast::SyntaxNode;

mod ast;
mod engine;

pub trait Evaluator {
    fn evaluate(&self, syntax_node: SyntaxNode, variables: &HashMap<String, String>) -> Result<String, EvaluationError>;
}

#[derive(Debug, PartialEq)]
pub enum EvaluationError {
    UnexpectedElements {
        last_expected: SyntaxNode,
        unexpected_elements: Vec<SyntaxNode>,
    },
    UnknownSymbol {
        symbol: String
    },
    InvalidArgument {
        argument: SyntaxNode
    },
}

pub trait Function {
    fn evaluate(&self, evaluator: &dyn Evaluator, parameters: &Vec<SyntaxNode>) -> Result<String, EvaluationError>;
}