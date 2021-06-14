use std::fmt::{Display, Formatter};
use crate::units::Position;

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
        starts_at: Position,
        children: Vec<SyntaxNode>,
    },
    AnonymousNode {
        starts_at: Position,
        children: Vec<SyntaxNode>,
    },
}

impl SyntaxNode {
    pub fn is_anonymous(&self) -> bool {
        matches!(*self, SyntaxNode::AnonymousNode { .. })
    }

    pub fn add_child(self, child: SyntaxNode) -> SyntaxNode {
        let mut parent = self;
        match parent {
            SyntaxNode::AnonymousNode { ref mut children, .. } => children.push(child),
            SyntaxNode::NamedNode { ref mut children, .. } => children.push(child),
        };
        parent
    }

    pub fn with_identifier(self, new_identifier: &str, identifier_starts_at: Position) -> SyntaxNode {
        match self {
            SyntaxNode::AnonymousNode { children, .. } =>
                SyntaxNode::NamedNode {
                    identifier: new_identifier.to_string(),
                    children,
                    starts_at: identifier_starts_at,
                },

            SyntaxNode::NamedNode { children, .. } =>
                SyntaxNode::NamedNode {
                    identifier: new_identifier.to_string(),
                    children,
                    starts_at: identifier_starts_at,
                },
        }
    }
}

impl Display for SyntaxNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxNode::AnonymousNode { children, starts_at } =>
                write!(f, "AnonymousNode at {} ({} children)", starts_at, children.len()),
            SyntaxNode::NamedNode { identifier, children, starts_at} =>
                write!(f, "SyntaxNode \"{}\" at {} ({} children)", identifier, starts_at, children.len())
        }
    }
}