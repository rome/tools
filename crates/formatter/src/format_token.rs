use crate::intersperse::Intersperse;
use rslint_parser::{GreenNode, SyntaxToken};
use rslint_rowan::GreenToken;
use std::ops::Deref;

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
	// TODO Revisit naming, structure is a bit weird
	IfBreak(IfBreakToken),
	Token(TokenToken),
	Node(NodeToken),
	RawNode(RawNodeToken),
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

	/// Emits a list of [ListToken] which contains a list of [FormatToken]s
	///
	/// The implementation flattens the result if any of its children is a list token
	///
	/// # Examples
	///
	/// ```
	/// use rome_formatter::{ListToken, FormatToken, Tokens};
	/// use rslint_parser::SyntaxKind;
	///
	/// let mut tokens = Tokens::default();
	/// let one: FormatToken = tokens.get(SyntaxKind::NUMBER, "1").into();
	/// let two: FormatToken = tokens.get(SyntaxKind::NUMBER, "2").into();
	///
	/// let sub_list = ListToken::concat(vec![one.clone()]);
	/// let parent_list = ListToken::concat(vec![
	///     FormatToken::List(sub_list),
	///     two.clone(),
	/// ]);
	///
	/// assert_eq!(
	///     parent_list,
	///     ListToken::concat(vec![
	///         one,
	///         two
	///     ])
	/// );
	/// ```
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
	///
	/// # Examples
	///
	/// ```
	/// use rome_formatter::{FormatToken, ListToken, Tokens};
	/// use rslint_parser::SyntaxKind;
	///
	/// let mut tokens = Tokens::default();
	/// let comma: FormatToken = tokens.comma().into();
	/// let one: FormatToken = tokens.get(SyntaxKind::NUMBER, "1").into();
	/// let two: FormatToken = tokens.get(SyntaxKind::NUMBER, "2").into();
	///
	/// let result = ListToken::join(comma.clone(), vec![one.clone(), two.clone()]);
	///
	/// assert_eq!(result, ListToken::concat(vec![one, comma, two]));
	/// ```
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TokenToken {
	pub token: GreenToken,
}

impl TokenToken {
	pub fn new<T: Into<GreenToken>>(token: T) -> Self {
		Self {
			token: token.into(),
		}
	}
}

impl From<GreenToken> for TokenToken {
	fn from(token: GreenToken) -> Self {
		TokenToken::new(token)
	}
}

impl From<SyntaxToken> for TokenToken {
	fn from(token: SyntaxToken) -> Self {
		TokenToken::new(token.green().to_owned())
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeToken {
	pub node: GreenNode,
	pub content: Content,
}

impl NodeToken {
	pub fn new<TNode: Into<GreenNode>, TContent: Into<FormatToken>>(
		node: TNode,
		content: TContent,
	) -> Self {
		Self {
			node: node.into(),
			content: Box::new(content.into()),
		}
	}
}

/// The node gets printed as is. Helpful if you want to keep a CST node unaffected by formatting,
/// e.g. because it contains erroneous syntax.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RawNodeToken {
	pub node: GreenNode,
}

impl RawNodeToken {
	pub fn new<TNode: Into<GreenNode>>(node: TNode) -> Self {
		Self { node: node.into() }
	}
}

impl From<GreenNode> for RawNodeToken {
	fn from(node: GreenNode) -> Self {
		RawNodeToken::new(node)
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

impl<T: Into<TokenToken>> From<T> for FormatToken {
	fn from(token: T) -> Self {
		FormatToken::Token(token.into())
	}
}

impl From<NodeToken> for FormatToken {
	fn from(node: NodeToken) -> Self {
		FormatToken::Node(node)
	}
}

impl From<RawNodeToken> for FormatToken {
	fn from(node: RawNodeToken) -> Self {
		FormatToken::RawNode(node)
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

	use crate::format_token::LineMode;
	use crate::format_token::LineToken;

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
