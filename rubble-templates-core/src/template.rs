/// Represents content that can be both template parts and other fragments that should be evaluated.
///
/// It can be used to represent some template source that has mixed content - eg. text, code,
/// other templates and needs to be evaluated/compiled.
///
pub trait EvaluableMixedContent: IntoIterator {}

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
