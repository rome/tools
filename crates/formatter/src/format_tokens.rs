use crate::intersperse::Intersperse;

type Content = Box<FormatTokens>;
pub type Tokens = Vec<FormatTokens>;

/// The tokens that are used to apply formatting.
///
/// These tokens are language agnostic.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FormatTokens {
	/// Simple space
	Space,
	Line {
		mode: LineMode,
	},
	/// Content that is indented one level deeper than its parent.
	Indent {
		content: Content,
	},
	Group(GroupToken),
	List {
		content: Tokens,
	},
	// TODO Revisit, structure is a bit weird
	IfBreak {
		break_contents: Content,
		flat_contents: Option<Content>,
	},
	/// A literal string, the content will be printed with quotes
	StringLiteral(String),
}

/// Struct to use when the content should be wrapped into a group
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupToken {
	/// `false` if you want that the content is printed on a single line if it fits and is only
	/// broken across multiple lines if it doesn't. `true` if the content should always be printed
	/// across multiple lines. Using `true` has the same meaning as replacing all non hard line breaks
	/// with hard line breaks.
	pub should_break: bool,
	pub content: Content,
}

impl GroupToken {
	pub fn new(content: Content, should_break: bool) -> Self {
		Self {
			content,
			should_break,
		}
	}
}

/// Struct to use when there's need to create collection of tokens
#[derive(Debug, PartialEq, Eq)]
pub struct ConcatTokens {
	pub tokens: Tokens,
}

impl ConcatTokens {
	pub fn new() -> Self {
		Self { tokens: vec![] }
	}

	/// Use this utility if you know ahead of time how many tokens you will store
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			tokens: Vec::with_capacity(capacity),
		}
	}

	pub fn push_token<T: Into<FormatTokens>>(mut self, value: T) -> Self {
		self.tokens.push(value.into());
		self
	}

	pub fn format_tokens(self) -> FormatTokens {
		FormatTokens::concat(self.tokens)
	}
}

impl<'a> FormatTokens {
	const SOFT_LINE: FormatTokens = FormatTokens::Line {
		mode: LineMode::Soft,
	};
	const HARD_LINE: FormatTokens = FormatTokens::Line {
		mode: LineMode::Hard,
	};
	const NEW_LINE_OR_SPACE: FormatTokens = FormatTokens::Line {
		mode: LineMode::Space,
	};
	/// Group is a special token that controls how the child tokens are printed.
	///
	/// The printer first tries to print all tokens in the group onto a single line (ignoring soft line wraps)
	/// but breaks the array cross multiple lines if it would exceed the specified `line_width`, if a child token is a hard line break or if a string contains a line break.
	pub fn group<T: Into<FormatTokens>>(content: T) -> FormatTokens {
		FormatTokens::Group(GroupToken::new(Box::new(content.into()), false))
	}

	/// Apply an additional level of indentation to `content`
	pub fn indent<T: Into<FormatTokens>>(content: T) -> FormatTokens {
		FormatTokens::Indent {
			content: Box::new(content.into()),
		}
	}

	/// Stores lint a list a `Vec` of `FormatTokens`
	pub fn concat<T: Into<Tokens>>(tokens: T) -> FormatTokens {
		let tokens = tokens.into();

		if tokens.len() == 1 {
			tokens.first().unwrap().clone()
		} else {
			FormatTokens::List { content: tokens }
		}
	}

	/// Takes a list of tokens and a separator as input and creates a list of tokens where they are separated by the separator.
	pub fn join<Separator: Into<FormatTokens>, T: Into<Tokens>>(
		separator: Separator,
		tokens: T,
	) -> FormatTokens {
		let joined: Tokens =
			Intersperse::new(tokens.into().into_iter(), separator.into()).collect();
		Self::concat(joined)
	}

	pub fn string<T: Into<&'a str>>(content: T) -> FormatTokens {
		FormatTokens::StringLiteral(String::from(content.into()))
	}

	/// A forced line break that always must be printed
	pub fn hardline() -> FormatTokens {
		Self::HARD_LINE
	}

	/// An optional line that the printer is allowed to emit to e.g. fit an array expression on a
	/// single line but gets emitted if the array expression spans across multiple lines anyway.
	pub fn softline() -> FormatTokens {
		Self::SOFT_LINE
	}

	/// Gets printed as a space if used inside of a group that fits on a single line and otherwise
	/// gets printed as a new line (e.g. if the array expression spans multiple lines).
	pub fn new_line_or_space() -> FormatTokens {
		Self::NEW_LINE_OR_SPACE
	}
}

impl From<&str> for FormatTokens {
	fn from(value: &str) -> Self {
		FormatTokens::StringLiteral(String::from(value))
	}
}

impl From<u64> for FormatTokens {
	fn from(value: u64) -> Self {
		FormatTokens::StringLiteral(value.to_string())
	}
}

impl From<&bool> for FormatTokens {
	fn from(value: &bool) -> Self {
		FormatTokens::StringLiteral(value.to_string())
	}
}

impl From<bool> for FormatTokens {
	fn from(value: bool) -> Self {
		FormatTokens::StringLiteral(value.to_string())
	}
}

impl From<GroupToken> for FormatTokens {
	fn from(group: GroupToken) -> Self {
		FormatTokens::Group(group)
	}
}

impl From<Tokens> for FormatTokens {
	fn from(tokens: Tokens) -> Self {
		FormatTokens::concat(tokens)
	}
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LineMode {
	Space,
	Soft,
	Hard,
}

#[cfg(test)]
mod tests {

	use super::ConcatTokens;
	use crate::{
		format_tokens::{GroupToken, LineMode},
		FormatTokens,
	};

	#[test]
	fn should_join() {
		let separator = ",";
		let tokens = ConcatTokens::new()
			.push_token("foo")
			.push_token("bar")
			.tokens;
		let result = FormatTokens::join(separator, tokens);
		let expected = ConcatTokens::new()
			.push_token("foo")
			.push_token(",")
			.push_token("bar")
			.format_tokens();

		assert_eq!(result, expected);
	}

	#[test]
	fn should_concat() {
		let tokens = vec![
			FormatTokens::StringLiteral("foo".to_string()),
			FormatTokens::StringLiteral("bar".to_string()),
		];
		let result = FormatTokens::concat(tokens);
		let expected = ConcatTokens::new()
			.push_token("foo")
			.push_token("bar")
			.format_tokens();

		assert_eq!(result, expected);
	}

	#[test]
	fn should_group() {
		let tokens = ConcatTokens::new()
			.push_token("foo")
			.push_token("bar")
			.format_tokens();

		let tokens_expected = ConcatTokens::new()
			.push_token("foo")
			.push_token("bar")
			.format_tokens();

		let result = FormatTokens::group(tokens);
		let expected = GroupToken::new(Box::new(tokens_expected), false);
		match result {
			FormatTokens::Group(result) => {
				assert_eq!(result, expected)
			}
			_ => unreachable!(),
		}
	}

	#[test]
	fn should_give_line_tokens() {
		let hard_line = FormatTokens::hardline();
		let soft_line = FormatTokens::softline();
		let line_or_space = FormatTokens::new_line_or_space();

		assert_eq!(
			hard_line,
			FormatTokens::Line {
				mode: LineMode::Hard
			}
		);

		assert_eq!(
			soft_line,
			FormatTokens::Line {
				mode: LineMode::Soft
			}
		);

		assert_eq!(
			line_or_space,
			FormatTokens::Line {
				mode: LineMode::Space
			}
		);
	}
}
