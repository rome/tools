use crate::lexer::Lexer;
use rome_markdown_syntax::MdSyntaxKind;
use rome_parser::diagnostic::ParseDiagnostic;
use rome_parser::prelude::TokenSource;
use rome_parser::token_source::Trivia;

pub(crate) struct MdTokenSource<'source> {
    lexer: Lexer<'source>,
    trivia: Vec<Trivia>,
    current: MdSyntaxKind,
    current_range: rome_markdown_syntax::TextRange,
    preceding_line_break: bool,
}

impl<'source> MdTokenSource<'source> {
    pub fn from_str(source: &'source str) -> Self {
        let lexer = Lexer::from_str(source);

        let mut source = Self {
            lexer,
            trivia: Vec::new(),
            current: rome_markdown_syntax::MdSyntaxKind::TOMBSTONE,
            current_range: rome_markdown_syntax::TextRange::default(),
            preceding_line_break: false,
        };

        source.next_non_trivia_token(true);
        source
    }

    fn next_non_trivia_token(&mut self, first_token: bool) {
        let mut trailing = !first_token;
        self.preceding_line_break = false;

        while let Some(token) = self.lexer.next_token() {
            let trivia_kind = rome_markdown_syntax::TriviaPieceKind::try_from(token.kind());

            match trivia_kind {
                Err(_) => {
                    self.current = token.kind();
                    self.current_range = token.range();

                    // Not trivia
                    break;
                }
                Ok(trivia_kind) => {
                    if trivia_kind.is_newline() {
                        trailing = false;
                        self.preceding_line_break = true;
                    }

                    self.trivia
                        .push(Trivia::new(trivia_kind, token.range(), trailing));
                }
            }
        }
    }
}

impl<'source> TokenSource for MdTokenSource<'source> {
    type Kind = MdSyntaxKind;

    fn current(&self) -> Self::Kind {
        self.current
    }

    fn current_range(&self) -> rome_markdown_syntax::TextRange {
        self.current_range
    }

    fn text(&self) -> &str {
        self.lexer.source()
    }

    fn has_preceding_line_break(&self) -> bool {
        self.preceding_line_break
    }

    fn bump(&mut self) {
        if self.current != rome_markdown_syntax::MdSyntaxKind::EOF {
            self.next_non_trivia_token(false)
        }
    }

    fn skip_as_trivia(&mut self) {
        if self.current() != rome_markdown_syntax::MdSyntaxKind::EOF {
            self.trivia.push(Trivia::new(
                rome_markdown_syntax::TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));

            self.next_non_trivia_token(false)
        }
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia, self.lexer.finish())
    }
}
