use crate::token_cache::TokensCache;
use lazy_static::lazy_static;
use rslint_parser::SyntaxKind;
use rslint_rowan::{GreenToken, SmolStr};

/// Helper for building rowan green tree tokens.
///
/// Caches the tokens internally to reduce the memory foot-print.
#[derive(Debug, Clone, Default)]
pub struct Tokens(TokensCache);

#[inline]
fn create_small_token(kind: SyntaxKind, text: &str) -> GreenToken {
	GreenToken::new(
		rslint_rowan::SyntaxKind(kind.into()),
		SmolStr::new_inline(text),
	)
}

impl Tokens {
	/// Creates a token with the specified kind and text or retrieves a cached instance
	pub fn get(&mut self, kind: SyntaxKind, text: &str) -> GreenToken {
		self.0.get(kind, text)
	}

	/// Creates a string token with the specified text
	#[inline]
	pub fn double_quoted_string(&mut self, text: &str) -> GreenToken {
		self.get(SyntaxKind::STRING, format!("\"{}\"", text).as_str())
	}

	/// Creates a whitespace token
	#[inline]
	pub fn whitespace(&mut self, whitespace: &str) -> GreenToken {
		self.get(SyntaxKind::WHITESPACE, whitespace)
	}

	/// Returns a `,` token
	#[inline]
	pub fn comma(&self) -> GreenToken {
		lazy_static! {
			static ref COMMA: GreenToken = create_small_token(SyntaxKind::COMMA, ",");
		}

		(*COMMA).clone()
	}

	/// Returns a `'` token
	#[inline]
	pub fn colon(&self) -> GreenToken {
		lazy_static! {
			static ref COLON: GreenToken = create_small_token(SyntaxKind::COLON, ":");
		}

		(*COLON).clone()
	}

	/// Returns a `[` token
	#[inline]
	pub fn left_bracket(&self) -> GreenToken {
		lazy_static! {
			static ref LBRACK: GreenToken = create_small_token(SyntaxKind::L_BRACK, "[");
		}

		(*LBRACK).clone()
	}

	/// Returns a `]` token
	#[inline]
	pub fn right_bracket(&self) -> GreenToken {
		lazy_static! {
			static ref RBRACK: GreenToken = create_small_token(SyntaxKind::R_BRACK, "]");
		}

		(*RBRACK).clone()
	}

	/// Returns a `{` token
	#[inline]
	pub fn left_brace(&self) -> GreenToken {
		lazy_static! {
			static ref LBRACE: GreenToken = create_small_token(SyntaxKind::L_CURLY, "{");
		}

		(*LBRACE).clone()
	}

	/// Returns a `}` token
	#[inline]
	pub fn right_brace(&self) -> GreenToken {
		lazy_static! {
			static ref RBRACE: GreenToken = create_small_token(SyntaxKind::R_CURLY, "}");
		}

		(*RBRACE).clone()
	}

	/// Returns a `null` token
	#[inline]
	pub fn null(&self) -> GreenToken {
		lazy_static! {
			static ref NULL: GreenToken = create_small_token(SyntaxKind::NULL_KW, "null");
		}

		(*NULL).clone()
	}
}
