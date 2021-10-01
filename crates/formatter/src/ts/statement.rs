use crate::{FormatToken,  GroupToken, LineToken, ListToken, format_tokens, ts::format_nodes};
use syntax::{
	SyntaxNode,
};

pub fn format(node: SyntaxNode) -> FormatToken {
			// doesn't have any children, so it's an empty block
			if let None = node.first_child() {
				return format_tokens!("{}");
			}
			let group = GroupToken::new(format_tokens!(
				"{",
				FormatToken::indent(format_tokens!(
					LineToken::soft_or_space(),
					ListToken::join(LineToken::soft_or_space(), format_nodes(node))
				)),
				LineToken::soft_or_space(),
				"}"
			));

			FormatToken::from(group)
}
