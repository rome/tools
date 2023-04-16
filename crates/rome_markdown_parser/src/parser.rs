use crate::token_source::MdTokenSource;
use rome_markdown_syntax::MdSyntaxKind;
use rome_parser::diagnostic::merge_diagnostics;
use rome_parser::event::Event;
use rome_parser::prelude::*;
use rome_parser::token_source::{TokenSource, Trivia};
use rome_parser::{Parser, ParserContext};

pub(crate) struct MarkdownParser<'source> {
    context: ParserContext<MdSyntaxKind>,
    source: MdTokenSource<'source>,
}

impl<'source> MarkdownParser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            context: ParserContext::default(),
            source: MdTokenSource::from_str(source),
        }
    }

    pub fn finish(self) -> (Vec<Event<MdSyntaxKind>>, Vec<ParseDiagnostic>, Vec<Trivia>) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        (events, diagnostics, trivia)
    }
}

impl<'source> Parser for MarkdownParser<'source> {
    type Kind = MdSyntaxKind;
    type Source = MdTokenSource<'source>;

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
}
