use crate::intersperse::Intersperse;

type Content = Box<Token>;
type Tokens = Vec<Token>;

/// TODO Rename to something different than Token?
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
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

impl Token {
	const SOFT_LINE: Token = Token::Line {
		mode: LineMode::Soft,
	};
	const HARD_LINE: Token = Token::Line {
		mode: LineMode::Hard,
	};
	const NEW_LINE_OR_SPACE: Token = Token::Line {
		mode: LineMode::Space,
	};

	pub fn group(content: Token) -> Token {
		Token::Group(GroupToken::new(Box::new(content), false))
	}

	pub fn indent(content: Token) -> Token {
		Token::Indent {
			content: Box::new(content),
		}
	}

	pub fn concat<T: Into<Tokens>>(tokens: T) -> Token {
		let tokens = tokens.into();

		if tokens.len() == 1 {
			tokens.first().unwrap().clone()
		} else {
			Token::List { content: tokens }
		}
	}

	pub fn join<T: Into<Tokens>>(separator: Token, tokens: T) -> Token {
		let joined: Tokens = Intersperse::new(tokens.into().into_iter(), separator).collect();
		Self::concat(joined)
	}

	pub fn string(content: &str) -> Token {
		Token::String(String::from(content))
	}
}

impl From<&str> for Token {
	fn from(value: &str) -> Self {
		Token::String(String::from(value))
	}
}

impl From<GroupToken> for Token {
	fn from(group: GroupToken) -> Self {
		Token::Group(group)
	}
}

impl From<Tokens> for Token {
	fn from(tokens: Tokens) -> Self {
		Token::concat(tokens)
	}
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LineMode {
	Space,
	Soft,
	Hard,
}
