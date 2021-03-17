use std::collections::HashMap;
use crate::template::Template;

pub struct EvaluationEngine {
    variables: HashMap<String, String>
}

impl EvaluationEngine {
    pub fn from(variables: HashMap<String, String>) -> EvaluationEngine {
        EvaluationEngine {
            variables
        }
    }

    pub fn compile(template: &Template) -> Result<String, EvaluationError> {
        unimplemented!()
    }
}

pub enum EvaluationError {
    UnknownSymbol {
        symbol: String,
        position: usize
    }
}
