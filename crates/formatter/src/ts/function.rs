use crate::{format_tokens, FormatToken};
use syntax::SyntaxToken;

pub fn format_token(_node: SyntaxToken) -> FormatToken {
	format_tokens!("function")
}
