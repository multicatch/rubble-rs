use crate::template::Template;

/// Represents content that can be both template parts and other fragments that should be evaluated.
///
/// It can be used to represent some template source that has mixed content - eg. text, code,
/// other templates and needs to be evaluated/compiled.
///
pub trait EvaluableMixedContent<T>: IntoIterator {}

/// A slice of template that can be returned by an iterator.
///
/// Usually used to represent a fragment of template that needs to be evaluated.
/// Can be used for finding template parts depending on what rules are used to detect
/// text and code or other patterns in the source file.
///
#[derive(Debug, Eq, PartialEq)]
pub enum TemplateSlice<'a> {
    Text {
        value: &'a str,
        start_position: usize,
        end_position: usize,
    },
    Code {
        value: &'a str,
        start_position: usize,
        end_position: usize,
    },
}

/// Iterates over some template source and returns code fragments that needs evaluation.
///
/// It can be used to return all evaluation spots from a template. For example, there is an implementation
/// that looks for all embedded code fragments and returns them as `TemplateFragment`s for further evaluation.
pub struct EvaluableMixedContentIterator<'a, T> {
    source: &'a T,
    current_position: usize,
}

impl<'a> EvaluableMixedContent<&'a Template> for &'a Template {}

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

pub(crate) const START_PATTERN: &'static str = "{{";
pub(crate) const END_PATTERN: &'static str = "}}";

/// Used to iterate over a template and extract all code blocks.
///
/// ```
/// use rubble_rs::template::Template;
/// use rubble_rs::template::content::{EvaluableMixedContent, TemplateSlice};
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
    use crate::template::Template;
    use crate::template::content::TemplateSlice;
    use std::path::PathBuf;

    #[test]
    fn should_find_all_evaluation_spots() {
        let path = PathBuf::from("test-assets/template");
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

