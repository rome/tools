use rome_js_syntax::{JsSyntaxKind, JsSyntaxNode, TextRange};
use rome_rowan::NodeOrToken;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub range: TextRange,
}

pub struct SymbolIterator(VecDeque<JsSyntaxNode>);

impl Iterator for SymbolIterator {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.0.pop_front() {
            let symbol = match node.kind() {
                JsSyntaxKind::JS_THIS_EXPRESSION
                | JsSyntaxKind::JS_NAME
                | JsSyntaxKind::JS_IDENTIFIER_BINDING
                | JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT
                | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
                | JsSyntaxKind::TS_TYPE_PARAMETER_NAME
                | JsSyntaxKind::TS_IDENTIFIER_BINDING
                | JsSyntaxKind::TS_QUALIFIED_NAME
                | JsSyntaxKind::JS_COMPUTED_MEMBER_NAME
                | JsSyntaxKind::JS_SUPER_EXPRESSION => Some(Symbol {
                    name: node.text_trimmed().to_string(),
                    range: node.text_range(),
                }),
                JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => match node.element_in_slot(0) {
                    Some(NodeOrToken::Node(first_child)) => match first_child.kind() {
                        JsSyntaxKind::JS_IDENTIFIER_EXPRESSION
                        | JsSyntaxKind::JS_THIS_EXPRESSION
                        | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                        | JsSyntaxKind::JS_CALL_EXPRESSION => Some(Symbol {
                            name: node.text_trimmed().to_string(),
                            range: node.text_range(),
                        }),
                        _ => None,
                    },
                    _ => None,
                },
                JsSyntaxKind::JS_LITERAL_MEMBER_NAME => {
                    let parent_kind = node.parent().map(|parent| parent.kind());
                    let parent_ok = match parent_kind {
                        Some(
                            JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER
                            | JsSyntaxKind::TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER,
                        ) => false,
                        Some(_) => true,
                        None => false,
                    };

                    let first_child_ok = match node.first_token().map(|token| token.kind()) {
                        Some(JsSyntaxKind::JS_STRING_LITERAL) => false,
                        _ => true,
                    };

                    if parent_ok && first_child_ok {
                        Some(Symbol {
                            name: node.text_trimmed().to_string(),
                            range: node.text_range(),
                        })
                    } else {
                        None
                    }
                }
                JsSyntaxKind::TS_THIS_PARAMETER => {
                    let token = node
                        .element_in_slot(0)
                        .and_then(NodeOrToken::into_token)
                        .unwrap();
                    Some(Symbol {
                        name: token.text_trimmed().to_string(),
                        range: token.text_range(),
                    })
                }
                JsSyntaxKind::JS_REFERENCE_IDENTIFIER => match node.first_token() {
                    Some(token) => match token.text_trimmed() {
                        "const" | "undefined" => None,
                        _ => Some(Symbol {
                            name: token.text_trimmed().to_string(),
                            range: token.text_range(),
                        }),
                    },
                    _ => None,
                },
                JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => match node.element_in_slot(3) {
                    Some(NodeOrToken::Node(node)) => match node.kind() {
                        JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION => Some(Symbol {
                            name: node.text_trimmed().to_string(),
                            range: node.text_range(),
                        }),
                        _ => None,
                    },
                    _ => None,
                },
                JsSyntaxKind::TS_GLOBAL_DECLARATION => match node.first_token() {
                    Some(token) => Some(Symbol {
                        name: token.text_trimmed().to_string(),
                        range: token.text_range(),
                    }),
                    None => None,
                },
                _ => None,
            };

            for child in node.children() {
                self.0.push_back(child);
            }

            if let Some(s) = symbol {
                return Some(s);
            }
        }

        None
    }
}

pub fn symbols(root: JsSyntaxNode) -> SymbolIterator {
    let mut queue = VecDeque::new();
    queue.push_back(root);
    SymbolIterator(queue)
}
