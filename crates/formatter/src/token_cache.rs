use hashbrown::hash_map::RawEntryMut;
use hashbrown::HashMap;
use rslint_parser::SyntaxKind;
use rslint_rowan::{GreenToken, SmolStr};
use std::hash::{BuildHasher, Hash, Hasher};

/// Cache for re-using rowan green tokens.
///
/// Manually hashes the tokens to avoid storing the token text as part of the key and the token.
#[derive(Debug, Clone, Default)]
pub struct TokensCache(HashMap<GreenToken, ()>);

impl TokensCache {
	pub fn get(&mut self, kind: SyntaxKind, text: &str) -> GreenToken {
		let kind = rslint_rowan::SyntaxKind(kind.into());
		let hash = {
			let mut hasher = self.0.hasher().build_hasher();
			kind.hash(&mut hasher);
			text.hash(&mut hasher);
			hasher.finish()
		};

		let entry = self
			.0
			.raw_entry_mut()
			.from_hash(hash, |token| token.kind() == kind && token.text() == text);

		match entry {
			RawEntryMut::Occupied(entry) => entry.key().clone(),
			RawEntryMut::Vacant(entry) => {
				let token = GreenToken::new(kind, SmolStr::new(text));

				entry.insert_hashed_nocheck(hash, token.clone(), ());
				token
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::tokens::Tokens;
	use rslint_parser::SyntaxKind;

	#[test]
	fn it_returns_a_token_with_the_specified_kind_and_text() {
		let mut cache = Tokens::default();

		let one = cache.get(SyntaxKind::NUMBER, "1");

		assert_eq!("1", one.text());
		assert_eq!(
			rslint_rowan::SyntaxKind(SyntaxKind::NUMBER.into()),
			one.kind()
		);
	}

	#[test]
	fn it_returns_the_same_green_nodes_for_identical_text_and_kind() {
		let mut cache = Tokens::default();

		let indent = cache.get(SyntaxKind::WHITESPACE, "  ");
		let indent_2 = cache.get(SyntaxKind::WHITESPACE, "  ");

		assert_eq!(indent, indent_2);

		assert!(
			indent.shallow_eq(&indent_2),
			"Point to the same green token data"
		);
	}

	#[test]
	fn it_returns_different_tokens_if_text_differs() {
		let mut cache = Tokens::default();

		let one = cache.get(SyntaxKind::NUMBER, "1");
		let two = cache.get(SyntaxKind::NUMBER, "2");

		assert_eq!("1", one.text());
		assert_eq!("2", two.text());
	}

	#[test]
	fn it_returns_different_tokens_if_the_kind_differs() {
		let mut cache = Tokens::default();

		let whitespace = cache.get(SyntaxKind::WHITESPACE, " ");
		let string = cache.get(SyntaxKind::STRING, " ");

		assert_eq!(
			rslint_rowan::SyntaxKind(SyntaxKind::WHITESPACE.into()),
			whitespace.kind()
		);
		assert_eq!(
			rslint_rowan::SyntaxKind(SyntaxKind::STRING.into()),
			string.kind()
		);
	}
}
