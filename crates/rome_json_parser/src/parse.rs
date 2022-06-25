use std::marker::PhantomData;

use rome_diagnostics::{Diagnostic, Severity};
use rome_json_syntax::{JsonSyntaxKind, T, JsonSyntaxNode, JsonLanguage, JsonRoot};
use rome_parse::ParseDiagnostic;
use rome_rowan::AstNode;

use crate::{
    event::Event,
    parse_error::{expected_any, expected_node},
    parser::CompletedMarker,
    token_source::Trivia,
    Parser, lossless_tree_sink::LosslessTreeSink,
};

/// A utility struct for managing the result of a parser job
#[derive(Debug, Clone)]
pub struct Parse<T> {
    root: JsonSyntaxNode,
    errors: Vec<ParseDiagnostic>,
    _ty: PhantomData<T>,
}

impl<T> Parse<T> {
    pub fn new(root: JsonSyntaxNode, errors: Vec<ParseDiagnostic>) -> Parse<T> {
        Parse {
            root,
            errors,
            _ty: PhantomData,
        }
    }

    pub fn cast<N: AstNode<Language = JsonLanguage>>(self) -> Option<Parse<N>> {
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
    /// use rome_js_syntax::{JsIfStatement, JsSyntaxKind};
    /// use rome_rowan::{AstNode, AstNodeList};
    ///
    /// let parse = parse_script(
    ///     "
    ///     if (a > 5) {
    ///         /* something */
    ///     }
    /// ",
    ///     0,
    /// );
    ///
    /// // The first stmt in the root syntax node (Script) is the if statement.
    /// let if_stmt = parse.tree().statements().first().unwrap();
    ///
    /// assert_eq!(if_stmt.syntax().kind(), JsSyntaxKind::JS_IF_STATEMENT);
    /// ```
    pub fn syntax(&self) -> JsonSyntaxNode {
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

impl<T: AstNode<Language = JsonLanguage>> Parse<T> {
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

pub fn parse(text: &str, file_id: usize) -> Parse<JsonRoot> {
    let (events, errors, tokens) = parse_common(text, file_id);
    let mut tree_sink = LosslessTreeSink::new(text, &tokens);
    crate::event::process(&mut tree_sink, events, errors);
    let (green, parse_errors) = tree_sink.finish();
    Parse::new(green, parse_errors)
}

pub fn parse_common(text: &str, file_id: usize) -> (Vec<Event>, Vec<ParseDiagnostic>, Vec<Trivia>) {
    let mut parser = Parser::new(text, file_id);
    parse_root(&mut parser);
    let (events, trivia, errors) = parser.finish();

    (events, errors, trivia)
}

fn parse_root(p: &mut Parser) -> CompletedMarker {
    let marker = p.start();
    parse_value(p);
    match p.cur() {
        JsonSyntaxKind::EOF => {
            marker.complete(p, JsonSyntaxKind::JSON_ROOT)
        },
        _ => {
            p.error(expected_node("EOF", p.cur_range()));
            while !p.at(JsonSyntaxKind::EOF) {
                p.bump_any();
            }
            marker.complete(p, JsonSyntaxKind::JSON_UNKNOWN)
        }
    }
}

fn parse_value(p: &mut Parser) -> CompletedMarker {
    match p.cur() {
        JsonSyntaxKind::EOF => {
            expected_any(
                &["{", "[", "number", "string", "null", "true", "false"],
                p.cur_range(),
            );
        }
        JsonSyntaxKind::COLON => {
            // TODO: Recover
            expected_any(
                &["{", "[", "number", "string", "null", "true", "false"],
                p.cur_range(),
            );
        }
        JsonSyntaxKind::COMMA => {
            // TODO: Recover
            expected_any(
                &["{", "[", "number", "string", "null", "true", "false"],
                p.cur_range(),
            );
        }
        JsonSyntaxKind::L_CURLY => {
            parse_object(p);
        }
        JsonSyntaxKind::R_CURLY => {
            // TODO: Recover
            expected_any(
                &["{", "[", "number", "string", "null", "true", "false"],
                p.cur_range(),
            );
        }
        JsonSyntaxKind::L_BRACK => {
            parse_array(p);
        }
        JsonSyntaxKind::R_BRACK => {
            // TODO: Recover
            expected_any(
                &["{", "[", "number", "string", "null", "true", "false"],
                p.cur_range(),
            );
        }
        JsonSyntaxKind::NULL_KW => {
            return parse_null(p);
        }
        JsonSyntaxKind::TRUE_KW | JsonSyntaxKind::FALSE_KW => {
            return parse_boolean(p);
        }
        JsonSyntaxKind::JSON_STRING_LITERAL => {
            return parse_string(p);
        }
        JsonSyntaxKind::JSON_NUMBER_LITERAL => {
            println!("number {:?}", p.cur());
            return parse_number(p);
        }
        JsonSyntaxKind::ERROR_TOKEN => {
            // TODO: Recover
            expected_any(
                &["{", "[", "number", "string", "null", "true", "false"],
                p.cur_range(),
            );
        }
        _ => unreachable!(),
    }
    todo!()
    // let mut marker = p.start();
    // marker.complete(p, JsonSyntaxKind::JSON_ROOT)
}

fn parse_number(p: &mut Parser) -> CompletedMarker {
    let marker = p.start();
    p.bump(JsonSyntaxKind::JSON_NUMBER_LITERAL);
    marker.complete(p, JsonSyntaxKind::JSON_NUMBER)
}

fn parse_string(p: &mut Parser) -> CompletedMarker {
    let marker = p.start();
    p.bump(JsonSyntaxKind::JSON_STRING_LITERAL);
    marker.complete(p, JsonSyntaxKind::JSON_STRING)
}

fn parse_boolean(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(T![true]) || p.at(T![false]));
    let marker = p.start();
    p.bump_any();
    marker.complete(p, JsonSyntaxKind::JSON_BOOLEAN)
}

fn parse_null(p: &mut Parser) -> CompletedMarker {
    let marker = p.start();
    p.bump(T![null]);
    marker.complete(p, JsonSyntaxKind::JSON_NULL)
}

fn parse_object(p: &mut Parser) -> CompletedMarker {
    todo!()
}

fn parse_array(p: &mut Parser) -> CompletedMarker {
    todo!()
}
