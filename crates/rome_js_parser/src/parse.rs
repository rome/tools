//! Utilities for high level parsing of js code.

use crate::token_source::Trivia;
use crate::*;
use rome_diagnostics::Severity;
use rome_js_syntax::{
    JsAnyRoot, JsExpressionSnipped, JsLanguage, JsModule, JsScript, JsSyntaxNode, JsSyntaxToken,
};
use rome_rowan::{AstNode, NodeOrToken};
use std::{collections::VecDeque, marker::PhantomData};

/// A utility struct for managing the result of a parser job
#[derive(Debug, Clone)]
pub struct Parse<T> {
    root: JsSyntaxNode,
    errors: Vec<ParseDiagnostic>,
    _ty: PhantomData<T>,
}

impl<T> Parse<T> {
    pub fn new_module(root: JsSyntaxNode, errors: Vec<ParseDiagnostic>) -> Parse<T> {
        Self::new(root, errors)
    }

    pub fn new_script(root: JsSyntaxNode, errors: Vec<ParseDiagnostic>) -> Parse<T> {
        Self::new(root, errors)
    }

    pub fn new(root: JsSyntaxNode, errors: Vec<ParseDiagnostic>) -> Parse<T> {
        Parse {
            root,
            errors,
            _ty: PhantomData,
        }
    }

    pub fn cast<N: AstNode<Language = JsLanguage>>(self) -> Option<Parse<N>> {
        if N::can_cast(self.syntax().kind()) {
            Some(Parse::new(self.root, self.errors))
        } else {
            None
        }
    }

    /// The syntax node represented by this Parse result
    ///
    /// ```
    /// use rome_js_parser::parse_script;
    /// use rome_rowan::{AstNode, AstNodeList};
    /// use rome_js_syntax::{JsIfStatement, JsSyntaxKind};
    ///
    /// let parse = parse_script(
    /// "
    ///     if (a > 5) {
    ///         /* something */
    ///     }
    /// ", 0);
    ///
    /// // The first stmt in the root syntax node (Script) is the if statement.
    /// let if_stmt = parse.tree().statements().first().unwrap();
    ///
    /// assert_eq!(if_stmt.syntax().kind(), JsSyntaxKind::JS_IF_STATEMENT);
    /// ```
    pub fn syntax(&self) -> JsSyntaxNode {
        self.root.clone()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn diagnostics(&self) -> &[Diagnostic] {
        self.errors.as_slice()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.errors
    }

    /// Returns [true] if the parser encountered some errors during the parsing.
    pub fn has_errors(&self) -> bool {
        self.errors.iter().any(|diagnostic| diagnostic.is_error())
    }
}

impl<T: AstNode<Language = JsLanguage>> Parse<T> {
    /// Convert this parse result into a typed AST node.
    ///
    /// # Panics
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> T {
        self.try_tree().unwrap_or_else(|| {
            panic!(
                "Expected tree to be a {} but root is:\n{:#?}",
                std::any::type_name::<T>(),
                self.syntax()
            )
        })
    }

    /// Try to convert this parse's untyped syntax node into an AST node.
    pub fn try_tree(&self) -> Option<T> {
        T::cast(self.syntax())
    }

