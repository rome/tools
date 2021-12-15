//! An extremely fast, lookup table based, ECMAScript lexer which yields SyntaxKind tokens used by the rslint_parse parser.
//! For the purposes of error recovery, tokens may have an error attached to them, which is reflected in the Iterator Item.
//! The lexer will also yield `COMMENT` and `WHITESPACE` tokens.
//!
//! The lexer operates on raw bytes to take full advantage of lookup table optimizations, these bytes **must** be valid utf8,
//! therefore making a lexer from a `&[u8]` is unsafe since you must make sure the bytes are valid utf8.
//! Do not use this to learn how to lex JavaScript, this is just needlessly fast and demonic because i can't control myself :)
//!
//! basic ANSI syntax highlighting is also offered through the `highlight` feature.
//!
//! # Warning ⚠️
//!
//! `>>` and `>>>` are not emitted as single tokens, they are emitted as multiple `>` tokens. This is because of
//! TypeScript parsing and productions such as `T<U<N>>`

#![allow(clippy::or_fun_call)]

#[macro_use]
mod token;
mod state;
mod tests;


#[rustfmt::skip]
mod tables;

pub use token::Token;

#[cfg(feature = "highlight")]
mod highlight;
#[cfg(feature = "highlight")]
pub use highlight::*;

use rslint_errors::Diagnostic;
use state::LexerState;
use tables::derived_property::*;

pub use rslint_syntax::*;
pub type LexerReturn = (Token, Option<Diagnostic>);

// Simple macro for unwinding a loop
macro_rules! unwind_loop {
    ($($iter:tt)*) => {
        $($iter)*
        $($iter)*
        $($iter)*
        $($iter)*
        $($iter)*

        loop {
            $($iter)*
            $($iter)*
            $($iter)*
            $($iter)*
            $($iter)*
        }
    };
}

// The first utf8 byte of every valid unicode whitespace char, used for short circuiting whitespace checks
const UNICODE_WHITESPACE_STARTS: [u8; 5] = [
	// NBSP
	0xC2, // BOM
	0xEF, // Ogham space mark
	0xE1, // En quad .. Hair space, narrow no break space, mathematical space
	0xE2, // Ideographic space
	0xE3,
];

// Unicode spaces, designated by the `Zs` unicode property
const UNICODE_SPACES: [char; 19] = [
	'\u{0020}', '\u{00A0}', '\u{1680}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}',
	'\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}', '\u{200A}', '\u{200B}', '\u{202F}',
	'\u{205F}', '\u{3000}', '\u{FEFF}',
];

fn is_id_start(c: char) -> bool {
	c == '_' || c == '$' || ID_Start(c)
}

fn is_id_continue(c: char) -> bool {
	c == '$' || c == '\u{200d}' || c == '\u{200c}' || ID_Continue(c)
}

/// An extremely fast, lookup table based, lossless ECMAScript lexer
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lexer<'src> {
	bytes: &'src [u8],
	cur: usize,
	state: LexerState,
	pub file_id: usize,
	returned_eof: bool,
}

impl<'src> Lexer<'src> {
	/// Make a new lexer from raw bytes.
	///
	/// # Safety
	/// You must make sure the bytes are valid utf8, failure to do so is undefined behavior.
	pub unsafe fn from_bytes(bytes: &'src [u8], file_id: usize) -> Self {
		Self {
			bytes,
			cur: 0,
			file_id,
			state: LexerState::new(),
			returned_eof: false,
		}
	}

	/// Make a new lexer from a str, this is safe because strs are valid utf8
	pub fn from_str(string: &'src str, file_id: usize) -> Self {
		Self {
			bytes: string.as_bytes(),
			cur: 0,
			file_id,
			state: LexerState::new(),
			returned_eof: false,
		}
	}

	// Bump the lexer and return the token given in
	fn eat(&mut self, tok: LexerReturn) -> LexerReturn {
		self.next();
		tok
	}

	fn consume_newlines(&mut self) {
		while self.current().is_some() {
			let chr = self.get_unicode_char();
			if is_linebreak(chr) {
				self.state.had_linebreak = true;
				self.cur += chr.len_utf8();
			} else {
				break;
			}
		}
	}

	fn consume_whitespace_until_newline(&mut self) {
		while let Some(current) = self.current().copied() {
			let chr = self.get_unicode_char();

			if is_linebreak(chr) {
				break;
			}

			let dispatcher = DISPATCHER[current as usize];
			if dispatcher == Dispatch::WHS
				|| (UNICODE_WHITESPACE_STARTS.contains(&current) && UNICODE_SPACES.contains(&chr))
			{
				self.cur += chr.len_utf8();
			} else {
				break;
			}
		}
	}

	// Consume all whitespace starting from the current byte
	fn consume_whitespace(&mut self) {
		self.consume_newlines();
		self.consume_whitespace_until_newline();
	}

	// Get the unicode char which starts at the current byte and advance the lexer's cursor
	fn get_unicode_char(&self) -> char {
		// This is unreachable for all intents and purposes, but this is just a precautionary measure
		debug_assert!(self.cur < self.bytes.len());

		// Safety: We know this is safe because we require the input to the lexer to be valid utf8 and we always call this when we are at a char
		let string = unsafe { std::str::from_utf8_unchecked(self.bytes.get_unchecked(self.cur..)) };
		let chr = if let Some(chr) = string.chars().next() {
			chr
		} else {
			// Safety: we always call this when we are at a valid char, so this branch is completely unreachable
			unsafe {
				core::hint::unreachable_unchecked();
			}
		};

		chr
	}

	// Get the next byte and advance the index
	#[inline]
	fn current(&mut self) -> Option<&u8> {
		self.bytes.get(self.cur)
	}

