//! Extensions for things which are not easily generated in ast expr nodes

use crate::{ast::*, numbers::*, util::*, TextRange, T};
use rome_rowan::{SyntaxText, TextSize};
use SyntaxKind::*;

impl JsComputedMemberExpression {
	pub fn member(&self) -> SyntaxResult<JsAnyExpression> {
		support::children(self.syntax())
			.nth(1)
			.ok_or_else(|| SyntaxError::MissingRequiredChild(self.syntax().clone()))
	}
}

impl JsLiteralMemberName {
	/// Returns the name of the member as a syntax text
	///
	/// ## Examples
	///
	/// Getting the name of a static member containing a string literal
	///
	/// ```
	/// use rome_rowan::TreeBuilder;
	/// use rslint_parser::{SyntaxKind, JsLanguage, SyntaxNode, SyntaxNodeExt};
	/// use rslint_parser::ast::JsLiteralMemberName;
	///
	/// let node: SyntaxNode = TreeBuilder::wrap_with_node(SyntaxKind::JS_LITERAL_MEMBER_NAME, |builder| {
	///   builder.token(SyntaxKind::JS_STRING_LITERAL, "\"abcd\"");
	/// });
	///
	/// let static_member_name = node.to::<JsLiteralMemberName>();
	///
	/// assert_eq!("abcd", static_member_name.name().unwrap());
	/// ```
	///
	/// Getting the name of a static member containing a number literal
	///
	/// ```
	/// use rome_rowan::TreeBuilder;
	/// use rslint_parser::{SyntaxKind, JsLanguage, SyntaxNode, SyntaxNodeExt};
	/// use rslint_parser::ast::JsLiteralMemberName;
	///
	/// let node: SyntaxNode = TreeBuilder::wrap_with_node(SyntaxKind::JS_LITERAL_MEMBER_NAME, |builder| {
	///   builder.token(SyntaxKind::JS_NUMBER_LITERAL, "5");
	/// });
	///
	/// let static_member_name = node.to::<JsLiteralMemberName>();
	///
	/// assert_eq!("5", static_member_name.name().unwrap());
	/// ```
	///
	/// Getting the name of a static member containing an identifier
	///
	/// ```
	/// use rome_rowan::TreeBuilder;
	/// use rslint_parser::{SyntaxKind, JsLanguage, SyntaxNode, SyntaxNodeExt};
	/// use rslint_parser::ast::JsLiteralMemberName;
	///
	/// let node: SyntaxNode = TreeBuilder::wrap_with_node(SyntaxKind::JS_LITERAL_MEMBER_NAME, |builder| {
	///   builder.token(SyntaxKind::IDENT, "abcd");
	/// });
	///
	/// let static_member_name = node.to::<JsLiteralMemberName>();
	///
	/// assert_eq!("abcd", static_member_name.name().unwrap());
	/// ```
	pub fn name(&self) -> SyntaxResult<String> {
		let value = self.value()?;
		let name = value.text_trimmed();

		let result = match value.kind() {
			JS_STRING_LITERAL => String::from(&name[1..name.len() - 1]),
			_ => String::from(name),
		};

		Ok(result)
	}
}

impl JsConditionalExpression {
	pub fn consequent(&self) -> SyntaxResult<JsAnyExpression> {
		support::children(self.syntax())
			.nth(1)
			.ok_or_else(|| SyntaxError::MissingRequiredChild(self.syntax().clone()))
	}

	pub fn alternate(&self) -> SyntaxResult<JsAnyExpression> {
		support::children(self.syntax())
			.nth(2)
			.ok_or_else(|| SyntaxError::MissingRequiredChild(self.syntax().clone()))
	}
}

