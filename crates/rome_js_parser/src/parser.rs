//! The physical parser structure.
//! This may not hold your expectations of a traditional parser,
//! the parser yields events like `Start node`, `Error`, etc.
//! These events are then applied to a `TreeSink`.

use rome_parser::*;
pub(crate) mod rewrite_parser;
pub(crate) mod single_token_parse_recovery;

use crate::lexer::ReLexContext;
pub(crate) use crate::parser::parse_recovery::{ParseRecovery, RecoveryError, RecoveryResult};
use crate::prelude::*;
use crate::state::{ChangeParserState, ParserStateGuard};
use crate::*;
use crate::{
    state::ParserStateCheckpoint,
    token_source::{JsTokenSource, TokenSourceCheckpoint},
};
pub(crate) use parsed_syntax::ParsedSyntax;
use rome_js_syntax::{
    JsSyntaxKind::{self},
    SourceType,
};
use rome_parser::diagnostic::merge_diagnostics;
use rome_parser::event::Event;
use rome_parser::token_source::Trivia;
use rome_parser::{ParserContext, ParserContextCheckpoint};

/// An extremely fast, error tolerant, completely lossless JavaScript parser
///
/// The Parser yields lower level events instead of nodes.
/// These events are then processed into a syntax tree through a [`TreeSink`] implementation.
pub struct JsParser<'source> {
    pub(super) state: ParserState,
    pub source_type: SourceType,
    context: ParserContext<JsSyntaxKind>,
    source: JsTokenSource<'source>,
}

impl<'source> JsParser<'source> {
    /// Creates a new parser that parses the `source`.
    pub fn new(source: &'source str, source_type: SourceType) -> Self {
        let source = JsTokenSource::from_str(source);

        JsParser {
            state: ParserState::new(&source_type),
            source_type,
            context: ParserContext::default(),
            source,
        }
    }

    pub(crate) fn state(&self) -> &ParserState {
        &self.state
    }

    pub(crate) fn state_mut(&mut self) -> &mut ParserState {
        &mut self.state
    }

    pub fn source_type(&self) -> SourceType {
        self.source_type
    }

    /// Whether the code we are parsing is a module
    pub const fn is_module(&self) -> bool {
        self.source_type.module_kind().is_module()
    }

    /// Re-lexes the current token in the specified context. Returns the kind
    /// of the re-lexed token (can be the same as before if the context doesn't make a difference for the current token)
    pub fn re_lex(&mut self, context: ReLexContext) -> JsSyntaxKind {
        self.source_mut().re_lex(context)
    }

    /// Stores the parser state and position before calling the function and restores the state
    /// and position before returning.
    ///
    /// Useful in situation where the parser must advance a few tokens to determine whatever a syntax is
    /// of one or the other kind.
    #[inline]
    pub fn lookahead<F, R>(&mut self, op: F) -> R
    where
        F: FnOnce(&mut JsParser) -> R,
    {
        let checkpoint = self.checkpoint();
        let result = op(self);
        self.rewind(checkpoint);

        result
    }

    /// Applies the passed in change to the parser's state and reverts the
    /// changes when the returned [ParserStateGuard] goes out of scope.
    pub(crate) fn with_scoped_state<'p, C: ChangeParserState>(
        &'p mut self,
        change: C,
    ) -> ParserStateGuard<'p, 'source, C> {
        let snapshot = change.apply(self.state_mut());
        ParserStateGuard::new(self, snapshot)
    }

    /// Applies the passed in change to the parser state before applying the passed `func` and
    /// restores the state to before the change before returning the result.
    #[inline]
    pub(crate) fn with_state<C, F, R>(&mut self, change: C, func: F) -> R
    where
        C: ChangeParserState,
        F: FnOnce(&mut JsParser) -> R,
    {
        let snapshot = change.apply(self.state_mut());
        let result = func(self);
        C::restore(self.state_mut(), snapshot);
        result
    }

    pub fn checkpoint(&self) -> JsParserCheckpoint {
        JsParserCheckpoint {
            context: self.context.checkpoint(),
            source: self.source.checkpoint(),
            state: self.state.checkpoint(),
        }
    }

    pub fn rewind(&mut self, checkpoint: JsParserCheckpoint) {
        let JsParserCheckpoint {
            context,
            source,
            state,
        } = checkpoint;

        self.context.rewind(context);
        self.source.rewind(source);
        self.state.restore(state);
    }

    pub fn finish(self) -> (Vec<Event<JsSyntaxKind>>, Vec<Trivia>, Vec<ParseDiagnostic>) {
        let (trivia, source_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(source_diagnostics, parse_diagnostics);

        (events, trivia, diagnostics)
    }
}

