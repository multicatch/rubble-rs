use crate::evaluator::{Evaluator, SyntaxError, Function};
use crate::evaluator::ast::SyntaxNode;
use std::collections::HashMap;

/// Provides a set of standard functions.
///
pub fn std_functions() -> HashMap<String, Box<dyn Function>> {
    let mut functions = HashMap::new();
    functions.insert("concat".to_string(), Box::new(concat_function) as Box<dyn Function>);
    functions
}

/// Concatenates the arguments.
///
/// This function evaluates parameters and concatenates them.
///
/// Eg.
/// ```text
/// concat 1 "hello" " " 3.14 "world!"
/// ```
/// Expected output:
/// ```text
/// 1hello 3.14world!
/// ```
pub fn concat_function(evaluator: &dyn Evaluator, parameters: &[SyntaxNode], variables: &HashMap<String, String>, _offset: usize) -> Result<String, SyntaxError> {
    let mut result = "".to_string();
    for parameter in parameters {
        let fragment = evaluator.evaluate(parameter, &variables)?;
        result += &fragment;
    }

    Ok(result)
}


#[cfg(test)]
mod tests {
    use crate::compile_template_from_file;
    use std::path::PathBuf;
    use std::collections::HashMap;
    use crate::functions::std_functions;
    use std::fs;

    #[test]
    fn should_compile_template() {
        let expected = fs::read_to_string(PathBuf::from("test-assets/stdlib-template-expected")).unwrap();
        let file = PathBuf::from("test-assets/stdlib-template");
        let functions = std_functions();
        let variables: HashMap<String, String> = HashMap::new();

        let result = compile_template_from_file(file, variables, functions);

        assert_eq!(result.ok(), Some(expected));
    }
}

