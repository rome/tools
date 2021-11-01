use crate::{
	ParserError, SyntaxKind, SyntaxNode, SyntaxTreeBuilder, TextRange, TextSize, TreeSink,
};
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
}

#[derive(Debug, Clone, Copy)]
enum State {
	PendingStart,
	Normal,
	PendingFinish,
}

impl<'a> TreeSink for LossyTreeSink<'a> {
	fn consume_multiple_tokens(&mut self, amount: u8, kind: SyntaxKind) {
		match mem::replace(&mut self.state, State::Normal) {
			State::PendingStart => unreachable!(),
			State::PendingFinish => self.inner.finish_node(),
			State::Normal => (),
		}
		self.eat_trivias();
		let len = TextSize::from(
			self.tokens[self.token_pos..amount as usize]
				.iter()
				.map(|x| x.len)
				.sum::<usize>() as u32,
		);

		let range = TextRange::at(self.text_pos, len);
		let text = &self.text[range];
		self.text_pos += len;
		self.token_pos += amount as usize;
		self.inner.token(kind, text, vec![], vec![]);
	}

	fn token(&mut self, kind: SyntaxKind) {
		match mem::replace(&mut self.state, State::Normal) {
			State::PendingStart => unreachable!(),
			State::PendingFinish => self.inner.finish_node(),
			State::Normal => (),
		}
		self.eat_trivias();
		let len = TextSize::from(self.tokens[self.token_pos].len as u32);
		self.do_token(kind, len, false);
	}

	fn missing(&mut self) {
		match mem::replace(&mut self.state, State::Normal) {
			State::PendingStart => unreachable!(),
			State::PendingFinish => self.inner.finish_node(),
			State::Normal => (),
		}

		self.inner.missing();
	}

	fn start_node(&mut self, kind: SyntaxKind) {
		match mem::replace(&mut self.state, State::Normal) {
			State::PendingStart => {
				self.inner.start_node(kind);
				return;
			}
			State::PendingFinish => self.inner.finish_node(),
			State::Normal => (),
		}

		let n_trivias = self.tokens[self.token_pos..]
			.iter()
			.take_while(|it| it.kind.is_trivia())
			.count();

		self.eat_n_trivias(n_trivias);
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
		}
	}

	/// Make a new tree sink but start the sink at a specific token, this is used for making completed markers
	/// into AST nodes for rules which need them.
	///
	/// # Panics
	/// Panics if the token start does not line up to a token's start index or is out of bounds
	pub fn with_offset(text: &'a str, tokens: &'a [Token], token_start: usize) -> Self {
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
				};
			}
			len += tok.len;
		}
		panic!("Token start does not line up to a token or is out of bounds")
	}

	pub fn finish(mut self) -> (SyntaxNode, Vec<ParserError>) {
		match mem::replace(&mut self.state, State::Normal) {
			State::PendingFinish => {
				self.eat_trivias();
				self.inner.finish_node()
			}
			State::PendingStart | State::Normal => unreachable!(),
		}

		(self.inner.finish(), self.errors)
	}

	fn eat_trivias(&mut self) {
		while let Some(&token) = self.tokens.get(self.token_pos) {
			if !token.kind.is_trivia() {
				break;
			}
			self.do_token(token.kind, TextSize::from(token.len as u32), true);
		}
	}

	fn eat_n_trivias(&mut self, n: usize) {
		for _ in 0..n {
			let token = self.tokens[self.token_pos];
			assert!(token.kind.is_trivia());
			self.do_token(token.kind, TextSize::from(token.len as u32), true);
		}
	}

	fn do_token(&mut self, kind: SyntaxKind, len: TextSize, skip: bool) {
		let range = TextRange::at(self.text_pos, len);
		let text = &self.text[range];
		self.text_pos += len;
		self.token_pos += 1;
		if !skip {
			self.inner.token(kind, text, vec![], vec![]);
		}
	}
}
