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

#[rustfmt::skip]
mod tables;
mod errors;
mod tests;

pub mod buffered_lexer;
mod bytes;
#[cfg(feature = "highlight")]
mod highlight;

use bitflags::bitflags;
#[cfg(feature = "highlight")]
pub use highlight::*;

use rslint_errors::Diagnostic;
use tables::derived_property::*;

pub use rome_js_syntax::*;

use crate::bytes::{
    lookup_byte,
    Dispatch::{self, *},
};
use rome_js_syntax::JsSyntaxKind::*;
use rslint_errors::file::FileId;

use crate::errors::invalid_digits_after_unicode_escape_sequence;

#[derive(Debug)]
pub struct LexerReturn {
    /// The token kind
    pub kind: JsSyntaxKind,

    /// Diagnostics associated with the current token, if any.
    pub diagnostic: Option<Box<Diagnostic>>,
}

impl LexerReturn {
    /// Creates a lexer return for a token with the given kind but without any diagnostic
    pub fn ok(kind: JsSyntaxKind) -> Self {
        Self::new(kind, None)
    }

    pub fn new(kind: JsSyntaxKind, diagnostic: Option<Box<Diagnostic>>) -> Self {
        Self { kind, diagnostic }
    }

    /// Creates a lexer return for a token of the given kind and with the given diagnostic
    pub fn with_diagnostic(kind: JsSyntaxKind, diagnostic: Box<Diagnostic>) -> Self {
        Self::new(kind, Some(diagnostic))
    }
}

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

/// Context in which the lexer should lex the next token
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LexContext {
    /// Default context for if the lexer isn't in any specific other context
    Regular,

    /// For lexing the elements of a JS template literal or TS template type.
    /// Doesn't skip whitespace trivia.
    TemplateElement { tagged: bool },
}

impl Default for LexContext {
    fn default() -> Self {
        LexContext::Regular
    }
}

impl LexContext {
    /// Returns true if this is [LexContext:Regular]
    pub fn is_regular(&self) -> bool {
        matches!(self, LexContext::Regular)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ReLexContext {
    /// Re-lexes a `/` or `/=` token as a regular expression.
    Regex,
    /// Re-lexes `'>', '>'` as `>>` and `'>', '>', '>'` as `>>>`
    BinaryOperator,
    /// Re-lexes `'<', '<'` as `<<` in places where a type argument is expected to support
    /// `B<<A>()>`
    TypeArgumentLessThan,
}

bitflags! {
    /// Flags for a lexed token.
    pub(crate) struct TokenFlags: u8 {
        /// Indicates that there has been a line break between the last non-trivia token
        const PRECEDING_LINE_BREAK = 1 << 0;

        /// Indicates that an identifier contains an unicode escape sequence
        const UNICODE_ESCAPE = 1 << 1;
    }
}

impl TokenFlags {
    pub const fn has_preceding_line_break(&self) -> bool {
        self.contains(TokenFlags::PRECEDING_LINE_BREAK)
    }

    pub const fn has_unicode_escape(&self) -> bool {
        self.contains(TokenFlags::UNICODE_ESCAPE)
    }
}

/// An extremely fast, lookup table based, lossless ECMAScript lexer
#[derive(Debug)]
pub struct Lexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    /// `true` if there has been a line break between the last non-trivia token and the next non-trivia token.
    after_newline: bool,

    /// Byte offset of the current token from the start of the source
    /// The range of the current token can be computed by `self.position - self.current_start`
    current_start: TextSize,

    /// The kind of the current token
    current_kind: JsSyntaxKind,

    /// Flags for the current token
    current_flags: TokenFlags,

    /// The id of the file, used for diagnostics
    file_id: FileId,
}

impl<'src> Lexer<'src> {
    /// Make a new lexer from a str, this is safe because strs are valid utf8
    pub fn from_str(string: &'src str, file_id: FileId) -> Self {
        Self {
            source: string,
            after_newline: false,
            current_kind: TOMBSTONE,
            current_start: TextSize::from(0),
            current_flags: TokenFlags::empty(),
            position: 0,
            file_id,
        }
    }

    /// Returns the source code
    pub fn source(&self) -> &'src str {
        self.source
    }

    /// Returns the kind of the current token
    #[inline]
    pub const fn current(&self) -> JsSyntaxKind {
        self.current_kind
    }

    /// Returns the range of the current token (The token that was lexed by the last `next` call)
    #[inline]
    pub fn current_range(&self) -> TextRange {
        TextRange::new(self.current_start, TextSize::from(self.position as u32))
    }

    /// Returns true if a line break precedes the current token.
    #[inline]
    pub const fn has_preceding_line_break(&self) -> bool {
        self.current_flags.has_preceding_line_break()
    }

    /// Returns `true` if the current token is an identifier and it contains a unicode escape sequence (`\u...`).
    #[inline]
    pub const fn has_unicode_escape(&self) -> bool {
        self.current_flags.has_unicode_escape()
    }

    /// Creates a checkpoint storing the current lexer state.
    ///
    /// Use `rewind` to restore the lexer to the state stored in the checkpoint.
    pub fn checkpoint(&self) -> LexerCheckpoint {
        LexerCheckpoint {
            position: TextSize::from(self.position as u32),
            current_start: self.current_start,
            current_flags: self.current_flags,
            current_kind: self.current_kind,
            after_line_break: self.after_newline,
        }
    }

    /// Rewinds the lexer to the same state as when the passed in `checkpoint` was created.
    pub fn rewind(&mut self, checkpoint: LexerCheckpoint) {
        let LexerCheckpoint {
            position,
            current_start,
            current_flags,
            current_kind,
            after_line_break,
        } = checkpoint;

        let new_pos = u32::from(position) as usize;

        self.position = new_pos;
        self.current_kind = current_kind;
        self.current_start = current_start;
        self.current_flags = current_flags;
        self.after_newline = after_line_break;
    }

    /// Lex's the next token.
    ///
    /// ## Return
    /// Returns its kind and any potential error.
    pub fn next_token(&mut self, context: LexContext) -> LexerReturn {
        self.current_start = TextSize::from(self.position as u32);
        self.current_flags = TokenFlags::empty();

        let result = if self.is_eof() {
            LexerReturn::ok(EOF)
        } else {
            match context {
                LexContext::Regular => self.lex_token(),
                LexContext::TemplateElement { tagged } => self.lex_template(tagged),
            }
        };

        self.current_flags
            .set(TokenFlags::PRECEDING_LINE_BREAK, self.after_newline);
        self.current_kind = result.kind;

        if !result.kind.is_trivia() {
            self.after_newline = false;
        }

        result
    }

