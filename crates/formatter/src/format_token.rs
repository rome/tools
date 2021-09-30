use crate::intersperse::Intersperse;
use std::ops::Deref;
use syntax::SyntaxNode;

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
	String(StringToken),
	Syntax(SyntaxNode),
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

/// A token used to gather a list of tokens; optionally they can be printed with a separator, using [ListToken::join]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListToken {
	content: Vec<FormatToken>,
}

impl ListToken {
	fn new(content: Vec<FormatToken>) -> Self {
		Self { content }
	}

	/// Emits a list of [ListToken] which contains a list of [FormatToken]
	pub fn concat<T: IntoIterator<Item = FormatToken>>(tokens: T) -> Self {
		let tokens: Vec<FormatToken> = tokens
			.into_iter()
			.flat_map(|t| match t {
				FormatToken::List(list) => list.content,
				_ => vec![t],
			})
			.collect();
		Self::new(tokens)
	}

	/// Takes a list of tokens and a separator as input and creates a list of tokens where they are separated by that separator.
	pub fn join<Separator: Into<FormatToken>, T: IntoIterator<Item = FormatToken>>(
		separator: Separator,
		tokens: T,
	) -> ListToken {
		Self::concat(Intersperse::new(tokens.into_iter(), separator.into()))
	}
}

impl<T: Into<Vec<FormatToken>>> From<T> for ListToken {
	fn from(tokens: T) -> Self {
		ListToken::concat(tokens.into())
	}
}

impl Deref for ListToken {
	type Target = Vec<FormatToken>;

	fn deref(&self) -> &Self::Target {
		&self.content
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

/// A code fragment that gets written to the output. Newlines must be encoded with line feeds `\n`.
/// The [Printer] takes care of converting the line feeds to the line ending specified in the options.
///
/// # Example
/// ```
/// use rome_formatter::{FormatToken, format_token, FormatOptions};
/// let token = FormatToken::string(r#"["a", "b"]"#);
/// let formatted = format_token(&token, FormatOptions::default());
///
/// assert_eq!(r#"["a", "b"]"#, formatted.code());
/// ```
/// Is not allowed to contain any new lines
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StringToken(String);

impl StringToken {
	pub fn new(content: &str) -> Self {
		debug_assert!(!content.contains('\r'), "The content '{}' contains a carriage return '\\r' character but string tokens must only use line feeds '\\n' as line separator. Use '\\n' instead of '\\r' and '\\r\\n' to insert a line break in strings.", content);
		Self(String::from(content))
	}
}

impl From<&str> for StringToken {
	fn from(content: &str) -> Self {
		StringToken::new(content)
	}
}

impl From<String> for StringToken {
	fn from(content: String) -> Self {
		StringToken::new(content.as_str())
	}
}

impl Deref for StringToken {
	type Target = String;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> FormatToken {
	/// Stores lint a list a `Vec` of `FormatToken`
	pub fn concat<T: Into<Tokens>>(tokens: T) -> FormatToken {
		let tokens = tokens.into();

		if tokens.len() == 1 {
			tokens.first().unwrap().clone()
		} else {
			FormatToken::List(ListToken::concat(tokens))
		}
	}

	pub fn join<TSep: Into<FormatToken>, I: IntoIterator<Item = FormatToken>>(
		separator: TSep,
		tokens: I,
	) -> FormatToken {
		FormatToken::List(ListToken::join(separator, tokens))
	}

	pub fn indent<T: Into<FormatToken>>(content: T) -> FormatToken {
		FormatToken::Indent(IndentToken::new(content))
	}

	/// Utility to tokenize a string
	pub fn string<T: Into<&'a str>>(content: T) -> FormatToken {
		FormatToken::String(StringToken::new(content.into()))
	}

	/// Utility to tokenize a f64
	pub fn f64<T: Into<f64>>(content: T) -> FormatToken {
		FormatToken::string(content.into().to_string().as_str())
	}

	/// Utility to tokenize a u64
	pub fn u64<T: Into<u64>>(content: T) -> FormatToken {
		FormatToken::string(content.into().to_string().as_str())
	}

	/// Utility to tokenize a boolean
	pub fn boolean<T: Into<bool>>(content: T) -> FormatToken {
		FormatToken::string(content.into().to_string().as_str())
	}
}

impl From<&str> for FormatToken {
	fn from(value: &str) -> Self {
		FormatToken::string(value)
	}
}

impl From<u64> for FormatToken {
	fn from(value: u64) -> Self {
		FormatToken::u64(value)
	}
}

impl From<f64> for FormatToken {
	fn from(value: f64) -> Self {
		FormatToken::f64(value)
	}
}

impl From<&bool> for FormatToken {
	fn from(value: &bool) -> Self {
		FormatToken::boolean(*value)
	}
}

impl From<bool> for FormatToken {
	fn from(value: bool) -> Self {
		FormatToken::boolean(value)
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

impl From<SyntaxNode> for FormatToken {
	fn from(token: SyntaxNode) -> Self {
		FormatToken::Syntax(token)
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

	use crate::format_token::{LineToken, ListToken};
	use crate::{format_token::LineMode, FormatToken};

	#[test]
	fn should_join() {
		let separator = ",";
		let tokens = vec![FormatToken::string("foo"), FormatToken::string("bar")];

		let result = FormatToken::join(separator, tokens);

		let expected = FormatToken::concat(vec![
			FormatToken::string("foo"),
			FormatToken::string(","),
			FormatToken::string("bar"),
		]);

		assert_eq!(result, expected);
	}

	#[test]
	fn should_concat() {
		let tokens = vec![FormatToken::string("foo"), FormatToken::string("bar")];

		let result = FormatToken::concat(tokens);

		let expected = FormatToken::List(ListToken::new(vec![
			FormatToken::string("foo"),
			FormatToken::string("bar"),
		]));

		assert_eq!(result, expected);
	}

	#[test]
	fn flattens_lists() {
		let sub_list = ListToken::concat(vec![FormatToken::string("sub_list")]);
		let parent_list = ListToken::concat(vec![
			FormatToken::string("parent"),
			FormatToken::List(sub_list),
		]);

		assert_eq!(
			parent_list,
			ListToken::concat(vec![
				FormatToken::string("parent"),
				FormatToken::string("sub_list")
			])
		)
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
