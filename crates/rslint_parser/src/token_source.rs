use crate::{JsSyntaxKind::EOF, TextRange, TextSize, Token};
use std::collections::HashSet;
use std::iter::FusedIterator;

/// The source of tokens for the parser
pub struct TokenSource<'t> {
	source: &'t str,

	/// A list of the tokens including whitespace.
	pub raw_tokens: &'t [rslint_lexer::Token],

	/// Current token and position
	cur: (Token, usize),
}

fn mk_token2(pos: usize, raw_tokens: &[rslint_lexer::Token]) -> Token {
	let t = &raw_tokens[pos];
	Token {
		kind: t.kind,
		range: t.offset..(t.offset + t.len),
		len: TextSize::from(t.len as u32),
	}
}

impl<'t> TokenSource<'t> {
	/// Generate input from tokens(except comments and whitespace).
	///
	/// # Panics
	/// This method will panic in case the source and raw tokens do not match
	/// as it relies on the source code for checking if trivia contains linebreaks
	pub fn new(source: &'t str, raw_tokens: &'t [rslint_lexer::Token]) -> TokenSource<'t> {
		let mut pos = 0usize;
		while raw_tokens[pos].kind.is_trivia() {
			pos += 1;
		}
		let first = mk_token2(pos, raw_tokens);
		TokenSource {
			source,
			cur: (first, pos),
			raw_tokens,
		}
	}

	fn next_non_trivia(&self, pos: usize, dir: isize) -> Option<usize> {
		let mut pos = pos as isize + dir;
		if (pos < 0) || ((pos as usize) >= self.raw_tokens.len()) {
			return None
		} 
		while self.raw_tokens[pos as usize].kind.is_trivia() {
			pos += dir;
			if (pos < 0) || ((pos as usize) >= self.raw_tokens.len()) {
				return None
			}
		}
		Some(pos as usize)
	}

	/// Rewind the current position to a former position.
	pub fn rewind(&mut self, pos: usize) {
		//println!("rewind: {}", pos);
		self.cur = (mk_token2(pos, &self.raw_tokens), pos);
	}

	pub fn last_tok(&self) -> Option<Token> {
		//println!("last_tok");
		self.next_non_trivia(self.cur.1, -1).map(|idx| {
			mk_token2(idx, &self.raw_tokens)
		})
	}

	pub fn current(&self) -> Token {
		//println!("current");
		self.cur.0.to_owned()
	}

	pub fn source(&self) -> &str {
		//println!("source");
		self.source
	}

	fn raw_lookahead_nth(&self, n: usize) -> usize {
		let mut idx = self.cur.1;
		for _ in 0..n {
			idx = self.next_non_trivia(idx, 1).unwrap();
		}
		idx
	}

	pub fn lookahead_nth(&self, n: usize) -> Token {
		//println!("lookahead_nth: {} {:?}", n, self.cur);
		let idx = self.raw_lookahead_nth(n);
		mk_token2(idx, &self.raw_tokens)
	}

	pub fn bump(&mut self) {
		//println!("bump");
		if self.cur.0.kind == EOF {
			return;
		}

		let pos = self.next_non_trivia(self.cur.1, 1).unwrap();
		self.cur = (mk_token2(pos, &self.raw_tokens), pos);
	}

	pub fn is_keyword(&self, kw: &str) -> bool {
		//println!("is_keyword");
		let t = self.current();
		&self.source[t.range] == kw
	}

	pub fn had_linebreak_before_nth(&self, n: usize) -> bool {
		//println!("had_linebreak_before_nth");
		let idx = self.raw_lookahead_nth(n);
		self.raw_tokens[idx].after_newline
	}

	pub fn cur_pos(&self) -> usize {
		//println!("cur_pos");
		self.raw_tokens[self.cur.1].offset
	}

	pub fn cur_token_idx(&self) -> usize {
		//println!("cur_token_idx");
		self.cur.1
	}

	pub fn size_hint(&self) -> usize {
		//println!("size_hint");
		self.raw_tokens.len()
	}
}

impl Iterator for TokenSource<'_> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		while self.cur.0.kind.is_trivia() {
			self.bump();
		}

		let cur = self.cur.0.clone();
		if cur.kind != EOF {
			self.bump();
			Some(cur)
		} else {
			None
		}
	}
}

impl FusedIterator for TokenSource<'_> {}
