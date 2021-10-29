use std::{char, collections::HashSet, ops::Range};

use ir::CharacterClass;

use crate::{
	ir::{self, AssertionKind, Node},
	unicode::*,
	Error, Result, Span,
};

// I know you may be tempted to get out your swords and condemn my use of inline everywhere,
// but *trust* me i actually backed this up with benchmarks and it showed about a 50% speedup.

#[inline]
fn is_syntax_character(cp: char) -> bool {
	cp == '^'
		|| cp == '$'
		|| cp == '\\'
		|| cp == '.'
		|| cp == '*'
		|| cp == '+'
		|| cp == '?'
		|| cp == '('
		|| cp == ')'
		|| cp == '['
		|| cp == ']'
		|| cp == '{'
		|| cp == '}'
		|| cp == '|'
}

#[inline]
fn is_regexp_identifier_start(cp: char) -> bool {
	is_id_start(cp) || cp == '$' || cp == '_'
}

#[inline]
fn is_regexp_identifier_part(cp: char) -> bool {
	is_id_continue(cp) ||
      cp == '$' ||
      cp == '_' ||
      cp == '\u{200c}' ||  // unicode zero-width non-joiner
      cp == '\u{200d}' // unicode zero-width joiner
}

#[inline]
fn is_id_start(cp: char) -> bool {
	if (cp as u32) < 0x41 {
		false
	} else if (cp as u32) < 0x5b {
		true
	} else if (cp as u32) < 0x61 {
		false
	} else if (cp as u32) < 0x7b {
		true
	} else {
		is_large_id_start(cp)
	}
}

#[inline]
fn is_id_continue(cp: char) -> bool {
	if (cp as u32) < 0x30 {
		false
	} else if (cp as u32) < 0x3a {
		true
	} else if (cp as u32) < 0x41 {
		false
	} else if (cp as u32) < 0x5b || (cp as u32) == 0x5f {
		true
	} else if (cp as u32) < 0x61 {
		false
	} else if (cp as u32) < 0x7b {
		true
	} else {
		is_large_id_start(cp) || is_large_id_continue(cp)
	}
}

#[derive(Debug, Clone, Default)]
pub struct State {
	group_count: u32,
	u_flag: bool,
	n_flag: bool,
	group_names: Vec<String>,
	backref_names: Vec<String>,
	last_assertion_is_quantifiable: bool,
}

pub fn validate_flags(flags_str: &str, ecma_version: EcmaVersion) -> Result<ir::Flags, String> {
	let mut existing_flags = HashSet::<char>::new();
	let mut flags = ir::Flags::empty();

	for flag in flags_str.chars() {
		if existing_flags.contains(&flag) {
			return Err(format!("Duplicated flag {}", flag));
		}
		existing_flags.insert(flag);

		match flag {
			'g' => flags |= ir::Flags::G,
			'i' => flags |= ir::Flags::I,
			'm' => flags |= ir::Flags::M,
			'u' if ecma_version >= EcmaVersion::ES2015 => flags |= ir::Flags::U,
			'y' if ecma_version >= EcmaVersion::ES2015 => flags |= ir::Flags::Y,
			's' if ecma_version >= EcmaVersion::ES2018 => flags |= ir::Flags::S,
			_ => return Err(format!("Invalid flag {}", flag)),
		}
	}
	Ok(flags)
}

/// The actual parser that is responsible for parsing regex.
pub struct Parser<'pat> {
	pub pattern: &'pat str,
	offset: usize,
	cur: usize,
	pub file_id: usize,
	state: State,
	pub ecma_version: EcmaVersion,
	strict: bool,
	flags: ir::Flags,
}

impl<'pat> Parser<'pat> {
	/// Creates a new `Parser` from a given full pattern.
	///
	/// The given offset is used to convert the relative position in the pattern
	/// into an absolute position inside a file. The `pattern` must be the pattern without
	/// the leading and trailing `/`
	///
	/// # Panics
	///
	/// Panics if there is no leading or trailing `/`
	///
	/// # Returns
	///
	/// Returns a `Result` from parsing flags
	pub fn new(
		pattern: &'pat str,
		file_id: usize,
		offset: usize,
		ecma_version: EcmaVersion,
		strict: bool,
	) -> Result<Self> {
		let left_slash = pattern
			.find('/')
			.expect("regex pattern is missing leading `/`");
		let rest = &pattern[left_slash + 1..];
		let right_slash = rest
			.rfind('/')
			.expect("regex pattern is missing trailing `/`")
			+ left_slash + 1;
		let pat = &pattern[left_slash + 1..right_slash];

		let flags = match validate_flags(&pattern[right_slash + 1..], ecma_version) {
			Ok(flags) => flags,
			Err(err) => {
				return Err(Error::new(
					err,
					Span::new(offset, right_slash, pattern.len()),
				));
			}
		};

		Ok(Self {
			pattern: pat,
			offset: offset + left_slash + 1,
			file_id,
			cur: 0,
			state: State {
				u_flag: flags.contains(ir::Flags::U) && ecma_version >= EcmaVersion::ES2015,
				n_flag: flags.contains(ir::Flags::U) && ecma_version >= EcmaVersion::ES2018,
				..Default::default()
			},
			ecma_version,
			strict: strict || flags.contains(ir::Flags::U),
			flags,
		})
	}

