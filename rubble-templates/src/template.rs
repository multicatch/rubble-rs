use std::{fs, io};
use std::path::Path;

pub mod content;

/// A simple template that represents a source text.
///
/// This template is a raw template that can be reused with different variables or Evaluators.
/// It contains a raw source that can be parsed.
#[derive(Debug, Eq, PartialEq)]
pub struct Template {
    pub raw_content: String
}

impl Template {
    pub fn read_from(path: &Path) -> Result<Template, io::Error> {
        let raw_content = fs::read_to_string(path)?;
        Ok(Template {
            raw_content
        })
    }

    pub fn from(raw_content: String) -> Template {
        Template {
            raw_content
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::template::Template;

    #[test]
    fn should_create_template() {
        let path_buf = PathBuf::from("../test-assets/simple-template");
        let result = Template::read_from(&path_buf);

        let expected = Template::from("Some template {{ variable }} - or something".to_string());

        let result = result.map_err(|e| e.kind());
        assert_eq!(result, Ok(expected));
    }
}

