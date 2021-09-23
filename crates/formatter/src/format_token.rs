use crate::intersperse::Intersperse;

type Content = Box<FormatToken>;
pub type Tokens = Vec<FormatToken>;

/// The tokens that are used to apply formatting.
///
/// These tokens are language agnostic.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FormatToken {
	/// Simple space
	Space,
	Line(LineToken),
	/// Content that is indented one level deeper than its parent.
	Indent(IndentToken),
	Group(GroupToken),
	List(ListToken),
	// TODO Revisit, structure is a bit weird
	IfBreak(IfBreakToken),
	/// A string that will be printed as is.
	String(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LineToken {
	pub mode: LineMode,
}

impl LineToken {
	pub const fn new(mode: LineMode) -> Self {
		Self { mode }
	}

	/// An optional line that the printer is allowed to emit to e.g. fit an array expression on a
	/// single line but gets emitted if the array expression spans across multiple lines anyway.
	pub const fn soft() -> Self {
		Self::new(LineMode::Soft)
	}

	/// A forced line break that always must be printed
	pub const fn hard() -> Self {
		Self::new(LineMode::Hard)
	}

	/// Gets printed as a space if used inside of a group that fits on a single line and otherwise
	/// gets printed as a new line (e.g. if the array expression spans multiple lines).
	pub const fn soft_or_space() -> Self {
		Self::new(LineMode::SoftOrSpace)
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IndentToken {
	pub content: Content,
}

impl IndentToken {
	pub fn new<T: Into<FormatToken>>(content: T) -> Self {
		Self {
			content: Box::new(content.into()),
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListToken {
	pub content: Vec<FormatToken>,
}

impl ListToken {
	pub fn new<T: Into<Vec<FormatToken>>>(content: T) -> Self {
		Self {
			content: content.into(),
		}
	}

	/// Takes a list of tokens and a separator as input and creates a list of tokens where they are separated by the separator.
	pub fn join<Separator: Into<FormatToken>, T: Into<Tokens>>(
		separator: Separator,
		tokens: T,
	) -> ListToken {
		let joined: Tokens =
			Intersperse::new(tokens.into().into_iter(), separator.into()).collect();
		ListToken::new(joined)
	}
}

impl<T: Into<Vec<FormatToken>>> From<T> for ListToken {
	fn from(tokens: T) -> Self {
		ListToken::new(tokens)
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IfBreakToken {
	pub break_contents: Content,
	pub flat_contents: Option<Content>,
}

impl IfBreakToken {
	pub fn new<T: Into<FormatToken>>(break_content: T) -> Self {
		Self {
			break_contents: Box::new(break_content.into()),
			flat_contents: None,
		}
	}

	pub fn new_with_flat_content<TBreak: Into<FormatToken>, TFlat: Into<FormatToken>>(
		break_content: TBreak,
		flat_content: TFlat,
	) -> Self {
		Self {
			break_contents: Box::new(break_content.into()),
			flat_contents: Some(Box::new(flat_content.into())),
		}
	}
}

/// Group is a special token that controls how the child tokens are printed.
///
/// The printer first tries to print all tokens in the group onto a single line (ignoring soft line wraps)
/// but breaks the array cross multiple lines if it would exceed the specified `line_width`, if a child token is a hard line break or if a string contains a line break.
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
	pub fn new<T: Into<FormatToken>>(content: T) -> Self {
		Self {
			content: Box::new(content.into()),
			should_break: false,
		}
	}

	pub fn new_multiline<T: Into<FormatToken>>(content: T) -> Self {
		Self {
			content: Box::new(content.into()),
			should_break: true,
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

	pub fn push_token<T: Into<FormatToken>>(mut self, value: T) -> Self {
		self.tokens.push(value.into());
		self
	}

	pub fn format_tokens(self) -> FormatToken {
		FormatToken::concat(self.tokens)
	}
}

impl<'a> FormatToken {
	/// Stores lint a list a `Vec` of `FormatToken`
	pub fn concat<T: Into<Tokens>>(tokens: T) -> FormatToken {
		let tokens = tokens.into();

		if tokens.len() == 1 {
			tokens.first().unwrap().clone()
		} else {
			FormatToken::List(ListToken::new(tokens))
		}
	}

	/// Utility to tokenize a string
	pub fn string<T: Into<&'a str>>(content: T) -> FormatToken {
		FormatToken::String(String::from(content.into()))
	}

	/// Utility to tokenize a f64
	pub fn f64<T: Into<f64>>(content: T) -> FormatToken {
		FormatToken::from(content.into())
	}

	/// Utility to tokenize a u64
	pub fn u64<T: Into<u64>>(content: T) -> FormatToken {
		FormatToken::from(content.into())
	}

	/// Utility to tokenize a boolean
	pub fn boolean<T: Into<bool>>(content: T) -> FormatToken {
		FormatToken::from(content.into())
	}
}

impl From<&str> for FormatToken {
	fn from(value: &str) -> Self {
		FormatToken::String(String::from(value))
	}
}

impl From<u64> for FormatToken {
	fn from(value: u64) -> Self {
		FormatToken::String(value.to_string())
	}
}

impl From<f64> for FormatToken {
	fn from(value: f64) -> Self {
		FormatToken::String(value.to_string())
	}
}

impl From<&bool> for FormatToken {
	fn from(value: &bool) -> Self {
		FormatToken::String(value.to_string())
	}
}

impl From<bool> for FormatToken {
	fn from(value: bool) -> Self {
		FormatToken::String(value.to_string())
	}
}

impl From<GroupToken> for FormatToken {
	fn from(group: GroupToken) -> Self {
		FormatToken::Group(group)
	}
}

impl From<Tokens> for FormatToken {
	fn from(tokens: Tokens) -> Self {
		FormatToken::concat(tokens)
	}
}

impl From<ListToken> for FormatToken {
	fn from(token: ListToken) -> Self {
		FormatToken::List(token)
	}
}

impl From<IfBreakToken> for FormatToken {
	fn from(token: IfBreakToken) -> Self {
		FormatToken::IfBreak(token)
	}
}

impl From<LineToken> for FormatToken {
	fn from(token: LineToken) -> Self {
		FormatToken::Line(token)
	}
}

impl From<IndentToken> for FormatToken {
	fn from(token: IndentToken) -> Self {
		FormatToken::Indent(token)
	}
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LineMode {
	SoftOrSpace,
	Soft,
	Hard,
}

#[cfg(test)]
mod tests {

	use super::ConcatTokens;
	use crate::format_token::{LineToken, ListToken};
	use crate::{
		format_token::{GroupToken, LineMode},
		FormatToken,
	};

	#[test]
	fn should_join() {
		let separator = ",";
		let tokens = ConcatTokens::new()
			.push_token("foo")
			.push_token("bar")
			.tokens;
		let result = FormatToken::List(ListToken::join(separator, tokens));
		let expected = ConcatTokens::new()
			.push_token("foo")
			.push_token(",")
			.push_token("bar")
			.format_tokens();

		assert_eq!(result, expected);
	}

	#[test]
	fn should_concat() {
		let tokens = vec![FormatToken::string("foo"), FormatToken::string("bar")];
		let result = FormatToken::concat(tokens);
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

		let result = GroupToken::new(tokens);
		let expected = GroupToken::new(tokens_expected);
		assert_eq!(result, expected)
	}

	#[test]
	fn should_give_line_tokens() {
		assert_eq!(LineToken::hard(), LineToken::new(LineMode::Hard));

		assert_eq!(LineToken::soft(), LineToken::new(LineMode::Soft));

		assert_eq!(
			LineToken::soft_or_space(),
			LineToken::new(LineMode::SoftOrSpace)
		);
	}
}
