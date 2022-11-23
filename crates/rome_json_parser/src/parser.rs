use crate::token_source::JsonTokenSource;
use rome_diagnostics::FileId;
use rome_json_syntax::JsonSyntaxKind;
use rome_parser::event::Event;
use rome_parser::prelude::*;
use rome_parser::token_source::Trivia;
use rome_parser::ParserContext;

pub(crate) struct JsonParser<'source> {
    context: ParserContext<JsonSyntaxKind>,
    source: JsonTokenSource<'source>,
}

impl<'source> JsonParser<'source> {
    pub fn new(source: &'source str, file_id: FileId) -> Self {
        Self {
            context: ParserContext::new(file_id),
            source: JsonTokenSource::from_str(source, file_id),
        }
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<JsonSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (events, parse_diagnostics) = self.context.finish();
        let (trivia, mut lexer_diagnostics) = self.source.finish();

        lexer_diagnostics.extend(parse_diagnostics);

        (events, lexer_diagnostics, trivia)
    }
}

impl<'source> Parser for JsonParser<'source> {
    type Kind = JsonSyntaxKind;
    type Source = JsonTokenSource<'source>;

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
