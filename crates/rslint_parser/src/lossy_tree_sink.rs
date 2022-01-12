use crate::{
	JsSyntaxKind, ParserError, SyntaxNode, SyntaxTreeBuilder, TextRange, TextSize, TreeSink,
};
use rome_rowan::TriviaPiece;
use rslint_lexer::Token;
use std::mem;

/// Structure to convert events to a lossy syntax tree which does not preserve whitespace.
#[derive(Debug)]
pub struct LossyTreeSink<'a> {
	text: &'a str,
	tokens: &'a [Token],
	text_pos: TextSize,
	token_pos: usize,
	state: State,
	inner: SyntaxTreeBuilder,
	errors: Vec<ParserError>,
	/// Signal that the sink must generate an EOF token when its finishing. See [LosslessTreeSink::finish] for more details.
	needs_eof: bool,
	/// Trivia start offset and its [TriviaPiece].
	next_token_leading_trivia: (TextRange, Vec<TriviaPiece>),
}

#[derive(Debug, Clone, Copy)]
enum State {
	PendingStart,
	Normal,
	PendingFinish,
}

impl<'a> TreeSink for LossyTreeSink<'a> {
	fn consume_multiple_tokens(&mut self, amount: u8, kind: JsSyntaxKind) {
		match mem::replace(&mut self.state, State::Normal) {
			State::PendingStart => unreachable!(),
			State::PendingFinish => self.inner.finish_node(),
			State::Normal => (),
		}

		let len = TextSize::from(
			self.tokens[self.token_pos..self.token_pos + amount as usize]
				.iter()
				.map(|x| x.len)
				.sum::<u32>(),
		);

		self.do_tokens(kind, len, amount)
	}

	fn token(&mut self, kind: JsSyntaxKind) {
		match mem::replace(&mut self.state, State::Normal) {
			State::PendingStart => unreachable!(),
			State::PendingFinish => self.inner.finish_node(),
			State::Normal => (),
		}

		let len = TextSize::from(self.tokens[self.token_pos].len as u32);
		self.do_token(kind, len);
	}

	fn start_node(&mut self, kind: JsSyntaxKind) {
		match mem::replace(&mut self.state, State::Normal) {
			State::PendingStart => {
				self.inner.start_node(kind);
				self.next_token_leading_trivia = self.get_trivia(false);
				return;
			}
			State::PendingFinish => self.inner.finish_node(),
			State::Normal => (),
		}

		self.inner.start_node(kind);
	}

	fn finish_node(&mut self) {
		match mem::replace(&mut self.state, State::PendingFinish) {
			State::PendingStart => unreachable!(),
			State::PendingFinish => self.inner.finish_node(),
			State::Normal => (),
		}
	}

	fn errors(&mut self, errors: Vec<ParserError>) {
		self.errors = errors;
	}
}

impl<'a> LossyTreeSink<'a> {
	pub fn new(text: &'a str, tokens: &'a [Token]) -> Self {
		Self {
			text,
			tokens,
			text_pos: 0.into(),
			token_pos: 0,
			state: State::PendingStart,
			inner: SyntaxTreeBuilder::default(),
			errors: vec![],
			needs_eof: true,
			next_token_leading_trivia: (TextRange::at(0.into(), 0.into()), vec![]),
		}
	}

	/// Make a new tree sink but start the sink at a specific token, this is used for making completed markers
	/// into AST nodes for rules which need them.
	///
	/// # Panics
	/// Panics if the token start does not line up to a token's start index or is out of bounds
	pub fn with_offset(text: &'a str, tokens: &'a [Token], token_start: u32) -> Self {
		let mut len = 0;
		for (idx, tok) in tokens.iter().enumerate() {
			if len == token_start {
				return Self {
					text,
					tokens,
					text_pos: (len as u32).into(),
					token_pos: idx,
					state: State::PendingStart,
					inner: SyntaxTreeBuilder::default(),
					errors: vec![],
					needs_eof: true,
					next_token_leading_trivia: (TextRange::at(0.into(), 0.into()), vec![]),
				};
			}
			len += tok.len;
		}
		panic!("Token start does not line up to a token or is out of bounds")
	}

	/// Finishes the tree and return the root node with possible parser errors.
	///
	/// If tree is finished with pending trivia, but no tokens were generated, for example,
	/// a completely commented file, a [SyntaxKind::EOF] will be generated and all pending trivia
	/// will be appended to its leading trivia.
	pub fn finish(mut self) -> (SyntaxNode, Vec<ParserError>) {
		if self.needs_eof {
			self.do_token(JsSyntaxKind::EOF, 0.into());
		}

		match mem::replace(&mut self.state, State::Normal) {
			State::PendingFinish => self.inner.finish_node(),
			State::PendingStart | State::Normal => unreachable!(),
		}

		(self.inner.finish(), self.errors)
	}

	fn is_eof(&self) -> bool {
		match self.tokens.get(self.token_pos) {
			Some(token) if token.kind == JsSyntaxKind::EOF => true,
			None => true,
			_ => false,
		}
	}

	#[inline]
	fn do_token(&mut self, kind: JsSyntaxKind, len: TextSize) {
		self.do_tokens(kind, len, 1)
	}

	fn do_tokens(&mut self, kind: JsSyntaxKind, len: TextSize, token_count: u8) {
		let token_range = TextRange::at(self.text_pos, len);

		self.text_pos += len;
		self.token_pos += token_count as usize;

		// Everything until the next linebreak (but not including it)
		// will be the trailing trivia...
		let (mut trailing_range, mut trailing) = self.get_trivia(true);

		// ... and everything after and including the linebreak will be in the next
		// token leading trivia...
		let next_token_leading = {
			let (range, pieces) = self.get_trivia(false);
			// ... unless there is no more tokens. Then treat the remaining
			// trivia as the trailing of the last one.
			// See "finish_node" for when we do not have any tokens.
			if self.is_eof() {
				trailing_range = trailing_range.cover(range);
				trailing.extend(pieces);
				(TextRange::new(0.into(), 0.into()), vec![])
			} else {
				(range, pieces)
			}
		};

		let (leading_range, leading) =
			std::mem::replace(&mut self.next_token_leading_trivia, next_token_leading);

		let range = leading_range.cover(token_range).cover(trailing_range);
		let text = &self.text[range];

		self.inner.token_with_trivia(kind, text, &leading, &trailing);
	}

	fn get_trivia(&mut self, break_on_newline: bool) -> (TextRange, Vec<TriviaPiece>) {
		let mut trivia = vec![];

		let start_text_pos = self.text_pos;
		let mut length = TextSize::of("");

		while let Some(&token) = self.tokens.get(self.token_pos) {
			if !token.kind.is_trivia() {
				break;
			}

			if break_on_newline && token.kind == JsSyntaxKind::NEWLINE {
				break;
			}

			self.token_pos += 1;
			let len = TextSize::from(token.len as u32);
			self.text_pos += len;
			length += len;

			let current_trivia = match token.kind {
				JsSyntaxKind::WHITESPACE | JsSyntaxKind::NEWLINE => continue,
				JsSyntaxKind::COMMENT => TriviaPiece::Comments(token.len, false),
				JsSyntaxKind::MULTILINE_COMMENT => TriviaPiece::Comments(token.len, true),
				_ => unreachable!("Not Trivia"),
			};

			trivia.push(current_trivia);
		}

		(TextRange::at(start_text_pos, length), trivia)
	}
}
