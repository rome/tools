//! An extremely fast, lookup table based, ECMAScript lexer which yields SyntaxKind tokens used by the rome-js parser.
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

use rome_diagnostics::Diagnostic;
use tables::derived_property::*;

pub(crate) use buffered_lexer::BufferedLexer;
pub use rome_js_syntax::*;

use self::bytes::{
    lookup_byte,
    Dispatch::{self, *},
};
use rome_diagnostics::file::FileId;
use rome_js_syntax::JsSyntaxKind::*;

use self::errors::invalid_digits_after_unicode_escape_sequence;

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

    /// Lexes a token in a JSX children context.
    /// Returns one of
    /// - Whitespace trivia
    /// - JsxText
    /// - `<` end of the current element, or start of a new element
    /// - expression start: `{`
    /// - EOF
    JsxChild,

    /// Lexes a JSX Attribute value. Calls into normal lex token if positioned at anything
    /// that isn't `'` or `"`.
    JsxAttributeValue,
}

impl Default for LexContext {
    fn default() -> Self {
        LexContext::Regular
    }
}

impl LexContext {
    /// Returns true if this is [LexContext::Regular]
    pub fn is_regular(&self) -> bool {
        matches!(self, LexContext::Regular)
    }
}

/// Context in which the [Lexer]'s current should be re-lexed.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ReLexContext {
    /// Re-lexes a `/` or `/=` token as a regular expression.
    Regex,
    /// Re-lexes `'>', '>'` as `>>` and `'>', '>', '>'` as `>>>`
    BinaryOperator,
    /// Re-lexes `'<', '<'` as `<<` in places where a type argument is expected to support
    /// `B<<A>()>`
    TypeArgumentLessThan,
    /// Re-lexes an identifier or keyword as a JSX identifier (that allows `-` tokens)
    JsxIdentifier,

    /// See [LexContext::JsxChild]
    JsxChild,
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
pub(crate) struct Lexer<'src> {
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

    diagnostics: Vec<Diagnostic>,
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
            diagnostics: vec![],
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
            diagnostics_pos: self.diagnostics.len() as u32,
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
            diagnostics_pos,
        } = checkpoint;

        let new_pos = u32::from(position) as usize;

        self.position = new_pos;
        self.current_kind = current_kind;
        self.current_start = current_start;
        self.current_flags = current_flags;
        self.after_newline = after_line_break;
        self.diagnostics.truncate(diagnostics_pos as usize);
    }

    pub fn finish(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    /// Lexes the next token.
    ///
    /// ## Return
    /// Returns its kind and any potential error.
    pub fn next_token(&mut self, context: LexContext) -> JsSyntaxKind {
        self.current_start = TextSize::from(self.position as u32);
        self.current_flags = TokenFlags::empty();

        let kind = if self.is_eof() {
            EOF
        } else {
            match context {
                LexContext::Regular => self.lex_token(),
                LexContext::TemplateElement { tagged } => self.lex_template(tagged),
                LexContext::JsxChild => self.lex_jsx_child_token(),
                LexContext::JsxAttributeValue => self.lex_jsx_attribute_value(),
            }
        };

        self.current_flags
            .set(TokenFlags::PRECEDING_LINE_BREAK, self.after_newline);
        self.current_kind = kind;

        if !kind.is_trivia() {
            self.after_newline = false;
        }

        kind
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
    pub fn re_lex(&mut self, context: ReLexContext) -> JsSyntaxKind {
        let old_position = self.position;
        self.position = u32::from(self.current_start) as usize;

        let re_lexed_kind = match context {
            ReLexContext::Regex if matches!(self.current(), T![/] | T![/=]) => self.read_regex(),
            ReLexContext::BinaryOperator => self.re_lex_binary_operator(),
            ReLexContext::TypeArgumentLessThan => self.re_lex_type_argument_less_than(),
            ReLexContext::JsxIdentifier => self.re_lex_jsx_identifier(old_position),
            ReLexContext::JsxChild => self.lex_jsx_child_token(),
            _ => self.current(),
        };

        if self.current() == re_lexed_kind {
            // Didn't re-lex anything. Return existing token again
            self.position = old_position;
        } else {
            self.current_kind = re_lexed_kind;
        }

        re_lexed_kind
    }

    fn re_lex_binary_operator(&mut self) -> JsSyntaxKind {
        if self.current_byte() == Some(b'>') {
            match self.next_byte() {
                Some(b'>') => match self.next_byte() {
                    Some(b'>') => match self.next_byte() {
                        Some(b'=') => self.eat_byte(T![>>>=]),
                        _ => T![>>>],
                    },
                    Some(b'=') => self.eat_byte(T![>>=]),
                    _ => T![>>],
                },
                Some(b'=') => self.eat_byte(T![>=]),
                _ => T![>],
            }
        } else {
            self.current_kind
        }
    }

    fn re_lex_type_argument_less_than(&mut self) -> JsSyntaxKind {
        if self.current() == T![<<] {
            self.advance(1);
            T![<]
        } else {
            self.current()
        }
    }

    fn re_lex_jsx_identifier(&mut self, current_end: usize) -> JsSyntaxKind {
        if self.current_kind.is_keyword() || self.current_kind == T![ident] {
            self.position = current_end;

            while let Some(current_byte) = self.current_byte() {
                match current_byte {
                    b'-' => {
                        self.advance(1);
                    }
                    b':' => {
                        break;
                    }
                    _ => {
                        let start = self.position;

                        // consume ident advances by one position, so move back by one
                        self.position -= 1;
                        self.consume_ident();

                        // Didn't eat any identifier parts, break out
                        if start == self.position {
                            self.position = start;
                            break;
                        }
                    }
                }
            }

            JSX_IDENT
        } else {
            self.current_kind
        }
    }

    fn lex_jsx_child_token(&mut self) -> JsSyntaxKind {
        debug_assert!(!self.is_eof());

        // SAFETY: `lex_token` only calls this method if it isn't passed the EOF
        let byte = unsafe { self.current_unchecked() };

        match byte {
            // `<`: empty jsx text, directly followed by another element or closing element
            b'<' => self.eat_byte(T![<]),
            // `{`: empty jsx text, directly followed by an expression
            b'{' => self.eat_byte(T!['{']),
            _ => {
                while let Some(byte) = self.current_byte() {
                    // but not one of: { or < or > or }
                    match byte {
                        // Start of a new element, the closing tag, or an expression
                        b'<' | b'{' => {
                            break;
                        }
                        b'>' => {
                            self.diagnostics.push(
                                Diagnostic::error(
                                    self.file_id,
                                    "",
                                    "Unexpected token. Did you mean `{'>'}` or `&gt;`?",
                                )
                                .primary(self.position..self.position + 1, ""),
                            );
                            self.next_byte();
                        }
                        b'}' => {
                            self.diagnostics.push(
                                Diagnostic::error(
                                    self.file_id,
                                    "",
                                    "Unexpected token. Did you mean `{'}'}` or `&rbrace;`?",
                                )
                                .primary(self.position..self.position + 1, ""),
                            );
                            self.next_byte();
                        }
                        _ => {
                            self.next_byte();
                        }
                    }
                }

                JSX_TEXT_LITERAL
            }
        }
    }

    fn lex_jsx_attribute_value(&mut self) -> JsSyntaxKind {
        debug_assert!(!self.is_eof());

        // Safety: Guaranteed because we aren't at the end of the file
        let byte = unsafe { self.current_unchecked() };

        match byte {
            b'\'' | b'"' => {
                self.read_str_literal(true);

                JSX_STRING_LITERAL
            }
            _ => self.lex_token(),
        }
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind
    fn eat_byte(&mut self, tok: JsSyntaxKind) -> JsSyntaxKind {
        self.next_byte();
        tok
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
            if dispatcher == self::bytes::Dispatch::WHS
                || (UNICODE_WHITESPACE_STARTS.contains(&current) && UNICODE_SPACES.contains(&chr))
            {
                self.advance(chr.len_utf8());
            } else {
                break;
            }
        }
    }

    fn consume_newline_or_whitespace(&mut self) -> JsSyntaxKind {
        if self.consume_newlines() {
            self.after_newline = true;
            NEWLINE
        } else {
            self.consume_whitespace_until_newline();
            WHITESPACE
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
    ///
    /// ## Safety
    /// Calling this function if the lexer is at or passed the end of file is undefined behaviour.
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
    fn read_codepoint_escape(&mut self) -> Result<char, ()> {
        let start = self.position + 1;
        self.read_hexnumber();

        if self.current_byte() != Some(b'}') {
            // We should not yield diagnostics on a unicode char boundary. That wont make codespan panic
            // but it may cause a panic for other crates which just consume the diagnostics
            let invalid = self.current_char_unchecked();
            let err = Diagnostic::error(self.file_id, "", "expected hex digits for a unicode code point escape, but encountered an invalid character")
                .primary(self.position.. self.position + invalid.len_utf8(), "");
            self.diagnostics.push(err);
            self.position -= 1;
            return Err(());
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
                    self.diagnostics.push(err);
                    Err(())
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
                self.diagnostics.push(err);
                Err(())
            }
        }
    }

    // Read a `\u0000` escape sequence, this expects the current char to be the `u`, it also does not skip over the escape sequence
    // The pos after this method is the last hex digit
    fn read_unicode_escape(&mut self, advance: bool) -> Result<char, ()> {
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
                    self.diagnostics.push(err);
                    return Err(());
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
                    self.diagnostics.push(err);
                    return Err(());
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
    fn validate_hex_escape(&mut self) -> bool {
        self.assert_byte(b'x');

        let diagnostic =
            Diagnostic::error(self.file_id, "", "invalid digits after hex escape sequence")
                .primary(
                    (self.position - 1)..(self.position + 1),
                    "Expected 2 hex digits following this",
                );

        for _ in 0..2 {
            match self.next_byte_bounded() {
                None => {
                    self.diagnostics.push(diagnostic);
                    return false;
                }
                Some(b) if !(b as u8).is_ascii_hexdigit() => {
                    self.diagnostics.push(diagnostic);
                    return false;
                }
                _ => {}
            }
        }

        true
    }

    // Validate a `\..` escape sequence and advance the lexer based on it
    fn validate_escape_sequence(&mut self) -> bool {
        self.assert_byte(b'\\');
        let cur = self.position;
        self.next_byte(); // eat over the \
        if let Some(escape) = self.current_byte() {
            match escape {
                // Single escape character
                b'\\' | b'n' | b'r' | b't' | b'b' | b'v' | b'f' | b'\'' | b'"' => {
                    self.next_byte();
                    true
                }
                b'u' if self.peek_byte() == Some(b'{') => {
                    self.next_byte(); // jump over '{'
                    self.read_codepoint_escape().is_ok()
                }
                b'u' => self.read_unicode_escape(true).is_ok(),
                // hexadecimal escape sequence
                b'x' => self.validate_hex_escape(),
                b'\r' => {
                    if self.next_byte() == Some(b'\n') {
                        self.advance(1);
                    }

                    true
                }
                b if is_linebreak(b as char) => {
                    self.next_byte();
                    true
                }
                _ => {
                    // We use get_unicode_char to account for escaped source characters which are unicode
                    let chr = self.current_char_unchecked();
                    self.advance(chr.len_utf8());
                    true
                }
            }
        } else {
            self.diagnostics
                .push(Diagnostic::error(self.file_id, "", "").primary(
                    cur..cur + 1,
                    "expected an escape sequence following a backslash, but found none",
                ));
            false
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
    fn read_str_literal(&mut self, jsx_attribute: bool) -> bool {
        // Safety: this is only ever called from lex_token, which is guaranteed to be called on a char position
        let quote = unsafe { self.current_unchecked() };
        let start = self.position;
        let mut valid = true;

        self.next_byte(); // skip quote;

        while let Some(byte) = self.current_byte() {
            match byte {
                b'\\' if !jsx_attribute => {
                    if !self.validate_escape_sequence() {
                        valid = false;
                    }
                }
                b if b == quote => {
                    self.next_byte();
                    return valid;
                }
                b if is_linebreak(b as char) && !jsx_attribute => {
                    let unterminated =
                        Diagnostic::error(self.file_id, "", "unterminated string literal")
                            .primary(start..self.position, "")
                            .secondary(self.position..self.position + 2, "line breaks here");
                    self.diagnostics.push(unterminated);
                    return false;
                }
                _ => {
                    self.next_byte();
                }
            }
        }

        let unterminated = Diagnostic::error(self.file_id, "", "unterminated string literal")
            .primary(self.position..self.position, "input ends here")
            .secondary(start..start + 1, "string literal starts here");

        self.diagnostics.push(unterminated);

        false
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
    fn resolve_identifier(&mut self, first: char) -> JsSyntaxKind {
        use JsSyntaxKind::*;

        // Note to keep the buffer large enough to fit every possible keyword that
        // the lexer can return
        let mut buf = [0u8; 16];
        let len = first.encode_utf8(&mut buf).len();

        let (count, escaped) = self.consume_and_get_ident(&mut buf[len..]);

        if escaped {
            self.current_flags |= TokenFlags::UNICODE_ESCAPE;
        }

        match &buf[..count + len] {
            // Keywords
            b"break" => BREAK_KW,
            b"case" => CASE_KW,
            b"catch" => CATCH_KW,
            b"class" => CLASS_KW,
            b"const" => CONST_KW,
            b"continue" => CONTINUE_KW,
            b"debugger" => DEBUGGER_KW,
            b"default" => DEFAULT_KW,
            b"delete" => DELETE_KW,
            b"do" => DO_KW,
            b"else" => ELSE_KW,
            b"enum" => ENUM_KW,
            b"export" => EXPORT_KW,
            b"extends" => EXTENDS_KW,
            b"false" => FALSE_KW,
            b"finally" => FINALLY_KW,
            b"for" => FOR_KW,
            b"function" => FUNCTION_KW,
            b"if" => IF_KW,
            b"in" => IN_KW,
            b"import" => IMPORT_KW,
            b"instanceof" => INSTANCEOF_KW,
            b"new" => NEW_KW,
            b"null" => NULL_KW,
            b"return" => RETURN_KW,
            b"super" => SUPER_KW,
            b"switch" => SWITCH_KW,
            b"this" => THIS_KW,
            b"throw" => THROW_KW,
            b"try" => TRY_KW,
            b"true" => TRUE_KW,
            b"typeof" => TYPEOF_KW,
            b"var" => VAR_KW,
            b"void" => VOID_KW,
            b"while" => WHILE_KW,
            b"with" => WITH_KW,
            // Strict mode contextual Keywords
            b"implements" => IMPLEMENTS_KW,
            b"interface" => INTERFACE_KW,
            b"let" => LET_KW,
            b"package" => PACKAGE_KW,
            b"private" => PRIVATE_KW,
            b"protected" => PROTECTED_KW,
            b"public" => PUBLIC_KW,
            b"static" => STATIC_KW,
            b"yield" => YIELD_KW,
            // contextual keywords
            b"abstract" => ABSTRACT_KW,
            b"as" => AS_KW,
            b"asserts" => ASSERTS_KW,
            b"assert" => ASSERT_KW,
            b"any" => ANY_KW,
            b"async" => ASYNC_KW,
            b"await" => AWAIT_KW,
            b"boolean" => BOOLEAN_KW,
            b"constructor" => CONSTRUCTOR_KW,
            b"declare" => DECLARE_KW,
            b"get" => GET_KW,
            b"infer" => INFER_KW,
            b"is" => IS_KW,
            b"keyof" => KEYOF_KW,
            b"module" => MODULE_KW,
            b"namespace" => NAMESPACE_KW,
            b"never" => NEVER_KW,
            b"readonly" => READONLY_KW,
            b"require" => REQUIRE_KW,
            b"number" => NUMBER_KW,
            b"object" => OBJECT_KW,
            b"set" => SET_KW,
            b"string" => STRING_KW,
            b"symbol" => SYMBOL_KW,
            b"type" => TYPE_KW,
            b"undefined" => UNDEFINED_KW,
            b"unique" => UNIQUE_KW,
            b"unknown" => UNKNOWN_KW,
            b"from" => FROM_KW,
            b"global" => GLOBAL_KW,
            b"bigint" => BIGINT_KW,
            b"override" => OVERRIDE_KW,
            b"of" => OF_KW,
            _ => T![ident],
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
    fn read_zero(&mut self) {
        match self.peek_byte() {
            Some(b'x') | Some(b'X') => {
                if self.special_number_start(|c| c.is_ascii_hexdigit()) {
                    self.read_hexnumber();
                    self.maybe_bigint();
                } else {
                    self.next_byte();
                }
            }
            Some(b'b') | Some(b'B') => {
                if self.special_number_start(|c| c == '0' || c == '1') {
                    self.read_bindigits();
                    self.maybe_bigint();
                } else {
                    self.next_byte();
                }
            }
            Some(b'o') | Some(b'O') => {
                if self.special_number_start(|c| ('0'..='7').contains(&c)) {
                    self.read_octaldigits();
                    self.maybe_bigint();
                } else {
                    self.next_byte();
                }
            }
            Some(b'n') => {
                self.advance(2);
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
                            self.read_exponent();
                        } else {
                            self.next_byte();
                        }
                    }
                    Some(b'0'..=b'9') => {
                        self.next_byte();
                        self.read_exponent();
                    }
                    _ => {
                        self.next_byte();
                    }
                }
            }
            _ => self.read_number(true),
        }
    }

    #[inline]
    fn read_hexnumber(&mut self) {
        unwind_loop! {
            match self.next_byte() {
                Some(b'_') => self.handle_numeric_separator(16),
                Some(b) if char::from(b).is_ascii_hexdigit() => {},
                _ => {return;},
            }
        }
    }

    #[inline]
    fn handle_numeric_separator(&mut self, radix: u8) {
        self.assert_byte(b'_');

        let err_diag = Diagnostic::error(
            self.file_id,
            "",
            "numeric separators are only allowed between two digits",
        )
        .primary(self.position..self.position + 1, "");

        let peeked = self.peek_byte();

        if peeked.is_none() || !char::from(peeked.unwrap()).is_digit(radix as u32) {
            self.diagnostics.push(err_diag);
            return;
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
            self.diagnostics.push(err_diag);
            return;
        }

        self.next_byte_bounded();
    }

    #[inline]
    fn read_number(&mut self, leading_zero: bool) {
        let start = self.position;
        unwind_loop! {
            match self.next_byte_bounded() {
                Some(b'_') => {
                    if leading_zero {
                        self.diagnostics.push(
                            Diagnostic::error(
                                self.file_id,
                                "",
                                "numeric separator can not be used after leading 0",
                            )
                            .primary(self.position..self.position, ""),
                        );
                    }
                    self.handle_numeric_separator(10);
                },
                Some(b'0'..=b'9') => {},
                Some(b'.') => {
                    if leading_zero {
                        self.diagnostics.push(
                                Diagnostic::error(self.file_id, "", "unexpected number")
                                .primary(start..self.position + 1, ""),
                        );
                    }
                    return self.read_float();
                },
                // TODO: merge this, and read_float's implementation into one so we dont duplicate exponent code
                Some(b'e') | Some(b'E') => {
                    // At least one digit is required
                    match self.peek_byte() {
                        Some(b'-') | Some(b'+') => {
                            if let Some(b'0'..=b'9') = self.byte_at(2) {
                                self.next_byte();
                                self.read_exponent();
                                return
                            } else {
                                return;
                            }
                        },
                        Some(b'0'..=b'9') => {
                            self.read_exponent();
                            return;
                        },
                        _ => {
                            return;
                        },
                    }
                },
                Some(b'n') => {
                    if leading_zero {
                        self.diagnostics.push(
                                Diagnostic::error(
                                    self.file_id,
                                    "",
                                    "Octal literals are not allowed for BigInts.",
                                )
                                .primary(start..self.position + 1, ""),
                        );
                    }
                    self.next_byte();
                    return;
                }
                _ => {
                    return;
                }
            }
        }
    }

    #[inline]
    fn read_float(&mut self) {
        unwind_loop! {
            match self.next_byte_bounded() {
                Some(b'_') => self.handle_numeric_separator(10),
                // LLVM has a hard time optimizing inclusive patterns, perhaps we should check if it makes llvm sad,
                // and optimize this into a lookup table
                Some(b'0'..=b'9') => {},
                Some(b'e') | Some(b'E') => {
                    // At least one digit is required
                    match self.peek_byte() {
                        Some(b'-') | Some(b'+') => {
                            if let Some(b'0'..=b'9') = self.byte_at(2) {
                                self.next_byte();
                                self.read_exponent();
                                return;
                            } else {
                                return;
                            }
                        },
                        Some(b'0'..=b'9') => {
                            self.read_exponent();
                            return;
                        },
                        _ => {
                            return;
                        },
                    }
                },
                _ => {
                    return;
                }
            }
        }
    }

    #[inline]
    fn read_exponent(&mut self) {
        if let Some(b'-') | Some(b'+') = self.peek_byte() {
            self.next_byte();
        }

        unwind_loop! {
            match self.next_byte() {
                Some(b'_') => self.handle_numeric_separator(10),
                Some(b'0'..=b'9') => {},
                _ => {return;},
            }
        }
    }

    #[inline]
    fn read_bindigits(&mut self) {
        unwind_loop! {
            match self.next_byte() {
                Some(b'_') => self.handle_numeric_separator(2),
                Some(b'0') | Some(b'1') => {},
                _ => {return;},
            }
        }
    }

    #[inline]
    fn read_octaldigits(&mut self) {
        unwind_loop! {
            match self.next_byte() {
                Some(b'_') => self.handle_numeric_separator(8),
                Some(b'0'..=b'7') => {},
                _ => {return; },
            }
        }
    }

    #[inline]
    fn verify_number_end(&mut self) -> JsSyntaxKind {
        let err_start = self.position;
        if !self.is_eof() && self.cur_is_ident_start() {
            self.consume_ident();
            let err = Diagnostic::error(
                self.file_id,
                "",
                "numbers cannot be followed by identifiers directly after",
            )
            .primary(err_start..self.position, "an identifier cannot appear here");

            self.diagnostics.push(err);
            JsSyntaxKind::ERROR_TOKEN
        } else {
            JS_NUMBER_LITERAL
        }
    }

    #[inline]
    fn read_shebang(&mut self) -> JsSyntaxKind {
        let start = self.position;
        self.next_byte();
        if start != 0 {
            return T![#];
        }

        if let Some(b'!') = self.current_byte() {
            while self.next_byte().is_some() {
                let chr = self.current_char_unchecked();

                if is_linebreak(chr) {
                    return JS_SHEBANG;
                }
                self.advance(chr.len_utf8() - 1);
            }
            JS_SHEBANG
        } else {
            let err = Diagnostic::error(
                self.file_id,
                "",
                "expected `!` following a `#`, but found none",
            )
            .primary(0usize..1usize, "");
            self.diagnostics.push(err);

            JsSyntaxKind::ERROR_TOKEN
        }
    }

    #[inline]
    fn read_slash(&mut self) -> JsSyntaxKind {
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
                                return MULTILINE_COMMENT;
                            } else {
                                return COMMENT;
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
                self.diagnostics.push(err);

                JsSyntaxKind::COMMENT
            }
            Some(b'/') => {
                self.next_byte();
                while self.next_byte().is_some() {
                    let chr = self.current_char_unchecked();

                    if is_linebreak(chr) {
                        return COMMENT;
                    }
                    self.advance(chr.len_utf8() - 1);
                }
                COMMENT
            }
            // _ if self.state.expr_allowed => self.read_regex(),
            Some(b'=') => {
                self.advance(2);
                SLASHEQ
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
    fn read_regex(&mut self) -> JsSyntaxKind {
        let current = unsafe { self.current_unchecked() };

        if current != b'/' {
            return self.lex_token();
        }

        let start = self.position;
        let mut in_class = false;

        unwind_loop! {
            match self.next_byte() {
                Some(b'[') => in_class = true,
                Some(b']') => in_class = false,
                Some(b'/') => {
                    if !in_class {
                        let (mut g, mut i, mut m, mut s, mut u, mut y, mut d) = (false, false, false, false, false, false, false);

                        while let Some(next) = self.next_byte_bounded() {
                            let chr_start = self.position;

                            match next {
                                b'g' => {
                                    if g  {
                                        self.diagnostics.push(self.flag_err('g'));
                                    }
                                    g = true;
                                },
                                b'i' => {
                                    if i  {
                                        self.diagnostics.push(self.flag_err('i'));
                                    }
                                    i = true;
                                },
                                b'm' => {
                                    if m {
                                        self.diagnostics.push(self.flag_err('m'));
                                    }
                                    m = true;
                                },
                                b's' => {
                                    if s {
                                        self.diagnostics.push(self.flag_err('s'));
                                    }
                                    s = true;
                                },
                                b'u' => {
                                    if u {
                                        self.diagnostics.push(self.flag_err('u'));
                                    }
                                    u = true;
                                },
                                b'y' => {
                                    if y {
                                        self.diagnostics.push(self.flag_err('y'));
                                    }
                                    y = true;
                                },
                                b'd' => {
                                    if d {
                                        self.diagnostics.push(self.flag_err('d'));
                                    }
                                    d = true;
                                },
                                _ if self.cur_ident_part().is_some() => {
                                    self.diagnostics.push(
                                        Diagnostic::error(self.file_id, "", "invalid regex flag")
                                            .primary(chr_start .. self.position + 1, "this is not a valid regex flag")
                                    );
                                }
                                _ => {break}
                            };
                        }

                        return JsSyntaxKind::JS_REGEX_LITERAL;
                    }
                },
                Some(b'\\') => {
                    if self.next_byte_bounded().is_none() {
                        self.diagnostics.push(
                            Diagnostic::error(self.file_id, "", "expected a character after a regex escape, but found none")
                                .primary(self.position..self.position + 1, "expected a character following this")
                        );

                        return JsSyntaxKind::JS_REGEX_LITERAL;
                    }
                },
                Some(_) if is_linebreak(self.current_char_unchecked()) => {
                    self.diagnostics.push(
                        Diagnostic::error(self.file_id, "", "unterminated regex literal")
                            .primary(self.position..self.position, "...but the line ends here")
                            .secondary(start..start + 1, "a regex literal starts there...")
                    );

                    // Undo the read of the new line trivia
                    self.position -= 1;

                    return JsSyntaxKind::JS_REGEX_LITERAL;
                },
                None => {
                    self.diagnostics.push(
                        Diagnostic::error(self.file_id, "", "unterminated regex literal")
                            .primary(self.position..self.position, "...but the file ends here")
                            .secondary(start..start + 1, "a regex literal starts there...")
                    );

                    return JsSyntaxKind::JS_REGEX_LITERAL;
                },
                _ => {},
            }
        }
    }

    #[inline]
    fn bin_or_assign(&mut self, bin: JsSyntaxKind, assign: JsSyntaxKind) -> JsSyntaxKind {
        if let Some(b'=') = self.next_byte() {
            self.next_byte();
            assign
        } else {
            bin
        }
    }

    #[inline]
    fn resolve_bang(&mut self) -> JsSyntaxKind {
        match self.next_byte() {
            Some(b'=') => {
                if let Some(b'=') = self.next_byte() {
                    self.next_byte();
                    NEQ2
                } else {
                    NEQ
                }
            }
            _ => T![!],
        }
    }

    #[inline]
    fn resolve_amp(&mut self) -> JsSyntaxKind {
        match self.next_byte() {
            Some(b'&') => {
                if let Some(b'=') = self.next_byte() {
                    self.next_byte();
                    AMP2EQ
                } else {
                    AMP2
                }
            }
            Some(b'=') => {
                self.next_byte();
                AMPEQ
            }
            _ => T![&],
        }
    }

    #[inline]
    fn resolve_plus(&mut self) -> JsSyntaxKind {
        match self.next_byte() {
            Some(b'+') => {
                self.next_byte();
                PLUS2
            }
            Some(b'=') => {
                self.next_byte();
                PLUSEQ
            }
            _ => T![+],
        }
    }

    #[inline]
    fn resolve_minus(&mut self) -> JsSyntaxKind {
        match self.next_byte() {
            Some(b'-') => {
                self.next_byte();
                MINUS2
            }
            Some(b'=') => {
                self.next_byte();
                MINUSEQ
            }
            _ => T![-],
        }
    }

    #[inline]
    fn resolve_less_than(&mut self) -> JsSyntaxKind {
        match self.next_byte() {
            Some(b'<') => {
                if let Some(b'=') = self.next_byte() {
                    self.next_byte();
                    SHLEQ
                } else {
                    SHL
                }
            }
            Some(b'=') => {
                self.next_byte();
                LTEQ
            }
            _ => T![<],
        }
    }

    #[inline]
    fn resolve_greater_than(&mut self) -> JsSyntaxKind {
        match self.next_byte() {
            Some(b'>') => {
                if let Some(b'>') = self.peek_byte() {
                    if let Some(b'=') = self.byte_at(2) {
                        self.advance(3);
                        USHREQ
                    } else {
                        T![>]
                    }
                } else if self.peek_byte() == Some(b'=') {
                    self.advance(2);
                    SHREQ
                } else {
                    T![>]
                }
            }
            Some(b'=') => {
                self.next_byte();
                GTEQ
            }
            _ => T![>],
        }
    }

    #[inline]
    fn resolve_eq(&mut self) -> JsSyntaxKind {
        match self.next_byte() {
            Some(b'=') => {
                if let Some(b'=') = self.next_byte() {
                    self.next_byte();
                    EQ3
                } else {
                    EQ2
                }
            }
            Some(b'>') => {
                self.next_byte();
                FAT_ARROW
            }
            _ => T![=],
        }
    }

    #[inline]
    fn resolve_pipe(&mut self) -> JsSyntaxKind {
        match self.next_byte() {
            Some(b'|') => {
                if let Some(b'=') = self.next_byte() {
                    self.next_byte();
                    PIPE2EQ
                } else {
                    PIPE2
                }
            }
            Some(b'=') => {
                self.next_byte();
                PIPEEQ
            }
            _ => T![|],
        }
    }

    // Dont ask it to resolve the question of life's meaning because you'll be disappointed
    #[inline]
    fn resolve_question(&mut self) -> JsSyntaxKind {
        match self.next_byte() {
            Some(b'?') => {
                if let Some(b'=') = self.next_byte() {
                    self.next_byte();
                    QUESTION2EQ
                } else {
                    QUESTION2
                }
            }
            Some(b'.') => {
                // 11.7 Optional chaining punctuator
                if let Some(b'0'..=b'9') = self.peek_byte() {
                    T![?]
                } else {
                    self.next_byte();
                    QUESTIONDOT
                }
            }
            _ => T![?],
        }
    }

    #[inline]
    fn resolve_star(&mut self) -> JsSyntaxKind {
        match self.next_byte() {
            Some(b'*') => {
                if let Some(b'=') = self.next_byte() {
                    self.next_byte();
                    STAR2EQ
                } else {
                    STAR2
                }
            }
            Some(b'=') => {
                self.next_byte();
                STAREQ
            }
            _ => T![*],
        }
    }

    /// Lex the next token
    fn lex_token(&mut self) -> JsSyntaxKind {
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
                self.read_zero();
                self.verify_number_end()
            }
            PRD => {
                if self.peek_byte() == Some(b'.') && self.byte_at(2) == Some(b'.') {
                    self.advance(3);
                    return DOT3;
                }
                if let Some(b'0'..=b'9') = self.peek_byte() {
                    self.read_float();
                    self.verify_number_end()
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
                                self.diagnostics.push(err);
                                self.next_byte();
                                JsSyntaxKind::ERROR_TOKEN
                            }
                        }
                        Err(_) => JsSyntaxKind::ERROR_TOKEN,
                    }
                } else {
                    let err = Diagnostic::error(
                        self.file_id,
                        "",
                        format!("unexpected token `{}`", byte as char),
                    )
                    .primary(start..self.position + 1, "");
                    self.diagnostics.push(err);
                    self.next_byte();
                    JsSyntaxKind::ERROR_TOKEN
                }
            }
            QOT => {
                if self.read_str_literal(false) {
                    JS_STRING_LITERAL
                } else {
                    ERROR_TOKEN
                }
            }
            IDT => self.resolve_identifier(byte as char),
            DIG => {
                self.read_number(false);
                self.verify_number_end()
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
                        self.diagnostics.push(err);
                        self.next_byte();

                        JsSyntaxKind::ERROR_TOKEN
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
                self.diagnostics.push(err);
                self.next_byte();

                JsSyntaxKind::ERROR_TOKEN
            }
        }
    }

    fn lex_template(&mut self, tagged: bool) -> JsSyntaxKind {
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
                    let diags_len = self.diagnostics.len();
                    self.validate_escape_sequence();

                    if tagged {
                        self.diagnostics.truncate(diags_len);
                    }
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
                self.diagnostics.push(err);
                JsSyntaxKind::TEMPLATE_CHUNK
            }
            Some(token) => token,
        }
    }
}

/// Check if a char is a JS linebreak
fn is_linebreak(chr: char) -> bool {
    matches!(chr, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

/// Stores the state of the lexer so that it may later be restored to that position.
#[derive(Debug, Clone)]
pub(crate) struct LexerCheckpoint {
    pub(crate) position: TextSize,
    pub(crate) current_start: TextSize,
    pub(crate) current_kind: JsSyntaxKind,
    pub(crate) current_flags: TokenFlags,
    pub(crate) after_line_break: bool,
    pub(crate) diagnostics_pos: u32,
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