    /// Lexes the current token again under the passed [ReLexContext].
    /// Useful in case a token can have different meaning depending on the context.
    ///
    /// For example, a `/` must either be lexed as a `/` token or as a regular expression if it
    /// appears at the start of an expression. Re-lexing allows to always lex the `/` as a `/` token and
    /// call into `re_lex` when the parser is at a valid regular expression position, to see if the
    /// current token can be lexed out as a regular expression literal.
    ///
    /// ## Returns
    /// The new token kind and any associated diagnostic if current token has a different meaning under
    /// the passed [ReLexContext].
    ///
    /// Returns the current kind without any diagnostic if not. Any cached lookahead remains valid in that case.
    pub fn re_lex(&mut self, context: ReLexContext) -> LexerReturn {
        let old_position = self.position;
        self.position = u32::from(self.current_start) as usize;

        let result = match context {
            ReLexContext::Regex if matches!(self.current(), T![/] | T![/=]) => self.read_regex(),
            ReLexContext::BinaryOperator => self.re_lex_binary_operator(),
            ReLexContext::TypeArgumentLessThan => self.re_lex_type_argument_less_than(),
            _ => LexerReturn::ok(self.current()),
        };

        if self.current() == result.kind {
            // Didn't re-lex anything. Return existing token again
            self.position = old_position;
        } else {
            self.current_kind = result.kind;
        }

        result
    }

    fn re_lex_binary_operator(&mut self) -> LexerReturn {
        if self.current_byte() == Some(b'>') {
            match self.next_byte() {
                Some(b'>') => match self.next_byte() {
                    Some(b'>') => match self.next_byte() {
                        Some(b'=') => self.eat_byte(T![>>>=]),
                        _ => LexerReturn::ok(T![>>>]),
                    },
                    Some(b'=') => self.eat_byte(T![>>=]),
                    _ => LexerReturn::ok(T![>>]),
                },
                Some(b'=') => self.eat_byte(T![>=]),
                _ => LexerReturn::ok(T![>]),
            }
        } else {
            LexerReturn::ok(self.current_kind)
        }
    }

    fn re_lex_type_argument_less_than(&mut self) -> LexerReturn {
        if self.current() == T![<<] {
            self.advance(1);
            LexerReturn::ok(T![<])
        } else {
            LexerReturn::ok(self.current())
        }
    }

    /// Bumps the current byte and creates a lexer return for a token of the passed in kind
    fn eat_byte(&mut self, tok: JsSyntaxKind) -> LexerReturn {
        self.next_byte();
        LexerReturn::ok(tok)
    }

    fn consume_newlines(&mut self) -> bool {
        let start = self.position;
        if self.current_byte().is_some() {
            let chr = self.current_char_unchecked();
            if is_linebreak(chr) {
                self.advance(chr.len_utf8());
                if chr == '\r' && self.current_byte() == Some(b'\n') {
                    self.advance('\n'.len_utf8());
                }
            }
        }
        self.position != start
    }

    fn consume_whitespace_until_newline(&mut self) {
        while let Some(current) = self.current_byte() {
            let chr = self.current_char_unchecked();

            if is_linebreak(chr) {
                break;
            }

            let dispatcher = lookup_byte(current);
            if dispatcher == crate::bytes::Dispatch::WHS
                || (UNICODE_WHITESPACE_STARTS.contains(&current) && UNICODE_SPACES.contains(&chr))
            {
                self.advance(chr.len_utf8());
            } else {
                break;
            }
        }
    }

    fn consume_newline_or_whitespace(&mut self) -> LexerReturn {
        if self.consume_newlines() {
            self.after_newline = true;
            LexerReturn::ok(NEWLINE)
        } else {
            self.consume_whitespace_until_newline();
            LexerReturn::ok(WHITESPACE)
        }
    }