    /// Convert this parse into a result
    pub fn ok(self) -> Result<T, Vec<ParseDiagnostic>> {
        if !self.errors.iter().any(|d| d.severity == Severity::Error) {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}

pub fn parse_common(
    text: &str,
    file_id: usize,
    source_type: SourceType,
) -> (Vec<Event>, Vec<ParseDiagnostic>, Vec<Trivia>) {
    let mut parser = crate::Parser::new(text, file_id, source_type);
    crate::syntax::program::parse(&mut parser);

    let (events, trivia, errors) = parser.finish();

    (events, errors, trivia)
}

/// Parse text into a [`Parse`](Parse) which can then be turned into an untyped root [`SyntaxNode`](SyntaxNode).
/// Or turned into a typed [`Script`](Script) with [`tree`](Parse::tree).
///
/// ```
/// use rome_js_parser::parse_script;
/// use rome_js_syntax::{JsSyntaxToken, JsSyntaxList, JsComputedMemberExpression};
/// use rome_rowan::AstNode;
///
/// let parse = parse_script("foo.bar[2]", 0);
/// // Parse returns a JS Root which contains two lists, the directives and the statements, let's get the statements
/// let stmt = parse.syntax().children().nth(1).unwrap();
/// // The untyped syntax node of `foo.bar[2]`, the root node is `Script`.
/// let untyped_expr_node = stmt.first_child().unwrap();
///
/// // SyntaxNodes can be turned into a nice string representation.
/// println!("{:#?}", untyped_expr_node);
///
/// // You can then cast syntax nodes into a typed AST node.
/// let typed_ast_node = JsComputedMemberExpression::cast(untyped_expr_node.first_child().unwrap()).unwrap();
///
/// // Everything on every ast node is optional because of error recovery.
/// let prop = dbg!(typed_ast_node.member()).unwrap();
///
/// // You can then go back to an untyped SyntaxNode and get its range, text, parents, children, etc.
/// assert_eq!(prop.syntax().text(), "2");
///
/// // Util has a function for yielding all tokens of a node.
/// let tokens = untyped_expr_node.descendants_tokens().map(|token| token.text_trimmed().to_string()).collect::<Vec<_>>();
///
/// assert_eq!(&tokens, &vec!["foo", ".", "bar", "[", "2", "]"]);
/// ```
pub fn parse_script(text: &str, file_id: usize) -> Parse<JsScript> {
    parse(
        text,
        file_id,
        SourceType::js_module().with_module_kind(ModuleKind::Script),
    )
    .cast::<JsScript>()
    .unwrap()
}

/// Same as [`parse_text`] but configures the parser to parse an ECMAScript module instead of a script
pub fn parse_module(text: &str, file_id: usize) -> Parse<JsModule> {
    parse(text, file_id, SourceType::js_module())
        .cast::<JsModule>()
        .unwrap()
}

/// Parses the provided string as a EcmaScript program using the provided syntax features.
pub fn parse(text: &str, file_id: usize, source_type: SourceType) -> Parse<JsAnyRoot> {
    tracing::debug_span!("parse", file_id = file_id).in_scope(move || {
        let (events, errors, tokens) = parse_common(text, file_id, source_type);
        let mut tree_sink = LosslessTreeSink::new(text, &tokens);
        crate::process(&mut tree_sink, events, errors);
        let (green, parse_errors) = tree_sink.finish();
        Parse::new(green, parse_errors)
    })
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub range: rome_js_syntax::TextRange,
}

#[derive(Debug)]
pub struct Symbols {
    pub symbols: Vec<Symbol>,
}

macro_rules! symbol_at {
    ($node:expr, $($slot:expr),*; $token:expr) => {
        Some(&$node)$(.and_then(|x| x.element_in_slot($slot)).and_then(|x| x.into_node()))*
        .and_then(|x| x.element_in_slot($token)).and_then(|x| x.into_token())
        .map(|x| Symbol {
            name: x.text_trimmed().to_string(),
            range: x.text_range()
        })
    };
}

pub fn symbols(root: JsSyntaxNode) -> Symbols {
    let mut symbols = vec![];

    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(node) = queue.pop_front() {
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
            | JsSyntaxKind::JS_SUPER_EXPRESSION
            | JsSyntaxKind::TS_IDENTIFIER_BINDING => Some(Symbol {
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

        if let Some(s) = symbol {
            symbols.push(s);
        }

        for child in node.children() {
            queue.push_back(child);
        }
    }

    // for node in root.descendants_with_tokens() {
    //     use rome_rowan::NodeOrToken;
    //     match node {
    //         NodeOrToken::Node(node) => match node {
    //             JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {}
    //         },
    //         NodeOrToken::Token(token) => match token.kind() {
    //             JsSyntaxKind::IDENT => {
    //                 let name = token.text_trimmed();
    //                 symbols.push(Symbol {
    //                     name: name.to_string(),
    //                 });
    //             }
    //             JsSyntaxKind::THIS_KW => {
    //                 let name = token.text_trimmed();
    //                 symbols.push(Symbol {
    //                     name: name.to_string(),
    //                 });
    //             }
    //             _ => {}
    //         },
    //     }
    // }

    Symbols { symbols }
}

/// Losslessly Parse text into an expression [`Parse`](Parse) which can then be turned into an untyped root [`SyntaxNode`](SyntaxNode).
/// Or turned into a typed [`Expr`](Expr) with [`tree`](Parse::tree).
pub fn parse_expression(text: &str, file_id: usize) -> Parse<JsExpressionSnipped> {
    let mut parser = crate::Parser::new(text, file_id, SourceType::js_module());
    crate::syntax::expr::parse_expression_snipped(&mut parser).unwrap();
    let (events, tokens, errors) = parser.finish();

    let mut tree_sink = LosslessTreeSink::new(text, &tokens);
    crate::process(&mut tree_sink, events, errors);
    let (green, parse_errors) = tree_sink.finish();
    Parse::new_script(green, parse_errors)
}
