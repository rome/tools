//! Extensions for things which are not easily generated in ast expr nodes

use crate::{ast::*, numbers::*, util::*, SyntaxText, TextRange, TextSize, TokenSet, T};
use SyntaxKind::*;

impl BracketExpr {
	pub fn object(&self) -> Option<Expr> {
		support::children(self.syntax()).next()
	}

	pub fn prop(&self) -> Option<Expr> {
		support::children(self.syntax()).nth(1)
	}
}

impl CondExpr {
	pub fn test(&self) -> Option<Expr> {
		support::children(self.syntax()).next()
	}

	pub fn cons(&self) -> Option<Expr> {
		support::children(self.syntax()).nth(1)
	}

	pub fn alt(&self) -> Option<Expr> {
		support::children(self.syntax()).nth(2)
	}
}

impl LiteralProp {
	pub fn key(&self) -> Option<PropName> {
		if PropName::can_cast(
			support::children::<PropName>(self.syntax())
				.next()?
				.syntax()
				.kind(),
		) {
			PropName::cast(
				support::children::<PropName>(self.syntax())
					.next()
					.unwrap()
					.syntax()
					.to_owned(),
			)
		} else {
			None
		}
	}

	pub fn value(&self) -> Option<Expr> {
		self.syntax().children().nth(1)?.try_to()
	}
}

/// A binary operation applied to two expressions
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BinOp {
	/// `<`
	LessThan,
	/// `>`
	GreaterThan,
	/// `<=`
	LessThanOrEqual,
	/// `>=`
	GreaterThanOrEqual,
	/// `==`
	Equality,
	/// `===`
	StrictEquality,
	/// `!=`
	Inequality,
	/// `!==`
	StrictInequality,
	/// `+`
	Plus,
	/// `-`
	Minus,
	/// `*`
	Times,
	/// `/`
	Divide,
	/// `%`
	Remainder,
	/// `**`
	Exponent,
	/// `<<`
	LeftShift,
	/// `>>`
	RightShift,
	/// `>>>`
	UnsignedRightShift,
	/// `&`
	BitwiseAnd,
	/// `|`
	BitwiseOr,
	/// `^`
	BitwiseXor,
	/// `??`
	NullishCoalescing,
	/// `||`
	LogicalOr,
	/// `&&`
	LogicalAnd,
	/// `in`
	In,
	/// `instanceof`
	Instanceof,
}

impl BinExpr {
	pub fn op_details(&self) -> Option<(SyntaxToken, BinOp)> {
		self.syntax()
			.children_with_tokens()
			.filter_map(|x| x.into_token())
			.find_map(|t| {
				let op = match t.kind() {
					T![<] => BinOp::LessThan,
					T![>] => BinOp::GreaterThan,
					T![<=] => BinOp::LessThanOrEqual,
					T![>=] => BinOp::GreaterThanOrEqual,
					T![==] => BinOp::Equality,
					T![===] => BinOp::StrictEquality,
					T![!=] => BinOp::Inequality,
					T![!==] => BinOp::StrictInequality,
					T![+] => BinOp::Plus,
					T![-] => BinOp::Minus,
					T![*] => BinOp::Times,
					T![/] => BinOp::Divide,
					T![%] => BinOp::Remainder,
					T![**] => BinOp::Exponent,
					T![<<] => BinOp::LeftShift,
					T![>>] => BinOp::RightShift,
					T![>>>] => BinOp::UnsignedRightShift,
					T![&] => BinOp::BitwiseAnd,
					T![|] => BinOp::BitwiseOr,
					T![^] => BinOp::BitwiseXor,
					T![??] => BinOp::NullishCoalescing,
					T![||] => BinOp::LogicalOr,
					T![&&] => BinOp::LogicalAnd,
					T![in] => BinOp::In,
					T![instanceof] => BinOp::Instanceof,
					_ => return None,
				};
				Some((t, op))
			})
	}

	pub fn op(&self) -> Option<BinOp> {
		self.op_details().map(|t| t.1)
	}

	pub fn op_token(&self) -> Option<SyntaxToken> {
		self.op_details().map(|t| t.0)
	}

	pub fn lhs(&self) -> Option<Expr> {
		support::children(self.syntax()).next()
	}

	pub fn rhs(&self) -> Option<Expr> {
		support::children(self.syntax()).nth(1)
	}

