use crate::JsSyntaxKind::EOF;
use std::iter::FusedIterator;

/// The source of tokens for the parser
pub struct TokenSource<'t> {
	source: &'t str,

	/// A list of the tokens including whitespace.
	pub raw_tokens: &'t [rslint_lexer::Token],

	/// Current token index at raw_tokens.
	cur: usize,
}

#[repr(isize)]
enum Direction
{
	Forward = 1,
	Backward = -1
}

impl<'t> TokenSource<'t> {
	/// Creates a new [TokenSource] and set its
	/// [TokenSource::cur] to the first non-trivia
	/// token from [raw_tokens]
	pub fn new(source: &'t str, raw_tokens: &'t [rslint_lexer::Token]) -> TokenSource<'t> {
		let mut pos = 0usize;
		while raw_tokens[pos].kind.is_trivia() {
			pos += 1;
		}
		TokenSource {
			source,
			cur: pos,
			raw_tokens,
		}
	}

	#[inline(always)]
	fn next_non_trivia(&self, pos: usize, dir: Direction) -> usize {
		let dir = dir as isize;
		let mut pos = pos as isize + dir;

		if pos < 0 {
			return 0;
		}

		if (pos as usize) >= self.raw_tokens.len() {
			return self.raw_tokens.len() - 1;
		}

		while self.raw_tokens[pos as usize].kind.is_trivia() {
			pos += dir;
			
			if pos < 0 {
				return 0;
			}
			
			if (pos as usize) >= self.raw_tokens.len() {
				return self.raw_tokens.len() - 1;
			}
		}

		pos as usize
	}

	#[inline(always)]
	fn raw_lookahead_nth(&self, n: usize) -> usize {
		let mut idx = self.cur;
		for _ in 0..n {
			idx = self.next_non_trivia(idx, Direction::Forward)
		}
		idx
	}

	/// Rewind the current position to a former position.
	#[inline(always)]
	pub fn rewind(&mut self, pos: usize) {
		self.cur = pos;
	}

	#[inline(always)]
	pub fn last_tok(&self) -> Option<&rslint_lexer::Token> {
		let idx = self.next_non_trivia(self.cur, Direction::Backward);
		self.raw_tokens.get(idx)
	}

	#[inline(always)]
	pub fn current(&self) -> &rslint_lexer::Token {
		&self.raw_tokens[self.cur]
	}

	#[inline(always)]
	pub fn source(&self) -> &str {
		self.source
	}

	#[inline(always)]
	pub fn lookahead_nth(&self, n: usize) -> &'t rslint_lexer::Token {
		let idx = self.raw_lookahead_nth(n);
		&self.raw_tokens[idx]
	}

	#[inline(always)]
	pub fn bump(&mut self) {
		if self.current().kind == EOF {
			return;
		}

		self.cur = self
			.next_non_trivia(self.cur, Direction::Forward)
	}

	#[inline(always)]
	pub fn is_keyword(&self, kw: &str) -> bool {
		let range = self.current().range();
		&self.source[range] == kw
	}

	#[inline(always)]
	pub fn had_linebreak_before_nth(&self, n: usize) -> bool {
		self.lookahead_nth(n).after_newline
	}

	#[inline(always)]
	pub fn cur_pos(&self) -> usize {
		self.current().offset as usize
	}

	#[inline(always)]
	pub fn cur_token_idx(&self) -> usize {
		self.cur
	}

	pub fn size_hint(&self) -> usize {
		self.raw_tokens.len()
	}
}

impl Iterator for TokenSource<'_> {
	type Item = rslint_lexer::Token;

	fn next(&mut self) -> Option<Self::Item> {
		while self.current().kind.is_trivia() {
			self.bump();
		}

		let cur = self.current().to_owned();
		if cur.kind != EOF {
			self.bump();
			Some(cur)
		} else {
			None
		}
	}
}

impl FusedIterator for TokenSource<'_> {}