impl JsPropertyObjectMember {
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
pub enum JsBinaryOperation {
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
	/// `in`
	In,
	/// `instanceof`
	Instanceof,
}

impl JsBinaryExpression {
	pub fn operator_kind(&self) -> SyntaxResult<JsBinaryOperation> {
		let kind = match self.operator()?.kind() {
			T![<] => JsBinaryOperation::LessThan,
			T![>] => JsBinaryOperation::GreaterThan,
			T![<=] => JsBinaryOperation::LessThanOrEqual,
			T![>=] => JsBinaryOperation::GreaterThanOrEqual,
			T![==] => JsBinaryOperation::Equality,
			T![===] => JsBinaryOperation::StrictEquality,
			T![!=] => JsBinaryOperation::Inequality,
			T![!==] => JsBinaryOperation::StrictInequality,
			T![+] => JsBinaryOperation::Plus,
			T![-] => JsBinaryOperation::Minus,
			T![*] => JsBinaryOperation::Times,
			T![/] => JsBinaryOperation::Divide,
			T![%] => JsBinaryOperation::Remainder,
			T![**] => JsBinaryOperation::Exponent,
			T![<<] => JsBinaryOperation::LeftShift,
			T![>>] => JsBinaryOperation::RightShift,
			T![>>>] => JsBinaryOperation::UnsignedRightShift,
			T![&] => JsBinaryOperation::BitwiseAnd,
			T![|] => JsBinaryOperation::BitwiseOr,
			T![^] => JsBinaryOperation::BitwiseXor,
			T![in] => JsBinaryOperation::In,
			T![instanceof] => JsBinaryOperation::Instanceof,
			_ => unreachable!(),
		};

		Ok(kind)
	}

	pub fn right(&self) -> Option<JsAnyExpression> {
		support::children(self.syntax()).nth(1)
	}

	/// Whether this is a comparison operation, such as `>`, `<`, `==`, `!=`, `===`, etc.
	pub fn is_comparison_operator(&self) -> bool {
		matches!(
			self.operator().map(|t| t.kind()),
			Ok(T![>] | T![<] | T![>=] | T![<=] | T![==] | T![===] | T![!=] | T![!==])
		)
	}
}

pub enum JsLogicalOperation {
	/// `??`
	NullishCoalescing,
	/// `||`
	LogicalOr,
	/// `&&`
	LogicalAnd,
}

impl JsLogicalExpression {
	pub fn operator_kind(&self) -> SyntaxResult<JsLogicalOperation> {
		let kind = match self.operator()?.kind() {
			T![&&] => JsLogicalOperation::LogicalAnd,
			T![||] => JsLogicalOperation::LogicalOr,
			T![??] => JsLogicalOperation::NullishCoalescing,
			_ => unreachable!(),
		};

		Ok(kind)
	}

	pub fn right(&self) -> Option<JsAnyExpression> {
		support::children(self.syntax()).nth(1)
	}
}

impl JsSequenceExpression {
	pub fn right(&self) -> SyntaxResult<JsAnyExpression> {
		support::children(self.syntax())
			.nth(1)
			.ok_or_else(|| SyntaxError::MissingRequiredChild(self.syntax().clone()))
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JsUnaryOperation {
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
}

impl JsUnaryExpression {
	pub fn operation(&self) -> SyntaxResult<JsUnaryOperation> {
		let operator = self.operator()?;

		Ok(match operator.kind() {
			T![+] => JsUnaryOperation::Plus,
			T![-] => JsUnaryOperation::Minus,
			T![~] => JsUnaryOperation::BitwiseNot,
			T![!] => JsUnaryOperation::LogicalNot,
			T![typeof] => JsUnaryOperation::Typeof,
			T![void] => JsUnaryOperation::Void,
			T![delete] => JsUnaryOperation::Delete,
			_ => unreachable!(),
		})
	}
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

impl JsArrayExpression {
	pub fn has_trailing_comma(&self) -> bool {
		self.elements().trailing_separator().is_some()
	}
}

impl JsObjectExpression {
	pub fn has_trailing_comma(&self) -> bool {
		self.members().trailing_separator().is_some()
	}
}

impl JsNumberLiteralExpression {
	pub fn as_number(&self) -> Option<f64> {
		parse_js_number(self.value_token().unwrap().text())
	}
}

impl JsBigIntLiteralExpression {
	pub fn as_number(&self) -> Option<BigInt> {
		parse_js_big_int(self.value_token().ok()?.text())
	}
}

impl JsStringLiteralExpression {
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

impl JsArrowFunctionExpression {
	pub fn body(&self) -> Option<JsAnyArrowFunctionBody> {
		JsAnyArrowFunctionBody::cast(self.syntax().children().last()?)
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

impl CallExpr {
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
