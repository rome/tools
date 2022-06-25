pub(crate) mod event;
pub mod lexer;
pub(crate) mod lossless_tree_sink;
pub(crate) mod parse;
pub(crate) mod parse_error;
pub(crate) mod parser;
pub(crate) mod token_set;
pub mod token_source;

pub(crate) use parser::Parser;
pub use parse::parse;
pub fn parse_json_root() {
    // let parser = rome_js_parser::Parser::new("", 0, SourceType::js_module());
    // let source = "(void b)";

    // // File id is used for the labels inside parser errors to report them, the file id
    // // is used to look up a file's source code and path inside of a codespan `Files` implementation.
    // let mut parser = Parser::new(source, 0, SourceType::default());

    // // Use one of the syntax parsing functions to parse an expression.
    // // This adds node and token events to the parser which are then used to make a node.
    // // A completed marker marks the start and end indices in the events vec which signify
    // // the Start event, and the Finish event.
    // // Completed markers can be turned into an ast node with parse_marker on the parser
    // parse_expression_snipped(&mut parser).unwrap();

    // // Consume the parser and get its events, then apply them to a tree sink with `process`.
    // let (events, tokens, errors) = parser.finish();

    // // Make a new text tree sink, its job is assembling events into a rowan GreenNode.
    // // At each point (Start, Token, Finish, Error) it also consumes whitespace.
    // // Other abstractions can also yield lossy syntax nodes if whitespace is not wanted.
    // // Swap this for a LossyTreeSink for a lossy AST result.
    // let mut sink = LosslessTreeSink::new(source, &tokens);

    // process(&mut sink, events, errors);

    // let (untyped_node, errors) = sink.finish();

    // assert!(errors.is_empty());
    // assert!(JsExpressionSnipped::can_cast(untyped_node.kind()));

    // // Convert the untyped SyntaxNode into a typed AST node
    // let expression_snipped = JsExpressionSnipped::cast(untyped_node).unwrap();
    // let expression = expression_snipped.expression().unwrap();

    // match expression {
    //     JsAnyExpression::JsParenthesizedExpression(parenthesized) => {
    //         assert_eq!(
    //             parenthesized.expression().unwrap().syntax().text(),
    //             "void b"
    //         );
    //     }
    //     _ => panic!("Expected parenthesized expression"),
    // }
}

use lexer::Lexer;
// use crate::lexer::{BufferedLexer, LexContext, Lexer, LexerCheckpoint, ReLexContext, TextRange};
use rome_diagnostics::file::FileId;
use rome_diagnostics::Diagnostic;
// use rome_js_syntax::JsonSyntaxKind;
// use rome_js_syntax::JsonSyntaxKind::EOF;
use rome_json_syntax::{
    JsonSyntaxKind::{self, EOF},
    TextRange,
};
use rome_rowan::{TextSize, TriviaPieceKind};
use std::collections::VecDeque;