    /// Get the unicode char which starts at the current byte
    fn current_char_unchecked(&self) -> char {
        // This is unreachable for all intents and purposes, but this is just a precautionary measure
        debug_assert!(!self.is_eof());

        // Safety: We know this is safe because we require the input to the lexer to be valid utf8 and we always call this when we are at a char
        let string = unsafe {
            std::str::from_utf8_unchecked(self.source.as_bytes().get_unchecked(self.position..))
        };
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

    /// Gets the current byte.
    ///
    /// ## Returns
    /// The current byte if the lexer isn't at the end of the file.
    #[inline]
    fn current_byte(&self) -> Option<u8> {
        if self.is_eof() {
            None
        } else {
            Some(self.source.as_bytes()[self.position])
        }
    }

    /// Asserts that the lexer is currently positioned at `byte`
    #[inline]
    fn assert_byte(&self, byte: u8) {
        debug_assert_eq!(self.source.as_bytes()[self.position], byte);
    }

    /// Returns the current byte without checking if the lexer is at the end of the file.
    #[inline]
    unsafe fn current_unchecked(&self) -> u8 {
        *self.source.as_bytes().get_unchecked(self.position)
    }

    /// Advances the position by one and returns the next byte value
    #[inline]
    fn next_byte(&mut self) -> Option<u8> {
        self.advance(1);
        self.current_byte()
    }

    /// Get the next byte but only advance the index if there is a next byte.
    /// This is really just a hack for certain methods like escapes
    #[inline]
    fn next_byte_bounded(&mut self) -> Option<u8> {
        if let Some(b) = self.source.as_bytes().get(self.position + 1) {
            self.advance(1);
            Some(*b)
        } else {
            if !self.is_eof() {
                // Move the cursor by one to position the Lexer at the EOF token
                self.advance(1);
            }
            None
        }
    }

    /// Peeks at the next byte
    #[inline]
    fn peek_byte(&self) -> Option<u8> {
        self.byte_at(1)
    }

    /// Returns the byte at position `self.position + offset` or `None` if it is out of bounds.
    #[inline]
    fn byte_at(&self, offset: usize) -> Option<u8> {
        self.source.as_bytes().get(self.position + offset).copied()
    }

    /// Advances the current position by `n` bytes.
    #[inline]
    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    /// Returns `true` if the parser is at or passed the end of the file.
    #[inline]
    fn is_eof(&self) -> bool {
        self.position >= self.source.len()
    }

    // Read a `\u{000...}` escape sequence, this expects the cur char to be the `{`
    fn read_codepoint_escape(&mut self) -> Result<char, Box<Diagnostic>> {
        let start = self.position + 1;
        self.read_hexnumber();

        if self.current_byte() != Some(b'}') {
            // We should not yield diagnostics on a unicode char boundary. That wont make codespan panic
            // but it may cause a panic for other crates which just consume the diagnostics
            let invalid = self.current_char_unchecked();
            let err = Diagnostic::error(self.file_id, "", "expected hex digits for a unicode code point escape, but encountered an invalid character")
                .primary(self.position.. self.position + invalid.len_utf8(), "");
            self.position -= 1;
            return Err(Box::new(err));
        }

        // Safety: We know for a fact this is in bounds because we must be on the possible char after the } at this point
        // which means its impossible for the range of the digits to be out of bounds.
        // We also know we cant possibly be indexing a unicode char boundary because a unicode char (which cant be a hexdigit)
        // would have triggered the if statement above. We also know this must be valid utf8, both because of read_hexnumber's behavior
        // and because input to the lexer must be valid utf8
        let digits_str = unsafe {
            debug_assert!(self.source.as_bytes().get(start..self.position).is_some());
            debug_assert!(std::str::from_utf8(
                self.source.as_bytes().get_unchecked(start..self.position)
            )
            .is_ok());

            std::str::from_utf8_unchecked(
                self.source.as_bytes().get_unchecked(start..self.position),
            )
        };

        match u32::from_str_radix(digits_str, 16) {
            Ok(digits) if digits <= 0x10FFFF => {
                let res = std::char::from_u32(digits);
                if let Some(chr) = res {
                    Ok(chr)
                } else {
                    let err =
                        Diagnostic::error(self.file_id, "", "invalid codepoint for unicode escape")
                            .primary(start..self.position, "");
                    Err(Box::new(err))
                }
            }

            _ => {
                let err = Diagnostic::error(
                    self.file_id,
                    "",
                    "out of bounds codepoint for unicode codepoint escape sequence",
                )
                .primary(start..self.position, "")
                .footer_note("Codepoints range from 0 to 0x10FFFF (1114111)");
                Err(Box::new(err))
            }
        }
    }

    // Read a `\u0000` escape sequence, this expects the current char to be the `u`, it also does not skip over the escape sequence
    // The pos after this method is the last hex digit
    fn read_unicode_escape(&mut self, advance: bool) -> Result<char, Box<Diagnostic>> {
        self.assert_byte(b'u');

        for idx in 0..4 {
            match self.next_byte_bounded() {
                None => {
                    if !advance {
                        self.position -= idx + 1;
                    }
                    let err = invalid_digits_after_unicode_escape_sequence(
                        self.file_id,
                        self.position - 1,
                        self.position + 1,
                    );
                    return Err(err);
                }
                Some(b) if !b.is_ascii_hexdigit() => {
                    let err = invalid_digits_after_unicode_escape_sequence(
                        self.file_id,
                        self.position - 1,
                        self.position + 1,
                    );
                    if !advance {
                        self.position -= idx + 1;
                    }
                    return Err(err);
                }
                _ => {}
            }
        }

        unsafe {
            // Safety: input to the lexer is guaranteed to be valid utf8 and so is the range since we return if there is a wrong amount of digits beforehand
            let digits_str = std::str::from_utf8_unchecked(
                self.source
                    .as_bytes()
                    .get_unchecked((self.position - 3)..(self.position + 1)),
            );
            if let Ok(digits) = u32::from_str_radix(digits_str, 16) {
                if !advance {
                    self.position -= 4;
                }
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
    fn validate_hex_escape(&mut self) -> Option<Box<Diagnostic>> {
        self.assert_byte(b'x');

        let diagnostic =
            Diagnostic::error(self.file_id, "", "invalid digits after hex escape sequence")
                .primary(
                    (self.position - 1)..(self.position + 1),
                    "Expected 2 hex digits following this",
                );

        for _ in 0..2 {
            match self.next_byte_bounded() {
                None => return Some(Box::new(diagnostic)),
                Some(b) if !(b as u8).is_ascii_hexdigit() => return Some(Box::new(diagnostic)),
                _ => {}
            }
        }
        None
    }

    // Validate a `\..` escape sequence and advance the lexer based on it
    fn validate_escape_sequence(&mut self) -> Option<Box<Diagnostic>> {
        self.assert_byte(b'\\');
        let cur = self.position;
        self.next_byte(); // eat over the \
        if let Some(escape) = self.current_byte() {
            match escape {
                // Single escape character
                b'\\' | b'n' | b'r' | b't' | b'b' | b'v' | b'f' | b'\'' | b'"' => {
                    self.next_byte();
                    None
                }
                b'u' if self.peek_byte() == Some(b'{') => {
                    self.next_byte(); // jump over '{'
                    self.read_codepoint_escape().err()
                }
                b'u' => self.read_unicode_escape(true).err(),
                // hexadecimal escape sequence
                b'x' => self.validate_hex_escape(),
                _ => {
                    // We use get_unicode_char to account for escaped source characters which are unicode
                    let chr = self.current_char_unchecked();
                    self.advance(chr.len_utf8());
                    None
                }
            }
        } else {
            Some(Box::new(Diagnostic::error(self.file_id, "", "").primary(
                cur..cur + 1,
                "expected an escape sequence following a backslash, but found none",
            )))
        }
    }

    // Consume an identifier by recursively consuming IDENTIFIER_PART kind chars
    #[inline]
    fn consume_ident(&mut self) {
        unwind_loop! {
            if self.next_byte_bounded().is_some() {
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
    /// Returns the number of bytes written into the buffer, and if any char was escaped.
    /// This method will stop writing into the buffer if the buffer is too small to
    /// fit the whole identifier.
    #[inline]
    fn consume_and_get_ident(&mut self, buf: &mut [u8]) -> (usize, bool) {
        let mut idx = 0;
        let mut any_escaped = false;
        unwind_loop! {
            if self.next_byte_bounded().is_some() {
                if let Some((c, escaped)) = self.cur_ident_part() {
                    if let Some(buf) = buf.get_mut(idx..idx + 4) {
                        let res = c.encode_utf8(buf);
                        idx += res.len();
                        any_escaped |= escaped;
                    }
                } else {
                    return (idx, any_escaped);
                }
            } else {
                return (idx, any_escaped);
            }
        }
    }

    // Consume a string literal and advance the lexer, and returning a list of errors that occurred when reading the string
    // This could include unterminated string and invalid escape sequences
    fn read_str_literal(&mut self) -> Option<Box<Diagnostic>> {
        // Safety: this is only ever called from lex_token, which is guaranteed to be called on a char position
        let quote = unsafe { self.current_unchecked() };
        let start = self.position;
        let mut diagnostic = None;

        self.next_byte(); // skip quote;

        while let Some(byte) = self.current_byte() {
            match byte {
                b'\\' => {
                    let r = self.validate_escape_sequence();
                    diagnostic = match (diagnostic, r) {
                        (None, new) => new,
                        (old, None) => old,
                        (Some(mut old), Some(new)) => {
                            old.children.extend(old.primary.take());
                            old.children.extend(new.primary);
                            Some(old)
                        }
                    }
                }
                b if b == quote => {
                    self.next_byte();
                    return diagnostic;
                }
                _ => {
                    self.next_byte();
                }
            }
        }

        let unterminated = Diagnostic::error(self.file_id, "", "unterminated string literal")
            .primary(self.position..self.position, "input ends here")
            .secondary(start..start + 1, "string literal starts here");

        Some(Box::new(unterminated))
    }

    /// Returns `Some(x)` if the current position is an identifier, with the character at
    /// the position.
    ///
    /// Boolean states if there are escaped characters.
    ///
    /// The character may be a char that was generated from a unicode escape sequence,
    /// e.g. `t` is returned, the actual source code is `\u{74}`
    #[inline]
    fn cur_ident_part(&mut self) -> Option<(char, bool)> {
        debug_assert!(!self.is_eof());

        // Safety: we always call this method on a char
        let b = unsafe { self.current_unchecked() };

        match lookup_byte(b) {
            IDT | DIG | ZER => Some((b as char, false)),
            // FIXME: This should use ID_Continue, not XID_Continue
            UNI => {
                let chr = self.current_char_unchecked();
                let res = is_id_continue(chr);
                if res {
                    self.advance(chr.len_utf8() - 1);
                    Some((chr, false))
                } else {
                    None
                }
            }
            BSL if self.peek_byte() == Some(b'u') => {
                let start = self.position;
                self.next_byte();
                let res = if self.peek_byte() == Some(b'{') {
                    self.next_byte();
                    self.read_codepoint_escape()
                } else {
                    self.read_unicode_escape(true)
                };

                if let Ok(c) = res {
                    if is_id_continue(c) {
                        Some((c, true))
                    } else {
                        self.position = start;
                        None
                    }
                } else {
                    self.position = start;
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
        debug_assert!(!self.is_eof());

        // Safety: we always call this method on a char
        let b = unsafe { self.current_unchecked() };

        match lookup_byte(b) {
            BSL if self.peek_byte() == Some(b'u') => {
                self.next_byte();
                if let Ok(chr) = self.read_unicode_escape(false) {
                    if is_id_start(chr) {
                        self.advance(5);
                        return true;
                    }
                }
                self.position -= 1;
                false
            }
            UNI => {
                let chr = self.current_char_unchecked();
                if is_id_start(chr) {
                    self.advance(chr.len_utf8() - 1);
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
    fn resolve_identifier(&mut self, first: char) -> LexerReturn {
        use JsSyntaxKind::*;

        // Note to keep the buffer large enough to fit every possible keyword that
        // the lexer can return
        let mut buf = [0u8; 16];
        let len = first.encode_utf8(&mut buf).len();

        let (count, escaped) = self.consume_and_get_ident(&mut buf[len..]);

        if escaped {
            self.current_flags |= TokenFlags::UNICODE_ESCAPE;
        }

        let kind = match &buf[..count + len] {
            // Keywords
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
            // Strict mode contextual Keywords
            b"implements" => Some(IMPLEMENTS_KW),
            b"interface" => Some(INTERFACE_KW),
            b"let" => Some(LET_KW),
            b"package" => Some(PACKAGE_KW),
            b"private" => Some(PRIVATE_KW),
            b"protected" => Some(PROTECTED_KW),
            b"public" => Some(PUBLIC_KW),
            b"static" => Some(STATIC_KW),
            b"yield" => Some(YIELD_KW),
            // contextual keywords
            b"abstract" => Some(ABSTRACT_KW),
            b"as" => Some(AS_KW),
            b"asserts" => Some(ASSERTS_KW),
            b"assert" => Some(ASSERT_KW),
            b"any" => Some(ANY_KW),
            b"async" => Some(ASYNC_KW),
            b"await" => Some(AWAIT_KW),
            b"boolean" => Some(BOOLEAN_KW),
            b"constructor" => Some(CONSTRUCTOR_KW),
            b"declare" => Some(DECLARE_KW),
            b"get" => Some(GET_KW),
            b"infer" => Some(INFER_KW),
            b"is" => Some(IS_KW),
            b"keyof" => Some(KEYOF_KW),
            b"module" => Some(MODULE_KW),
            b"namespace" => Some(NAMESPACE_KW),
            b"never" => Some(NEVER_KW),
            b"readonly" => Some(READONLY_KW),
            b"require" => Some(REQUIRE_KW),
            b"number" => Some(NUMBER_KW),
            b"object" => Some(OBJECT_KW),
            b"set" => Some(SET_KW),
            b"string" => Some(STRING_KW),
            b"symbol" => Some(SYMBOL_KW),
            b"type" => Some(TYPE_KW),
            b"undefined" => Some(UNDEFINED_KW),
            b"unique" => Some(UNIQUE_KW),
            b"unknown" => Some(UNKNOWN_KW),
            b"from" => Some(FROM_KW),
            b"global" => Some(GLOBAL_KW),
            b"bigint" => Some(BIGINT_KW),
            b"override" => Some(OVERRIDE_KW),
            b"of" => Some(OF_KW),
            _ => None,
        };

        if let Some(kind) = kind {
            LexerReturn::ok(kind)
        } else {
            LexerReturn::ok(T![ident])
        }
    }

    #[inline]
    fn special_number_start<F: Fn(char) -> bool>(&mut self, func: F) -> bool {
        if self.byte_at(2).map(|b| func(b as char)).unwrap_or(false) {
            self.advance(1);
            true
        } else {
            false
        }
    }

    #[inline]
    fn maybe_bigint(&mut self) {
        if let Some(b'n') = self.current_byte() {
            self.next_byte();
        }
    }

    #[inline]
    fn read_zero(&mut self) -> Option<Box<Diagnostic>> {
        match self.peek_byte() {
            Some(b'x') | Some(b'X') => {
                if self.special_number_start(|c| c.is_ascii_hexdigit()) {
                    let diag = self.read_hexnumber();
                    self.maybe_bigint();
                    diag
                } else {
                    self.next_byte();
                    None
                }
            }
            Some(b'b') | Some(b'B') => {
                if self.special_number_start(|c| c == '0' || c == '1') {
                    let diag = self.read_bindigits();
                    self.maybe_bigint();
                    diag
                } else {
                    self.next_byte();
                    None
                }
            }
            Some(b'o') | Some(b'O') => {
                if self.special_number_start(|c| ('0'..='7').contains(&c)) {
                    let diag = self.read_octaldigits();
                    self.maybe_bigint();
                    diag
                } else {
                    self.next_byte();
                    None
                }
            }
            Some(b'n') => {
                self.advance(2);
                None
            }
            Some(b'.') => {
                self.advance(1);
                self.read_float()
            }
            Some(b'e') | Some(b'E') => {
                // At least one digit is required
                match self.byte_at(2) {
                    Some(b'-') | Some(b'+') => {
                        if let Some(b'0'..=b'9') = self.byte_at(3) {
                            self.next_byte();
                            self.read_exponent()
                        } else {
                            self.next_byte();
                            None
                        }
                    }
                    Some(b'0'..=b'9') => {
                        self.next_byte();
                        self.read_exponent()
                    }
                    _ => {
                        self.next_byte();
                        None
                    }
                }
            }
            _ => self.read_number(true),
        }
    }

    #[inline]
    fn read_hexnumber(&mut self) -> Option<Box<Diagnostic>> {
        let mut diag = None;
        unwind_loop! {
            match self.next_byte() {
                Some(b'_') => diag = diag.or_else(|| self.handle_numeric_separator(16)),
                Some(b) if char::from(b).is_ascii_hexdigit() => {},
                _ => return diag,
            }
        }
    }

    #[inline]
    fn handle_numeric_separator(&mut self, radix: u8) -> Option<Box<Diagnostic>> {
        self.assert_byte(b'_');

        let err_diag = Diagnostic::error(
            self.file_id,
            "",
            "numeric separators are only allowed between two digits",
        )
        .primary(self.position..self.position + 1, "");

        let peeked = self.peek_byte();

        if peeked.is_none() || !char::from(peeked.unwrap()).is_digit(radix as u32) {
            return Some(Box::new(err_diag));
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

        let prev = self.source.as_bytes().get(self.position - 1).copied();

        if forbidden(prev) || forbidden(peeked) {
            return Some(Box::new(err_diag));
        }

        self.next_byte_bounded();
        None
    }

    #[inline]
    fn read_number(&mut self, leading_zero: bool) -> Option<Box<Diagnostic>> {
        let start = self.position;
        let mut diag = None;
        unwind_loop! {
            match self.next_byte_bounded() {
                Some(b'_') => {
                    if leading_zero {
                        diag = Some(Box::new(
                            Diagnostic::error(
                                self.file_id,
                                "",
                                "numeric separator can not be used after leading 0",
                            )
                            .primary(self.position..self.position, ""),
                        ));
                    }
                    diag = diag.or(self.handle_numeric_separator(10))
                },
                Some(b'0'..=b'9') => {},
                Some(b'.') => {
                    if leading_zero {
                        diag = Some(Box::new(
                                Diagnostic::error(self.file_id, "", "unexpected number")
                                .primary(start..self.position + 1, ""),
                        ));
                    }
                    return diag.or(self.read_float());
                },
                // TODO: merge this, and read_float's implementation into one so we dont duplicate exponent code
                Some(b'e') | Some(b'E') => {
                    // At least one digit is required
                    match self.peek_byte() {
                        Some(b'-') | Some(b'+') => {
                            if let Some(b'0'..=b'9') = self.byte_at(2) {
                                self.next_byte();
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
                    if leading_zero {
                        diag = Some(Box::new(
                                Diagnostic::error(
                                    self.file_id,
                                    "",
                                    "Octal literals are not allowed for BigInts.",
                                )
                                .primary(start..self.position + 1, ""),
                        ));
                    }
                    self.next_byte();
                    return diag;
                }
                _ => return diag,
            }
        }
    }

    #[inline]
    fn read_float(&mut self) -> Option<Box<Diagnostic>> {
        let mut diag = None;

        unwind_loop! {
            match self.next_byte_bounded() {
                Some(b'_') => diag = diag.or(self.handle_numeric_separator(10)),
                // LLVM has a hard time optimizing inclusive patterns, perhaps we should check if it makes llvm sad,
                // and optimize this into a lookup table
                Some(b'0'..=b'9') => {},
                Some(b'e') | Some(b'E') => {
                    // At least one digit is required
                    match self.peek_byte() {
                        Some(b'-') | Some(b'+') => {
                            if let Some(b'0'..=b'9') = self.byte_at(2) {
                                self.next_byte();
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
    fn read_exponent(&mut self) -> Option<Box<Diagnostic>> {
        if let Some(b'-') | Some(b'+') = self.peek_byte() {
            self.next_byte();
        }

        let mut diag = None;
        unwind_loop! {
            match self.next_byte() {
                Some(b'_') => diag = diag.or(self.handle_numeric_separator(10)),
                Some(b'0'..=b'9') => {},
                _ => return diag,
            }
        }
    }

    #[inline]
    fn read_bindigits(&mut self) -> Option<Box<Diagnostic>> {
        let mut diag = None;
        unwind_loop! {
            match self.next_byte() {
                Some(b'_') => diag = diag.or(self.handle_numeric_separator(2)),
                Some(b'0') | Some(b'1') => {},
                _ => return diag,
            }
        }
    }

    #[inline]
    fn read_octaldigits(&mut self) -> Option<Box<Diagnostic>> {
        let mut diag = None;
        unwind_loop! {
            match self.next_byte() {
                Some(b'_') => diag = diag.or(self.handle_numeric_separator(8)),
                Some(b'0'..=b'7') => {},
                _ => return diag,
            }
        }
    }

    #[inline]
    fn verify_number_end(&mut self) -> LexerReturn {
        let err_start = self.position;
        if !self.is_eof() && self.cur_is_ident_start() {
            self.consume_ident();
            let err = Diagnostic::error(
                self.file_id,
                "",
                "numbers cannot be followed by identifiers directly after",
            )
            .primary(err_start..self.position, "an identifier cannot appear here");

            LexerReturn::with_diagnostic(JsSyntaxKind::ERROR_TOKEN, Box::new(err))
        } else {
            LexerReturn::ok(JS_NUMBER_LITERAL)
        }
    }

    #[inline]
    fn read_shebang(&mut self) -> LexerReturn {
        let start = self.position;
        self.next_byte();
        if start != 0 {
            return LexerReturn::ok(T![#]);
        }

        if let Some(b'!') = self.current_byte() {
            while self.next_byte().is_some() {
                let chr = self.current_char_unchecked();

                if is_linebreak(chr) {
                    return LexerReturn::ok(JS_SHEBANG);
                }
                self.advance(chr.len_utf8() - 1);
            }
            LexerReturn::ok(JS_SHEBANG)
        } else {
            let err = Diagnostic::error(
                self.file_id,
                "",
                "expected `!` following a `#`, but found none",
            )
            .primary(0usize..1usize, "");

            LexerReturn::with_diagnostic(JsSyntaxKind::ERROR_TOKEN, Box::new(err))
        }
    }

    #[inline]
    fn read_slash(&mut self) -> LexerReturn {
        let start = self.position;
        match self.peek_byte() {
            Some(b'*') => {
                self.next_byte();
                let mut has_newline = false;
                while let Some(b) = self.next_byte() {
                    match b {
                        b'*' if self.peek_byte() == Some(b'/') => {
                            self.advance(2);
                            if has_newline {
                                self.after_newline = true;
                                return LexerReturn::ok(MULTILINE_COMMENT);
                            } else {
                                return LexerReturn::ok(COMMENT);
                            }
                        }
                        x => {
                            if is_linebreak(x as char) {
                                has_newline = true;
                            } else if UNICODE_WHITESPACE_STARTS.contains(&x) {
                                let x = self.current_char_unchecked();
                                has_newline |= is_linebreak(x as char);
                            }
                        }
                    }
                }

                let err = Diagnostic::error(self.file_id, "", "unterminated block comment")
                    .primary(
                        self.position..self.position + 1,
                        "... but the file ends here",
                    )
                    .secondary(start..start + 2, "A block comment starts here");

                LexerReturn::with_diagnostic(JsSyntaxKind::COMMENT, Box::new(err))
            }
            Some(b'/') => {
                self.next_byte();
                while self.next_byte().is_some() {
                    let chr = self.current_char_unchecked();

                    if is_linebreak(chr) {
                        return LexerReturn::ok(COMMENT);
                    }
                    self.advance(chr.len_utf8() - 1);
                }
                LexerReturn::ok(COMMENT)
            }
            // _ if self.state.expr_allowed => self.read_regex(),
            Some(b'=') => {
                self.advance(2);
                LexerReturn::ok(SLASHEQ)
            }
            _ => self.eat_byte(T![/]),
        }
    }

    #[inline]
    fn flag_err(&self, flag: char) -> Diagnostic {
        Diagnostic::error(self.file_id, "", format!("duplicate flag `{}`", flag)).primary(
            self.position..self.position + 1,
            "this flag was already used",
        )
    }

    // TODO: Due to our return of (Token, Option<Error>) we cant issue more than one regex error
    // This is not a huge issue but it would be helpful to users
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn read_regex(&mut self) -> LexerReturn {
        self.assert_byte(b'/');
        let start = self.position;
        let mut in_class = false;
        let mut diagnostic = None;

        unwind_loop! {
            match self.next_byte() {
                Some(b'[') => in_class = true,
                Some(b']') => in_class = false,
                Some(b'/') => {
                    if !in_class {
                        let (mut g, mut i, mut m, mut s, mut u, mut y, mut d) = (false, false, false, false, false, false, false);

                        unwind_loop! {
                            let next = self.next_byte_bounded();
                            let chr_start = self.position;
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
                                Some(b'd') => {
                                    if d && diagnostic.is_none() {
                                        diagnostic = Some(self.flag_err('d'))
                                    }
                                    d = true;
                                },
                                Some(_) if self.cur_ident_part().is_some() => {
                                    if diagnostic.is_none() {
                                        diagnostic = Some(Diagnostic::error(self.file_id, "", "invalid regex flag")
                                            .primary(chr_start .. self.position + 1, "this is not a valid regex flag"));
                                    }
                                },
                                _ => {
                                    return LexerReturn::new(
                                        JsSyntaxKind::JS_REGEX_LITERAL,
                                        diagnostic.map(Box::new)
                                    )
                                }
                            }
                        }
                    }
                },
                Some(b'\\') => {
                    if self.next_byte_bounded().is_none() {
                        let err = Diagnostic::error(self.file_id, "", "expected a character after a regex escape, but found none")
                            .primary(self.position..self.position + 1, "expected a character following this");

                        return LexerReturn::with_diagnostic(JsSyntaxKind::JS_REGEX_LITERAL, Box::new(err));
                    }
                },
                Some(_) if is_linebreak(self.current_char_unchecked()) => {
                    let err = Diagnostic::error(self.file_id, "", "unterminated regex literal")
                        .primary(self.position..self.position, "...but the line ends here")
                        .secondary(start..start + 1, "a regex literal starts there...");

                    // Undo the read of the new line trivia
                    self.position -= 1;
                    return LexerReturn::with_diagnostic(JsSyntaxKind::JS_REGEX_LITERAL, Box::new(err));
                },
                None => {
                    let err = Diagnostic::error(self.file_id, "", "unterminated regex literal")
                        .primary(self.position..self.position, "...but the file ends here")
                        .secondary(start..start + 1, "a regex literal starts there...");

                    return LexerReturn::with_diagnostic(JsSyntaxKind::JS_REGEX_LITERAL, Box::new(err));
                },
                _ => {},
            }
        }
    }

    #[inline]
    fn bin_or_assign(&mut self, bin: JsSyntaxKind, assign: JsSyntaxKind) -> LexerReturn {
        if let Some(b'=') = self.next_byte() {
            self.next_byte();
            LexerReturn::ok(assign)
        } else {
            LexerReturn::ok(bin)
        }
    }

    #[inline]
    fn resolve_bang(&mut self) -> LexerReturn {
        match self.next_byte() {
            Some(b'=') => {
                if let Some(b'=') = self.next_byte() {
                    self.next_byte();
                    LexerReturn::ok(NEQ2)
                } else {
                    LexerReturn::ok(NEQ)
                }
            }
            _ => LexerReturn::ok(T![!]),
        }
    }

    #[inline]
    fn resolve_amp(&mut self) -> LexerReturn {
        match self.next_byte() {
            Some(b'&') => {
                if let Some(b'=') = self.next_byte() {
                    self.next_byte();
                    LexerReturn::ok(AMP2EQ)
                } else {
                    LexerReturn::ok(AMP2)
                }
            }
            Some(b'=') => {
                self.next_byte();
                LexerReturn::ok(AMPEQ)
            }
            _ => LexerReturn::ok(T![&]),
        }
    }

    #[inline]
    fn resolve_plus(&mut self) -> LexerReturn {
        match self.next_byte() {
            Some(b'+') => {
                self.next_byte();
                LexerReturn::ok(PLUS2)
            }
            Some(b'=') => {
                self.next_byte();
                LexerReturn::ok(PLUSEQ)
            }
            _ => LexerReturn::ok(T![+]),
        }
    }

    #[inline]
    fn resolve_minus(&mut self) -> LexerReturn {
        match self.next_byte() {
            Some(b'-') => {
                self.next_byte();
                LexerReturn::ok(MINUS2)
            }
            Some(b'=') => {
                self.next_byte();
                LexerReturn::ok(MINUSEQ)
            }
            _ => LexerReturn::ok(T![-]),
        }
    }

    #[inline]
    fn resolve_less_than(&mut self) -> LexerReturn {
        match self.next_byte() {
            Some(b'<') => {
                if let Some(b'=') = self.next_byte() {
                    self.next_byte();
                    LexerReturn::ok(SHLEQ)
                } else {
                    LexerReturn::ok(SHL)
                }
            }
            Some(b'=') => {
                self.next_byte();
                LexerReturn::ok(LTEQ)
            }
            _ => LexerReturn::ok(T![<]),
        }
    }

    #[inline]
    fn resolve_greater_than(&mut self) -> LexerReturn {
        match self.next_byte() {
            Some(b'>') => {
                if let Some(b'>') = self.peek_byte() {
                    if let Some(b'=') = self.byte_at(2) {
                        self.advance(3);
                        LexerReturn::ok(USHREQ)
                    } else {
                        LexerReturn::ok(T![>])
                    }
                } else if self.peek_byte() == Some(b'=') {
                    self.advance(2);
                    LexerReturn::ok(SHREQ)
                } else {
                    LexerReturn::ok(T![>])
                }
            }
            Some(b'=') => {
                self.next_byte();
                LexerReturn::ok(GTEQ)
            }
            _ => LexerReturn::ok(T![>]),
        }
    }

    #[inline]
    fn resolve_eq(&mut self) -> LexerReturn {
        match self.next_byte() {
            Some(b'=') => {
                if let Some(b'=') = self.next_byte() {
                    self.next_byte();
                    LexerReturn::ok(EQ3)
                } else {
                    LexerReturn::ok(EQ2)
                }
            }
            Some(b'>') => {
                self.next_byte();
                LexerReturn::ok(FAT_ARROW)
            }
            _ => LexerReturn::ok(T![=]),
        }
    }

    #[inline]
    fn resolve_pipe(&mut self) -> LexerReturn {
        match self.next_byte() {
            Some(b'|') => {
                if let Some(b'=') = self.next_byte() {
                    self.next_byte();
                    LexerReturn::ok(PIPE2EQ)
                } else {
                    LexerReturn::ok(PIPE2)
                }
            }
            Some(b'=') => {
                self.next_byte();
                LexerReturn::ok(PIPEEQ)
            }
            _ => LexerReturn::ok(T![|]),
        }
    }

    // Dont ask it to resolve the question of life's meaning because you'll be disappointed
    #[inline]
    fn resolve_question(&mut self) -> LexerReturn {
        match self.next_byte() {
            Some(b'?') => {
                if let Some(b'=') = self.next_byte() {
                    self.next_byte();
                    LexerReturn::ok(QUESTION2EQ)
                } else {
                    LexerReturn::ok(QUESTION2)
                }
            }
            Some(b'.') => {
                // 11.7 Optional chaining punctuator
                if let Some(b'0'..=b'9') = self.peek_byte() {
                    LexerReturn::ok(T![?])
                } else {
                    self.next_byte();
                    LexerReturn::ok(QUESTIONDOT)
                }
            }
            _ => LexerReturn::ok(T![?]),
        }
    }

    #[inline]
    fn resolve_star(&mut self) -> LexerReturn {
        match self.next_byte() {
            Some(b'*') => {
                if let Some(b'=') = self.next_byte() {
                    self.next_byte();
                    LexerReturn::ok(STAR2EQ)
                } else {
                    LexerReturn::ok(STAR2)
                }
            }
            Some(b'=') => {
                self.next_byte();
                LexerReturn::ok(STAREQ)
            }
            _ => LexerReturn::ok(T![*]),
        }
    }

    /// Lex the next token
    fn lex_token(&mut self) -> LexerReturn {
        // Safety: we always call lex_token when we are at a valid char
        let byte = unsafe { self.current_unchecked() };
        let start = self.position;

        // A lookup table of `byte -> fn(l: &mut Lexer) -> Token` is exponentially slower than this approach
        // The speed difference comes from the difference in table size, a 2kb table is easily fit into cpu cache
        // While a 16kb table will be ejected from cache very often leading to slowdowns, this also allows LLVM
        // to do more aggressive optimizations on the match regarding how to map it to instructions
        let dispatched = lookup_byte(byte);

        match dispatched {
            WHS => self.consume_newline_or_whitespace(),
            EXL => self.resolve_bang(),
            HAS => self.read_shebang(),
            PRC => self.bin_or_assign(T![%], T![%=]),
            Dispatch::AMP => self.resolve_amp(),
            PNO => self.eat_byte(T!['(']),
            PNC => self.eat_byte(T![')']),
            MUL => self.resolve_star(),
            PLS => self.resolve_plus(),
            COM => self.eat_byte(T![,]),
            MIN => self.resolve_minus(),
            SLH => self.read_slash(),
            // This simply changes state on the start
            TPL => self.eat_byte(T!['`']),
            ZER => {
                let diag = self.read_zero();
                let LexerReturn { kind, diagnostic } = self.verify_number_end();
                LexerReturn::new(kind, diagnostic.or(diag))
            }
            PRD => {
                if self.peek_byte() == Some(b'.') && self.byte_at(2) == Some(b'.') {
                    self.advance(3);
                    return LexerReturn::ok(DOT3);
                }
                if let Some(b'0'..=b'9') = self.peek_byte() {
                    let diag = self.read_float();
                    let LexerReturn { kind, diagnostic } = self.verify_number_end();
                    LexerReturn::new(kind, diagnostic.or(diag))
                } else {
                    self.eat_byte(T![.])
                }
            }
            BSL => {
                if self.peek_byte() == Some(b'u') {
                    self.next_byte();
                    let res = if self.peek_byte() == Some(b'{') {
                        self.next_byte();
                        self.read_codepoint_escape()
                    } else {
                        self.read_unicode_escape(true)
                    };

                    match res {
                        Ok(chr) => {
                            if is_id_start(chr) {
                                self.current_flags |= TokenFlags::UNICODE_ESCAPE;
                                self.resolve_identifier(chr)
                            } else {
                                let err = Diagnostic::error(self.file_id, "", "unexpected unicode escape")
                                    .primary(start..self.position, "this escape is unexpected, as it does not designate the start of an identifier");
                                self.next_byte();
                                LexerReturn::with_diagnostic(
                                    JsSyntaxKind::ERROR_TOKEN,
                                    Box::new(err),
                                )
                            }
                        }
                        Err(err) => LexerReturn::with_diagnostic(JsSyntaxKind::ERROR_TOKEN, err),
                    }
                } else {
                    let err = Diagnostic::error(
                        self.file_id,
                        "",
                        format!("unexpected token `{}`", byte as char),
                    )
                    .primary(start..self.position + 1, "");
                    self.next_byte();
                    LexerReturn::with_diagnostic(JsSyntaxKind::ERROR_TOKEN, Box::new(err))
                }
            }
            QOT => {
                if let Some(err) = self.read_str_literal() {
                    LexerReturn::with_diagnostic(JsSyntaxKind::ERROR_TOKEN, err)
                } else {
                    LexerReturn::ok(JS_STRING_LITERAL)
                }
            }
            IDT => self.resolve_identifier(byte as char),
            DIG => {
                let diag = self.read_number(false);
                let LexerReturn { kind, diagnostic } = self.verify_number_end();
                LexerReturn::new(kind, diagnostic.or(diag))
            }
            COL => self.eat_byte(T![:]),
            SEM => self.eat_byte(T![;]),
            LSS => self.resolve_less_than(),
            EQL => self.resolve_eq(),
            MOR => self.resolve_greater_than(),
            QST => self.resolve_question(),
            BTO => self.eat_byte(T!('[')),
            BTC => self.eat_byte(T![']']),
            CRT => self.bin_or_assign(T![^], T![^=]),
            BEO => self.eat_byte(T!['{']),
            BEC => self.eat_byte(T!['}']),
            PIP => self.resolve_pipe(),
            TLD => self.eat_byte(T![~]),
            UNI => {
                let chr = self.current_char_unchecked();
                if is_linebreak(chr)
                    || (UNICODE_WHITESPACE_STARTS.contains(&byte) && UNICODE_SPACES.contains(&chr))
                {
                    self.consume_newline_or_whitespace()
                } else {
                    self.advance(chr.len_utf8() - 1);
                    if is_id_start(chr) {
                        self.resolve_identifier(chr)
                    } else {
                        let err = Diagnostic::error(
                            self.file_id,
                            "",
                            format!("Unexpected token `{}`", chr as char),
                        )
                        .primary(start..self.position + 1, "");
                        self.next_byte();

                        LexerReturn::with_diagnostic(JsSyntaxKind::ERROR_TOKEN, Box::new(err))
                    }
                }
            }
            AT_ => self.eat_byte(T![@]),
            _ => {
                let err = Diagnostic::error(
                    self.file_id,
                    "",
                    format!("unexpected token `{}`", byte as char),
                )
                .primary(start..self.position + 1, "");
                self.next_byte();

                LexerReturn::with_diagnostic(JsSyntaxKind::ERROR_TOKEN, Box::new(err))
            }
        }
    }

    fn lex_template(&mut self, tagged: bool) -> LexerReturn {
        let mut diagnostic: Option<Box<Diagnostic>> = None;
        let mut token: Option<JsSyntaxKind> = None;
        let start = self.position;

        loop {
            match self.current_byte() {
                Some(b'`') if self.position == start => {
                    self.next_byte();
                    token = Some(BACKTICK);
                    break;
                }
                Some(b'`') => {
                    token = Some(JsSyntaxKind::TEMPLATE_CHUNK);
                    break;
                }
                Some(b'\\') => {
                    let diag = self.validate_escape_sequence();
                    if let Some(diag) = diag {
                        if !tagged {
                            diagnostic = Some(diag);
                        }
                    };
                }
                Some(b'$') if self.peek_byte() == Some(b'{') && self.position == start => {
                    self.advance(2);
                    token = Some(JsSyntaxKind::DOLLAR_CURLY);
                    break;
                }
                Some(b'$') if self.peek_byte() == Some(b'{') => {
                    token = Some(JsSyntaxKind::TEMPLATE_CHUNK);
                    break;
                }
                Some(_) => {
                    let _ = self.next_byte();
                }
                None => {
                    break;
                }
            }
        }

        match token {
            None => {
                let err = Diagnostic::error(self.file_id, "", "unterminated template literal")
                    .primary(start..self.position + 1, "");
                LexerReturn::with_diagnostic(JsSyntaxKind::ERROR_TOKEN, Box::new(err))
            }
            Some(token) => match diagnostic {
                None => LexerReturn::ok(token),
                Some(diagnostic) => {
                    LexerReturn::with_diagnostic(JsSyntaxKind::ERROR_TOKEN, diagnostic)
                }
            },
        }
    }
}

/// Check if a char is a JS linebreak
fn is_linebreak(chr: char) -> bool {
    matches!(chr, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

/// Stores the state of the lexer so that it may later be restored to that position.
#[derive(Debug, Clone)]
pub struct LexerCheckpoint {
    pub(crate) position: TextSize,
    pub(crate) current_start: TextSize,
    pub(crate) current_kind: JsSyntaxKind,
    pub(crate) current_flags: TokenFlags,
    pub(crate) after_line_break: bool,
}

impl LexerCheckpoint {
    /// Returns the byte offset of the current token.
    pub fn current_start(&self) -> TextSize {
        self.current_start
    }

    pub(crate) fn has_preceding_line_break(&self) -> bool {
        self.current_flags.has_preceding_line_break()
    }

    pub(crate) fn has_unicode_escape(&self) -> bool {
        self.current_flags.has_unicode_escape()
    }
}
