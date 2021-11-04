use crate::{
	ast, AstNode, ParserError,
	SyntaxKind::{self, *},
	SyntaxNode, SyntaxTreeBuilder, TextRange, TextSize, TreeSink,
};
use rome_rowan::{GreenTokenTrivia, Trivia};
use rslint_lexer::Token;
use std::mem;

/// Structure for converting events to a syntax tree representation, while preserving whitespace.
///
/// `LosslessTreeSink` also handles attachment of trivia (whitespace) to nodes.
#[derive(Debug)]
pub struct LosslessTreeSink<'a> {
	text: &'a str,
	tokens: &'a [Token],
	text_pos: TextSize,
	token_pos: usize,
	state: State,
	errors: Vec<ParserError>,
	inner: SyntaxTreeBuilder,
	next_token_leading_trivia: GreenTokenTrivia,
}

#[derive(Debug, Clone, Copy)]
enum State {
	PendingStart,
	Normal,
	PendingFinish,
}

impl<'a> TreeSink for LosslessTreeSink<'a> {
	fn consume_multiple_tokens(&mut self, amount: u8, kind: SyntaxKind) {
		//println!(
		// 	"LosslessTreeSink::consume_multiple_tokens: {} {:?} {:?}",
		// 	amount, kind, self.state
		// );
		match mem::replace(&mut self.state, State::Normal) {
			State::PendingStart => unreachable!(),
			State::PendingFinish => self.inner.finish_node(),
			State::Normal => (),
		}
		let len = TextSize::from(
			self.tokens[self.token_pos..self.token_pos + amount as usize]
				.iter()
				.map(|x| x.len)
				.sum::<usize>() as u32,
		);

		let range = TextRange::at(self.text_pos, len);
		let text = &self.text[range];
		self.text_pos += len;
		self.token_pos += amount as usize;
		self.inner.token(kind, text);
	}

	fn token(&mut self, kind: SyntaxKind) {
		println!("LosslessTreeSink::token: {:?} {:?}", kind, self.text_pos);
		match mem::replace(&mut self.state, State::Normal) {
			State::PendingStart => unreachable!(),
			State::PendingFinish => self.inner.finish_node(),
			State::Normal => (),
		}
		let len = TextSize::from(self.tokens[self.token_pos].len as u32);

		self.do_token(kind, len);
	}

	fn missing(&mut self) {
		match mem::replace(&mut self.state, State::Normal) {
			State::PendingStart => unreachable!(),
			State::PendingFinish => self.inner.finish_node(),
			State::Normal => (),
		}

		self.inner.missing();
	}

	// TODO: Attach comment whitespace to nodes
	fn start_node(&mut self, kind: SyntaxKind) {
		//println!("LosslessTreeSink::start_node: {:?} {:?}", kind, self.state);
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
		//println!("LosslessTreeSink::finish_node");
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

impl<'a> LosslessTreeSink<'a> {
	pub fn new(text: &'a str, tokens: &'a [Token]) -> Self {
		Self {
			text,
			tokens,
			text_pos: 0.into(),
			token_pos: 0,
			state: State::PendingStart,
			inner: SyntaxTreeBuilder::default(),
			errors: vec![],
			next_token_leading_trivia: GreenTokenTrivia::None,
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
					next_token_leading_trivia: GreenTokenTrivia::None,
				};
			}
			len += tok.len;
		}
		panic!("Token start does not line up to a token or is out of bounds")
	}

	pub fn finish(mut self) -> (SyntaxNode, Vec<ParserError>) {
		match mem::replace(&mut self.state, State::Normal) {
			State::PendingFinish => self.inner.finish_node(),
			State::PendingStart | State::Normal => unreachable!(),
		}

		(self.inner.finish(), self.errors)
	}

	fn do_token(&mut self, kind: SyntaxKind, len: TextSize) {
		println!(
			"LosslessTreeSink::do_token: {:?} len {:?} pos {:?}",
			kind, len, self.text_pos
		);
		let range = TextRange::at(self.text_pos, len);
		let text = &self.text[range];
		self.text_pos += len;
		self.token_pos += 1;

		let trailing = self.get_trivia(true);
		let leading = self.get_trivia(false);
		let leading = std::mem::replace(&mut self.next_token_leading_trivia, leading);
		self.inner.token_with_trivia(kind, text, leading, trailing);
	}

	fn get_trivia(&mut self, break_on_newline: bool) -> rome_rowan::GreenTokenTrivia {
		use rome_rowan::{GreenTokenTrivia, Trivia};

		let mut trivia = GreenTokenTrivia::None;

		while let Some(&token) = self.tokens.get(self.token_pos) {
			if !token.kind.is_trivia() {
				break;
			}

			//TODO ask if is new line
			let pos: u32 = self.text_pos.into();
			let pos = pos as usize;
			let text = &self.text[pos..(pos + token.len)];
			if break_on_newline && text.contains("\n") {
				break;
			}

			self.token_pos += 1;
			let len = TextSize::from(token.len as u32);
			self.text_pos += len;

			let current_trivia = match token.kind {
				WHITESPACE => Trivia::Whitespace(token.len).as_thin(text),
				COMMENT => Trivia::Comment(token.len).as_thin(text),
				_ => unreachable!("Not Trivia"),
			};

			trivia = match (trivia, current_trivia) {
				(GreenTokenTrivia::None, fist) => GreenTokenTrivia::One(fist),
				(GreenTokenTrivia::One(fist), second) => GreenTokenTrivia::Many(vec![fist, second]),
				(GreenTokenTrivia::Many(mut v), second) => {
					v.push(second);
					GreenTokenTrivia::Many(v)
				}
			}
		}

		trivia
	}
}
