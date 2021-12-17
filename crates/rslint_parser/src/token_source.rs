use crate::{JsSyntaxKind::EOF, TextRange, TextSize, Token};
use rslint_lexer::is_linebreak;
use std::collections::HashSet;

/// The source of tokens for the parser
pub struct TokenSource<'t> {
	source: &'t str,
	/// Hashset of offsets for tokens which occur after a linebreak.
	/// This is required for things such as ASI and postfix expressions
	tokens_after_linebreaks: HashSet<TextSize>,

	/// A vector of tokens and their offset from the start.
	token_offset_pairs: Vec<(rslint_lexer::Token, TextSize)>,
	/// A list of the tokens including whitespace.
	pub raw_tokens: &'t [rslint_lexer::Token],

	/// Current token and position
	cur: (Token, usize),
}

fn mk_token(pos: usize, token_offset_pairs: &[(rslint_lexer::Token, TextSize)]) -> Token {
	let kind = match token_offset_pairs.get(pos) {
		Some((token, _)) => token.kind,
		None => EOF,
	};
	let range = token_offset_pairs
		.get(pos)
		.map(|x| {
			let start: usize = x.1.into();
			let end = start + x.0.len;
			start..end
		})
		.unwrap_or_else(|| {
			token_offset_pairs
				.last()
				.map(|x| {
					let start: usize = x.1.into();
					let end = start + x.0.len;
					start..end
				})
				.unwrap_or(0..0)
		});

	Token {
		kind,
		range: range.to_owned(),
		len: TextSize::from(range.len() as u32),
	}
}

impl<'t> TokenSource<'t> {
	/// Generate input from tokens(except comments and whitespace).
	///
	/// # Panics
	/// This method will panic in case the source and raw tokens do not match
	/// as it relies on the source code for checking if trivia contains linebreaks
	pub fn new(source: &'t str, raw_tokens: &'t [rslint_lexer::Token]) -> TokenSource<'t> {
		let mut tokens_after_linebreaks = HashSet::new();
		let mut token_offset_pairs = Vec::with_capacity(raw_tokens.len() / 2);

		let mut len: TextSize = 0.into();
		let mut has_linebreak = false;

		for token in raw_tokens {
			if token.kind.is_trivia() {
				let src = source
					.get(len.into()..(usize::from(len) + token.len))
					.expect("src and tokens do not match");
				if !has_linebreak && src.chars().any(is_linebreak) {
					has_linebreak = true;
				}
			} else {
				if has_linebreak {
					tokens_after_linebreaks.insert(len);
					has_linebreak = false;
				}
				token_offset_pairs.push((*token, len));
			};

			len += TextSize::from(token.len as u32);
		}

		let first = mk_token(0, token_offset_pairs.as_slice());
		TokenSource {
			source,
			token_offset_pairs,
			cur: (first, 0),
			tokens_after_linebreaks,
			raw_tokens,
		}
	}

	/// Rewind the current position to a former position.
	pub fn rewind(&mut self, pos: usize) {
		self.cur = (mk_token(pos, &self.token_offset_pairs), pos);
	}

	pub fn last_tok(&self) -> Option<Token> {
		if self.cur.1 == 0 {
			return None;
		}
		Some(mk_token(self.cur.1 - 1, &self.token_offset_pairs))
	}

	pub fn current(&self) -> Token {
		self.cur.0.to_owned()
	}

	pub fn source(&self) -> &str {
		self.source
	}

	pub fn lookahead_nth(&self, n: usize) -> Token {
		mk_token(self.cur.1 + n, &self.token_offset_pairs)
	}

	pub fn bump(&mut self) {
		if self.cur.0.kind == EOF {
			return;
		}

		let pos = self.cur.1 + 1;
		self.cur = (mk_token(pos, &self.token_offset_pairs), pos);
	}

	pub fn is_keyword(&self, kw: &str) -> bool {
		self.token_offset_pairs
			.get(self.cur.1)
			.map(|(token, offset)| {
				&self.source[TextRange::at(*offset, TextSize::from(token.len as u32))] == kw
			})
			.unwrap_or(false)
	}

	pub fn had_linebreak_before_nth(&self, n: usize) -> bool {
		if let Some(i) = self.token_offset_pairs.get(self.cur.1 + n) {
			self.tokens_after_linebreaks.contains(&i.1)
		} else {
			false
		}
	}

	pub fn cur_pos(&self) -> usize {
		self.token_offset_pairs[self.cur.1].1.into()
	}

	pub fn cur_token_idx(&self) -> usize {
		self.cur.1
	}

	pub fn size_hint(&self) -> usize {
		self.token_offset_pairs.len()
	}
}

impl Iterator for TokenSource<'_> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		let cur = self.cur.0.clone();
		if cur.kind != EOF {
			self.bump();
			Some(cur)
		} else {
			None
		}
	}
}
