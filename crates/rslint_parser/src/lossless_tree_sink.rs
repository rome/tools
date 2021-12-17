use crate::{
	JsSyntaxKind::{self, *},
	ParserError, SyntaxNode, SyntaxTreeBuilder, TextRange, TextSize, TreeSink,
};
use rome_rowan::TriviaPiece;
use rslint_lexer::Token;

/// Structure for converting events to a syntax tree representation, while preserving whitespace.
///
/// `LosslessTreeSink` also handles attachment of trivia (whitespace) to nodes.
#[derive(Debug)]
pub struct LosslessTreeSink<'a> {
	text: &'a str,
	tokens: &'a [Token],
	text_pos: TextSize,
	token_pos: usize,
	parents_count: usize,
	errors: Vec<ParserError>,
	inner: SyntaxTreeBuilder,
	/// Signal that the sink must generate an EOF token when its finishing. See [LosslessTreeSink::finish] for more details.
	needs_eof: bool,
	/// Trivia start Offset and its pieces.
	next_token_leading_trivia: (TextRange, Vec<TriviaPiece>),
}

impl<'a> TreeSink for LosslessTreeSink<'a> {
	fn consume_multiple_tokens(&mut self, amount: u8, kind: JsSyntaxKind) {
		let len = TextSize::from(
			self.tokens[self.token_pos..self.token_pos + amount as usize]
				.iter()
				.map(|x| x.len)
				.sum::<usize>() as u32,
		);

		self.do_tokens(kind, len, amount)
	}

	fn token(&mut self, kind: JsSyntaxKind) {
		let len = TextSize::from(self.tokens[self.token_pos].len as u32);
		self.do_token(kind, len);
	}

	fn start_node(&mut self, kind: JsSyntaxKind) {
		self.inner.start_node(kind);
		if self.parents_count == 0 {
			self.next_token_leading_trivia = self.get_trivia(false);
		}

		self.parents_count += 1;
	}

	fn finish_node(&mut self) {
		self.parents_count -= 1;

		if self.parents_count == 0 && self.needs_eof {
			self.do_token(JsSyntaxKind::EOF, 0.into());
		}

		self.inner.finish_node();
	}

	fn errors(&mut self, errors: Vec<ParserError>) {
		self.errors = errors;
	}
}

impl<'a> LosslessTreeSink<'a> {
	pub fn new(text: &'a str, tokens: &'a [Token]) -> Self {
		Self {
			text,
			tokens,
			text_pos: 0.into(),
			token_pos: 0,
			parents_count: 0,
			inner: SyntaxTreeBuilder::default(),
			errors: vec![],
			needs_eof: true,
			next_token_leading_trivia: (TextRange::at(0.into(), 0.into()), vec![]),
		}
	}

	/// Finishes the tree and return the root node with possible parser errors.
	///
	/// If tree is finished without a [SyntaxKind::EOF], one will be generated and all pending trivia
	/// will be appended to its leading trivia.
	pub fn finish(self) -> (SyntaxNode, Vec<ParserError>) {
		(self.inner.finish(), self.errors)
	}

	#[inline]
	fn do_token(&mut self, kind: JsSyntaxKind, len: TextSize) {
		if kind == JsSyntaxKind::EOF {
			self.needs_eof = false;
		}

		self.do_tokens(kind, len, 1)
	}

	fn do_tokens(&mut self, kind: JsSyntaxKind, len: TextSize, token_count: u8) {
		let token_range = TextRange::at(self.text_pos, len);

		self.text_pos += len;
		self.token_pos += token_count as usize;

		// Everything until the next linebreak (but not including it)
		// will be the trailing trivia...
		let (trailing_range, trailing) = self.get_trivia(true);

		// ... and everything after and including the linebreak will be in the next
		// token leading trivia. If there is none, we will use the EOF.
		let next_token_leading = self.get_trivia(false);

		let (leading_range, leading) =
			std::mem::replace(&mut self.next_token_leading_trivia, next_token_leading);

		let range = leading_range.cover(token_range).cover(trailing_range);
		let text = &self.text[range];

		self.inner.token_with_trivia(kind, text, leading, trailing);
	}

	fn get_trivia(&mut self, break_on_newline: bool) -> (TextRange, Vec<TriviaPiece>) {
		let mut trivia = vec![];

		let start_text_pos = self.text_pos;
		let mut length = TextSize::of("");

		while let Some(&token) = self.tokens.get(self.token_pos) {
			if !token.kind.is_trivia() {
				break;
			}

			let pos: u32 = self.text_pos.into();
			let pos = pos as usize;
			let text = &self.text[pos..(pos + token.len)];
			if break_on_newline && text.chars().any(rslint_lexer::is_linebreak) {
				break;
			}

			self.token_pos += 1;
			let len = TextSize::from(token.len as u32);
			self.text_pos += len;
			length += len;

			let current_trivia = match token.kind {
				WHITESPACE => TriviaPiece::Whitespace(token.len),
				COMMENT => TriviaPiece::Comments(token.len),
				_ => unreachable!("Not Trivia"),
			};

			trivia.push(current_trivia);
		}

		(TextRange::at(start_text_pos, length), trivia)
	}
}