impl<'source> Parser for JsParser<'source> {
    type Kind = JsSyntaxKind;
    type Source = JsTokenSource<'source>;

    fn context(&self) -> &ParserContext<Self::Kind> {
        &self.context
    }

    fn context_mut(&mut self) -> &mut ParserContext<Self::Kind> {
        &mut self.context
    }

    fn source(&self) -> &Self::Source {
        &self.source
    }

    fn source_mut(&mut self) -> &mut Self::Source {
        &mut self.source
    }

    fn is_speculative_parsing(&self) -> bool {
        self.state.speculative_parsing
    }

    fn do_bump_with_context(
        &mut self,
        kind: Self::Kind,
        context: <Self::Source as BumpWithContext>::Context,
    ) where
        Self::Source: BumpWithContext,
    {
        let kind = if kind.is_keyword() && self.source().has_unicode_escape() {
            self.error(self.err_builder(
                format!(
                    "'{}' keyword cannot contain escape character.",
                    kind.to_string().expect("to return a value for a keyword")
                ),
                self.cur_range(),
            ));
            JsSyntaxKind::ERROR_TOKEN
        } else {
            kind
        };

        let end = self.cur_range().end();
        self.context_mut().push_token(kind, end);

        if self.context().is_skipping() {
            self.source_mut().skip_as_trivia_with_context(context);
        } else {
            self.source_mut().bump_with_context(context);
        }
    }

    fn do_bump(&mut self, kind: Self::Kind) {
        self.do_bump_with_context(kind, LexContext::Regular)
    }
}

pub struct JsParserCheckpoint {
    pub(super) context: ParserContextCheckpoint,
    pub(super) source: TokenSourceCheckpoint,
    state: ParserStateCheckpoint,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::JsLosslessTreeSink;
    use rome_js_syntax::{JsSyntaxKind, SourceType};
    use rome_rowan::AstNode;

    #[test]
    fn example() {
        use crate::syntax::expr::parse_expression_snipped;
        use crate::JsParser;
        use rome_js_syntax::{AnyJsExpression, JsExpressionSnipped};

        let source = "(void b)";

        // File id is used for the labels inside parser errors to report them, the file id
        // is used to look up a file's source code and path inside of a codespan `Files` implementation.
        let mut parser = JsParser::new(source, SourceType::default());

        // Use one of the syntax parsing functions to parse an expression.
        // This adds node and token events to the parser which are then used to make a node.
        // A completed marker marks the start and end indices in the events vec which signify
        // the Start event, and the Finish event.
        // Completed markers can be turned into an ast node with parse_marker on the parser
        parse_expression_snipped(&mut parser).unwrap();

        // Consume the parser and get its events, then apply them to a tree sink with `process`.
        let (events, tokens, errors) = parser.finish();

        // Make a new text tree sink, its job is assembling events into a rowan GreenNode.
        // At each point (Start, Token, Finish, Error) it also consumes whitespace.
        // Other abstractions can also yield lossy syntax nodes if whitespace is not wanted.
        // Swap this for a LossyTreeSink for a lossy AST result.
        let mut sink = JsLosslessTreeSink::new(source, &tokens);

        rome_parser::event::process(&mut sink, events, errors);

        let (untyped_node, errors) = sink.finish();

        assert!(errors.is_empty());
        assert!(JsExpressionSnipped::can_cast(untyped_node.kind()));

        // Convert the untyped SyntaxNode into a typed AST node
        let expression_snipped = JsExpressionSnipped::cast(untyped_node).unwrap();
        let expression = expression_snipped.expression().unwrap();

        match expression {
            AnyJsExpression::JsParenthesizedExpression(parenthesized) => {
                assert_eq!(
                    parenthesized.expression().unwrap().syntax().text(),
                    "void b"
                );
            }
            _ => panic!("Expected parenthesized expression"),
        }
    }

    #[test]
    #[should_panic(
        expected = "Marker must either be `completed` or `abandoned` to avoid that children are implicitly attached to a marker's parent."
    )]
    fn uncompleted_markers_panic() {
        let mut parser = JsParser::new("'use strict'", SourceType::default());

        let _ = parser.start();
        // drop the marker without calling complete or abandon
    }

    #[test]
    fn completed_marker_doesnt_panic() {
        let mut p = JsParser::new("'use strict'", SourceType::default());

        let m = p.start();
        p.expect(JsSyntaxKind::JS_STRING_LITERAL);
        m.complete(&mut p, JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION);
    }

    #[test]
    fn abandoned_marker_doesnt_panic() {
        let mut p = JsParser::new("'use strict'", SourceType::default());

        let m = p.start();
        m.abandon(&mut p);
    }
}
