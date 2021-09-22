use crate::{intersperse::Intersperse, FormatValue};

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
	/// The content should be have indentation of one
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
		flat_contents: Content,
	},
	/// A literal string, the content will be printed with quotes
	StringLiteral(String),
	/// A number
	Number(u64),
	/// A generic boolean
	Boolean(bool),
}

/// Struct to use when the content should be wrapped into a group
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupToken {
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

	pub fn to_format_tokens(mut self) -> FormatTokens {
		FormatTokens::concat(self.tokens)
	}

	pub fn to_tokens(mut self) -> Tokens {
		self.tokens
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
	///	Group is a special token that controls how the child tokens are printed.
	///
	/// The printer first tries to print all tokens in the group onto a single line (ignoring soft line wraps)
	/// but breaks the array cross multiple lines if it would exceed the specified `line_width`, if a child token is a hard line break or if a string contains a line break.
	pub fn group(content: FormatTokens) -> FormatTokens {
		FormatTokens::Group(GroupToken::new(Box::new(content), false))
	}

	/// Apply an additional level of indentation to `content`
	pub fn indent(content: FormatTokens) -> FormatTokens {
		FormatTokens::Indent {
			content: Box::new(content),
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
	///
	///
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

	pub fn hardline() -> FormatTokens {
		Self::HARD_LINE
	}

	pub fn softline() -> FormatTokens {
		Self::SOFT_LINE
	}

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
		FormatTokens::Number(value)
	}
}

impl From<&bool> for FormatTokens {
	fn from(value: &bool) -> Self {
		FormatTokens::Boolean(*value)
	}
}

impl From<bool> for FormatTokens {
	fn from(value: bool) -> Self {
		FormatTokens::Boolean(value)
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
		FormatTokens, FormatValue,
	};

	#[test]
	fn should_join() {
		let separator = ",";
		let tokens = ConcatTokens::new()
			.push_token("foo")
			.push_token("bar")
			.to_tokens();
		let result = FormatTokens::join(separator, tokens);
		let expected = ConcatTokens::new()
			.push_token("foo")
			.push_token(",")
			.push_token("bar")
			.to_format_tokens();

		assert_eq!(result, expected);
	}

	#[test]
	fn should_concat() {
		let tokens = vec![
			FormatTokens::StringLiteral(format!("foo")),
			FormatTokens::StringLiteral(format!("bar")),
		];
		let result = FormatTokens::concat(tokens);
		let expected = ConcatTokens::new()
			.push_token("foo")
			.push_token("bar")
			.to_format_tokens();

		assert_eq!(result, expected);
	}

	#[test]
	fn should_group() {
		let tokens = ConcatTokens::new()
			.push_token("foo")
			.push_token("bar")
			.to_format_tokens();

		let tokens_expected = ConcatTokens::new()
			.push_token("foo")
			.push_token("bar")
			.to_format_tokens();

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
