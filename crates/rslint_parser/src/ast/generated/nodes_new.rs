//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{
	ast::*,
	SyntaxKind::{self, *},
	SyntaxNode, SyntaxToken, T,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
	pub(crate) syntax: SyntaxNode,
}
impl Ident {
	pub fn ident_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![ident]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Script {
	pub(crate) syntax: SyntaxNode,
}
impl Script {
	pub fn shebang_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![shebang]) }
	pub fn stmt(&self) -> AstChildren<Stmt> { support::children(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockStmt {
	pub(crate) syntax: SyntaxNode,
}
impl BlockStmt {
	pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
	pub fn stmt(&self) -> AstChildren<Stmt> { support::children(&self.syntax) }
	pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmptyStmt {
	pub(crate) syntax: SyntaxNode,
}
impl EmptyStmt {
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprStmt {
	pub(crate) syntax: SyntaxNode,
}
impl ExprStmt {
	pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrowExpr {
	pub(crate) syntax: SyntaxNode,
}
impl ArrowExpr {
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn type_params(&self) -> Option<TsTypeParams> { support::child(&self.syntax) }
	pub fn params(&self) -> Option<ArrowExprParams> { support::child(&self.syntax) }
	pub fn fat_arrow_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [=>]) }
	pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [:]) }
	pub fn return_type(&self) -> Option<TsType> { support::child(&self.syntax) }
	pub fn body(&self) -> Option<ExprOrBlock> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsTypeParams {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeParams {
	pub fn l_angle_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [<]) }
	pub fn r_angle_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [>]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
	pub(crate) syntax: SyntaxNode,
}
impl Name {
	pub fn ident(&self) -> Option<Ident> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParameterList {
	pub(crate) syntax: SyntaxNode,
}
impl ParameterList {
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn pattern(&self) -> AstChildren<Pattern> { support::children(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringValue {
	pub(crate) syntax: SyntaxNode,
}
impl StringValue {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BooleanValue {
	pub(crate) syntax: SyntaxNode,
}
impl BooleanValue {
	pub fn true_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![true]) }
	pub fn false_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![false]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumberValue {
	pub(crate) syntax: SyntaxNode,
}
impl NumberValue {
	pub fn number_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![number]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegexValue {
	pub(crate) syntax: SyntaxNode,
}
impl RegexValue {
	pub fn regex_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![regex]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FloatValue {
	pub(crate) syntax: SyntaxNode,
}
impl FloatValue {
	pub fn float_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![float]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BigIntValue {
	pub(crate) syntax: SyntaxNode,
}
impl BigIntValue {
	pub fn big_int_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![big_int]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NullValue {
	pub(crate) syntax: SyntaxNode,
}
impl NullValue {
	pub fn null_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![null]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UndefinedValue {
	pub(crate) syntax: SyntaxNode,
}
impl UndefinedValue {
	pub fn undefined_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![undefined])
	}
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pattern {
	pub(crate) syntax: SyntaxNode,
}
impl Pattern {
	pub fn single_pattern(&self) -> Option<SinglePattern> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SinglePattern {
	pub(crate) syntax: SyntaxNode,
}
impl SinglePattern {
	pub fn name(&self) -> Option<Ident> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsAny {
	pub(crate) syntax: SyntaxNode,
}
impl TsAny {
	pub fn any_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![any]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsUnknown {
	pub(crate) syntax: SyntaxNode,
}
impl TsUnknown {
	pub fn unknown_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![unknown]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
	BlockStmt(BlockStmt),
	EmptyStmt(EmptyStmt),
	ExprStmt(ExprStmt),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
	ArrowExpr(ArrowExpr),
	Literal(Literal),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
	StringValue(StringValue),
	BooleanValue(BooleanValue),
	NumberValue(NumberValue),
	RegexValue(RegexValue),
	NumberValue(NumberValue),
	FloatValue(FloatValue),
	BigIntValue(BigIntValue),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArrowExprParams {
	Name(Name),
	ParameterList(ParameterList),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TsType {
	TsAny(TsAny),
	TsUnknown(TsUnknown),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprOrBlock {
	Expr(Expr),
	BlockStmt(BlockStmt),
}
impl AstNode for Ident {
	fn can_cast(kind: SyntaxKind) -> bool { kind == IDENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Script {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SCRIPT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BlockStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == BLOCK_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for EmptyStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EMPTY_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ExprStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPR_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ArrowExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == ARROW_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsTypeParams {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_PARAMS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Name {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ParameterList {
	fn can_cast(kind: SyntaxKind) -> bool { kind == PARAMETER_LIST }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for StringValue {
	fn can_cast(kind: SyntaxKind) -> bool { kind == STRING_VALUE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BooleanValue {
	fn can_cast(kind: SyntaxKind) -> bool { kind == BOOLEAN_VALUE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for NumberValue {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NUMBER_VALUE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for RegexValue {
	fn can_cast(kind: SyntaxKind) -> bool { kind == REGEX_VALUE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for FloatValue {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FLOAT_VALUE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BigIntValue {
	fn can_cast(kind: SyntaxKind) -> bool { kind == BIG_INT_VALUE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for NullValue {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NULL_VALUE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for UndefinedValue {
	fn can_cast(kind: SyntaxKind) -> bool { kind == UNDEFINED_VALUE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Pattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SinglePattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SINGLE_PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsAny {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ANY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TsUnknown {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_UNKNOWN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl From<BlockStmt> for Stmt {
	fn from(node: BlockStmt) -> Stmt { Stmt::BlockStmt(node) }
}
impl From<EmptyStmt> for Stmt {
	fn from(node: EmptyStmt) -> Stmt { Stmt::EmptyStmt(node) }
}
impl From<ExprStmt> for Stmt {
	fn from(node: ExprStmt) -> Stmt { Stmt::ExprStmt(node) }
}
impl AstNode for Stmt {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, BLOCK_STMT | EMPTY_STMT | EXPR_STMT) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			BLOCK_STMT => Stmt::BlockStmt(BlockStmt { syntax }),
			EMPTY_STMT => Stmt::EmptyStmt(EmptyStmt { syntax }),
			EXPR_STMT => Stmt::ExprStmt(ExprStmt { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Stmt::BlockStmt(it) => &it.syntax,
			Stmt::EmptyStmt(it) => &it.syntax,
			Stmt::ExprStmt(it) => &it.syntax,
		}
	}
}
impl From<ArrowExpr> for Expr {
	fn from(node: ArrowExpr) -> Expr { Expr::ArrowExpr(node) }
}
impl From<Literal> for Expr {
	fn from(node: Literal) -> Expr { Expr::Literal(node) }
}
impl AstNode for Expr {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, ARROW_EXPR | LITERAL) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			ARROW_EXPR => Expr::ArrowExpr(ArrowExpr { syntax }),
			LITERAL => Expr::Literal(Literal { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Expr::ArrowExpr(it) => &it.syntax,
			Expr::Literal(it) => &it.syntax,
		}
	}
}
impl From<StringValue> for Literal {
	fn from(node: StringValue) -> Literal { Literal::StringValue(node) }
}
impl From<BooleanValue> for Literal {
	fn from(node: BooleanValue) -> Literal { Literal::BooleanValue(node) }
}
impl From<NumberValue> for Literal {
	fn from(node: NumberValue) -> Literal { Literal::NumberValue(node) }
}
impl From<RegexValue> for Literal {
	fn from(node: RegexValue) -> Literal { Literal::RegexValue(node) }
}
impl From<NumberValue> for Literal {
	fn from(node: NumberValue) -> Literal { Literal::NumberValue(node) }
}
impl From<FloatValue> for Literal {
	fn from(node: FloatValue) -> Literal { Literal::FloatValue(node) }
}
impl From<BigIntValue> for Literal {
	fn from(node: BigIntValue) -> Literal { Literal::BigIntValue(node) }
}
impl AstNode for Literal {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			STRING_VALUE
				| BOOLEAN_VALUE | NUMBER_VALUE
				| REGEX_VALUE | NUMBER_VALUE
				| FLOAT_VALUE | BIG_INT_VALUE
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			STRING_VALUE => Literal::StringValue(StringValue { syntax }),
			BOOLEAN_VALUE => Literal::BooleanValue(BooleanValue { syntax }),
			NUMBER_VALUE => Literal::NumberValue(NumberValue { syntax }),
			REGEX_VALUE => Literal::RegexValue(RegexValue { syntax }),
			NUMBER_VALUE => Literal::NumberValue(NumberValue { syntax }),
			FLOAT_VALUE => Literal::FloatValue(FloatValue { syntax }),
			BIG_INT_VALUE => Literal::BigIntValue(BigIntValue { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Literal::StringValue(it) => &it.syntax,
			Literal::BooleanValue(it) => &it.syntax,
			Literal::NumberValue(it) => &it.syntax,
			Literal::RegexValue(it) => &it.syntax,
			Literal::NumberValue(it) => &it.syntax,
			Literal::FloatValue(it) => &it.syntax,
			Literal::BigIntValue(it) => &it.syntax,
		}
	}
}
impl From<Name> for ArrowExprParams {
	fn from(node: Name) -> ArrowExprParams { ArrowExprParams::Name(node) }
}
impl From<ParameterList> for ArrowExprParams {
	fn from(node: ParameterList) -> ArrowExprParams { ArrowExprParams::ParameterList(node) }
}
impl AstNode for ArrowExprParams {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, NAME | PARAMETER_LIST) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			NAME => ArrowExprParams::Name(Name { syntax }),
			PARAMETER_LIST => ArrowExprParams::ParameterList(ParameterList { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ArrowExprParams::Name(it) => &it.syntax,
			ArrowExprParams::ParameterList(it) => &it.syntax,
		}
	}
}
impl From<TsAny> for TsType {
	fn from(node: TsAny) -> TsType { TsType::TsAny(node) }
}
impl From<TsUnknown> for TsType {
	fn from(node: TsUnknown) -> TsType { TsType::TsUnknown(node) }
}
impl AstNode for TsType {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, TS_ANY | TS_UNKNOWN) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_ANY => TsType::TsAny(TsAny { syntax }),
			TS_UNKNOWN => TsType::TsUnknown(TsUnknown { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsType::TsAny(it) => &it.syntax,
			TsType::TsUnknown(it) => &it.syntax,
		}
	}
}
impl From<Expr> for ExprOrBlock {
	fn from(node: Expr) -> ExprOrBlock { ExprOrBlock::Expr(node) }
}
impl From<BlockStmt> for ExprOrBlock {
	fn from(node: BlockStmt) -> ExprOrBlock { ExprOrBlock::BlockStmt(node) }
}
impl AstNode for ExprOrBlock {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, EXPR | BLOCK_STMT) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			EXPR => ExprOrBlock::Expr(Expr { syntax }),
			BLOCK_STMT => ExprOrBlock::BlockStmt(BlockStmt { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ExprOrBlock::Expr(it) => &it.syntax,
			ExprOrBlock::BlockStmt(it) => &it.syntax,
		}
	}
}
impl std::fmt::Display for Stmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ArrowExprParams {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExprOrBlock {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Ident {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Script {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BlockStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for EmptyStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExprStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ArrowExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeParams {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Name {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ParameterList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for StringValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BooleanValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NumberValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for RegexValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FloatValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BigIntValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NullValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for UndefinedValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Pattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SinglePattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsAny {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsUnknown {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
