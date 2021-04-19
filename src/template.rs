use std::{fs, io};
use std::path::PathBuf;

pub mod content;

#[derive(Debug, Eq, PartialEq)]
pub struct Template {
    pub raw_content: String
}

impl Template {
    pub fn read_from(path: &PathBuf) -> Result<Template, io::Error> {
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
        let path_buf = PathBuf::from("test-assets/template");
        let result = Template::read_from(&path_buf);

        let expected = Template::from("Some template {{ variable }} - or something".to_string());

        let result = result.map_err(|e| e.kind());
        assert_eq!(result, Ok(expected));
    }
}