	// Get the next byte and advance the index
	#[inline]
	fn next(&mut self) -> Option<&u8> {
		self.cur += 1;
		self.bytes.get(self.cur)
	}

	// Get the next byte but only advance the index if there is a next byte
	// This is really just a hack for certain methods like escapes
	#[inline]
	fn next_bounded(&mut self) -> Option<&u8> {
		if let Some(b) = self.bytes.get(self.cur + 1) {
			self.cur += 1;
			Some(b)
		} else {
			if self.cur != self.bytes.len() {
				self.cur += 1;
			}
			None
		}
	}

	fn advance(&mut self, amount: usize) {
		self.cur += amount;
	}

	fn lookup(byte: u8) -> Dispatch {
		// Safety: our lookup table maps all values of u8, so its impossible for a u8 to be out of bounds
		unsafe { *DISPATCHER.get_unchecked(byte as usize) }
	}

	// Read a `\u{000...}` escape sequence, this expects the cur char to be the `{`
	fn read_codepoint_escape(&mut self) -> Result<char, Diagnostic> {
		let start = self.cur + 1;
		self.read_hexnumber();

		if self.bytes.get(self.cur) != Some(&b'}') {
			// We should not yield diagnostics on a unicode char boundary. That wont make codespan panic
			// but it may cause a panic for other crates which just consume the diagnostics
			let invalid = self.get_unicode_char();
			let err = Diagnostic::error(self.file_id, "", "expected hex digits for a unicode code point escape, but encountered an invalid character")
                .primary(self.cur .. invalid.len_utf8(), "");

			self.cur -= 1;
			return Err(err);
		}

		// Safety: We know for a fact this is in bounds because we must be on the possible char after the } at this point
		// which means its impossible for the range of the digits to be out of bounds.
		// We also know we cant possibly be indexing a unicode char boundary because a unicode char (which cant be a hexdigit)
		// would have triggered the if statement above. We also know this must be valid utf8, both because of read_hexnumber's behavior
		// and because input to the lexer must be valid utf8
		let digits_str = unsafe {
			debug_assert!(self.bytes.get(start..self.cur).is_some());
			debug_assert!(std::str::from_utf8(self.bytes.get_unchecked(start..self.cur)).is_ok());

			std::str::from_utf8_unchecked(self.bytes.get_unchecked(start..self.cur))
		};

		match u32::from_str_radix(digits_str, 16) {
			Ok(digits) if digits <= 0x10FFFF => {
				let res = std::char::from_u32(digits);
				if let Some(chr) = res {
					Ok(chr)
				} else {
					let err =
						Diagnostic::error(self.file_id, "", "invalid codepoint for unicode escape")
							.primary(start..self.cur, "");

					Err(err)
				}
			}

			_ => {
				let err = Diagnostic::error(
					self.file_id,
					"",
					"out of bounds codepoint for unicode codepoint escape sequence",
				)
				.primary(start..self.cur, "")
				.footer_note("Codepoints range from 0 to 0x10FFFF (1114111)");

				Err(err)
			}
		}
	}

	// Read a `\u0000` escape sequence, this expects the current char to be the `u`, it also does not skip over the escape sequence
	// The pos after this method is the last hex digit
	fn read_unicode_escape(&mut self, advance: bool) -> Result<char, Diagnostic> {
		debug_assert_eq!(self.bytes[self.cur], b'u');

		let diagnostic = Diagnostic::error(
			self.file_id,
			"",
			"invalid digits after unicode escape sequence",
		)
		.primary(
			self.cur - 1..self.cur + 1,
			"expected 4 hex digits following this",
		);

		for idx in 0..4 {
			match self.next_bounded() {
				None => {
					if !advance {
						self.cur -= idx + 1;
					}
					return Err(diagnostic);
				}
				Some(b) if !b.is_ascii_hexdigit() => {
					if !advance {
						self.cur -= idx + 1;
					}
					return Err(diagnostic);
				}
				_ => {}
			}
		}

		unsafe {
			// Safety: input to the lexer is guaranteed to be valid utf8 and so is the range since we return if there is a wrong amount of digits beforehand
			let digits_str = std::str::from_utf8_unchecked(
				self.bytes.get_unchecked((self.cur - 3)..(self.cur + 1)),
			);
			if let Ok(digits) = u32::from_str_radix(digits_str, 16) {
				if !advance {
					self.cur -= 4;
				}
				// Safety: we make sure the 4 chars are hex digits beforehand, and 4 hex digits cannot make an invalid char
				Ok(std::char::from_u32_unchecked(digits))
			} else {
				// Safety: we know this is unreachable because 4 hexdigits cannot make an out of bounds char,
				// and we make sure that the chars are actually hex digits
				core::hint::unreachable_unchecked();
			}
		}
	}

	// Validate a `\x00 escape sequence, this expects the current char to be the `x`, it also does not skip over the escape sequence
	// The pos after this method is the last hex digit
	fn validate_hex_escape(&mut self) -> Option<Diagnostic> {
		debug_assert_eq!(self.bytes[self.cur], b'x');

		let diagnostic =
			Diagnostic::error(self.file_id, "", "invalid digits after hex escape sequence")
				.primary(
					(self.cur - 1)..(self.cur + 1),
					"Expected 2 hex digits following this",
				);

		for _ in 0..2 {
			match self.next_bounded() {
				None => return Some(diagnostic),
				Some(b) if !(*b as u8).is_ascii_hexdigit() => return Some(diagnostic),
				_ => {}
			}
		}
		None
	}

