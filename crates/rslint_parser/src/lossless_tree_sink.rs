use crate::token_source::Trivia;
use crate::{ParseDiagnostic, TreeSink};
use rome_js_syntax::{JsSyntaxKind, SyntaxNode, SyntaxTreeBuilder, TextRange, TextSize};
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
    inner: SyntaxTreeBuilder,
    /// Signal that the sink must generate an EOF token when its finishing. See [LosslessTreeSink::finish] for more details.
    needs_eof: bool,
    trivia_pieces: Vec<TriviaPiece>,
}

impl<'a> TreeSink for LosslessTreeSink<'a> {
    fn token(&mut self, kind: JsSyntaxKind, length: TextSize) {
        self.do_token(kind, length);
    }

    fn start_node(&mut self, kind: JsSyntaxKind) {
        self.inner.start_node(kind);
        self.parents_count += 1;
    }

    fn finish_node(&mut self) {
        self.parents_count -= 1;

        if self.parents_count == 0 && self.needs_eof {
            self.do_token(JsSyntaxKind::EOF, TextSize::default());
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
            inner: SyntaxTreeBuilder::default(),
            errors: vec![],
            needs_eof: true,
            trivia_pieces: Vec::with_capacity(128),
        }
    }

    /// Finishes the tree and return the root node with possible parser errors.
    ///
    /// If tree is finished without a [SyntaxKind::EOF], one will be generated and all pending trivia
    /// will be appended to its leading trivia.
    pub fn finish(self) -> (SyntaxNode, Vec<ParseDiagnostic>) {
        (self.inner.finish(), self.errors)
    }

    #[inline]
    fn do_token(&mut self, kind: JsSyntaxKind, length: TextSize) {
        if kind == JsSyntaxKind::EOF {
            self.needs_eof = false;
        }

        // Every trivia up to the token (including line breaks) will be the leading trivia
        self.trivia_pieces.clear();
        let (leading_range, leading_end) = self.get_trivia(false);

        let token_range = TextRange::at(self.text_pos, length);
        self.text_pos += length;

        // Everything until the next linebreak (but not including it)
        // will be the trailing trivia...
        let trailing_start = self.trivia_pieces.len();
        let (trailing_range, _) = self.get_trivia(true);

        let range = leading_range.cover(token_range).cover(trailing_range);

        let text = &self.text[range];

        let leading = &self.trivia_pieces[0..leading_end];
        let trailing = &self.trivia_pieces[trailing_start..];

        self.inner.token_with_trivia(kind, text, leading, trailing);
    }

    fn get_trivia(&mut self, trailing: bool) -> (TextRange, usize) {
        let start_text_pos = self.text_pos;

        let mut count = 0;
        for trivia in &self.trivia_list[self.trivia_pos..] {
            if trailing != trivia.trailing() || self.text_pos != trivia.offset() {
                break;
            }

            self.text_pos += trivia.len();

            let trivia_piece = TriviaPiece::new(trivia.kind(), trivia.len());
            self.trivia_pieces.push(trivia_piece);
            count += 1;
        }

        self.trivia_pos += count;

        (TextRange::new(start_text_pos, self.text_pos), count)
    }
}
