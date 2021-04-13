#[cfg(test)]
mod tests {
    use crate::evaluator::ast::parse_ast;
    use crate::evaluator::ast::SyntaxNode::{AnonymousNode, NamedNode};

    #[test]
    fn should_parse_ast() {
        let input = "{{ (list 1 2 (if a b c)) }}".to_string();
        let actual = parse_ast(input);

        let expected = AnonymousNode {
            children: vec![
                NamedNode {
                    identifier: "list".to_string(),
                    children: vec![
                        NamedNode {
                            identifier: "1".to_string(),
                            children: vec![],
                        },
                        NamedNode {
                            identifier: "2".to_string(),
                            children: vec![],
                        },
                        NamedNode {
                            identifier: "if".to_string(),
                            children: vec![
                                NamedNode {
                                    identifier: "a".to_string(),
                                    children: vec![],
                                },
                                NamedNode {
                                    identifier: "b".to_string(),
                                    children: vec![],
                                },
                                NamedNode {
                                    identifier: "c".to_string(),
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

use crate::parser::{START_PATTERN, END_PATTERN};
use std::fmt::Debug;

/// Represents a node in an AST
///
/// Used to represent a template code for further evaluation.
///
/// Example:
/// `(plus 1 2)` can be represented as:
/// ```
/// /*
///     AnonymousNode {
///         children: vec![
///              NamedNode {
///                  identifier: "plus".to_string(),
///                  children: vec![
///                      NamedNode {
///                          identifier: "1".to_string(),
///                          children: vec![],
///                      },
///                      NamedNode {
///                          identifier: "2".to_string(),
///                          children: vec![],
///                      },
///                  ],
///              },
///         ]
///     };
/// */
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub enum SyntaxNode {
    NamedNode {
        identifier: String,
        children: Vec<SyntaxNode>,
    },
    AnonymousNode {
        children: Vec<SyntaxNode>
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
pub fn parse_ast(source: String) -> SyntaxNode {
    let source = source.strip_prefix(START_PATTERN).unwrap_or(source.as_str());
    let source = source.strip_suffix(END_PATTERN).unwrap_or(source);

    next_node_of(source, 0).0
}

struct SyntaxScanResult(SyntaxNode, usize);

fn next_node_of(source: &str, offset: usize) -> SyntaxScanResult {
    let mut syntax_node = SyntaxNode::AnonymousNode {
        children: vec![],
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
            if let Some(node) = syntax_node.add_identifier_or_child(&identifier) {
                syntax_node = node;
            }
            let SyntaxScanResult(child, skip_pos) = next_node_of(source_remainder, position);
            skip_end = skip_pos;
            syntax_node.add_child(child);
        } else {
            if char == ' ' || char == ')' {
                if let Some(node) = syntax_node.add_identifier_or_child(&identifier) {
                    syntax_node = node;
                }
                identifier.clear();
            } else {
                identifier.push(char)
            }

            if char == ')' {
                return SyntaxScanResult(syntax_node, position);
            }
        }
    }

    SyntaxScanResult(syntax_node, offset + source.len())
}

impl SyntaxNode {
    fn add_child(&mut self, child: SyntaxNode) {
        match self {
            SyntaxNode::AnonymousNode { children } => children.push(child),
            SyntaxNode::NamedNode { children, .. } => children.push(child),
        }
    }

    fn add_identifier_or_child(&self, new_identifier: &String) -> Option<SyntaxNode> {
        if new_identifier.is_empty() {
            return None;
        }

        match self {
            SyntaxNode::AnonymousNode { children } =>
                Some(SyntaxNode::NamedNode {
                    identifier: new_identifier.clone(),
                    children: children.clone(),
                }),

            SyntaxNode::NamedNode { identifier, children } => {
                let mut children = children.clone();
                children.push(SyntaxNode::NamedNode {
                    identifier: new_identifier.clone(),
                    children: vec![],
                });
                Some(SyntaxNode::NamedNode {
                    identifier: identifier.clone(),
                    children,
                })
            }
        }
    }
}