	// Validate a `\..` escape sequence and advance the lexer based on it
	fn validate_escape_sequence(&mut self) -> Option<Diagnostic> {
		let cur = self.cur;
		if let Some(escape) = self.bytes.get(self.cur + 1) {
			match escape {
				b'u' if self.bytes.get(self.cur + 2) == Some(&b'{') => {
					self.advance(2);
					self.read_codepoint_escape().err()
				}
				b'u' => {
					self.next();
					self.read_unicode_escape(true).err()
				}
				b'x' => {
					self.next();
					self.validate_hex_escape()
				}
				_ => {
					// We use get_unicode_char to account for escaped source characters which are unicode
					let chr = self.get_unicode_char();
					self.cur += chr.len_utf8();
					None
				}
			}
		} else {
			Some(Diagnostic::error(self.file_id, "", "").primary(
				cur..cur + 1,
				"expected an escape sequence following a backslash, but found none",
			))
		}
	}

	// Consume an identifier by recursively consuming IDENTIFIER_PART kind chars
	#[inline]
	fn consume_ident(&mut self) {
		unwind_loop! {
			if self.next_bounded().is_some() {
				if self.cur_ident_part().is_none() {
					return;
				}
			} else {
				return;
			}
		}
	}

	/// Consumes the identifier at the current position, and fills the given buf with the UTF-8
	/// encoded identifier that got consumed.
	///
	/// Returns the number of bytes written into the buffer.
	/// This method will stop writing into the buffer if the buffer is too small to
	/// fit the whole identifier.
	#[inline]
	fn consume_and_get_ident(&mut self, buf: &mut [u8]) -> usize {
		let mut idx = 0;

		unwind_loop! {
			if self.next_bounded().is_some() {
				if let Some(c) = self.cur_ident_part() {
					if let Some(buf) = buf.get_mut(idx..idx + 4) {
						let res = c.encode_utf8(buf);
						idx += res.len();
					}
				} else {
					return idx;
				}
			} else {
				return idx;
			}
		}
	}

	// Consume a string literal and advance the lexer, and returning a list of errors that occurred when reading the string
	// This could include unterminated string and invalid escape sequences
	fn read_str_literal(&mut self) -> Option<Diagnostic> {
		// Safety: this is only ever called from lex_token, which is guaranteed to be called on a char position
		let quote = unsafe { *self.bytes.get_unchecked(self.cur) };
		let start = self.cur;
		let mut diagnostic = None;

		while let Some(byte) = self.next_bounded() {
			match *byte {
				b'\\' => {
					diagnostic = self.validate_escape_sequence();
				}
				b if b == quote => {
					self.next();
					return diagnostic;
				}
				_ => {}
			}
		}

		let unterminated = Diagnostic::error(self.file_id, "", "unterminated string literal")
			.primary(self.cur..self.cur, "input ends here")
			.secondary(start..start + 1, "string literal starts here");

		Some(unterminated)
	}

	/// Returns `Some(x)` if the current position is an identifier, with the character at
	/// the position.
	///
	/// The character may be a char that was generated from a unicode escape sequence,
	/// e.g. `t` is returned, the actual source code is `\u{74}`
	#[inline]
	fn cur_ident_part(&mut self) -> Option<char> {
		debug_assert!(self.cur < self.bytes.len());

		// Safety: we always call this method on a char
		let b = unsafe { self.bytes.get_unchecked(self.cur) };

		match Self::lookup(*b) {
			IDT | DIG | ZER => Some(*b as char),
			// FIXME: This should use ID_Continue, not XID_Continue
			UNI => {
				let chr = self.get_unicode_char();
				let res = is_id_continue(chr);
				if res {
					self.cur += chr.len_utf8() - 1;
					Some(chr)
				} else {
					None
				}
			}
			BSL if self.bytes.get(self.cur + 1) == Some(&b'u') => {
				let start = self.cur;
				self.next();
				let res = if self.bytes.get(self.cur + 1).copied() == Some(b'{') {
					self.next();
					self.read_codepoint_escape()
				} else {
					self.read_unicode_escape(true)
				};

				if let Ok(c) = res {
					if is_id_continue(c) {
						Some(c)
					} else {
						self.cur -= 1;
						None
					}
				} else {
					self.cur = start;
					None
				}
			}
			_ => None,
		}
	}

	// check if the current char is an identifier start, this implicitly advances if the char being matched
	// is a `\uxxxx` sequence which is an identifier start, or if the char is a unicode char which is an identifier start
	#[inline]
	fn cur_is_ident_start(&mut self) -> bool {
		debug_assert!(self.cur < self.bytes.len());

		// Safety: we always call this method on a char
		let b = unsafe { self.bytes.get_unchecked(self.cur) };

		match Self::lookup(*b) {
			BSL if self.bytes.get(self.cur + 1) == Some(&b'u') => {
				self.next();
				if let Ok(chr) = self.read_unicode_escape(false) {
					if is_id_start(chr) {
						self.advance(5);
						return true;
					}
				}
				self.cur -= 1;
				false
			}
			UNI => {
				let chr = self.get_unicode_char();
				if is_id_start(chr) {
					self.cur += chr.len_utf8() - 1;
					true
				} else {
					false
				}
			}
			IDT => true,
			_ => false,
		}
	}

