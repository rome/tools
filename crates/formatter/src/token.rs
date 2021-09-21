use crate::intersperse::Intersperse;

type Content = Box<FormatTokens>;
pub type Tokens = Vec<FormatTokens>;

/// The tokens that are used to apply formatting.
///
/// These tokens are language agnostic.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FormatTokens {
	Space,
	Line {
		mode: LineMode,
	},
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
	String(String),
	Number(u64),
	Boolean(bool),
	Break,
}

#[derive(Debug, Clone, Eq, PartialEq)]
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

	pub fn group(content: FormatTokens) -> FormatTokens {
		FormatTokens::Group(GroupToken::new(Box::new(content), false))
	}

	pub fn indent(content: FormatTokens) -> FormatTokens {
		FormatTokens::Indent {
			content: Box::new(content),
		}
	}

	pub fn concat<T: Into<Tokens>>(tokens: T) -> FormatTokens {
		let tokens = tokens.into();

		if tokens.len() == 1 {
			tokens.first().unwrap().clone()
		} else {
			FormatTokens::List { content: tokens }
		}
	}

	pub fn join<T: Into<Tokens>>(separator: FormatTokens, tokens: T) -> FormatTokens {
		let joined: Tokens = Intersperse::new(tokens.into().into_iter(), separator).collect();
		Self::concat(joined)
	}

	pub fn string<T: Into<&'a str>>(content: T) -> FormatTokens {
		FormatTokens::String(String::from(content.into()))
	}
}

impl From<&str> for FormatTokens {
	fn from(value: &str) -> Self {
		FormatTokens::String(String::from(value))
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
