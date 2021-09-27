// Hacky prototyping tools

use crate::SyntaxToken;
use rowan::NodeOrToken;

pub fn hacky_whitespace(text: &str) -> Option<SyntaxToken> {
	let root = crate::parse(text).unwrap();
	let token = root
		.clone_for_update()
		.first_child_or_token()?
		.into_token()?;
	token.detach();
	Some(token)
}

pub fn replace_token(old: SyntaxToken, new: SyntaxToken) {
	let parent = old.parent().unwrap();
	let index = old.index();

	let to_insert = vec![NodeOrToken::Token(new)];
	parent.splice_children(index..index + 1, to_insert)
}

pub fn replace_with_whitespace(old: SyntaxToken, text: &str) {
	let token = hacky_whitespace(text).unwrap();
	replace_token(old, token);
}