	/// Returns the identifier token at the current position, or the keyword token if
	/// the identifier is a keyword.
	///
	/// `first` is a pair of a character that was already consumed,
	/// but is still part of the identifier, and the characters position.
	#[inline]
	fn resolve_identifier(&mut self, first: (char, usize)) -> LexerReturn {
		use SyntaxKind::*;

		// Note to keep the buffer large enough to fit every possible keyword that
		// the lexer can return
		let mut buf = [0u8; 16];
		let (len, start) = (first.0.encode_utf8(&mut buf).len(), first.1);

		let count = self.consume_and_get_ident(&mut buf[len..]);

		let kind = match &buf[..count + len] {
			b"await" => Some(AWAIT_KW),
			b"break" => Some(BREAK_KW),
			b"case" => Some(CASE_KW),
			b"catch" => Some(CATCH_KW),
			b"class" => Some(CLASS_KW),
			b"const" => Some(CONST_KW),
			b"continue" => Some(CONTINUE_KW),
			b"debugger" => Some(DEBUGGER_KW),
			b"default" => Some(DEFAULT_KW),
			b"delete" => Some(DELETE_KW),
			b"do" => Some(DO_KW),
			b"else" => Some(ELSE_KW),
			b"enum" => Some(ENUM_KW),
			b"export" => Some(EXPORT_KW),
			b"extends" => Some(EXTENDS_KW),
			b"false" => Some(FALSE_KW),
			b"finally" => Some(FINALLY_KW),
			b"for" => Some(FOR_KW),
			b"function" => Some(FUNCTION_KW),
			b"if" => Some(IF_KW),
			b"in" => Some(IN_KW),
			b"import" => Some(IMPORT_KW),
			b"instanceof" => Some(INSTANCEOF_KW),
			b"new" => Some(NEW_KW),
			b"null" => Some(NULL_KW),
			b"return" => Some(RETURN_KW),
			b"super" => Some(SUPER_KW),
			b"switch" => Some(SWITCH_KW),
			b"this" => Some(THIS_KW),
			b"throw" => Some(THROW_KW),
			b"try" => Some(TRY_KW),
			b"true" => Some(TRUE_KW),
			b"typeof" => Some(TYPEOF_KW),
			b"var" => Some(VAR_KW),
			b"void" => Some(VOID_KW),
			b"while" => Some(WHILE_KW),
			b"with" => Some(WITH_KW),
			b"yield" => Some(YIELD_KW),
			_ => None,
		};

		if let Some(kind) = kind {
			(Token::new(kind, self.cur - start), None)
		} else {
			(Token::new(T![ident], self.cur - start), None)
		}
	}

	#[inline]
	fn special_number_start<F: Fn(char) -> bool>(&mut self, func: F) -> bool {
		if self
			.bytes
			.get(self.cur + 2)
			.map(|b| func(*b as char))
			.unwrap_or(false)
		{
			self.cur += 1;
			true
		} else {
			false
		}
	}

	#[inline]
	fn maybe_bigint(&mut self) {
		if let Some(b'n') = self.bytes.get(self.cur) {
			self.next();
		}
	}

	#[inline]
	fn read_zero(&mut self) -> Option<Diagnostic> {
		// TODO: Octal literals
		match self.bytes.get(self.cur + 1) {
			Some(b'x') | Some(b'X') => {
				if self.special_number_start(|c| c.is_ascii_hexdigit()) {
					let diag = self.read_hexnumber();
					self.maybe_bigint();
					diag
				} else {
					self.next();
					None
				}
			}
			Some(b'b') | Some(b'B') => {
				if self.special_number_start(|c| c == '0' || c == '1') {
					let diag = self.read_bindigits();
					self.maybe_bigint();
					diag
				} else {
					self.next();
					None
				}
			}
			Some(b'o') | Some(b'O') => {
				if self.special_number_start(|c| ('0'..='7').contains(&c)) {
					let diag = self.read_octaldigits();
					self.maybe_bigint();
					diag
				} else {
					self.next();
					None
				}
			}
			Some(b'n') => {
				self.cur += 2;
				None
			}
			Some(b'.') => {
				self.cur += 1;
				self.read_float()
			}
			Some(b'e') | Some(b'E') => {
				// At least one digit is required
				match self.bytes.get(self.cur + 2) {
					Some(b'-') | Some(b'+') => {
						if let Some(b'0'..=b'9') = self.bytes.get(self.cur + 3) {
							self.next();
							self.read_exponent()
						} else {
							None
						}
					}
					Some(b'0'..=b'9') => self.read_exponent(),
					_ => {
						self.next();
						None
					}
				}
			}
			// FIXME: many engines actually allow things like `09`, but by the spec, this is not allowed
			// maybe we should not allow it if we want to go fully by the spec
			_ => self.read_number(),
		}
	}

	#[inline]
	fn read_hexnumber(&mut self) -> Option<Diagnostic> {
		let mut diag = None;
		unwind_loop! {
			match self.next() {
				Some(b'_') => diag = diag.or(self.handle_numeric_separator(16)),
				Some(b) if char::from(*b).is_ascii_hexdigit() => {},
				_ => return diag,
			}
		}
	}

	#[inline]
	fn handle_numeric_separator(&mut self, radix: u8) -> Option<Diagnostic> {
		debug_assert_eq!(self.bytes[self.cur], b'_');

		let err_diag = Diagnostic::error(
			self.file_id,
			"",
			"numeric separators are only allowed between two digits",
		)
		.primary(self.cur..self.cur + 1, "");

		let peeked = self.bytes.get(self.cur + 1).copied();

		if peeked.is_none() || !char::from(peeked.unwrap()).is_digit(radix as u32) {
			return Some(err_diag);
		}

		let forbidden = |c: Option<u8>| {
			if c.is_none() {
				return true;
			}
			let c = c.unwrap();

			if radix == 16 {
				matches!(c, b'.' | b'X' | b'_' | b'x')
			} else {
				matches!(c, b'.' | b'B' | b'E' | b'O' | b'_' | b'b' | b'e' | b'o')
			}
		};

		let prev = self.bytes.get(self.cur - 1).copied();

		if forbidden(prev) || forbidden(peeked) {
			return Some(err_diag);
		}

		self.next_bounded();
		None
	}

