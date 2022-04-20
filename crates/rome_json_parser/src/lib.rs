use rome_diagnostics::Diagnostic;
use rome_js_parser::{
    token_source::{TokenSource, Trivia},
    Event, LexContext, Marker, ParseDiagnostic, TokenSet, ToDiagnostic, SourceType,
};
use rome_js_syntax::{JsSyntaxKind, TextRange, TextSize};
pub fn parse_json_root() {
    let parser = rome_js_parser::Parser::new("", 0, SourceType::js_module());
}