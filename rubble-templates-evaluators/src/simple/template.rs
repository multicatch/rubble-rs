use std::{fs, io};
use std::path::Path;
use rubble_templates_core::template::{TemplateSlice, EvaluableMixedContent};

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


/// Iterates over some template source and returns code fragments that needs evaluation.
///
/// It can be used to return all evaluation spots from a template. For example, there is an implementation
/// that looks for all embedded code fragments and returns them as [TemplateSlice]s for further evaluation.
pub struct EvaluableMixedContentIterator<'a, T> {
    pub source: &'a T,
    pub current_position: usize,
}

impl<'a> EvaluableMixedContent for &'a Template {}

impl<'a> IntoIterator for &'a Template {
    type Item = TemplateSlice<'a>;
    type IntoIter = EvaluableMixedContentIterator<'a, Template>;

    fn into_iter(self) -> Self::IntoIter {
        EvaluableMixedContentIterator {
            source: &self,
            current_position: 0,
        }
    }
}

pub(crate) const START_PATTERN: &str = "{{";
pub(crate) const END_PATTERN: &str = "}}";

/// Used to iterate over a template and extract all code blocks.
///
/// ```
/// use rubble_templates_core::template::{EvaluableMixedContent, TemplateSlice};
/// use rubble_templates_evaluators::simple::template::Template;
///
/// let template = Template::from("Some template {{ variable }}".to_string());
/// let all_evaluation_spots: Vec<TemplateSlice> = template.into_iter().collect();
/// let expected = vec![
///             TemplateSlice::Text {
///                 value: "Some template ",
///                 start_position: 0,
///                 end_position: 14,
///             },
///             TemplateSlice::Code {
///                 value: "{{ variable }}",
///                 start_position: 14,
///                 end_position: 28,
///             },
///         ];
///
/// assert_eq!(all_evaluation_spots, expected);
/// ```
impl<'a> Iterator for EvaluableMixedContentIterator<'a, Template> {
    type Item = TemplateSlice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.current_position;
        let raw_content = self.source.raw_content.as_str();
        let source_length = raw_content.len();

        let start_position = raw_content[i..].find(START_PATTERN);
        if start_position.is_none() && i < source_length {
            self.current_position = source_length;

            return Some(TemplateSlice::Text {
                value: &raw_content[i..],
                start_position: i,
                end_position: source_length,
            });
        }

        let start_position = start_position? + i;
        if i < start_position {
            self.current_position = start_position;

            return Some(TemplateSlice::Text {
                value: &raw_content[i..start_position],
                start_position: i,
                end_position: start_position,
            });
        }

        let end_offset = raw_content[start_position..].find(END_PATTERN)?;
        let end_position = start_position + end_offset + END_PATTERN.len();
        self.current_position = end_position;

        Some(TemplateSlice::Code {
            value: &raw_content[start_position..end_position],
            start_position,
            end_position,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rubble_templates_core::template::TemplateSlice;
    use crate::simple::template::Template;

    #[test]
    fn should_create_template() {
        let path_buf = PathBuf::from("test-assets/simple-template");
        let result = Template::read_from(&path_buf);

        let expected = Template::from("Some template {{ variable }} - or something".to_string());

        let result = result.map_err(|e| e.kind());
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn should_find_all_evaluation_spots() {
        let path = PathBuf::from("test-assets/simple-template");
        let template = Template::read_from(&path).unwrap();
        let all_evaluation_spots: Vec<TemplateSlice> = (&template).into_iter().collect();
        let expected = vec![
            TemplateSlice::Text {
                value: "Some template ",
                start_position: 0,
                end_position: 14,
            },
            TemplateSlice::Code {
                value: "{{ variable }}",
                start_position: 14,
                end_position: 28,
            },
            TemplateSlice::Text {
                value: " - or something",
                start_position: 28,
                end_position: 43,
            },
        ];
        assert_eq!(all_evaluation_spots, expected);
    }
}

