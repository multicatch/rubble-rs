#[cfg(test)]
mod tests {
    use crate::ast::{parse_ast, SyntaxNode};

    #[test]
    fn it_works() {
        let input = "{{ (list 1 2 (if a b c)) }}".to_string();
        let node = parse_ast(input);
        let expected = "SyntaxNode { identifier: None, children: [SyntaxNode { identifier: Some(\"list\"), children: [SyntaxNode { identifier: Some(\"1\"), children: [], position: 6, end_position: 7 }, SyntaxNode { identifier: Some(\"2\"), children: [], position: 8, end_position: 9 }, SyntaxNode { identifier: Some(\"if\"), children: [SyntaxNode { identifier: Some(\"a\"), children: [], position: 13, end_position: 14 }, SyntaxNode { identifier: Some(\"b\"), children: [], position: 15, end_position: 16 }, SyntaxNode { identifier: Some(\"c\"), children: [], position: 17, end_position: 18 }], position: 10, end_position: 18 }], position: 1, end_position: 19 }], position: 0, end_position: 20 }".to_string();
        assert_eq!(expected, format!("{:?}", &node));
    }
}

use crate::parser::{START_PATTERN, END_PATTERN};
use std::fmt::Debug;

#[derive(Debug)]
pub struct SyntaxNode {
    identifier: Option<String>,
    children: Vec<SyntaxNode>,
    position: usize,
    end_position: usize,
}

impl Clone for SyntaxNode {
    fn clone(&self) -> Self {
        SyntaxNode {
            identifier: self.identifier.clone(),
            children: self.children.clone(),
            position: self.position.clone(),
            end_position: self.position.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.identifier = source.identifier.clone();
        self.children = source.children.clone();
        self.position = source.position.clone();
        self.end_position = source.end_position.clone();
    }
}

pub fn parse_ast(source: String) -> SyntaxNode {
    let source = source.strip_prefix(START_PATTERN).unwrap_or(source.as_str());
    let source = source.strip_suffix(END_PATTERN).unwrap_or(source);

    next_node_of(source, 0)
}

fn next_node_of(source: &str, offset: usize) -> SyntaxNode {
    let mut syntax_node = SyntaxNode {
        identifier: None,
        children: vec![],
        position: offset,
        end_position: offset,
    };
    let mut identifier = "".to_string();
    let mut skip_end: usize = 0;

    for (index, char) in source.chars().enumerate() {
        let position = offset + index;
        if position <= skip_end {
            continue;
        }
        let current_offset = index + 1;
        let source_remainder = &source[current_offset..];
        if char == '(' {
            syntax_node.add_child_or_update(identifier.clone(), position);
            let child = next_node_of(source_remainder, position);
            skip_end = child.end_position.clone();
            syntax_node.children.push(child);
        } else {
            if char == ' ' || char == ')' {
                syntax_node.add_child_or_update(identifier.clone(), position);
                identifier.clear();
            } else {
                identifier.push(char)
            }

            if char == ')' {
                syntax_node.end_position = position;
                let result = syntax_node;
                return result;
            }
        }
    }
    syntax_node.end_position = offset + source.len();
    syntax_node.clone()
}

impl SyntaxNode {
    fn add_child(&mut self, child: SyntaxNode) {
        self.children.push(child);
    }

    fn add_child_or_update(&mut self, identifier: String, position: usize) {
        if !identifier.is_empty() {
            let syntax_node_id = self.identifier.clone();
            if syntax_node_id == None {
                self.identifier = Some(identifier.clone());
            } else {
                let node = SyntaxNode {
                    identifier: Some(identifier.clone()),
                    children: vec![],
                    position: position - identifier.len(),
                    end_position: position,
                };
                self.add_child(node);
            }
        }
    }
}