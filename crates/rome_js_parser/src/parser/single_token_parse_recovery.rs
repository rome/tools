use crate::lexer::{JsSyntaxKind, T};
use crate::{JsParser, ParseDiagnostic, TokenSet};

/// This struct contains the information needed to the parser to recover from a certain error
///
/// By default it doesn't check curly braces, use [with_braces_included] to turn opt-in the check
#[derive(Debug)]
#[deprecated(note = "Use ParsedSyntax with ParseRecovery instead")]
pub(crate) struct SingleTokenParseRecovery {
    /// The [Diagnostic] to emit
    error: Option<ParseDiagnostic>,
    /// It tells the parser to recover if the position is inside a set of [tokens](TokenSet)
    recovery: TokenSet,
    /// It tells the parser to recover if the current token is a curly brace
    include_braces: bool,
    /// The kind of the unknown node the parser inserts if it isn't able to recover because
    /// the current token is neither in the recovery set nor any of `{` or `}`.
    unknown_node_kind: JsSyntaxKind,
}

#[allow(deprecated)]
impl SingleTokenParseRecovery {
    pub fn new(recovery: TokenSet, unknown_node_kind: JsSyntaxKind) -> Self {
        Self {
            error: None,
            recovery,
            include_braces: false,
            unknown_node_kind,
        }
    }

    /// The main function that tells to the parser how to recover itself.
    ///
    /// Recover from an error with a [recovery set](TokenSet) or by using a `{` or `}`.
    ///
    /// If [SingleTokenParseRecovery] has an error, it gets tracked in the events.
    pub fn recover(&self, p: &mut JsParser) {
        let error = self.get_error();
        if let Some(error) = error {
            p.error(error);
        }

        if p.state().speculative_parsing {
            return;
        }

        if !self.parsing_is_recoverable(p) {
            let m = p.start();
            p.bump_any();
            m.complete(p, self.get_unknown_node_kind());
        }
    }

    /// Checks if the parsing phase is recoverable by checking curly braces and [tokens set](TokenSet)
    fn parsing_is_recoverable(&self, parser: &JsParser) -> bool {
        self.is_at_token_set(parser) || self.is_at_braces(parser) || self.is_at_eof(parser)
    }

    /// It returns the diagnostic
    fn get_error(&self) -> Option<ParseDiagnostic> {
        self.error.to_owned()
    }

    /// It returns the unknown node kind that will be used to complete the parsing
    fn get_unknown_node_kind(&self) -> JsSyntaxKind {
        self.unknown_node_kind
    }

    fn is_at_braces(&self, parser: &JsParser) -> bool {
        matches!(parser.cur(), T!['{'] | T!['}'] if self.include_braces)
    }

    fn is_at_token_set(&self, parser: &JsParser) -> bool {
        parser.at_ts(self.recovery)
    }

    fn is_at_eof(&self, parser: &JsParser) -> bool {
        parser.cur() == JsSyntaxKind::EOF
    }
}
