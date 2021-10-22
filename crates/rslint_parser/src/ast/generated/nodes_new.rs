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
pub struct NameRef {
	pub(crate) syntax: SyntaxNode,
}
impl NameRef {
	pub fn ident(&self) -> Option<Ident> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThisExpr {
	pub(crate) syntax: SyntaxNode,
}
impl ThisExpr {
	pub fn this_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![this]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayExpr {
	pub(crate) syntax: SyntaxNode,
}
impl ArrayExpr {
	pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
	pub fn elements(&self) -> AstChildren<ExprOrSpread> { support::children(&self.syntax) }
	pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectExpr {
	pub(crate) syntax: SyntaxNode,
}
impl ObjectExpr {
	pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
	pub fn props(&self) -> AstChildren<ObjectProp> { support::children(&self.syntax) }
	pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupingExpr {
	pub(crate) syntax: SyntaxNode,
}
impl GroupingExpr {
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn inner(&self) -> Option<Expr> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BracketExpr {
	pub(crate) syntax: SyntaxNode,
}
impl BracketExpr {
	pub fn super_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![super]) }
	pub fn object(&self) -> Option<Expr> { support::child(&self.syntax) }
	pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
	pub fn prop(&self) -> Option<Expr> { support::child(&self.syntax) }
	pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DotExpr {
	pub(crate) syntax: SyntaxNode,
}
impl DotExpr {
	pub fn dot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [.]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewExpr {
	pub(crate) syntax: SyntaxNode,
}
impl NewExpr {
	pub fn new_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![new]) }
	pub fn object(&self) -> Option<Expr> { support::child(&self.syntax) }
	pub fn arguments(&self) -> Option<ArgList> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallExpr {
	pub(crate) syntax: SyntaxNode,
}
impl CallExpr {
	pub fn callee(&self) -> Option<Expr> { support::child(&self.syntax) }
	pub fn arguments(&self) -> Option<ArgList> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnaryExpr {
	pub(crate) syntax: SyntaxNode,
}
impl UnaryExpr {
	pub fn lhs(&self) -> Option<Expr> { support::child(&self.syntax) }
	pub fn increment_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [++]) }
	pub fn decrement_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [--]) }
	pub fn delete_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![delete]) }
	pub fn void_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![void]) }
	pub fn typeof_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![typeof]) }
	pub fn plus_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [+]) }
	pub fn minus_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [-]) }
	pub fn bitwise_not_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [~]) }
	pub fn excl_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![!]) }
	pub fn await_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![await]) }
	pub fn rhs(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinExpr {
	pub(crate) syntax: SyntaxNode,
}
impl BinExpr {
	pub fn lhs(&self) -> Option<Expr> { support::child(&self.syntax) }
	pub fn l_angle_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [<]) }
	pub fn r_angle_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [>]) }
	pub fn less_than_equal_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [<=])
	}
	pub fn greater_than_equal_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [>=])
	}
	pub fn equality_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [==]) }
	pub fn strict_equality_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [===])
	}
	pub fn inequality_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [!=]) }
	pub fn strict_inequality_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [!==])
	}
	pub fn plus_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [+]) }
	pub fn minus_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [-]) }
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn divide_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [/]) }
	pub fn reminder_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [%]) }
	pub fn exponent_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [**]) }
	pub fn left_shift_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [<<]) }
	pub fn right_shift_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [>>])
	}
	pub fn unsigned_right_shift_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [>>>])
	}
	pub fn amp_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [&]) }
	pub fn bitwise_or_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [|]) }
	pub fn bitwise_xor_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [^]) }
	pub fn nullish_coalescing_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [??])
	}
	pub fn logical_or_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [||]) }
	pub fn logical_and_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [&&])
	}
	pub fn in_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![in]) }
	pub fn instanceof_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![instanceof])
	}
	pub fn rhs(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CondExpr {
	pub(crate) syntax: SyntaxNode,
}
impl CondExpr {
	pub fn question_mark_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [?])
	}
	pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [:]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignExpr {
	pub(crate) syntax: SyntaxNode,
}
impl AssignExpr {
	pub fn lhs(&self) -> Option<PatternOrExpr> { support::child(&self.syntax) }
	pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [=]) }
	pub fn add_assign_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [+=]) }
	pub fn subtract_assign_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [-=])
	}
	pub fn times_assign_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [*=])
	}
	pub fn remainder_assign_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [%=])
	}
	pub fn exponent_assign_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [**=])
	}
	pub fn left_shift_assign_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [>>=])
	}
	pub fn right_shift_assign_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [<<=])
	}
	pub fn unsigned_right_shift_assign_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [>>>=])
	}
	pub fn bitwise_and_assign_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [&=])
	}
	pub fn bitwise_or_assign_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [|=])
	}
	pub fn bitwise_xor_assign_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [^=])
	}
	pub fn bitwise_logical_and_assign_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [&&=])
	}
	pub fn bitwise_logical_or_assign_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [||=])
	}
	pub fn bitwise_nullish_coalescing_assign_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [??=])
	}
	pub fn rhs(&self) -> Option<PatternOrExpr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SequenceExpr {
	pub(crate) syntax: SyntaxNode,
}
impl SequenceExpr {
	pub fn exprs(&self) -> AstChildren<Expr> { support::children(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnExpr {
	pub(crate) syntax: SyntaxNode,
}
impl FnExpr {
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn function_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![function])
	}
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn name(&self) -> Option<Name> { support::child(&self.syntax) }
	pub fn parameters(&self) -> Option<ArgList> { support::child(&self.syntax) }
	pub fn body(&self) -> Option<BlockStmt> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassExpr {
	pub(crate) syntax: SyntaxNode,
}
impl ClassExpr {
	pub fn class_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![class]) }
	pub fn name(&self) -> Option<Name> { support::child(&self.syntax) }
	pub fn extends_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![extends]) }
	pub fn parent(&self) -> Option<Name> { support::child(&self.syntax) }
	pub fn body(&self) -> Option<ClassBody> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewTarget {
	pub(crate) syntax: SyntaxNode,
}
impl NewTarget {
	pub fn new_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![new]) }
	pub fn dot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [.]) }
	pub fn target_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![target]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportMeta {
	pub(crate) syntax: SyntaxNode,
}
impl ImportMeta {
	pub fn import_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![import]) }
	pub fn dot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [.]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SuperCall {
	pub(crate) syntax: SyntaxNode,
}
impl SuperCall {
	pub fn super_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![super]) }
	pub fn arguments(&self) -> Option<ArgList> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportCall {
	pub(crate) syntax: SyntaxNode,
}
impl ImportCall {
	pub fn import_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![import]) }
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn argument(&self) -> Option<Expr> { support::child(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct YieldExpr {
	pub(crate) syntax: SyntaxNode,
}
impl YieldExpr {
	pub fn yield_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![yield]) }
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn value(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AwaitExpr {
	pub(crate) syntax: SyntaxNode,
}
impl AwaitExpr {
	pub fn await_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![await]) }
	pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrivatePropAccess {
	pub(crate) syntax: SyntaxNode,
}
impl PrivatePropAccess {
	pub fn lhs(&self) -> Option<Expr> { support::child(&self.syntax) }
	pub fn dot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [.]) }
	pub fn rhs(&self) -> Option<PrivateName> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArgList {
	pub(crate) syntax: SyntaxNode,
}
impl ArgList {
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn expr(&self) -> AstChildren<Expr> { support::children(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
	pub(crate) syntax: SyntaxNode,
}
impl Name {
	pub fn ident(&self) -> Option<Ident> { support::child(&self.syntax) }
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
pub struct ParameterList {
	pub(crate) syntax: SyntaxNode,
}
impl ParameterList {
	pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
	pub fn pattern(&self) -> AstChildren<Pattern> { support::children(&self.syntax) }
	pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassBody {
	pub(crate) syntax: SyntaxNode,
}
impl ClassBody {
	pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
	pub fn elements(&self) -> AstChildren<ClassElement> { support::children(&self.syntax) }
	pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Method {
	pub(crate) syntax: SyntaxNode,
}
impl Method {
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn name(&self) -> Option<PropName> { support::child(&self.syntax) }
	pub fn parameters(&self) -> Option<ParameterList> { support::child(&self.syntax) }
	pub fn body(&self) -> Option<BlockStmt> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrivateProp {
	pub(crate) syntax: SyntaxNode,
}
impl PrivateProp {
	pub fn class_prop(&self) -> Option<ClassProp> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassProp {
	pub(crate) syntax: SyntaxNode,
}
impl ClassProp {
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn hash_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [#]) }
	pub fn key(&self) -> Option<PropName> { support::child(&self.syntax) }
	pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [=]) }
	pub fn value(&self) -> Option<Expr> { support::child(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Constructor {
	pub(crate) syntax: SyntaxNode,
}
impl Constructor {
	pub fn name(&self) -> Option<PropName> { support::child(&self.syntax) }
	pub fn parameters(&self) -> Option<ParameterList> { support::child(&self.syntax) }
	pub fn body(&self) -> Option<BlockStmt> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Getter {
	pub(crate) syntax: SyntaxNode,
}
impl Getter {
	pub fn get_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![get]) }
	pub fn key(&self) -> Option<PropName> { support::child(&self.syntax) }
	pub fn parameters(&self) -> Option<ParameterList> { support::child(&self.syntax) }
	pub fn body(&self) -> Option<BlockStmt> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Setter {
	pub(crate) syntax: SyntaxNode,
}
impl Setter {
	pub fn set_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![set]) }
	pub fn key(&self) -> Option<PropName> { support::child(&self.syntax) }
	pub fn parameters(&self) -> Option<ParameterList> { support::child(&self.syntax) }
	pub fn body(&self) -> Option<BlockStmt> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropName {
	pub(crate) syntax: SyntaxNode,
}
impl PropName {
	pub fn computed_property_name(&self) -> Option<ComputedPropertyName> {
		support::child(&self.syntax)
	}
	pub fn literal(&self) -> Option<Literal> { support::child(&self.syntax) }
	pub fn ident(&self) -> Option<Ident> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpreadElement {
	pub(crate) syntax: SyntaxNode,
}
impl SpreadElement {
	pub fn dotdotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [...]) }
	pub fn element(&self) -> Option<Expr> { support::child(&self.syntax) }
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
pub struct SinglePattern {
	pub(crate) syntax: SyntaxNode,
}
impl SinglePattern {
	pub fn name(&self) -> Option<Ident> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RestPattern {
	pub(crate) syntax: SyntaxNode,
}
impl RestPattern {
	pub fn dotdotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [...]) }
	pub fn pa(&self) -> Option<Pattern> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LiteralProp {
	pub(crate) syntax: SyntaxNode,
}
impl LiteralProp {
	pub fn key(&self) -> Option<PropName> { support::child(&self.syntax) }
	pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [:]) }
	pub fn value(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComputedPropertyName {
	pub(crate) syntax: SyntaxNode,
}
impl ComputedPropertyName {
	pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
	pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
	pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrivateName {
	pub(crate) syntax: SyntaxNode,
}
impl PrivateName {
	pub fn hash_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [#]) }
	pub fn name(&self) -> Option<Name> { support::child(&self.syntax) }
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
	NameRef(NameRef),
	ThisExpr(ThisExpr),
	ArrayExpr(ArrayExpr),
	ObjectExpr(ObjectExpr),
	GroupingExpr(GroupingExpr),
	BracketExpr(BracketExpr),
	DotExpr(DotExpr),
	NewExpr(NewExpr),
	CallExpr(CallExpr),
	UnaryExpr(UnaryExpr),
	BinExpr(BinExpr),
	CondExpr(CondExpr),
	AssignExpr(AssignExpr),
	SequenceExpr(SequenceExpr),
	FnExpr(FnExpr),
	ClassExpr(ClassExpr),
	NewTarget(NewTarget),
	ImportMeta(ImportMeta),
	SuperCall(SuperCall),
	ImportCall(ImportCall),
	YieldExpr(YieldExpr),
	AwaitExpr(AwaitExpr),
	PrivatePropAccess(PrivatePropAccess),
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjectProp {
	LiteralProp(LiteralProp),
	Getter(Getter),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassElement {
	EmptyStmt(EmptyStmt),
	Method(Method),
	PrivateProp(PrivateProp),
	ClassProp(ClassProp),
	Constructor(Constructor),
	Getter(Getter),
	Setter(Setter),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprOrSpread {
	Expr(Expr),
	SpreadElement(SpreadElement),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternOrExpr {
	Pattern(Pattern),
	Expr(Expr),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
	SinglePattern(SinglePattern),
	RestPattern(RestPattern),
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
impl AstNode for NameRef {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NAME_REF }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ThisExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == THIS_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ArrayExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == ARRAY_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ObjectExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == OBJECT_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for GroupingExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == GROUPING_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BracketExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == BRACKET_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for DotExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == DOT_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for NewExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NEW_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for CallExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CALL_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for UnaryExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == UNARY_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BinExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == BIN_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for CondExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == COND_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for AssignExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == ASSIGN_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SequenceExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SEQUENCE_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for FnExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FN_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ClassExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CLASS_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for NewTarget {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NEW_TARGET }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ImportMeta {
	fn can_cast(kind: SyntaxKind) -> bool { kind == IMPORT_META }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SuperCall {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SUPER_CALL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ImportCall {
	fn can_cast(kind: SyntaxKind) -> bool { kind == IMPORT_CALL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for YieldExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == YIELD_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for AwaitExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == AWAIT_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PrivatePropAccess {
	fn can_cast(kind: SyntaxKind) -> bool { kind == PRIVATE_PROP_ACCESS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ArgList {
	fn can_cast(kind: SyntaxKind) -> bool { kind == ARG_LIST }
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
impl AstNode for ClassBody {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CLASS_BODY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Method {
	fn can_cast(kind: SyntaxKind) -> bool { kind == METHOD }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PrivateProp {
	fn can_cast(kind: SyntaxKind) -> bool { kind == PRIVATE_PROP }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ClassProp {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CLASS_PROP }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Constructor {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CONSTRUCTOR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Getter {
	fn can_cast(kind: SyntaxKind) -> bool { kind == GETTER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Setter {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SETTER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PropName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == PROP_NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SpreadElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SPREAD_ELEMENT }
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
impl AstNode for RestPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == REST_PATTERN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for LiteralProp {
	fn can_cast(kind: SyntaxKind) -> bool { kind == LITERAL_PROP }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ComputedPropertyName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == COMPUTED_PROPERTY_NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PrivateName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == PRIVATE_NAME }
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
impl From<NameRef> for Expr {
	fn from(node: NameRef) -> Expr { Expr::NameRef(node) }
}
impl From<ThisExpr> for Expr {
	fn from(node: ThisExpr) -> Expr { Expr::ThisExpr(node) }
}
impl From<ArrayExpr> for Expr {
	fn from(node: ArrayExpr) -> Expr { Expr::ArrayExpr(node) }
}
impl From<ObjectExpr> for Expr {
	fn from(node: ObjectExpr) -> Expr { Expr::ObjectExpr(node) }
}
impl From<GroupingExpr> for Expr {
	fn from(node: GroupingExpr) -> Expr { Expr::GroupingExpr(node) }
}
impl From<BracketExpr> for Expr {
	fn from(node: BracketExpr) -> Expr { Expr::BracketExpr(node) }
}
impl From<DotExpr> for Expr {
	fn from(node: DotExpr) -> Expr { Expr::DotExpr(node) }
}
impl From<NewExpr> for Expr {
	fn from(node: NewExpr) -> Expr { Expr::NewExpr(node) }
}
impl From<CallExpr> for Expr {
	fn from(node: CallExpr) -> Expr { Expr::CallExpr(node) }
}
impl From<UnaryExpr> for Expr {
	fn from(node: UnaryExpr) -> Expr { Expr::UnaryExpr(node) }
}
impl From<BinExpr> for Expr {
	fn from(node: BinExpr) -> Expr { Expr::BinExpr(node) }
}
impl From<CondExpr> for Expr {
	fn from(node: CondExpr) -> Expr { Expr::CondExpr(node) }
}
impl From<AssignExpr> for Expr {
	fn from(node: AssignExpr) -> Expr { Expr::AssignExpr(node) }
}
impl From<SequenceExpr> for Expr {
	fn from(node: SequenceExpr) -> Expr { Expr::SequenceExpr(node) }
}
impl From<FnExpr> for Expr {
	fn from(node: FnExpr) -> Expr { Expr::FnExpr(node) }
}
impl From<ClassExpr> for Expr {
	fn from(node: ClassExpr) -> Expr { Expr::ClassExpr(node) }
}
impl From<NewTarget> for Expr {
	fn from(node: NewTarget) -> Expr { Expr::NewTarget(node) }
}
impl From<ImportMeta> for Expr {
	fn from(node: ImportMeta) -> Expr { Expr::ImportMeta(node) }
}
impl From<SuperCall> for Expr {
	fn from(node: SuperCall) -> Expr { Expr::SuperCall(node) }
}
impl From<ImportCall> for Expr {
	fn from(node: ImportCall) -> Expr { Expr::ImportCall(node) }
}
impl From<YieldExpr> for Expr {
	fn from(node: YieldExpr) -> Expr { Expr::YieldExpr(node) }
}
impl From<AwaitExpr> for Expr {
	fn from(node: AwaitExpr) -> Expr { Expr::AwaitExpr(node) }
}
impl From<PrivatePropAccess> for Expr {
	fn from(node: PrivatePropAccess) -> Expr { Expr::PrivatePropAccess(node) }
}
impl AstNode for Expr {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			ARROW_EXPR
				| LITERAL | NAME_REF
				| THIS_EXPR | ARRAY_EXPR
				| OBJECT_EXPR | GROUPING_EXPR
				| BRACKET_EXPR | DOT_EXPR
				| NEW_EXPR | CALL_EXPR
				| UNARY_EXPR | BIN_EXPR
				| COND_EXPR | ASSIGN_EXPR
				| SEQUENCE_EXPR | FN_EXPR
				| CLASS_EXPR | NEW_TARGET
				| IMPORT_META | SUPER_CALL
				| IMPORT_CALL | YIELD_EXPR
				| AWAIT_EXPR | PRIVATE_PROP_ACCESS
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			ARROW_EXPR => Expr::ArrowExpr(ArrowExpr { syntax }),
			LITERAL => Expr::Literal(Literal { syntax }),
			NAME_REF => Expr::NameRef(NameRef { syntax }),
			THIS_EXPR => Expr::ThisExpr(ThisExpr { syntax }),
			ARRAY_EXPR => Expr::ArrayExpr(ArrayExpr { syntax }),
			OBJECT_EXPR => Expr::ObjectExpr(ObjectExpr { syntax }),
			GROUPING_EXPR => Expr::GroupingExpr(GroupingExpr { syntax }),
			BRACKET_EXPR => Expr::BracketExpr(BracketExpr { syntax }),
			DOT_EXPR => Expr::DotExpr(DotExpr { syntax }),
			NEW_EXPR => Expr::NewExpr(NewExpr { syntax }),
			CALL_EXPR => Expr::CallExpr(CallExpr { syntax }),
			UNARY_EXPR => Expr::UnaryExpr(UnaryExpr { syntax }),
			BIN_EXPR => Expr::BinExpr(BinExpr { syntax }),
			COND_EXPR => Expr::CondExpr(CondExpr { syntax }),
			ASSIGN_EXPR => Expr::AssignExpr(AssignExpr { syntax }),
			SEQUENCE_EXPR => Expr::SequenceExpr(SequenceExpr { syntax }),
			FN_EXPR => Expr::FnExpr(FnExpr { syntax }),
			CLASS_EXPR => Expr::ClassExpr(ClassExpr { syntax }),
			NEW_TARGET => Expr::NewTarget(NewTarget { syntax }),
			IMPORT_META => Expr::ImportMeta(ImportMeta { syntax }),
			SUPER_CALL => Expr::SuperCall(SuperCall { syntax }),
			IMPORT_CALL => Expr::ImportCall(ImportCall { syntax }),
			YIELD_EXPR => Expr::YieldExpr(YieldExpr { syntax }),
			AWAIT_EXPR => Expr::AwaitExpr(AwaitExpr { syntax }),
			PRIVATE_PROP_ACCESS => Expr::PrivatePropAccess(PrivatePropAccess { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Expr::ArrowExpr(it) => &it.syntax,
			Expr::Literal(it) => &it.syntax,
			Expr::NameRef(it) => &it.syntax,
			Expr::ThisExpr(it) => &it.syntax,
			Expr::ArrayExpr(it) => &it.syntax,
			Expr::ObjectExpr(it) => &it.syntax,
			Expr::GroupingExpr(it) => &it.syntax,
			Expr::BracketExpr(it) => &it.syntax,
			Expr::DotExpr(it) => &it.syntax,
			Expr::NewExpr(it) => &it.syntax,
			Expr::CallExpr(it) => &it.syntax,
			Expr::UnaryExpr(it) => &it.syntax,
			Expr::BinExpr(it) => &it.syntax,
			Expr::CondExpr(it) => &it.syntax,
			Expr::AssignExpr(it) => &it.syntax,
			Expr::SequenceExpr(it) => &it.syntax,
			Expr::FnExpr(it) => &it.syntax,
			Expr::ClassExpr(it) => &it.syntax,
			Expr::NewTarget(it) => &it.syntax,
			Expr::ImportMeta(it) => &it.syntax,
			Expr::SuperCall(it) => &it.syntax,
			Expr::ImportCall(it) => &it.syntax,
			Expr::YieldExpr(it) => &it.syntax,
			Expr::AwaitExpr(it) => &it.syntax,
			Expr::PrivatePropAccess(it) => &it.syntax,
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
impl From<LiteralProp> for ObjectProp {
	fn from(node: LiteralProp) -> ObjectProp { ObjectProp::LiteralProp(node) }
}
impl From<Getter> for ObjectProp {
	fn from(node: Getter) -> ObjectProp { ObjectProp::Getter(node) }
}
impl AstNode for ObjectProp {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, LITERAL_PROP | GETTER) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			LITERAL_PROP => ObjectProp::LiteralProp(LiteralProp { syntax }),
			GETTER => ObjectProp::Getter(Getter { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ObjectProp::LiteralProp(it) => &it.syntax,
			ObjectProp::Getter(it) => &it.syntax,
		}
	}
}
impl From<EmptyStmt> for ClassElement {
	fn from(node: EmptyStmt) -> ClassElement { ClassElement::EmptyStmt(node) }
}
impl From<Method> for ClassElement {
	fn from(node: Method) -> ClassElement { ClassElement::Method(node) }
}
impl From<PrivateProp> for ClassElement {
	fn from(node: PrivateProp) -> ClassElement { ClassElement::PrivateProp(node) }
}
impl From<ClassProp> for ClassElement {
	fn from(node: ClassProp) -> ClassElement { ClassElement::ClassProp(node) }
}
impl From<Constructor> for ClassElement {
	fn from(node: Constructor) -> ClassElement { ClassElement::Constructor(node) }
}
impl From<Getter> for ClassElement {
	fn from(node: Getter) -> ClassElement { ClassElement::Getter(node) }
}
impl From<Setter> for ClassElement {
	fn from(node: Setter) -> ClassElement { ClassElement::Setter(node) }
}
impl AstNode for ClassElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			EMPTY_STMT | METHOD | PRIVATE_PROP | CLASS_PROP | CONSTRUCTOR | GETTER | SETTER
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			EMPTY_STMT => ClassElement::EmptyStmt(EmptyStmt { syntax }),
			METHOD => ClassElement::Method(Method { syntax }),
			PRIVATE_PROP => ClassElement::PrivateProp(PrivateProp { syntax }),
			CLASS_PROP => ClassElement::ClassProp(ClassProp { syntax }),
			CONSTRUCTOR => ClassElement::Constructor(Constructor { syntax }),
			GETTER => ClassElement::Getter(Getter { syntax }),
			SETTER => ClassElement::Setter(Setter { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ClassElement::EmptyStmt(it) => &it.syntax,
			ClassElement::Method(it) => &it.syntax,
			ClassElement::PrivateProp(it) => &it.syntax,
			ClassElement::ClassProp(it) => &it.syntax,
			ClassElement::Constructor(it) => &it.syntax,
			ClassElement::Getter(it) => &it.syntax,
			ClassElement::Setter(it) => &it.syntax,
		}
	}
}
impl From<Expr> for ExprOrSpread {
	fn from(node: Expr) -> ExprOrSpread { ExprOrSpread::Expr(node) }
}
impl From<SpreadElement> for ExprOrSpread {
	fn from(node: SpreadElement) -> ExprOrSpread { ExprOrSpread::SpreadElement(node) }
}
impl AstNode for ExprOrSpread {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, EXPR | SPREAD_ELEMENT) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			EXPR => ExprOrSpread::Expr(Expr { syntax }),
			SPREAD_ELEMENT => ExprOrSpread::SpreadElement(SpreadElement { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ExprOrSpread::Expr(it) => &it.syntax,
			ExprOrSpread::SpreadElement(it) => &it.syntax,
		}
	}
}
impl From<Pattern> for PatternOrExpr {
	fn from(node: Pattern) -> PatternOrExpr { PatternOrExpr::Pattern(node) }
}
impl From<Expr> for PatternOrExpr {
	fn from(node: Expr) -> PatternOrExpr { PatternOrExpr::Expr(node) }
}
impl AstNode for PatternOrExpr {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, PATTERN | EXPR) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			PATTERN => PatternOrExpr::Pattern(Pattern { syntax }),
			EXPR => PatternOrExpr::Expr(Expr { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			PatternOrExpr::Pattern(it) => &it.syntax,
			PatternOrExpr::Expr(it) => &it.syntax,
		}
	}
}
impl From<SinglePattern> for Pattern {
	fn from(node: SinglePattern) -> Pattern { Pattern::SinglePattern(node) }
}
impl From<RestPattern> for Pattern {
	fn from(node: RestPattern) -> Pattern { Pattern::RestPattern(node) }
}
impl AstNode for Pattern {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, SINGLE_PATTERN | REST_PATTERN) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			SINGLE_PATTERN => Pattern::SinglePattern(SinglePattern { syntax }),
			REST_PATTERN => Pattern::RestPattern(RestPattern { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Pattern::SinglePattern(it) => &it.syntax,
			Pattern::RestPattern(it) => &it.syntax,
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
impl std::fmt::Display for ObjectProp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ClassElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExprOrSpread {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PatternOrExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Pattern {
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
impl std::fmt::Display for NameRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ThisExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ArrayExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ObjectExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for GroupingExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BracketExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DotExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NewExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CallExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for UnaryExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for BinExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CondExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AssignExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SequenceExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for FnExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ClassExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NewTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImportMeta {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SuperCall {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ImportCall {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for YieldExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for AwaitExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PrivatePropAccess {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ArgList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Name {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeParams {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ParameterList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ClassBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Method {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PrivateProp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ClassProp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Constructor {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Getter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Setter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PropName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for SpreadElement {
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
impl std::fmt::Display for SinglePattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for RestPattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for LiteralProp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ComputedPropertyName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for PrivateName {
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
