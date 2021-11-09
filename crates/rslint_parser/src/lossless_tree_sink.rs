use crate::{
	ast, AstNode, ParserError,
	SyntaxKind::{self, *},
	SyntaxNode, SyntaxTreeBuilder, TextRange, TextSize, TreeSink,
};
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
}

#[derive(Debug, Clone, Copy)]
enum State {
	PendingStart,
	Normal,
	PendingFinish,
}

impl<'a> TreeSink for LosslessTreeSink<'a> {
	fn consume_multiple_tokens(&mut self, amount: u8, kind: SyntaxKind) {
		match mem::replace(&mut self.state, State::Normal) {
			State::PendingStart => unreachable!(),
			State::PendingFinish => self.inner.finish_node(),
			State::Normal => (),
		}
		self.eat_trivias();
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
		match mem::replace(&mut self.state, State::Normal) {
			State::PendingStart => unreachable!(),
			State::PendingFinish => self.inner.finish_node(),
			State::Normal => (),
		}
		self.eat_trivias();
		if self.tokens.get(self.token_pos).is_none() {
			println!("{:#?}", self.tokens);
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
		match mem::replace(&mut self.state, State::Normal) {
			State::PendingStart => {
				self.inner.start_node(kind);
				// No need to attach trivias to previous node: there is no
				// previous node.
				return;
			}
			State::PendingFinish => self.inner.finish_node(),
			State::Normal => (),
		}

		// If this is a statement then attach a leading comment to it
		let n_trivias = self.tokens[self.token_pos..]
			.iter()
			.take_while(|it| it.kind.is_trivia())
			.count();
		let leading_trivias = &self.tokens[self.token_pos..self.token_pos + n_trivias];
		let mut trivia_end = self.text_pos
			+ leading_trivias
				.iter()
				.map(|it| TextSize::from(it.len as u32))
				.sum::<TextSize>()
			+ TextSize::from(1);

		let n_attached_trivias = {
			let leading_trivias = leading_trivias.iter().rev().map(|it| {
				let next_end = trivia_end - TextSize::from(it.len as u32);
				let (start, end) = (next_end.into(), trivia_end.into());
				trivia_end = next_end;
				(
					it.kind,
					self.text
						.get(start..end)
						.unwrap_or_else(|| self.text.get(start - 1..end).unwrap_or_else(|| "")),
				)
			});
			n_attached_trivias(kind, leading_trivias.rev())
		};
		self.eat_n_trivias(n_trivias - n_attached_trivias);
		self.inner.start_node(kind);
		self.eat_n_trivias(n_attached_trivias);
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
			self.do_token(token.kind, TextSize::from(token.len as u32));
		}
	}

	fn eat_n_trivias(&mut self, n: usize) {
		for _ in 0..n {
			let token = self.tokens[self.token_pos];
			assert!(token.kind.is_trivia());
			self.do_token(token.kind, TextSize::from(token.len as u32));
		}
	}

	fn do_token(&mut self, kind: SyntaxKind, len: TextSize) {
		let range = TextRange::at(self.text_pos, len);
		let text = &self.text[range];
		self.text_pos += len;
		self.token_pos += 1;
		self.inner.token(kind, text);
	}
}

/// We need to attach any comment to statements
fn n_attached_trivias<'a>(
	kind: SyntaxKind,
	trivias: impl Iterator<Item = (SyntaxKind, &'a str)>,
) -> usize {
	if ast::JsAnyStatement::can_cast(kind) {
		let mut trivias = trivias.enumerate().peekable();

		match trivias.next() {
			Some((idx, (kind, text))) => match kind {
				WHITESPACE => {
					if linebreak_count(text) > 1 {
						return 0;
					} else if trivias
						.peek()
						.map_or(false, |(_, (kind, _))| *kind == COMMENT)
					{
						return trivias.next().unwrap().0 + 1;
					}
				}
				COMMENT => {
					return idx + 1;
				}
				_ => {}
			},
			_ => return 0,
		}
		0
	} else {
		0
	}
}

fn linebreak_count(text: &str) -> usize {
	text.matches('\n').count()
		+ text.matches('\r').count()
		+ text.matches('\u{2028}').count()
		+ text.matches('\u{2029}').count()
}