	pub fn new_from_pattern_and_flags(
		pattern: &'pat str,
		file_id: usize,
		offset: usize,
		ecma_version: EcmaVersion,
		strict: bool,
		flags: ir::Flags,
	) -> Self {
		Self {
			pattern,
			offset,
			file_id,
			cur: 0,
			state: State {
				u_flag: flags.contains(ir::Flags::U) && ecma_version >= EcmaVersion::ES2015,
				n_flag: flags.contains(ir::Flags::U) && ecma_version >= EcmaVersion::ES2018,
				..Default::default()
			},
			ecma_version,
			strict: strict || flags.contains(ir::Flags::U),
			flags,
		}
	}

	fn error(&mut self, title: impl Into<String>) -> Error {
		Error::new(title.into(), Span::new(0, 0, 0))
	}

	#[inline]
	fn next(&mut self) -> Option<char> {
		let c = self.peek()?;
		self.cur += c.len_utf8();
		Some(c)
	}

	#[inline]
	fn next_with_range(&mut self) -> Option<(char, Range<usize>)> {
		let c = self.peek()?;
		let start = self.cur;
		let len = c.len_utf8();
		self.cur += len;
		Some((c, start..self.cur))
	}

	#[inline]
	fn peek(&mut self) -> Option<char> {
		let slice = &self.pattern.get(self.cur..)?;
		let c = slice.chars().next()?;
		Some(c)
	}

	#[inline]
	fn peek_many(&mut self, count: usize) -> Option<&'pat str> {
		self.pattern.get(self.cur..self.cur + count)
	}

	#[inline]
	fn take(&mut self, count: usize) -> Option<&'pat str> {
		let slice = self.peek_many(count)?;
		self.cur += slice.len();
		Some(slice)
	}

	#[inline]
	fn eat(&mut self, c: char) -> bool {
		debug_assert!((c as u8) < 128);
		// this is fine, utf8 is backwards compatible with ascii and eat is only ever called
		// on ascii chars
		if self.pattern.as_bytes().get(self.cur) == Some(&(c as u8)) {
			self.cur += 1;
			true
		} else {
			false
		}
	}

	#[inline]
	fn eat_err(&mut self, c: char) -> Result<()> {
		debug_assert!((c as u8) < 128);
		if self.pattern.as_bytes().get(self.cur) == Some(&(c as u8)) {
			self.cur += 1;
			Ok(())
		} else {
			Err(self
				.error(format!("expected `{}`", c))
				.primary(self.span(self.cur), ""))
		}
	}

	#[inline]
	fn try_eat_many(&mut self, eat: &str) -> bool {
		if self
			.peek_many(eat.len())
			.map_or(false, |actual| actual == eat)
		{
			self.take(eat.len());
			true
		} else {
			false
		}
	}

	#[inline]
	fn next_if<F: FnOnce(char) -> bool>(&mut self, pred: F) -> Option<char> {
		if pred(self.peek()?) {
			Some(self.next().unwrap())
		} else {
			None
		}
	}

	#[inline]
	fn span(&self, start: usize) -> Span {
		Span::new(self.offset, start, self.cur)
	}

	#[inline]
	fn rewind(&mut self, start: usize) {
		self.cur = start;
	}

	#[inline]
	fn cur_range(&self) -> Range<usize> {
		if self.cur == self.pattern.len() {
			return self.cur..self.cur + 1;
		}
		self.cur + self.offset
			..self.cur + self.pattern[self.cur..].chars().next().unwrap().len_utf8() + self.offset
	}
}