	// Read a number which does not start with 0, since that can be more things and is handled
	// by another function
	#[inline]
	fn read_number(&mut self) -> Option<Diagnostic> {
		let mut diag = None;
		unwind_loop! {
			match self.next_bounded() {
				Some(b'_') => diag = diag.or(self.handle_numeric_separator(10)),
				Some(b'0'..=b'9') => {},
				Some(b'.') => {
					return self.read_float();
				},
				// TODO: merge this, and read_float's implementation into one so we dont duplicate exponent code
				Some(b'e') | Some(b'E') => {
					// At least one digit is required
					match self.bytes.get(self.cur + 1) {
						Some(b'-') | Some(b'+') => {
							if let Some(b'0'..=b'9') = self.bytes.get(self.cur + 2) {
								self.next();
								return self.read_exponent();
							} else {
								return diag;
							}
						},
						Some(b'0'..=b'9') => return self.read_exponent(),
						_ => return diag,
					}
				},
				Some(b'n') => {
					self.next();
					return diag;
				}
				_ => return diag,
			}
		}
	}

	#[inline]
	fn read_float(&mut self) -> Option<Diagnostic> {
		let mut diag = None;

		unwind_loop! {
			match self.next_bounded() {
				Some(b'_') => diag = diag.or(self.handle_numeric_separator(16)),
				// LLVM has a hard time optimizing inclusive patterns, perhaps we should check if it makes llvm sad,
				// and optimize this into a lookup table
				Some(b'0'..=b'9') => {},
				Some(b'e') | Some(b'E') => {
					// At least one digit is required
					match self.bytes.get(self.cur + 1) {
						Some(b'-') | Some(b'+') => {
							if let Some(b'0'..=b'9') = self.bytes.get(self.cur + 2) {
								self.next();
								return self.read_exponent().or(diag);
							} else {
								return diag;
							}
						},
						Some(b'0'..=b'9') => return self.read_exponent().or(diag),
						_ => return diag,
					}
				},
				_ => return diag,
			}
		}
	}

	#[inline]
	fn read_exponent(&mut self) -> Option<Diagnostic> {
		if let Some(b'-') | Some(b'+') = self.bytes.get(self.cur + 1) {
			self.next();
		}

		let mut diag = None;
		unwind_loop! {
			match self.next() {
				Some(b'_') => diag = diag.or(self.handle_numeric_separator(10)),
				Some(b'0'..=b'9') => {},
				_ => return diag,
			}
		}
	}

	#[inline]
	fn read_bindigits(&mut self) -> Option<Diagnostic> {
		let mut diag = None;
		unwind_loop! {
			match self.next() {
				Some(b'_') => diag = diag.or(self.handle_numeric_separator(2)),
				Some(b'0') | Some(b'1') => {},
				_ => return diag,
			}
		}
	}

	#[inline]
	fn read_octaldigits(&mut self) -> Option<Diagnostic> {
		let mut diag = None;
		unwind_loop! {
			match self.next() {
				Some(b'_') => diag = diag.or(self.handle_numeric_separator(8)),
				Some(b'0'..=b'7') => {},
				_ => return diag,
			}
		}
	}

	#[inline]
	fn verify_number_end(&mut self, start: usize) -> LexerReturn {
		let err_start = self.cur;
		if self.cur < self.bytes.len() && self.cur_is_ident_start() {
			self.consume_ident();
			let err = Diagnostic::error(
				self.file_id,
				"",
				"numbers cannot be followed by identifiers directly after",
			)
			.primary(err_start..self.cur, "an identifier cannot appear here");

			(
				Token::new(SyntaxKind::ERROR_TOKEN, self.cur - start),
				Some(err),
			)
		} else {
			tok!(JS_NUMBER_LITERAL, self.cur - start)
		}
	}

