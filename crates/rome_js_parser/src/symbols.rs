use rome_js_syntax::{JsSyntaxKind, JsSyntaxNode, TextRange};
use rome_rowan::NodeOrToken;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Symbol {
    Declaration {
        name: String,
        range: TextRange,
    },
    Reference {
        name: String,
        range: TextRange,
        declared_at: Option<TextRange>,
    },
}

impl Symbol {
    pub fn name(&self) -> &str {
        match self {
            Symbol::Declaration { name, .. } => name,
            Symbol::Reference { name, .. } => name,
        }
    }

    pub fn range(&self) -> TextRange {
        match self {
            Symbol::Declaration { range, .. } => *range,
            Symbol::Reference { range, .. } => *range,
        }
    }
}

pub struct SymbolIterator(VecDeque<JsSyntaxNode>);

impl Iterator for SymbolIterator {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.0.pop_front() {
            // let symbol = match node.kind() {
            //
            //     | JsSyntaxKind::JS_NAME // (still missing cases to convert)
            //     | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
            //     | JsSyntaxKind::TS_TYPE_PARAMETER_NAME
            //     | JsSyntaxKind::TS_QUALIFIED_NAME
            //     | JsSyntaxKind::JS_COMPUTED_MEMBER_NAME => Some(Symbol {
            //         name: node.text_trimmed().to_string(),
            //         range: node.text_range(),
            //     }),

            //     _ => None,
            // };

            let symbol = match node.kind() {
                JsSyntaxKind::JS_IDENTIFIER_BINDING | JsSyntaxKind::TS_IDENTIFIER_BINDING => {
                    Some(Symbol::Declaration {
                        name: node.text_trimmed().to_string(),
                        range: node.text_range(),
                    })
                }
                JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT
                | JsSyntaxKind::JS_SUPER_EXPRESSION
                | JsSyntaxKind::JS_THIS_EXPRESSION => Some(Symbol::Reference {
                    name: node.text_trimmed().to_string(),
                    range: node.text_range(),
                    declared_at: None,
                }),
                // Some reference identifiers are not really references
                // - const on typescript const cast "10 as const"
                // - undefined
                JsSyntaxKind::JS_REFERENCE_IDENTIFIER => match node.first_token() {
                    Some(token) => match token.text_trimmed() {
                        "const" | "undefined" => None,
                        _ => Some(Symbol::Reference {
                            name: node.text_trimmed().to_string(),
                            range: node.text_range(),
                            declared_at: None,
                        }),
                    },
                    _ => None,
                },
                // JS_LITERAL_MEMBER_NAME to be a symbol:
                // - it cannot be a constructor
                // - it cannot be a string literal
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

                    let first_child_ok = !matches!(
                        node.first_token().map(|token| token.kind()),
                        Some(JsSyntaxKind::JS_STRING_LITERAL)
                    );
                    if parent_ok && first_child_ok {
                        Some(Symbol::Declaration {
                            name: node.text_trimmed().to_string(),
                            range: node.text_range(),
                        })
                    } else {
                        None
                    }
                }
                //
                // is JS_NAME under TS_NAMED_TUPLE_TYPE_ELEMENT a symbol?
                // example: type A = [ b: string ]; // <-- is b a symbol?
                JsSyntaxKind::JS_NAME => {
                    let parent_kind = node.parent().map(|parent| parent.kind());
                    let parent_ok = matches!(
                        parent_kind,
                        Some(
                            JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
                                | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                                | JsSyntaxKind::TS_EXPORT_AS_NAMESPACE_CLAUSE
                                | JsSyntaxKind::TS_QUALIFIED_MODULE_NAME
                                | JsSyntaxKind::TS_QUALIFIED_NAME
                        )
                    );
                    if parent_ok {
                        Some(Symbol::Reference {
                            name: node.text_trimmed().to_string(),
                            range: node.text_range(),
                            declared_at: None,
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
                    Some(Symbol::Declaration {
                        name: token.text_trimmed().to_string(),
                        range: token.text_range(),
                    })
                }
                JsSyntaxKind::TS_GLOBAL_DECLARATION => {
                    node.first_token().map(|token| Symbol::Declaration {
                        name: token.text_trimmed().to_string(),
                        range: token.text_range(),
                    })
                }
                JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => match node.element_in_slot(3) {
                    Some(NodeOrToken::Node(node)) => match node.kind() {
                        JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION => Some(Symbol::Reference {
                            name: node.text_trimmed().to_string(),
                            range: node.text_range(),
                            declared_at: None,
                        }),
                        _ => None,
                    },
                    _ => None,
                },
                JsSyntaxKind::TS_TYPE_PARAMETER_NAME => {
                    let is_in_type_parameter_list = matches!(
                        node.parent().and_then(|x| x.parent()).map(|x| x.kind()),
                        Some(JsSyntaxKind::TS_TYPE_PARAMETER_LIST)
                    );
                    let is_in_mapped_type = matches!(
                        node.parent().map(|x| x.kind()),
                        Some(JsSyntaxKind::TS_MAPPED_TYPE)
                    );

                    if is_in_type_parameter_list || is_in_mapped_type {
                        Some(Symbol::Declaration {
                            name: node.text_trimmed().to_string(),
                            range: node.text_range(),
                        })
                    } else {
                        None
                    }
                }
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
