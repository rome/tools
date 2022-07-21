use crate::token_source::Trivia;
use crate::{ParseDiagnostic, TreeSink};
use rome_js_factory::JsSyntaxTreeBuilder;
use rome_js_syntax::{JsSyntaxKind, JsSyntaxNode, TextRange, TextSize};
use rome_rowan::TriviaPiece;

/// Structure for converting events to a syntax tree representation, while preserving whitespace.
///
/// `LosslessTreeSink` also handles attachment of trivia (whitespace) to nodes.
#[derive(Debug)]
pub struct LosslessTreeSink<'a> {
    text: &'a str,
    trivia_list: &'a [Trivia],
    text_pos: TextSize,
    trivia_pos: usize,
    parents_count: usize,
    errors: Vec<ParseDiagnostic>,
    inner: JsSyntaxTreeBuilder,
    /// Signal that the sink must generate an EOF token when its finishing. See [LosslessTreeSink::finish] for more details.
    needs_eof: bool,
    trivia_pieces: Vec<TriviaPiece>,
}

impl<'a> TreeSink for LosslessTreeSink<'a> {
    fn token(&mut self, kind: JsSyntaxKind, end: TextSize) {
        self.do_token(kind, end);
    }

    fn start_node(&mut self, kind: JsSyntaxKind) {
        self.inner.start_node(kind);
        self.parents_count += 1;
    }

    fn finish_node(&mut self) {
        self.parents_count -= 1;

        if self.parents_count == 0 && self.needs_eof {
            self.do_token(JsSyntaxKind::EOF, TextSize::from(self.text.len() as u32));
        }

        self.inner.finish_node();
    }

    fn errors(&mut self, errors: Vec<ParseDiagnostic>) {
        self.errors = errors;
    }
}

impl<'a> LosslessTreeSink<'a> {
    pub fn new(text: &'a str, trivia: &'a [Trivia]) -> Self {
        Self {
            text,
            trivia_list: trivia,
            text_pos: 0.into(),
            trivia_pos: 0,
            parents_count: 0,
            inner: JsSyntaxTreeBuilder::default(),
            errors: vec![],
            needs_eof: true,
            trivia_pieces: Vec::with_capacity(128),
        }
    }

    /// Finishes the tree and return the root node with possible parser errors.
    ///
    /// If tree is finished without a [rome_js_syntax::JsSyntaxKind::EOF], one will be generated and all pending trivia
    /// will be appended to its leading trivia.
    pub fn finish(self) -> (JsSyntaxNode, Vec<ParseDiagnostic>) {
        (self.inner.finish(), self.errors)
    }

    #[inline]
    fn do_token(&mut self, kind: JsSyntaxKind, token_end: TextSize) {
        if kind == JsSyntaxKind::EOF {
            self.needs_eof = false;
        }

        let token_start = self.text_pos;

        // Every trivia up to the token (including line breaks) will be the leading trivia
        self.eat_trivia(false);
        let trailing_start = self.trivia_pieces.len();

        self.text_pos = token_end;

        // Everything until the next linebreak (but not including it)
        // will be the trailing trivia...
        self.eat_trivia(true);

        let token_range = TextRange::new(token_start, self.text_pos);

        let text = &self.text[token_range];
        let leading = &self.trivia_pieces[0..trailing_start];
        let trailing = &self.trivia_pieces[trailing_start..];

        self.inner.token_with_trivia(kind, text, leading, trailing);
        self.trivia_pieces.clear();
    }

    fn eat_trivia(&mut self, trailing: bool) {
        for trivia in &self.trivia_list[self.trivia_pos..] {
            if trailing != trivia.trailing() || self.text_pos != trivia.offset() {
                break;
            }

            let trivia_piece = TriviaPiece::new(trivia.kind(), trivia.len());
            self.trivia_pieces.push(trivia_piece);

            self.text_pos += trivia.len();
            self.trivia_pos += 1;
        }
    }
}
