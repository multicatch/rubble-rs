use crate::template::content::{START_PATTERN, END_PATTERN};
use std::fmt::Debug;

/// Represents a node in an AST
///
/// Used to represent a template code for further evaluation.
///
/// Example:
/// `(plus 1 2)` can be represented as:
/// ```text
///     AnonymousNode {
///         starts_at: 0,
///         children: vec![
///              NamedNode {
///                  identifier: "plus".to_string(),
///                  starts_at: 1,
///                  children: vec![
///                      NamedNode {
///                          identifier: "1".to_string(),
///                          starts_at: 6,
///                          children: vec![],
///                      },
///                      NamedNode {
///                          identifier: "2".to_string(),
///                          starts_at: 8,
///                          children: vec![],
///                      },
///                  ],
///              },
///         ]
///     };
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub enum SyntaxNode {
    NamedNode {
        identifier: String,
        starts_at: usize,
        children: Vec<SyntaxNode>,
    },
    AnonymousNode {
        starts_at: usize,
        children: Vec<SyntaxNode>,
    },
}

/// Used for parsing AST for further evaluation.
///
/// This function tries to parse AST assuming it is Lisp-like syntax, which is practically
/// `function arg0 arg1 arg2`, where `function` is the function name, and `arg0...` are the arguments.
///
/// It also allows to use parenthesis to evaluate a nested function.
///
/// Reserved characters (cannot be used in names):
/// * ` ` - space
/// * `(` - left parenthesis
/// * `)` - right parenthesis
///
/// Examples:
/// * `(function 1 2 3)` - interpreted as `function` call with parameters `1`, `2` and `3`
/// * `plus 1 2 (times 3 4)` - interpreted as `1 + 2 + (3 * 4)`, given `plus` is an addition function and `times` is a multiplication function
///
pub fn parse_ast(source: &str) -> SyntaxNode {
    let source = source.strip_prefix(START_PATTERN).unwrap_or(source);
    let source = source.strip_suffix(END_PATTERN).unwrap_or(source);

    next_node_of(source, 0).0
}

struct SyntaxScanResult(SyntaxNode, usize);

fn next_node_of(source: &str, offset: usize) -> SyntaxScanResult {
    let mut syntax_node = SyntaxNode::AnonymousNode {
        children: vec![],
        starts_at: offset
    };
    let mut identifier = "".to_string();
    let mut string_started = false;
    let mut identifier_start: usize = offset;
    let mut skip_end: usize = 0;
    let mut source_length: usize = 0;

    for (index, char) in source.chars().enumerate() {
        let position = offset + index;
        source_length += 1;
        if position <= skip_end {
            continue;
        }
        let current_offset = index + 1;

        let (id, started) = extract_string(identifier, char, string_started);
        identifier = id;
        string_started = started;
        if string_started {
            continue;
        }

        if char == '(' {
            let (new_node, skip_pos) = start_node(syntax_node, &identifier, &source[current_offset..], identifier_start + 1, position);
            syntax_node = new_node;
            skip_end = skip_pos;
        } else {
            if char == ' ' || char == ')' {
                if let Some(node) = syntax_node.add_identifier_or_child(
                    &identifier,
                    identifier_start + 1
                ) {
                    syntax_node = node;
                }
                identifier.clear();
                identifier_start = position + 1;
            } else {
                identifier.push(char);
            }

            if char == ')' {
                return SyntaxScanResult(syntax_node, position);
            }
        }
    }

    SyntaxScanResult(syntax_node, offset + source_length)
}

fn extract_string(identifier: String, char: char, string_started: bool) -> (String, bool) {
    let mut identifier = identifier;
    let mut string_started = string_started;

    if string_started && char != '"' {
        identifier.push(char);
    } else if string_started {
        string_started = false;
    } else if char == '"' {
        identifier.push(char);
        string_started = true;
    }

    (identifier, string_started)
}

fn start_node(syntax_node: SyntaxNode, identifier: &str, source_remainder: &str, identifier_start: usize, position: usize) -> (SyntaxNode, usize) {
    let mut syntax_node = syntax_node;
    if let Some(node) = syntax_node.add_identifier_or_child(
        identifier,
        identifier_start
    ) {
        syntax_node = node;
    }
    let SyntaxScanResult(child, skip_pos) = next_node_of(source_remainder, position);
    syntax_node.add_child(child);

    (syntax_node, skip_pos)
}

impl SyntaxNode {
    fn add_child(&mut self, child: SyntaxNode) {
        match self {
            SyntaxNode::AnonymousNode { children, .. } => children.push(child),
            SyntaxNode::NamedNode { children, .. } => children.push(child),
        }
    }

    fn add_identifier_or_child(&self, new_identifier: &str, identifier_starts_at: usize) -> Option<SyntaxNode> {
        if new_identifier.is_empty() {
            return None;
        }

        match self {
            SyntaxNode::AnonymousNode { children, .. } =>
                Some(SyntaxNode::NamedNode {
                    identifier: new_identifier.to_string(),
                    children: children.clone(),
                    starts_at: identifier_starts_at
                }),

            SyntaxNode::NamedNode { identifier, children, starts_at } => {
                let mut children = children.clone();
                children.push(SyntaxNode::NamedNode {
                    identifier: new_identifier.to_string(),
                    children: vec![],
                    starts_at: identifier_starts_at
                });
                Some(SyntaxNode::NamedNode {
                    identifier: identifier.clone(),
                    children,
                    starts_at: *starts_at,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluator::ast::parse_ast;
    use crate::evaluator::ast::SyntaxNode::{AnonymousNode, NamedNode};

    #[test]
    fn should_parse_ast() {
        let input = "{{ (list 1 2 (if a b c)) }}";
        let actual = parse_ast(input);

        let expected = AnonymousNode {
            starts_at: 0,
            children: vec![
                NamedNode {
                    identifier: "list".to_string(),
                    starts_at: 2,
                    children: vec![
                        NamedNode {
                            identifier: "1".to_string(),
                            children: vec![],
                            starts_at: 7
                        },
                        NamedNode {
                            identifier: "2".to_string(),
                            starts_at: 9,
                            children: vec![],
                        },
                        NamedNode {
                            identifier: "if".to_string(),
                            starts_at: 11,
                            children: vec![
                                NamedNode {
                                    identifier: "a".to_string(),
                                    starts_at: 14,
                                    children: vec![],
                                },
                                NamedNode {
                                    identifier: "b".to_string(),
                                    starts_at: 16,
                                    children: vec![],
                                },
                                NamedNode {
                                    identifier: "c".to_string(),
                                    starts_at: 18,
                                    children: vec![],
                                },
                            ],
                        },
                    ],
                },
            ]
        };
        assert_eq!(expected, actual);
    }
}