	#[inline]
	fn read_shebang(&mut self) -> LexerReturn {
		let start = self.cur;
		self.next();
		if start != 0 {
			return (Token::new(T![#], 1), None);
		}

		if let Some(b'!') = self.bytes.get(1) {
			while self.next().is_some() {
				let chr = self.get_unicode_char();

				if is_linebreak(chr) {
					return tok!(JS_SHEBANG, self.cur);
				}
				self.cur += chr.len_utf8() - 1;
			}
			tok!(JS_SHEBANG, self.cur)
		} else {
			let err = Diagnostic::error(
				self.file_id,
				"",
				"expected `!` following a `#`, but found none",
			)
			.primary(0usize..1usize, "");

			(Token::new(SyntaxKind::ERROR_TOKEN, 1), Some(err))
		}
	}

	#[inline]
	fn read_slash(&mut self) -> LexerReturn {
		let start = self.cur;
		match self.bytes.get(self.cur + 1) {
			Some(b'*') => {
				self.next();
				while let Some(b) = self.next().copied() {
					match b {
						b'*' if self.bytes.get(self.cur + 1) == Some(&b'/') => {
							self.advance(2);
							return tok!(COMMENT, self.cur - start);
						}
						_ => {}
					}
				}

				let err = Diagnostic::error(self.file_id, "", "unterminated block comment")
					.primary(self.cur..self.cur + 1, "... but the file ends here")
					.secondary(start..start + 2, "A block comment starts here");

				(Token::new(SyntaxKind::COMMENT, self.cur - start), Some(err))
			}
			Some(b'/') => {
				self.next();
				while self.next().is_some() {
					let chr = self.get_unicode_char();

					if is_linebreak(chr) {
						return tok!(COMMENT, self.cur - start);
					}
					self.cur += chr.len_utf8() - 1;
				}
				tok!(COMMENT, self.cur - start)
			}
			_ if self.state.expr_allowed => self.read_regex(),
			Some(b'=') => {
				self.advance(2);
				tok!(SLASHEQ, self.cur - start)
			}
			_ => self.eat(tok![/]),
		}
	}

	#[inline]
	fn flag_err(&self, flag: char) -> Diagnostic {
		Diagnostic::error(self.file_id, "", format!("duplicate flag `{}`", flag))
			.primary(self.cur..self.cur + 1, "this flag was already used")
	}

	// TODO: Due to our return of (Token, Option<Error>) we cant issue more than one regex error
	// This is not a huge issue but it would be helpful to users
	#[inline]
	#[allow(clippy::many_single_char_names)]
	fn read_regex(&mut self) -> LexerReturn {
		let start = self.cur;
		let mut in_class = false;
		let mut diagnostic = None;

		unwind_loop! {
			match self.next() {
				Some(b'[') => in_class = true,
				Some(b']') => in_class = false,
				Some(b'/') => {
					if !in_class {
						let (mut g, mut i, mut m, mut s, mut u, mut y) = (false, false, false, false, false, false);

						unwind_loop! {
							let next = self.next_bounded().copied();
							let chr_start = self.cur;
							match next {
							   Some(b'g') => {
								   if g && diagnostic.is_none() {
										diagnostic = Some(self.flag_err('g'))
								   }
								   g = true;
							   },
							   Some(b'i') => {
									if i && diagnostic.is_none() {
										diagnostic = Some(self.flag_err('i'))
									}
									i = true;
							   },
							   Some(b'm') => {
									if m && diagnostic.is_none() {
										diagnostic = Some(self.flag_err('m'))
									}
									m = true;
							   },
							   Some(b's') => {
									if s && diagnostic.is_none() {
										diagnostic = Some(self.flag_err('s'))
									}
									s = true;
								},
								Some(b'u') => {
									if u && diagnostic.is_none() {
										diagnostic = Some(self.flag_err('u'))
									}
									u = true;
							   },
							   Some(b'y') => {
									if y && diagnostic.is_none() {
										diagnostic = Some(self.flag_err('y'))
									}
									y = true;
								},
								Some(_) if self.cur_ident_part().is_some() => {
									if diagnostic.is_none() {
										diagnostic = Some(Diagnostic::error(self.file_id, "", "invalid regex flag")
											.primary(chr_start .. self.cur + 1, "this is not a valid regex flag"));
									}
								},
								_ => {
									return (Token::new(SyntaxKind::JS_REGEX_LITERAL, self.cur - start), diagnostic)
								}
							}
						}
					}
				},
				Some(b'\\') => {
					if self.next_bounded().is_none() {
						let err = Diagnostic::error(self.file_id, "", "expected a character after a regex escape, but found none")
							.primary(self.cur..self.cur + 1, "expected a character following this");

						return (Token::new(SyntaxKind::JS_REGEX_LITERAL, self.cur - start), Some(err));
					}
				},
				None => {
					let err = Diagnostic::error(self.file_id, "", "unterminated regex literal")
						.primary(self.cur..self.cur, "...but the file ends here")
						.secondary(start..start + 1, "a regex literal starts there...");

					return (Token::new(SyntaxKind::JS_REGEX_LITERAL, self.cur - start), Some(err));
				},
				_ => {},
			}
		}
	}

	#[inline]
	fn bin_or_assign(&mut self, bin: SyntaxKind, assign: SyntaxKind) -> LexerReturn {
		if let Some(b'=') = self.next() {
			self.next();
			(Token::new(assign, 2), None)
		} else {
			(Token::new(bin, 1), None)
		}
	}

	#[inline]
	fn resolve_bang(&mut self) -> LexerReturn {
		match self.next() {
			Some(b'=') => {
				if let Some(b'=') = self.next() {
					self.next();
					tok!(NEQ2, 3)
				} else {
					tok!(NEQ, 2)
				}
			}
			_ => tok!(!),
		}
	}

	#[inline]
	fn resolve_amp(&mut self) -> LexerReturn {
		match self.next() {
			Some(b'&') => {
				if let Some(b'=') = self.next() {
					self.next();
					tok!(AMP2EQ, 3)
				} else {
					tok!(AMP2, 2)
				}
			}
			Some(b'=') => {
				self.next();
				tok!(AMPEQ, 2)
			}
			_ => tok!(&),
		}
	}

	#[inline]
	fn resolve_plus(&mut self) -> LexerReturn {
		match self.next() {
			Some(b'+') => {
				self.next();
				tok!(PLUS2, 2)
			}
			Some(b'=') => {
				self.next();
				tok!(PLUSEQ, 2)
			}
			_ => tok!(+),
		}
	}

	#[inline]
	fn resolve_minus(&mut self) -> LexerReturn {
		match self.next() {
			Some(b'-') => {
				self.next();
				tok!(MINUS2, 2)
			}
			Some(b'=') => {
				self.next();
				tok!(MINUSEQ, 2)
			}
			_ => tok!(-),
		}
	}

	#[inline]
	fn resolve_less_than(&mut self) -> LexerReturn {
		match self.next() {
			Some(b'<') => {
				if let Some(b'=') = self.next() {
					self.next();
					tok!(SHLEQ, 3)
				} else {
					tok!(SHL, 2)
				}
			}
			Some(b'=') => {
				self.next();
				tok!(LTEQ, 2)
			}
			_ => tok!(<),
		}
	}

	#[inline]
	fn resolve_greater_than(&mut self) -> LexerReturn {
		match self.next() {
			Some(b'>') => {
				if let Some(b'>') = self.bytes.get(self.cur + 1).copied() {
					if let Some(b'=') = self.bytes.get(self.cur + 2).copied() {
						self.advance(3);
						tok!(USHREQ, 4)
					} else {
						tok!(>)
					}
				} else if self.bytes.get(self.cur + 1).copied() == Some(b'=') {
					self.advance(2);
					tok!(SHREQ, 3)
				} else {
					tok!(>)
				}
			}
			Some(b'=') => {
				self.next();
				tok!(GTEQ, 2)
			}
			_ => tok!(>),
		}
	}

	#[inline]
	fn resolve_eq(&mut self) -> LexerReturn {
		match self.next() {
			Some(b'=') => {
				if let Some(b'=') = self.next() {
					self.next();
					tok!(EQ3, 3)
				} else {
					tok!(EQ2, 2)
				}
			}
			Some(b'>') => {
				self.next();
				tok!(FAT_ARROW, 2)
			}
			_ => tok!(=),
		}
	}

	#[inline]
	fn resolve_pipe(&mut self) -> LexerReturn {
		match self.next() {
			Some(b'|') => {
				if let Some(b'=') = self.next() {
					self.next();
					tok!(PIPE2EQ, 3)
				} else {
					tok!(PIPE2, 2)
				}
			}
			Some(b'=') => {
				self.next();
				tok!(PIPEEQ, 2)
			}
			_ => tok!(|),
		}
	}

	// Dont ask it to resolve the question of life's meaning because you'll be dissapointed
	#[inline]
	fn resolve_question(&mut self) -> LexerReturn {
		match self.next() {
			Some(b'?') => {
				if let Some(b'=') = self.next() {
					self.next();
					tok!(QUESTION2EQ, 3)
				} else {
					tok!(QUESTION2, 2)
				}
			}
			Some(b'.') => {
				// 11.7 Optional chaining punctuator
				if let Some(b'0'..=b'9') = self.bytes.get(self.cur + 1) {
					tok!(?)
				} else {
					self.next();
					tok!(QUESTIONDOT, 2)
				}
			}
			_ => tok!(?),
		}
	}

	#[inline]
	fn resolve_star(&mut self) -> LexerReturn {
		match self.next() {
			Some(b'*') => {
				if let Some(b'=') = self.next() {
					self.next();
					tok!(STAR2EQ, 3)
				} else {
					tok!(STAR2, 2)
				}
			}
			Some(b'=') => {
				self.next();
				tok!(STAREQ, 2)
			}
			_ => tok!(*),
		}
	}

	/// Lex the next token
	fn lex_token(&mut self) -> LexerReturn {
		// Safety: we always call lex_token when we are at a valid char
		let byte = unsafe { *self.bytes.get_unchecked(self.cur) };
		let start = self.cur;

		// A lookup table of `byte -> fn(l: &mut Lexer) -> Token` is exponentially slower than this approach
		// The speed difference comes from the difference in table size, a 2kb table is easily fit into cpu cache
		// While a 16kb table will be ejected from cache very often leading to slowdowns, this also allows LLVM
		// to do more aggressive optimizations on the match regarding how to map it to instructions
		let dispatched = Self::lookup(byte);

		match dispatched {
			WHS => {
				self.consume_whitespace();
				tok!(WHITESPACE, self.cur - start)
			}
			EXL => self.resolve_bang(),
			HAS => self.read_shebang(),
			PRC => self.bin_or_assign(T![%], T![%=]),
			AMP => self.resolve_amp(),
			PNO => self.eat(tok!(L_PAREN, 1)),
			PNC => self.eat(tok!(R_PAREN, 1)),
			MUL => self.resolve_star(),
			PLS => self.resolve_plus(),
			COM => self.eat(tok![,]),
			MIN => self.resolve_minus(),
			SLH => self.read_slash(),
			// This simply changes state on the start
			TPL => self.eat(tok!(BACKTICK, 1)),
			ZER => {
				let diag = self.read_zero();
				let (token, err) = self.verify_number_end(start);
				(token, err.or(diag))
			}
			PRD => {
				if let Some(b"..") = self.bytes.get(self.cur + 1..self.cur + 3) {
					self.cur += 3;
					return tok!(DOT2, 3);
				}
				if let Some(b'0'..=b'9') = self.bytes.get(self.cur + 1) {
					let diag = self.read_float();
					let (token, err) = self.verify_number_end(start);
					(token, err.or(diag))
				} else {
					self.eat(tok![.])
				}
			}
			BSL => {
				if self.bytes.get(self.cur + 1) == Some(&b'u') {
					self.next();
					let res = if self.bytes.get(self.cur + 1).copied() == Some(b'{') {
						self.next();
						self.read_codepoint_escape()
					} else {
						self.read_unicode_escape(true)
					};

					match res {
						Ok(chr) => {
							if is_id_start(chr) {
								self.resolve_identifier((chr, start))
							} else {
								let err = Diagnostic::error(self.file_id, "", "unexpected unicode escape")
                                    .primary(start..self.cur, "this escape is unexpected, as it does not designate the start of an identifier");

								self.next();
								(
									Token::new(SyntaxKind::ERROR_TOKEN, self.cur - start),
									Some(err),
								)
							}
						}
						Err(err) => (
							Token::new(SyntaxKind::ERROR_TOKEN, self.cur - start),
							Some(err),
						),
					}
				} else {
					let err = Diagnostic::error(
						self.file_id,
						"",
						format!("unexpected token `{}`", byte as char),
					)
					.primary(start..self.cur + 1, "");
					self.next();

					(Token::new(SyntaxKind::ERROR_TOKEN, 1), Some(err))
				}
			}
			QOT => {
				if let Some(err) = self.read_str_literal() {
					(
						Token::new(SyntaxKind::ERROR_TOKEN, self.cur - start),
						Some(err),
					)
				} else {
					tok!(JS_STRING_LITERAL, self.cur - start)
				}
			}
			IDT => self.resolve_identifier((byte as char, start)),
			DIG => {
				let diag = self.read_number();
				let (token, err) = self.verify_number_end(start);
				(token, err.or(diag))
			}
			COL => self.eat(tok![:]),
			SEM => self.eat(tok![;]),
			LSS => self.resolve_less_than(),
			EQL => self.resolve_eq(),
			MOR => self.resolve_greater_than(),
			QST => self.resolve_question(),
			BTO => self.eat(tok!(L_BRACK, 1)),
			BTC => self.eat(tok![R_BRACK, 1]),
			CRT => self.bin_or_assign(T![^], T![^=]),
			BEO => self.eat(tok![L_CURLY, 1]),
			BEC => self.eat(tok![R_CURLY, 1]),
			PIP => self.resolve_pipe(),
			TLD => self.eat(tok![~]),
			UNI => {
				let chr = self.get_unicode_char();
				if is_linebreak(chr)
					|| (UNICODE_WHITESPACE_STARTS.contains(&byte) && UNICODE_SPACES.contains(&chr))
				{
					self.consume_whitespace();
					tok!(WHITESPACE, self.cur - start)
				} else {
					self.cur += chr.len_utf8() - 1;
					if is_id_start(chr) {
						self.resolve_identifier((chr, start))
					} else {
						let err = Diagnostic::error(
							self.file_id,
							"",
							format!("Unexpected token `{}`", chr as char),
						)
						.primary(start..self.cur + 1, "");
						self.next();

						(
							Token::new(SyntaxKind::ERROR_TOKEN, self.cur - start),
							Some(err),
						)
					}
				}
			}
			AT_ => self.eat(tok![@]),
			_ => {
				let err = Diagnostic::error(
					self.file_id,
					"",
					format!("unexpected token `{}`", byte as char),
				)
				.primary(start..self.cur + 1, "");
				self.next();

				(Token::new(SyntaxKind::ERROR_TOKEN, 1), Some(err))
			}
		}
	}

	fn lex_template(&mut self) -> LexerReturn {
		let start = self.cur;
		let mut diagnostic = None;

		while let Some(b) = self.bytes.get(self.cur) {
			match *b as char {
				'`' if self.cur == start => {
					self.next();
					return tok!(BACKTICK, 1);
				}
				'`' => {
					return (
						Token::new(SyntaxKind::TEMPLATE_CHUNK, self.cur - start),
						diagnostic,
					);
				}
				'\\' => {
					if let Some(err) = self.validate_escape_sequence() {
						diagnostic = Some(err);
					}
					self.next_bounded();
				}
				'$' if self.bytes.get(self.cur + 1) == Some(&b'{') && self.cur == start => {
					self.advance(2);
					return (Token::new(SyntaxKind::DOLLAR_CURLY, 2), diagnostic);
				}
				'$' if self.bytes.get(self.cur + 1) == Some(&b'{') => {
					return (
						Token::new(SyntaxKind::TEMPLATE_CHUNK, self.cur - start),
						diagnostic,
					)
				}
				_ => {
					let _ = self.next();
				}
			}
		}

		let err = Diagnostic::error(self.file_id, "", "unterminated template literal")
			.primary(self.cur..self.cur + 1, "");

		(
			Token::new(SyntaxKind::TEMPLATE_CHUNK, self.cur - start),
			Some(err),
		)
	}
}

/// Check if a char is a JS linebreak
pub fn is_linebreak(chr: char) -> bool {
	['\n', '\r', '\u{2028}', '\u{2029}'].contains(&chr)
}

impl Iterator for Lexer<'_> {
	type Item = LexerReturn;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cur >= self.bytes.len() {
			if !self.returned_eof {
				self.returned_eof = true;
				return Some(tok!(EOF, 0));
			}
			return None;
		}

		let token = if self.state.is_in_template() {
			self.lex_template()
		} else {
			self.lex_token()
		};

		if ![
			SyntaxKind::COMMENT,
			SyntaxKind::WHITESPACE,
			SyntaxKind::TEMPLATE_CHUNK,
		]
		.contains(&token.0.kind)
		{
			self.state.update(token.0.kind);
		}
		Some(token)
	}
}

// Every handler a byte coming in could be mapped to
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Dispatch {
	ERR,
	WHS,
	EXL,
	QOT,
	IDT,
	HAS,
	PRC,
	AMP,
	PNO,
	PNC,
	MUL,
	PLS,
	COM,
	MIN,
	PRD,
	SLH,
	ZER,
	DIG,
	COL,
	SEM,
	LSS,
	EQL,
	MOR,
	QST,
	AT_,
	BTO,
	BSL,
	BTC,
	CRT,
	TPL,
	BEO,
	PIP,
	BEC,
	TLD,
	UNI,
}
use Dispatch::*;

// A lookup table mapping any incoming byte to a handler function
// This is taken from the ratel project lexer and modified
// FIXME: Should we ignore the first ascii control chars which are nearly never seen instead of returning Err?
static DISPATCHER: [Dispatch; 256] = [
	//0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F   //
	ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, WHS, WHS, WHS, WHS, WHS, ERR, ERR, // 0
	ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, ERR, // 1
	WHS, EXL, QOT, HAS, IDT, PRC, AMP, QOT, PNO, PNC, MUL, PLS, COM, MIN, PRD, SLH, // 2
	ZER, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, DIG, COL, SEM, LSS, EQL, MOR, QST, // 3
	AT_, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, // 4
	IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, BTO, BSL, BTC, CRT, IDT, // 5
	TPL, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, // 6
	IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, IDT, BEO, PIP, BEC, TLD, ERR, // 7
	UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // 8
	UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // 9
	UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // A
	UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // B
	UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // C
	UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // D
	UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // E
	UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // F
];
