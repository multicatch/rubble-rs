#[cfg(test)]
mod tests {
    use crate::template::Template;
    use crate::parser::{MixedContent, TemplateSlice};
    use std::path::PathBuf;

    #[test]
    fn should_find_all_evaluation_spots() {
        let path = PathBuf::from("test-assets/template");
        let template = Template::create(&path).unwrap();
        let all_evaluation_spots: Vec<TemplateSlice> = template.iter().collect();
        let expected = vec![TemplateSlice {
            value: "{{ variable }}".to_string(),
            start_position: 14,
            end_position: 28
        }];
        assert_eq!(all_evaluation_spots, expected);
    }
}

use crate::template::Template;

pub trait MixedContent<T> {
    fn iter(&self) -> MixedContentIterator<T>;
}

impl<'a> MixedContent<Template> for Template {
    fn iter(&self) -> MixedContentIterator<Template> {
        MixedContentIterator {
            source: self,
            current_position: 0,
        }
    }
}

pub struct MixedContentIterator<'a, T> {
    source: &'a T,
    current_position: usize,
}

const START_PATTERN: &str = "{{";
const END_PATTERN: &str = "}}";

impl<'a> Iterator for MixedContentIterator<'a, Template> {
    type Item = TemplateSlice;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.current_position;
        let raw_content = self.source.raw_content.as_str();
        let next_position = raw_content[i..].find(START_PATTERN);

        if let Some(start_position) = next_position {
            let next_position = raw_content[start_position..].find(END_PATTERN);
            if let Some(end_offset) = next_position {
                let end_position = start_position + end_offset + END_PATTERN.len();
                self.current_position = end_position;
                return Some(TemplateSlice {
                    value: raw_content[start_position..end_position].to_string(),
                    start_position,
                    end_position
                })
            }
        }
        None
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct TemplateSlice {
    pub value: String,
    pub start_position: usize,
    pub end_position: usize,
}