	/// Whether this binary expr is a `||` or `&&` expression.
	pub fn conditional(&self) -> bool {
		token_set![T![||], T![&&]].contains(self.op_token().map(|x| x.kind()).unwrap_or(T![&]))
	}

	/// Whether this is a comparison operation, such as `>`, `<`, `==`, `!=`, `===`, etc.
	pub fn comparison(&self) -> bool {
		const SET: TokenSet = token_set![
			T![>],
			T![<],
			T![>=],
			T![<=],
			T![==],
			T![===],
			T![!=],
			T![!==]
		];
		SET.contains(self.op_token().map(|x| x.kind()).unwrap_or(T![&]))
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOp {
	/// `++`
	Increment,
	/// `--`
	Decrement,
	/// `delete`
	Delete,
	/// `void`
	Void,
	/// `typeof`
	Typeof,
	/// `+`
	Plus,
	/// `-`
	Minus,
	/// `~`
	BitwiseNot,
	/// `!`
	LogicalNot,
	/// `await`
	Await,
}

impl UnaryExpr {
	pub fn op_details(&self) -> Option<(SyntaxToken, UnaryOp)> {
		self.syntax()
			.children_with_tokens()
			.filter_map(|x| x.into_token())
			.find_map(|t| {
				let op = match t.kind() {
					T![++] => UnaryOp::Increment,
					T![--] => UnaryOp::Decrement,
					T![delete] => UnaryOp::Delete,
					T![void] => UnaryOp::Void,
					T![typeof] => UnaryOp::Typeof,
					T![+] => UnaryOp::Plus,
					T![-] => UnaryOp::Minus,
					T![~] => UnaryOp::BitwiseNot,
					T![!] => UnaryOp::LogicalNot,
					T![await] => UnaryOp::Await,
					_ => return None,
				};
				Some((t, op))
			})
	}

	pub fn op(&self) -> Option<UnaryOp> {
		self.op_details().map(|t| t.1)
	}

	pub fn op_token(&self) -> Option<SyntaxToken> {
		self.op_details().map(|t| t.0)
	}

	/// Whether this is an update expression.
	pub fn is_update(&self) -> bool {
		self.op().map_or(false, |op| {
			op == UnaryOp::Increment || op == UnaryOp::Decrement
		})
	}

	/// Whether this is an update expression and it is a prefix update expression
	pub fn is_prefix(&self) -> Option<bool> {
		if !self.is_update() {
			return None;
		}

		Some(self.op_token()?.text_range().start() > self.expr()?.syntax().text_range().end())
	}
}

impl KeyValuePattern {
	pub fn value(&self) -> Option<Pattern> {
		// This is to easily handle both `NAME NAME` and `: NAME`
		if self.syntax().children().count() == 2 {
			Pattern::cast(self.syntax().last_child().unwrap())
		} else {
			self.colon_token()?
				.next_sibling_or_token()?
				.into_node()?
				.try_to::<Pattern>()
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AssignOp {
	Assign,
	AddAssign,
	SubtractAssign,
	TimesAssign,
	RemainderAssign,
	ExponentAssign,
	LeftShiftAssign,
	RightShiftAssign,
	UnsignedRightShiftAssign,
	BitwiseAndAssign,
	BitwiseOrAssign,
	BitwiseXorAssign,
	LogicalAndAssign,
	LogicalOrAssign,
	NullishCoalescingAssign,
}

impl AssignExpr {
	pub fn op_details(&self) -> Option<(SyntaxToken, AssignOp)> {
		self.syntax()
			.children_with_tokens()
			.filter_map(|x| x.into_token())
			.find_map(|t| {
				let op = match t.kind() {
					T![=] => AssignOp::Assign,
					T![+=] => AssignOp::AddAssign,
					T![-=] => AssignOp::SubtractAssign,
					T![*=] => AssignOp::TimesAssign,
					T![%=] => AssignOp::RemainderAssign,
					T![**=] => AssignOp::ExponentAssign,
					T![>>=] => AssignOp::LeftShiftAssign,
					T![<<=] => AssignOp::RightShiftAssign,
					T![>>>=] => AssignOp::UnsignedRightShiftAssign,
					T![&=] => AssignOp::BitwiseAndAssign,
					T![|=] => AssignOp::BitwiseOrAssign,
					T![^=] => AssignOp::BitwiseXorAssign,
					T![&&=] => AssignOp::LogicalAndAssign,
					T![||=] => AssignOp::LogicalOrAssign,
					T![??=] => AssignOp::NullishCoalescingAssign,
					_ => return None,
				};
				Some((t, op))
			})
	}

	pub fn op(&self) -> Option<AssignOp> {
		self.op_details().map(|t| t.1)
	}

	pub fn op_token(&self) -> Option<SyntaxToken> {
		self.op_details().map(|t| t.0)
	}

	pub fn lhs(&self) -> Option<PatternOrExpr> {
		self.syntax.children().next().and_then(|n| n.try_to())
	}

	pub fn rhs(&self) -> Option<Expr> {
		self.syntax.children().nth(1).and_then(|n| n.try_to())
	}
}

impl ArrayExpr {
	pub fn has_trailing_comma(&self) -> bool {
		if let Some(last) = self.elements().last().map(|it| it.syntax().to_owned()) {
			if let Some(tok) = last
				.next_sibling_or_token()
				.map(|it| it.into_token())
				.flatten()
			{
				return tok.kind() == T![,];
			}
		}
		false
	}

	/// A list of all sparse elements as a vector of the comma tokens
	pub fn sparse_elements(&self) -> Vec<SyntaxToken> {
		let node = self.syntax();
		let commas = node
			.children_with_tokens()
			.filter_map(|x| x.into_token().filter(|tok| tok.kind() == COMMA));
		commas
			.filter(|comma| {
				let mut siblings = comma
					.siblings_with_tokens(crate::Direction::Prev)
					.skip(1)
					.skip_while(|item| {
						item.as_token()
							.filter(|tok| tok.kind().is_trivia())
							.is_some()
					});

				siblings
					.next()
					.and_then(|x| x.into_node()?.try_to::<ExprOrSpread>())
					.is_none()
			})
			.collect()
	}
}

impl ExprOrSpread {
	pub fn is_spread(&self) -> bool {
		matches!(self, ExprOrSpread::SpreadElement(_))
	}

	pub fn is_expr(&self) -> bool {
		matches!(self, ExprOrSpread::Expr(_))
	}
}

impl ObjectExpr {
	pub fn has_trailing_comma(&self) -> bool {
		if let Some(last) = self.props().last().map(|it| it.syntax().to_owned()) {
			if let Some(tok) = last
				.next_sibling_or_token()
				.map(|it| it.into_token())
				.flatten()
			{
				return tok.kind() == T![,];
			}
		}
		false
	}
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LiteralKind {
	Number(f64),
	BigInt(BigInt),
	String,
	Null,
	Bool(bool),
	Regex,
}

impl Literal {
	pub fn token(&self) -> SyntaxToken {
		self.syntax()
			.children_with_tokens()
			.find(|e| !e.kind().is_trivia())
			.and_then(|e| e.into_token())
			.unwrap()
	}

	pub fn kind(&self) -> LiteralKind {
		dbg!(self.token().kind());
		match self.token().kind() {
			T![null] => LiteralKind::Null,
			NUMBER => match parse_js_num(self.to_string()).unwrap() {
				JsNum::BigInt(bigint) => LiteralKind::BigInt(bigint),
				JsNum::Float(float) => LiteralKind::Number(float),
			},
			STRING => LiteralKind::String,
			TRUE_KW => LiteralKind::Bool(true),
			FALSE_KW => LiteralKind::Bool(false),
			REGEX => LiteralKind::Regex,
			_ => unreachable!(),
		}
	}

	pub fn as_number(&self) -> Option<f64> {
		if let LiteralKind::Number(num) = self.kind() {
			Some(num)
		} else {
			None
		}
	}

	pub fn is_number(&self) -> bool {
		matches!(self.kind(), LiteralKind::Number(_))
	}

	pub fn is_string(&self) -> bool {
		self.kind() == LiteralKind::String
	}

	pub fn is_null(&self) -> bool {
		self.kind() == LiteralKind::Null
	}

	pub fn is_bool(&self) -> bool {
		matches!(self.kind(), LiteralKind::Bool(_))
	}

	pub fn is_regex(&self) -> bool {
		self.kind() == LiteralKind::Regex
	}

	/// Get the inner text of a string not including the quotes
	pub fn inner_string_text(&self) -> Option<SyntaxText> {
		if !self.is_string() {
			return None;
		}

		let start = self.syntax().text_range().start() + TextSize::from(1);
		let end_char = self
			.syntax()
			.text()
			.char_at(self.syntax().text().len() - TextSize::from(1))
			.unwrap();
		let end = if end_char == '"' || end_char == '\'' {
			self.syntax().text_range().end() - TextSize::from(1)
		} else {
			self.syntax().text_range().end()
		};

		let offset = self.syntax().text_range().start();

		Some(
			self.syntax()
				.text()
				.slice(TextRange::new(start - offset, end - offset)),
		)
	}
}

impl ArrowExpr {
	pub fn body(&self) -> Option<ExprOrBlock> {
		dbg!("here", self.syntax().children().last());
		ExprOrBlock::cast(self.syntax().children().last()?)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternOrExpr {
	Pattern(Pattern),
	Expr(Expr),
}

impl AstNode for PatternOrExpr {
	fn can_cast(kind: SyntaxKind) -> bool {
		Expr::can_cast(kind) || Pattern::can_cast(kind)
	}

	fn cast(syntax: SyntaxNode) -> Option<Self> {
		Some(if Pattern::can_cast(syntax.kind()) {
			PatternOrExpr::Pattern(Pattern::cast(syntax).unwrap())
		} else {
			PatternOrExpr::Expr(Expr::cast(syntax).unwrap())
		})
	}

	fn syntax(&self) -> &SyntaxNode {
		match self {
			PatternOrExpr::Pattern(it) => it.syntax(),
			PatternOrExpr::Expr(it) => it.syntax(),
		}
	}
}

impl Template {
	/// The string chunks of the template. aka:
	/// `foo ${bar} foo` breaks down into:
	/// `QUASIS ELEMENT{EXPR} QUASIS`
	pub fn quasis(&self) -> impl Iterator<Item = SyntaxToken> {
		self.syntax()
			.children_with_tokens()
			.filter_map(NodeOrToken::into_token)
			.filter(|t| t.kind() == TEMPLATE_CHUNK)
	}

	pub fn template_range(&self) -> Option<TextRange> {
		let start = self
			.syntax()
			.children_with_tokens()
			.filter_map(|x| x.into_token())
			.find(|tok| tok.kind() == BACKTICK)?;
		Some(TextRange::new(
			start.text_range().start(),
			self.syntax().text_range().end(),
		))
	}
}

impl ObjectProp {
	pub fn key(&self) -> Option<std::string::String> {
		Some(self.key_element()?.to_string())
	}

	pub fn key_element(&self) -> Option<SyntaxElement> {
		Some(
			match self {
				ObjectProp::IdentProp(idt) => idt.syntax().clone(),
				ObjectProp::LiteralProp(litprop) => prop_name_syntax(litprop.key()?)?,
				ObjectProp::Getter(getter) => prop_name_syntax(getter.key()?)?,
				ObjectProp::Setter(setter) => prop_name_syntax(setter.key()?)?,
				ObjectProp::Method(method) => prop_name_syntax(method.name()?)?,
				ObjectProp::InitializedProp(init) => init.key()?.syntax().clone(),
				ObjectProp::SpreadProp(_) => return None,
			}
			.into(),
		)
	}
}

fn prop_name_syntax(name: PropName) -> Option<SyntaxNode> {
	Some(match name {
		PropName::Ident(idt) => idt.syntax().clone(),
		PropName::Literal(lit) => lit.syntax().clone(),
		PropName::Name(name) => name.syntax().clone(),
		PropName::ComputedPropertyName(_) => return None,
	})
}

impl Expr {
	/// Whether this is an optional chain expression.
	pub fn opt_chain(&self) -> bool {
		match self {
			Expr::DotExpr(dotexpr) => dotexpr.opt_chain_token(),
			Expr::CallExpr(callexpr) => callexpr.opt_chain_token(),
			Expr::BracketExpr(bracketexpr) => bracketexpr.opt_chain_token(),
			_ => return false,
		}
		.is_some()
	}
}

impl DotExpr {
	pub fn opt_chain_token(&self) -> Option<SyntaxToken> {
		self.syntax()
			.children_with_tokens()
			.filter_map(|child| child.into_token())
			.find(|tok| tok.kind() == QUESTIONDOT)
	}
}

impl CallExpr {
	pub fn opt_chain_token(&self) -> Option<SyntaxToken> {
		self.syntax()
			.children_with_tokens()
			.filter_map(|child| child.into_token())
			.find(|tok| tok.kind() == QUESTIONDOT)
	}
}

impl BracketExpr {
	pub fn opt_chain_token(&self) -> Option<SyntaxToken> {
		self.syntax()
			.children_with_tokens()
			.filter_map(|child| child.into_token())
			.find(|tok| tok.kind() == QUESTIONDOT)
	}
}

/// A simple macro for making assign, binop, or unary operators
#[macro_export]
macro_rules! op {
	(<) => {
		$crate::ast::BinOp::LessThan
	};
	(>) => {
		$crate::ast::BinOp::GreaterThan
	};
	(<=) => {
		$crate::ast::BinOp::LessThanOrEqual
	};
	(>=) => {
		$crate::ast::BinOp::GreaterThanOrEqual
	};
	(==) => {
		$crate::ast::BinOp::Equality
	};
	(===) => {
		$crate::ast::BinOp::StrictEquality
	};
	(!=) => {
		$crate::ast::BinOp::Inequality
	};
	(!==) => {
		$crate::ast::BinOp::StrictInequality
	};
	(+) => {
		$crate::ast::BinOp::Plus
	};
	(-) => {
		$crate::ast::BinOp::Minus
	};
	(*) => {
		$crate::ast::BinOp::Times
	};
	(/) => {
		$crate::ast::BinOp::Divide
	};
	(%) => {
		$crate::ast::BinOp::Remainder
	};
	(**) => {
		$crate::ast::BinOp::Exponent
	};
	(<<) => {
		$crate::ast::BinOp::LeftShift
	};
	(>>) => {
		$crate::ast::BinOp::RightShift
	};
	(>>>) => {
		$crate::ast::BinOp::UnsignedRightShift
	};
	(&) => {
		$crate::ast::BinOp::BitwiseAnd
	};
	(|) => {
		$crate::ast::BinOp::BitwiseOr
	};
	(^) => {
		$crate::ast::BinOp::BitwiseXor
	};
	(??) => {
		$crate::ast::BinOp::NullishCoalescing
	};
	(||) => {
		$crate::ast::BinOp::LogicalOr
	};
	(&&) => {
		$crate::ast::BinOp::LogicalAnd
	};
	(in) => {
		$crate::ast::BinOp::In
	};
	(instanceof) => {
		$crate::ast::BinOp::Instanceof
	};

	(=) => {
		$crate::ast::AssignOp::Assign
	};
	(+=) => {
		$crate::ast::AssignOp::AddAssign
	};
	(-=) => {
		$crate::ast::AssignOp::SubtractAssign
	};
	(*=) => {
		$crate::ast::AssignOp::TimesAssign
	};
	(%=) => {
		$crate::ast::AssignOp::RemainderAssign
	};
	(**=) => {
		$crate::ast::AssignOp::ExponentAssign
	};
	(>>=) => {
		$crate::ast::AssignOp::LeftShiftAssign
	};
	(<<=) => {
		$crate::ast::AssignOp::RightShiftAssign
	};
	(>>>=) => {
		$crate::ast::AssignOp::UnsignedRightShiftAssign
	};
	(&=) => {
		$crate::ast::AssignOp::BitwiseAndAssign
	};
	(|=) => {
		$crate::ast::AssignOp::BitwiseOrAssign
	};
	(^=) => {
		$crate::ast::AssignOp::BitwiseXorAssign
	};
	(&&=) => {
		$crate::ast::AssignOp::LogicalAndAssign
	};
	(||=) => {
		$crate::ast::AssignOp::LogicalOrAssign
	};
	(??=) => {
		$crate::ast::AssignOp::NullishCoalescingAssign
	};

	(++) => {
		$crate::ast::UnaryOp::Increment
	};
	(--) => {
		$crate::ast::UnaryOp::Decrement
	};
	(delete) => {
		$crate::ast::UnaryOp::Delete
	};
	(void) => {
		$crate::ast::UnaryOp::Void
	};
	(typeof) => {
		$crate::ast::UnaryOp::Typeof
	};
	(+) => {
		$crate::ast::UnaryOp::Plus
	};
	(-) => {
		$crate::ast::UnaryOp::Minus
	};
	(~) => {
		$crate::ast::UnaryOp::BitwiseNot
	};
	(!) => {
		$crate::ast::UnaryOp::LogicalNot
	};
	(await) => {
		$crate::ast::UnaryOp::Await
	};
}
