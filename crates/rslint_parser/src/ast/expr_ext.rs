//! Extensions for things which are not easily generated in ast expr nodes

use crate::{ast::*, numbers::*, util::*, TextRange, TokenSet, T};
use rome_rowan::{SyntaxText, TextSize};
use SyntaxKind::*;

impl BracketExpr {
	pub fn object(&self) -> Option<JsAnyExpression> {
		support::node(self.syntax())
	}

	pub fn prop(&self) -> Option<JsAnyExpression> {
		support::children(self.syntax()).nth(1)
	}
}

impl CondExpr {
	pub fn test(&self) -> Option<JsAnyExpression> {
		support::node(self.syntax())
	}

	pub fn cons(&self) -> Option<JsAnyExpression> {
		support::children(self.syntax()).nth(1)
	}

	pub fn alt(&self) -> Option<JsAnyExpression> {
		support::children(self.syntax()).nth(2)
	}
}

impl LiteralProp {
	pub fn key(&self) -> SyntaxResult<PropName> {
		support::required_node::<PropName>(self.syntax())
	}

	pub fn value(&self) -> SyntaxResult<JsAnyExpression> {
		let child = self.syntax().children().nth(1);
		match child {
			Some(child) => JsAnyExpression::cast(child)
				.ok_or_else(|| SyntaxError::MissingRequiredChild(self.syntax().clone())),
			None => Err(SyntaxError::MissingRequiredChild(self.syntax().clone())),
		}
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

	pub fn lhs(&self) -> Option<JsAnyExpression> {
		support::node(self.syntax())
	}

	pub fn rhs(&self) -> Option<JsAnyExpression> {
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

impl KeyValuePattern {
	pub fn value(&self) -> Option<Pattern> {
		// This is to easily handle both `NAME NAME` and `: NAME`
		if self.syntax().children().count() == 2 {
			Pattern::cast(self.syntax().last_child().unwrap())
		} else {
			match self.colon_token() {
				Ok(colon_token) => colon_token
					.next_sibling_or_token()?
					.into_node()?
					.try_to::<Pattern>(),
				Err(_) => None,
			}
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

	pub fn rhs(&self) -> Option<JsAnyExpression> {
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
		!self.is_spread()
	}
}

impl ObjectExpr {
	pub fn has_trailing_comma(&self) -> bool {
		self.props().trailing_separator().is_some()
	}
}

impl JsNumberLiteral {
	pub fn as_number(&self) -> Option<f64> {
		parse_js_number(self.value_token().unwrap().text())
	}
}

impl JsBigIntLiteral {
	pub fn as_number(&self) -> Option<BigInt> {
		parse_js_big_int(self.value_token().ok()?.text())
	}
}

impl JsStringLiteral {
	/// Get the inner text of a string not including the quotes
	pub fn inner_string_text(&self) -> SyntaxText {
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

		self.syntax()
			.text()
			.slice(TextRange::new(start - offset, end - offset))
	}
}

impl ArrowExpr {
	pub fn body(&self) -> Option<ExprOrBlock> {
		ExprOrBlock::cast(self.syntax().children().last()?)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternOrExpr {
	Pattern(Pattern),
	Expr(JsAnyExpression),
}

impl AstNode for PatternOrExpr {
	fn can_cast(kind: SyntaxKind) -> bool {
		JsAnyExpression::can_cast(kind) || Pattern::can_cast(kind)
	}

	fn cast(syntax: SyntaxNode) -> Option<Self> {
		Some(if Pattern::can_cast(syntax.kind()) {
			PatternOrExpr::Pattern(Pattern::cast(syntax).unwrap())
		} else {
			PatternOrExpr::Expr(JsAnyExpression::cast(syntax).unwrap())
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
				ObjectProp::LiteralProp(litprop) => {
					litprop.key().map_or_else(|_| None, prop_name_syntax)?
				}

				ObjectProp::Getter(getter) => {
					getter.key().map_or_else(|_| None, prop_name_syntax)?
				}
				ObjectProp::Setter(setter) => {
					setter.key().map_or_else(|_| None, prop_name_syntax)?
				}
				ObjectProp::Method(method) => {
					method.name().map_or_else(|_| None, prop_name_syntax)?
				}
				ObjectProp::InitializedProp(init) => init
					.key()
					.map_or_else(|_| None, |key| Some(key.syntax().clone()))?,
				ObjectProp::SpreadProp(_) => return None,
				ObjectProp::JsUnknownMember(_) => todo!(),
			}
			.into(),
		)
	}
}

fn prop_name_syntax(name: PropName) -> Option<SyntaxNode> {
	Some(match name {
		PropName::Ident(idt) => idt.syntax().clone(),
		PropName::JsStringLiteral(lit) => lit.syntax().clone(),
		PropName::JsNumberLiteral(lit) => lit.syntax().clone(),
		PropName::Name(name) => name.syntax().clone(),
		PropName::ComputedPropertyName(_) => return None,
		PropName::JsUnknownBinding(_) => todo!(),
	})
}

impl JsAnyExpression {
	/// Whether this is an optional chain expression.
	pub fn opt_chain(&self) -> bool {
		match self {
			JsAnyExpression::DotExpr(dotexpr) => dotexpr.opt_chain_token(),
			JsAnyExpression::CallExpr(callexpr) => callexpr.opt_chain_token(),
			JsAnyExpression::BracketExpr(bracketexpr) => bracketexpr.opt_chain_token(),
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
