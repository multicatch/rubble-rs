use crate::evaluator::Function;
use std::collections::HashMap;
use crate::evaluator::functions::SimpleFunction;

const EMPTY_STRING: &str = "";

/// Provides a set of standard functions.
///
pub fn std_functions() -> HashMap<String, Box<dyn Function>> {
    let mut functions: HashMap<String, Box<dyn Function>> = HashMap::new();
    functions.insert("concat".to_string(), SimpleFunction::new(concat_function));
    functions.insert("+".to_string(), SimpleFunction::new(plus_function));
    functions
}

/// Concatenates the parameters.
///
/// Eg.
/// ```text
/// concat 1 "hello" " " 3.14 "world!"
/// ```
/// Expected output:
/// ```text
/// 1hello 3.14world!
/// ```
pub fn concat_function(parameters: &[String]) -> String {
    let mut result = EMPTY_STRING.to_string();
    parameters.iter().for_each(|param| {
        result.push_str(param);
    });
    result
}

/// Adds (or concatenates) values.
/// If any of the parameters is not convertible to a number, then the rest will be concatenated.
///
/// Eg.
/// ```text
/// + 1 2 3.14
/// ```
/// Expected output:
/// ```text
/// 1hello 3.14world!
/// ```
pub fn plus_function(parameters: &[String]) -> String {
    let mut result: String = EMPTY_STRING.to_string();
    let mut floating_result: Option<f64> = None;

    parameters.iter().for_each(|param| {
        if result.is_empty() {
            if let Result::Ok(value) = param.parse::<f64>() {
                floating_result = Some(floating_result.unwrap_or(0 as f64) + value);
            } else {
                floating_result
                    .map(|number| number.to_string())
                    .map(|number| result += &number);

                result.push_str(param);
            }
        } else {
            result.push_str(param);
        };
    });

    if result.is_empty() && floating_result.is_some() {
        floating_result.map(|number| number.to_string()).unwrap()
    } else {
        result
    }
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