impl Parser<'_> {
	/// The main entrypoint for parsing a RegEx pattern.
	pub fn parse(mut self) -> Result<ir::Regex> {
		let mut node = self.pattern()?;

		if !self.state.n_flag
			&& self.ecma_version >= EcmaVersion::ES2018
			&& !self.state.group_names.is_empty()
		{
			self.state.n_flag = true;
			self.rewind(0);
			node = self.pattern()?;
		}
		// if !self.state.n_flag && self.ecma_version >= EcmaVersion::ES2018 && !sel
		Ok(ir::Regex {
			node,
			flags: self.flags,
		})
	}

	fn pattern(&mut self) -> Result<Node> {
		if self.pattern.is_empty() {
			return Ok(Node::Empty);
		}

		self.state.group_names.clear();
		self.state.backref_names.clear();
		self.infer_total_group_count();

		let node = self.disjunction()?;

		if let Some(c) = self.peek() {
			return Err(self
				.error(format!("unexpected character `{}`", c))
				.primary(self.cur_range(), ""));
		}

		if let Some(name) = self
			.state
			.backref_names
			.iter()
			.find(|x| !self.state.group_names.iter().any(|y| x == &y))
			.cloned()
		{
			return Err(self
				.error(format!("invalid backreference: {}", name))
				.primary(self.offset..self.pattern.len() + self.offset, ""));
		}
		Ok(node)
	}

	/// Infer the total number of groups in the regex using a mini parser.
	/// Taken from v8:
	/// <https://source.chromium.org/chromium/chromium/src/+/master:v8/src/regexp/regexp-parser.cc;l=728-734;drc=7ccffaf0933ccc647c744bf66971bcf5f33a676a>
	fn infer_total_group_count(&mut self) {
		let mut n = 0;
		while let Some(c) = self.next() {
			match c {
				'\\' => {
					self.next();
				}
				'[' => match c {
					'\\' => {
						self.next();
					}
					']' => break,
					_ => {}
				},
				'(' => {
					if self.peek_many(2) != Some("?:") {
						n += 1;
					}
				}
				_ => {}
			}
		}
		self.rewind(0);
		self.state.group_count = n;
	}

	/// A Disjunction is a list of nodes separated by `|`
	///
	/// ```ignore
	/// /a|b|c/
	/// ```
	#[inline]
	fn disjunction(&mut self) -> Result<Node> {
		let start = self.cur;
		let node = self.alternative()?;
		let mut nodes = Vec::with_capacity(3);
		nodes.push(node);
		while self.eat('|') {
			nodes.push(self.alternative()?);
		}

		if matches!(
			self.pattern.as_bytes().get(self.cur).copied(),
			Some(b'*') | Some(b'+') | Some(b'?')
		) {
			let inner = self.cur;
			if let Ok(Ok(_)) = self.quantifier(Node::Empty) {
				return Err(self
					.error("invalid quantifier")
					.primary(inner + self.offset..self.cur + self.offset, ""));
			} else {
				self.rewind(inner);
			}
		} else if self.peek() == Some('{') {
			return Err(self
				.error("invalid braced quantifier")
				.primary(self.cur_range(), ""));
		}

		Ok(if nodes.len() == 1 {
			nodes.pop().unwrap()
		} else {
			Node::Disjunction(self.span(start), nodes)
		})
	}

	#[inline]
	fn alternative(&mut self) -> Result<Node> {
		let start = self.cur;
		let mut terms = Vec::with_capacity(5);
		while let Some(b) = self.pattern.as_bytes().get(self.cur).copied() {
			if b == b')' || b == b'|' {
				break;
			}
			terms.push(self.term()?);
		}

		Ok(if terms.is_empty() {
			Node::Empty
		} else if terms.len() == 1 {
			terms.pop().unwrap()
		} else {
			Node::Alternative(self.span(start), terms)
		})
	}

	/// A term is either a `atom`, `assertion` or an `atom` followed by a `quantifier`.
	#[inline]
	fn term(&mut self) -> Result<Node> {
		if self.state.u_flag || self.strict {
			if let Some(mut node) = self.assertion()? {
				if matches!(self.peek(), Some('*') | Some('+') | Some('?') | Some('{')) {
					node = self.opt_quantifier(node, !self.state.last_assertion_is_quantifiable)?;
				}
				Ok(node)
			} else {
				let mut node = self.atom(false)?;
				if matches!(self.peek(), Some('*') | Some('+') | Some('?') | Some('{'))
					&& node != Node::Empty
				{
					node = self.opt_quantifier(node, false)?;
				}
				Ok(node)
			}
		} else if let Some(mut node) = self.assertion()? {
			if matches!(self.peek(), Some('*') | Some('+') | Some('?') | Some('{')) {
				node = self.opt_quantifier(node, !self.state.last_assertion_is_quantifiable)?;
			}
			Ok(node)
		} else {
			let mut node = self.atom(true)?;
			if matches!(self.peek(), Some('*') | Some('+') | Some('?') | Some('{'))
				&& node != Node::Empty
			{
				node = self.opt_quantifier(node, false)?;
			}
			Ok(node)
		}
	}

	#[inline]
	fn opt_quantifier(&mut self, node: Node, err: bool) -> Result<Node> {
		let start = self.cur;
		match self.quantifier(node.clone()) {
			Ok(Ok(n)) => {
				if err {
					return Err(self
						.error("quantifiers are not valid here")
						.primary(self.span(start), ""));
				}
				Ok(n)
			}
			Ok(Err(err)) => Err(err),
			Err(err) => {
				if self.strict || self.state.u_flag {
					Err(err)
				} else {
					self.rewind(start);
					Ok(node)
				}
			}
		}
	}

	// need to distinguish between parsing error and logic error (misordered range)
	#[inline]
	fn quantifier(&mut self, node: Node) -> Result<Result<Node>> {
		let start = self.cur;
		// quantifier is always called when the next char is guaranteed to be
		// the start of a quantifier
		let quantifier = match self.next().unwrap() {
			'*' => ir::QuantifierKind::Multiple,
			'+' => ir::QuantifierKind::AtLeastOne,
			'?' => ir::QuantifierKind::Optional,
			'{' => {
				let min = self.eat_digits(None, 10)?;
				let max = if self.eat(',') {
					if let Ok(max) = self.eat_digits(None, 10) {
						Some(Some(max))
					} else {
						Some(None)
					}
				} else {
					None
				};

				self.eat_err('}')?;
				if let Some(max) = max {
					if let Some(inner_max) = max {
						if min > inner_max {
							return Ok(Err(self
								.error("quantifier range start is higher than the end")
								.primary(self.span(start), "")));
						}
					}
					ir::QuantifierKind::Between(min, max)
				} else {
					ir::QuantifierKind::Number(min)
				}
			}
			_ => unreachable!(),
		};
		Ok(Ok(Node::Quantifier(
			self.span(start),
			Box::new(node),
			quantifier,
			self.eat('?'),
		)))
	}

	/// Tries to parse an assertion, but will rewind to the start if
	/// it failed to find a assertion.
	#[inline]
	fn assertion(&mut self) -> Result<Option<Node>> {
		let start = self.cur;
		self.state.last_assertion_is_quantifiable = false;

		if self.eat('^') || self.eat('$') {
			return Ok(Some(Node::Assertion(
				self.span(start),
				if self.pattern.as_bytes()[self.cur - 1] == b'^' {
					AssertionKind::StartOfLine
				} else {
					AssertionKind::EndOfLine
				},
			)));
		}

		if self.eat('\\') {
			if self.eat('b') || self.eat('B') {
				return Ok(Some(Node::Assertion(
					self.span(start),
					if self.pattern.as_bytes()[self.cur - 1] == b'b' {
						AssertionKind::WordBoundary
					} else {
						AssertionKind::NonWordBoundary
					},
				)));
			}
			self.rewind(start);
		}

		if self.try_eat_many("(?") {
			let is_lookbehind = self.ecma_version >= EcmaVersion::ES2018 && self.eat('<');

			if self.eat('=') || self.eat('!') {
				let cur_byte = self.pattern.as_bytes()[self.cur - 1];
				let node = self.disjunction()?;
				self.eat_err(')')
					.map_err(|err| err.primary(self.cur_range(), "expected a parentheses"))?;

				let kind = match (is_lookbehind, cur_byte) {
					(false, b'=') => AssertionKind::Lookahead,
					(false, b'!') => AssertionKind::NegativeLookahead,
					(true, b'=') => AssertionKind::Lookbehind,
					(true, b'!') => AssertionKind::NegativeLookbehind,
					_ => unreachable!(),
				};

				self.state.last_assertion_is_quantifiable = !is_lookbehind && !self.strict;
				return Ok(Some(Node::Assertion(
					self.span(start),
					kind(Box::new(node)),
				)));
			}
		}

		// the next token is no assertion, so rewind to the start.
		self.rewind(start);
		Ok(None)
	}

	#[inline]
	fn atom(&mut self, extended: bool) -> Result<Node> {
		const DISALLOWED_PATTERN_CHARS: &[char] =
			&['^', '$', '\\', '.', '*', '+', '?', '(', ')', '[', '|'];
		let start = self.cur;
		let c = match self.next() {
			Some(c) => c,
			None => return Err(self.error("expected an atom").primary(self.cur_range(), "")),
		};

		// technically per annex b we should treat { as an invalid braced quantifier, but
		// the production is always a syntax error so instead we just error on `{`
		let node = match c {
			'.' => Node::Dot(self.span(start)),
			'\\' if self.peek() == Some('c') && extended => {
				self.next();
				Node::Literal(self.span(start), 'c', 'c'.into())
			}
			'\\' => self.atom_escape(start, extended)?,
			'[' => self.character_class()?,
			'(' => self.group()?,
			'{' if extended => {
				self.cur -= 1;
				if let Ok(Ok(_)) = self.quantifier(Node::Empty) {
					return Err(self
						.error("invalid braced quantifier")
						.primary(self.span(start), ""));
				} else {
					self.rewind(start);
					self.next();
					Node::Literal(self.span(start), '{', '{'.into())
				}
			}
			c if extended => {
				if !DISALLOWED_PATTERN_CHARS.contains(&c) {
					Node::Literal(self.span(start), c, c.into())
				} else {
					return Err(self.error("expected an atom").primary(self.span(start), ""));
				}
			}
			c => {
				if !is_syntax_character(c) {
					Node::Literal(self.span(start), c, c.into())
				} else {
					return Err(self.error("expected an atom").primary(self.span(start), ""));
				}
			}
		};
		Ok(node)
	}

	#[inline]
	fn group(&mut self) -> Result<Node> {
		let start = self.cur;
		let noncapturing = if self.peek_many(2) == Some("?:") {
			self.cur += 2;
			true
		} else {
			false
		};
		let name = if !noncapturing && self.peek_many(2) == Some("?<") {
			self.cur += 1;
			let name = self.group_name()?;
			if !self.state.group_names.contains(&name) {
				self.state.group_names.push(name.clone());
			} else {
				return Err(self
					.error(format!("duplicate group name {}", name))
					.primary(self.span(start), ""));
			}
			Some(name)
		} else {
			None
		};
		let inner = Box::new(self.disjunction()?);
		self.eat_err(')')?;
		let group = ir::Group {
			noncapturing,
			inner,
			name,
		};
		Ok(Node::Group(self.span(start), group))
	}

	#[inline]
	fn group_name(&mut self) -> Result<String> {
		self.eat_err('<')?;
		let mut string = String::from(self.identifier_start()?);
		while let Ok(c) = self.identifier_part() {
			string.push(c);
		}
		self.eat_err('>')?;
		Ok(string)
	}

	#[inline]
	fn identifier_start(&mut self) -> Result<char> {
		let force_u = !self.state.u_flag && self.ecma_version >= EcmaVersion::ES2020;

		if let Some(mut c) = self.peek() {
			let c1 = self.next();
			if c == '\\' {
				self.eat_err('u')?;
				c = self.unicode_escape(force_u)?;
			} else if (0xD800..=0xDBFF).contains(&(c as u32))
				&& c1.is_some() && (0xDC00..=0xDFFF).contains(&(c1.unwrap() as u32))
			{
				c = char::from_u32(
					(c as u32 - 0xD800) * 0x400 + (c1.unwrap() as u32 - 0xDC00) + 0x10000,
				)
				.unwrap();
			}

			if is_regexp_identifier_start(c) {
				return Ok(c);
			}
		}
		Err(self
			.error("expected an identifier, but found none")
			.primary(self.cur_range(), ""))
	}

	#[inline]
	fn identifier_part(&mut self) -> Result<char> {
		let start = self.cur;
		let force_u = !self.state.u_flag && self.ecma_version >= EcmaVersion::ES2020;
		let mut cp = self.next();
		let cp1_start = self.cur;
		let cp1 = self.next();
		let mut composite = false;

		{
			match (cp, cp1) {
				(Some('\\'), Some('u')) => {
					composite = true;
					cp = Some(self.unicode_escape(force_u)?);
				}
				(Some(c1), Some(c2))
					if force_u
						&& (0xD800..=0xDBFF).contains(&(c1 as u32))
						&& (0xDC00..=0xDFFF).contains(&(c2 as u32)) =>
				{
					cp = Some(
						char::from_u32(
							(cp.unwrap() as u32 - 0xD800) * 0x400
								+ (cp1.unwrap() as u32 - 0xDC00) + 0x10000,
						)
						.unwrap(),
					);
					composite = true;
				}
				_ => {}
			}
		}

		if let Some(c) = cp {
			if is_regexp_identifier_part(c) {
				if !composite {
					self.rewind(cp1_start);
				}
				return Ok(c);
			} else {
				self.rewind(start);
			}
		} else {
			self.rewind(start);
		}

		Err(self
			.error("expected an identifier, but found none")
			.primary(self.span(start), ""))
	}

	#[inline]
	fn character_class_atom(&mut self) -> Result<Option<Node>> {
		let start = self.cur;
		if let Some(c) = self.peek() {
			if c != '\\' && c != ']' {
				self.next();
				return Ok(Some(Node::Literal(self.span(start), c, c.into())));
			}
		}

		if self.eat('\\') {
			let c = self.peek();
			let res = self.character_class_escape(start);
			if res.is_err() && c == Some('c') {
				if self.strict || self.state.u_flag {
					return Err(self
						.error("invalid character class escape")
						.primary(self.span(start), ""));
				} else {
					return Ok(Some(Node::Literal(self.span(start), 'c', 'c'.into())));
				}
			}

			return Ok(Some(res?));
		}
		Ok(None)
	}

	#[inline]
	fn character_class_escape(&mut self, start: usize) -> Result<Node> {
		if self.eat('b') {
			return Ok(Node::Literal(self.span(start), '\x08', "\\b".into()));
		}

		if self.state.u_flag && self.eat('-') {
			return Ok(Node::Literal(self.span(start), '-', '-'.into()));
		}

		if !self.strict && !self.state.u_flag && self.peek() == Some('c') {
			self.next();
			if let Some(c) = self.peek() {
				if c.is_digit(10) || c == '_' {
					self.next();
					return Ok(Node::Literal(
						self.span(start),
						char::from_u32(c as u32 % 0x20).unwrap(),
						self.pattern[start..self.cur].to_string(),
					));
				} else {
					self.cur -= 1;
				}
			} else {
				self.cur -= 1;
			}
		}

		self.atom_escape(start, false)
	}

	#[inline]
	fn character_class(&mut self) -> Result<Node> {
		let start = self.cur;
		let negated = self.eat('^');
		let mut members = vec![];

		loop {
			let member_start = self.cur;
			if let Some(atom) = self.character_class_atom()? {
				let inner_start = self.cur;
				if !self.eat('-') {
					members.push(ir::CharacterClassMember::Single(atom));
				} else {
					let end = if let Some(n) = self.character_class_atom()? {
						n
					} else {
						members.push(ir::CharacterClassMember::Single(atom));
						members.push(ir::CharacterClassMember::Single(Node::Literal(
							self.span(inner_start),
							'-',
							'-'.into(),
						)));
						continue;
					};

					let min = match atom {
						Node::Literal(_, c, _) => Some(c as u32),
						_ => None,
					};
					let max = match end {
						Node::Literal(_, c, _) => Some(c as u32),
						_ => None,
					};

					if (self.strict || self.state.u_flag) && (min.is_none() || max.is_none()) {
						return Err(self
							.error("invalid character class range")
							.primary(self.span(member_start), ""));
					}

					match (min, max) {
						(Some(x), Some(y)) if x > y => {
							return Err(self
								.error("out of order range for character class")
								.primary(self.span(member_start), ""));
						}
						_ => {}
					}
					members.push(ir::CharacterClassMember::Range(atom, end));
				}
			} else {
				break;
			}
		}
		self.eat_err(']')?;

		Ok(Node::CharacterClass(
			self.span(start),
			CharacterClass { negated, members },
		))
	}

	#[inline]
	fn word(&mut self, function: impl Fn(char) -> bool) -> Option<String> {
		let mut string = String::new();
		while let Some(c) = self.peek() {
			if function(c) {
				string.push(c);
				self.next();
			} else {
				break;
			}
		}
		Some(string).filter(|x| !x.is_empty())
	}

	fn unicode_property_name(&mut self) -> Option<String> {
		self.word(|c| c.is_ascii_alphabetic() || c == '_')
	}

	fn unicode_property_value(&mut self) -> Option<String> {
		self.word(|c| c.is_ascii_alphabetic() || c == '_' || c.is_digit(10))
	}

	#[inline]
	fn unicode_property_value_expr(&mut self) -> Result<(Option<String>, String)> {
		let start = self.cur;

		if let Some(name) = self.unicode_property_name().filter(|_| self.eat('=')) {
			if let Some(val) = self.unicode_property_value() {
				return if is_valid_unicode_property(self.ecma_version, &name, &val) {
					Ok((Some(name), val))
				} else {
					Err(self
						.error("invalid unicode property value")
						.primary(start..self.cur, ""))
				};
			}
		}
		self.rewind(start);

		if let Some(value) = self.unicode_property_value() {
			return if is_valid_unicode_property(self.ecma_version, "General_Category", &value) {
				Ok((Some(String::from("General_Category")), value))
			} else {
				if is_valid_lone_unicode_property(self.ecma_version, &value) {
					return Ok((None, value));
				}
				Err(self
					.error("invalid unicode property value")
					.primary(start..self.cur, ""))
			};
		}

		Err(self
			.error("expected a unicode property value")
			.primary(self.cur_range(), ""))
	}

	/// Parses anything that comes after a `\`.
	#[inline]
	#[allow(clippy::too_many_lines)]
	fn atom_escape(&mut self, start: usize, extended: bool) -> Result<Node> {
		let c = if let Some(c) = self.next() {
			c
		} else {
			let err = self
				.error("unexpected end of escape sequence")
				.primary(self.span(start), "");
			return Err(err);
		};

		let span = self.span(start);
		let node = match c {
			// ControlEscape
			't' => Node::Literal(span, '\t', "\\t".into()),
			'n' => Node::Literal(span, '\n', "\\n".into()),
			'v' => Node::Literal(span, '\x0B', "\\v".into()),
			'f' => Node::Literal(span, '\x0C', "\\f".into()),
			'r' => Node::Literal(span, '\r', "\\r".into()),

			'k' if self.state.n_flag => {
				let name = self.group_name()?;
				self.state.backref_names.push(name.clone());
				Node::NamedBackReference(self.span(start), name)
			}
			'c' if extended => {
				self.cur -= 1;
				Node::Literal(self.span(start), '\\', '\\'.into())
			}
			'c' if self.peek().map_or(false, |c| c.is_ascii_alphabetic()) => {
				let c = self.next().unwrap();
				Node::Literal(
					self.span(start),
					std::char::from_u32((c as u32) % 32).unwrap(),
					self.pattern[start..self.cur].into(),
				)
			}
			'0' if !self.peek().map_or(false, |c| c.is_digit(10)) => {
				Node::Literal(span, '\0', "\\0".into())
			}
			'0' | '4' if !self.strict && !self.state.u_flag => {
				self.cur -= 1;
				let n;
				let n1 = self.eat_digits(1, 8).unwrap();
				if let Ok(n2) = self.eat_digits(1, 8) {
					if n1 <= 3 {
						if let Ok(n3) = self.eat_digits(1, 8) {
							n = n3 + n1 * 64 + n2 * 8;
						} else {
							n = n1 * 8 + n2;
						}
					} else {
						n = n1 * 8 + n2;
					}
				} else {
					n = n1;
				}
				Node::Literal(
					self.span(start),
					char::from_u32(n).unwrap(),
					self.pattern[start..self.cur].into(),
				)
			}
			'x' => {
				let digit_start = self.cur;
				let digits = match self.eat_digits(2, 16) {
					Ok(d) => d,
					Err(err) => {
						if self.strict || self.state.u_flag {
							return Err(err);
						} else {
							self.rewind(digit_start);
							return Ok(Node::Literal(span, 'x', "\\x".into()));
						}
					}
				};

				Node::Literal(
					self.span(start),
					char::from_u32(digits).unwrap(),
					self.pattern[start..self.cur].into(),
				)
			}
			'u' => Node::Literal(
				self.span(start),
				self.unicode_escape(false)?,
				self.pattern[start..self.cur].into(),
			),
			'd' | 'D' => Node::PerlClass(span, ir::ClassPerlKind::Digit, c == 'D'),
			'w' | 'W' => Node::PerlClass(span, ir::ClassPerlKind::Word, c == 'W'),
			's' | 'S' => Node::PerlClass(span, ir::ClassPerlKind::Space, c == 'S'),
			'p' | 'P' if self.strict || self.state.u_flag => {
				let inner_start = self.cur;
				self.property_escape(start, c == 'P').map_err(|_| {
					self.error("invalid property escape")
						.primary(self.span(inner_start), "")
				})?
			}

			// a back reference: `/(foo)\1/`
			'1'..='9' => {
				let num = {
					let mut n = c.to_digit(10).unwrap();
					while let Some(c) = self.next_if(|c| c.is_digit(10)) {
						n = 10 * n + c.to_digit(10).unwrap();
					}
					n
				};

				// invalid group number
				if num > self.state.group_count {
					if self.strict || self.state.u_flag {
						return Err(self
							.error("invalid escape sequence")
							.primary(self.span(start), ""));
					}
					return Ok(Node::Literal(
						self.span(start),
						char::from_u32(num).unwrap(),
						self.pattern[start..self.cur].into(),
					));
				}

				Node::BackReference(self.span(start), num)
			}
			'/' if self.state.u_flag => Node::Literal(span, '/', "\\/".into()),
			c => {
				let valid = if self.state.u_flag {
					is_syntax_character(c) || c == '/'
				} else if self.strict {
					!is_id_continue(c)
				} else if self.state.n_flag {
					!(c == 'c' || c == 'k')
				} else {
					c != 'c'
				};

				if valid {
					Node::Literal(span, c, self.pattern[start..self.cur].into())
				} else {
					return Err(self
						.error("invalid escape sequence")
						.primary(self.span(start), ""));
				}
			}
		};

		Ok(node)
	}

	#[inline]
	fn property_escape(&mut self, start: usize, negated: bool) -> Result<Node> {
		self.eat_err('{')?;
		let (class, member) = self.unicode_property_value_expr()?;
		self.eat_err('}')?;
		Ok(Node::PerlClass(
			self.span(start),
			ir::ClassPerlKind::Unicode(class, member),
			negated,
		))
	}

	#[inline]
	fn unicode_escape(&mut self, force_u: bool) -> Result<char> {
		let u_flag = force_u || self.state.u_flag;
		let start = self.cur;
		if let Some(c) = self
			.unicode_surrogate_pair_escape()
			.transpose()?
			.filter(|_| u_flag)
		{
			return Ok(c);
		} else {
			self.rewind(start)
		}

		match self.peek() {
			Some('{') if u_flag => {
				self.next();
				let digits = self.eat_digits(None, 16)?;
				self.eat_err('}')?;
				if let Some(c) = char::from_u32(digits) {
					Ok(c)
				} else {
					Err(self
						.error(format!("{} is not a valid code point", digits))
						.primary(self.span(start).as_range(), ""))
				}
			}
			_ => {
				let digit_start = self.cur;
				let digits = match self.eat_digits(4, 16) {
					Ok(d) => d,
					Err(err) => {
						if self.strict || self.state.u_flag {
							return Err(err);
						} else {
							self.rewind(digit_start);
							return Ok('u');
						}
					}
				};

				if let Some(c) = char::from_u32(digits) {
					Ok(c)
				} else {
					Err(self
						.error(format!("{} is not a valid code point", digits))
						.primary(self.span(start).as_range(), ""))
				}
			}
		}
	}

	#[inline]
	fn unicode_surrogate_pair_escape(&mut self) -> Option<Result<char>> {
		let start = self.cur;
		if let Ok(lead) = self.eat_digits(4, 16) {
			if (0xD800..=0xDBFF).contains(&lead) && self.eat('\\') && self.eat('u') {
				if let Ok(trail) = self.eat_digits(4, 16) {
					if (0xDC00..=0xDFFF).contains(&trail) {
						let codepoint = (lead - 0xD800) * 0x400 + (trail - 0xDC00) + 0x10000;
						return Some(if let Some(c) = char::from_u32(codepoint) {
							Ok(c)
						} else {
							// Is this unreachable?
							Err(self
								.error(format!(
									"{} does not represent a valid codepoint",
									codepoint
								))
								.primary(self.span(start), ""))
						});
					}
				}
			}
		}

		self.rewind(start);
		None
	}

	#[inline]
	fn eat_digits(&mut self, count: impl Into<Option<usize>> + Clone, radix: u32) -> Result<u32> {
		let start = self.cur;
		let is_none = count.clone().into().is_none();
		let mut digits = String::with_capacity(count.clone().into().unwrap_or(10));
		if !self.peek().map_or(false, |c| c.is_digit(radix)) {
			return Err(self
				.error("expected a digit, but found none")
				.primary(self.cur_range(), ""));
		}
		// None count is kind of a huge hack so maybe we should refactor this in the future
		for _ in 0..count.into().unwrap_or(usize::MAX) {
			let inner_start = self.cur;
			if let Some((next, range)) = self.next_with_range() {
				if !next.is_digit(radix) {
					if is_none {
						self.rewind(inner_start);
						break;
					}
					return Err(self
						.error("expected a digit, but found none")
						.primary(range.start + self.offset..range.end + self.offset, ""));
				}
				digits.push(next);
			} else {
				if is_none {
					break;
				}
				return Err(self
					.error("expected a digit, but found none")
					.primary(self.cur_range(), ""));
			}
		}

		if let Ok(digits) = u32::from_str_radix(&digits, radix) {
			Ok(digits)
		} else {
			Err(self
				.error(format!("{} is out of bounds", digits))
				.primary(self.span(start), ""))
		}
	}
}
