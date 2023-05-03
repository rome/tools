use crate::token_source::JsonTokenSource;
use rome_json_syntax::JsonSyntaxKind;
use rome_parser::diagnostic::{expected_token, merge_diagnostics};
use rome_parser::event::Event;
use rome_parser::prelude::*;
use rome_parser::token_source::Trivia;
use rome_parser::ParserContext;

pub(crate) struct JsonParser<'source> {
    context: ParserContext<JsonSyntaxKind>,
    source: JsonTokenSource<'source>,
    config: JsonParserConfig,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct JsonParserConfig {
    pub allow_comments: bool,
}

impl<'source> JsonParser<'source> {
    pub fn new(source: &'source str, config: JsonParserConfig) -> Self {
        Self {
            context: ParserContext::default(),
            source: JsonTokenSource::from_str(source, config),
            config,
        }
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<JsonSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);

        (events, diagnostics, trivia)
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

    /// Try to eat a specific token kind, if the kind is not there then adds an error to the events stack.
    fn expect(&mut self, kind: Self::Kind) -> bool {
        if self.eat(kind) {
            true
        } else {
            if self.config.allow_comments {
                println!("something");
                while matches!(
                    kind,
                    JsonSyntaxKind::COMMENT | JsonSyntaxKind::MULTILINE_COMMENT
                ) {
                    self.bump_any();
                }
            }
            if self.eat(kind) {
                return true;
            }

            self.error(expected_token(kind));
            false
        }
    }
}
