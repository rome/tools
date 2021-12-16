//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{ast::*, JsLanguage, SyntaxKind::*, T};
use rome_rowan::AstTreeShape;
impl AstTreeShape for JsLanguage {
	fn fits_shape_of(
		kind: &Self::Kind,
		actual_len: usize,
		mut actual_kinds: impl Iterator<Item = Option<Self::Kind>>,
	) -> bool {
		match kind {
			JS_UNKNOWN
			| JS_UNKNOWN_ASSIGNMENT
			| JS_UNKNOWN_BINDING
			| JS_UNKNOWN_EXPRESSION
			| JS_UNKNOWN_IMPORT_ASSERTION_ENTRY
			| JS_UNKNOWN_MEMBER
			| JS_UNKNOWN_MODIFIER
			| JS_UNKNOWN_NAMED_IMPORT_SPECIFIER
			| JS_UNKNOWN_STATEMENT
			| ERROR => true,
			CALL_EXPR => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeArgs::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsCallArguments::can_cast) == Some(false) {
					return false;
				}
				true
			}
			EXPORT_DECL => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![export])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![type])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyExportDeclaration::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			EXPORT_DEFAULT_DECL => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![export])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![default])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![type])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(DefaultDecl::can_cast) == Some(false) {
					return false;
				}
				true
			}
			EXPORT_DEFAULT_EXPR => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![export])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![type])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![default])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			EXPORT_NAMED => {
				if actual_len != 6usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(ExportNamedSpecifierList::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![from])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == JS_STRING_LITERAL)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			EXPORT_WILDCARD => {
				if actual_len != 7usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![export])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![type])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [*]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![as]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![from])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == JS_STRING_LITERAL)
					== Some(false)
				{
					return false;
				}
				true
			}
			FOR_STMT => {
				if actual_len != 9usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![for]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyForInitializer::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(ForStmtTest::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(ForStmtUpdate::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyStatement::can_cast) == Some(false) {
					return false;
				}
				true
			}
			FOR_STMT_TEST => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			FOR_STMT_UPDATE => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			IDENT => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == IDENT) == Some(false) {
					return false;
				}
				true
			}
			IMPORT_META => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![import])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [.]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![meta])
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_ARRAY_ASSIGNMENT_PATTERN => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['[']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsArrayAssignmentPatternElementList::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![']']) == Some(false) {
					return false;
				}
				true
			}
			JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T ! [...])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyAssignmentPattern::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_ARRAY_BINDING_PATTERN => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['[']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsArrayBindingPatternElementList::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![']']) == Some(false) {
					return false;
				}
				true
			}
			JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T ! [...])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyBindingPattern::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_ARRAY_EXPRESSION => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['[']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsArrayElementList::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![']']) == Some(false) {
					return false;
				}
				true
			}
			JS_ARRAY_HOLE => {
				if actual_len != 0usize {
					return false;
				}
				true
			}
			JS_ARROW_FUNCTION_EXPRESSION => {
				if actual_len != 6usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![async])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeParams::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyArrowFunctionParameters::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeAnnotation::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T ! [=>])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyArrowFunctionBody::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_ASSIGNMENT_EXPRESSION => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyAssignmentPattern::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| {
					matches!(
						actual,
						T ! [=]
							| T ! [+=] | T ! [-=] | T ! [*=]
							| T ! [/=] | T ! [%=] | T ! [**=]
							| T ! [>>=] | T ! [<<=] | T ! [>>>=]
							| T ! [&=] | T ! [|=] | T ! [^=]
							| T ! [&&=] | T ! [||=] | T ! [??=]
					)
				}) == Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_ASSIGNMENT_WITH_DEFAULT => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyAssignmentPattern::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [=]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_AWAIT_EXPRESSION => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![await])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_BIG_INT_LITERAL_EXPRESSION => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == JS_BIG_INT_LITERAL)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_BINARY_EXPRESSION => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| {
					matches!(
						actual,
						T ! [<]
							| T ! [>] | T ! [<=] | T ! [>=]
							| T ! [==] | T ! [===] | T ! [!=]
							| T ! [!==] | T ! [+] | T ! [-]
							| T ! [*] | T ! [/] | T ! [%]
							| T ! [**] | T ! [<<] | T ! [>>]
							| T ! [>>>] | T ! [&] | T ! [|]
							| T ! [^] | T![in] | T![instanceof]
					)
				}) == Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_BINDING_PATTERN_WITH_DEFAULT => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyBindingPattern::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [=]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_BLOCK_STATEMENT => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsStatementList::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			JS_BOOLEAN_LITERAL_EXPRESSION => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, T![true] | T![false]))
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_BREAK_STATEMENT => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![break])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == IDENT) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			JS_CALL_ARGUMENTS => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsCallArgumentList::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				true
			}
			JS_CASE_CLAUSE => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![case])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsStatementList::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_CATCH_CLAUSE => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![catch])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsCatchDeclaration::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsBlockStatement::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_CATCH_DECLARATION => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyBindingPattern::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				true
			}
			JS_CLASS_DECLARATION => {
				if actual_len != 7usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![class])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyBinding::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsExtendsClause::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(TsImplementsClause::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsClassMemberList::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			JS_CLASS_EXPRESSION => {
				if actual_len != 7usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![class])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyBinding::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsExtendsClause::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(TsImplementsClause::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsClassMemberList::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			JS_COMPUTED_MEMBER_ASSIGNMENT => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['[']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![']']) == Some(false) {
					return false;
				}
				true
			}
			JS_COMPUTED_MEMBER_EXPRESSION => {
				if actual_len != 5usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T ! [?.])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['[']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![']']) == Some(false) {
					return false;
				}
				true
			}
			JS_COMPUTED_MEMBER_NAME => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['[']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![']']) == Some(false) {
					return false;
				}
				true
			}
			JS_CONDITIONAL_EXPRESSION => {
				if actual_len != 5usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [?]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_CONSTRUCTOR_CLASS_MEMBER => {
				if actual_len != 5usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, T![private] | T![protected] | T![public]))
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsLiteralMemberName::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeParams::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsConstructorParameters::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsFunctionBody::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_CONSTRUCTOR_PARAMETERS => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsConstructorParameterList::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				true
			}
			JS_CONTINUE_STATEMENT => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![continue])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == IDENT) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			JS_DEBUGGER_STATEMENT => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![debugger])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			JS_DEFAULT_CLAUSE => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![default])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsStatementList::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_DEFAULT_IMPORT_SPECIFIER => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyBinding::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [,]) == Some(false) {
					return false;
				}
				true
			}
			JS_DIRECTIVE => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == JS_STRING_LITERAL)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			JS_DO_WHILE_STATEMENT => {
				if actual_len != 7usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![do]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyStatement::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![while])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			JS_ELSE_CLAUSE => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![else])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyStatement::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_EMPTY_CLASS_MEMBER => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			JS_EMPTY_STATEMENT => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			JS_EXPRESSION_SNIPPED => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![EOF]) == Some(false) {
					return false;
				}
				true
			}
			JS_EXPRESSION_STATEMENT => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			JS_EXTENDS_CLAUSE => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![extends])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_FINALLY_CLAUSE => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![finally])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsBlockStatement::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_FOR_IN_STATEMENT => {
				if actual_len != 7usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![for]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyForInOrOfInitializer::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![in]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyStatement::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_FOR_OF_STATEMENT => {
				if actual_len != 8usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![for]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![await])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyForInOrOfInitializer::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![of]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyStatement::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_FOR_VARIABLE_DECLARATION => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, T![var] | T![let] | T![const]))
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsVariableDeclaration::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_FUNCTION_BODY => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsDirectiveList::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsStatementList::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			JS_FUNCTION_DECLARATION => {
				if actual_len != 8usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![async])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![function])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [*]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyBinding::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeParams::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsParameters::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeAnnotation::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsFunctionBody::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_FUNCTION_EXPRESSION => {
				if actual_len != 8usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![async])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![function])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [*]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyBinding::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeParams::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsParameters::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeAnnotation::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsFunctionBody::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_GETTER_CLASS_MEMBER => {
				if actual_len != 9usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, T![private] | T![protected] | T![public]))
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![static])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![abstract])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![get]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyClassMemberName::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeAnnotation::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsFunctionBody::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_GETTER_OBJECT_MEMBER => {
				if actual_len != 6usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![get]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyObjectMemberName::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeAnnotation::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsFunctionBody::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_IDENTIFIER_ASSIGNMENT => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == IDENT) == Some(false) {
					return false;
				}
				true
			}
			JS_IDENTIFIER_BINDING => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == IDENT) == Some(false) {
					return false;
				}
				true
			}
			JS_IDENTIFIER_EXPRESSION => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsReferenceIdentifier::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_IF_STATEMENT => {
				if actual_len != 6usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![if]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyStatement::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsElseClause::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_IMPORT => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![import])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(AnyJsImportClause::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			JS_IMPORT_ASSERTION => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![assert])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsImportAssertionEntryList::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			JS_IMPORT_ASSERTION_ENTRY => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, IDENT | JS_STRING_LITERAL))
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == JS_STRING_LITERAL)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_IMPORT_BARE_CLAUSE => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsModuleSource::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsImportAssertion::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_IMPORT_CALL_EXPRESSION => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![import])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				true
			}
			JS_IMPORT_DEFAULT_CLAUSE => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyBinding::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![from])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsModuleSource::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsImportAssertion::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_IMPORT_NAMED_CLAUSE => {
				if actual_len != 5usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsDefaultImportSpecifier::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyNamedImport::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![from])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsModuleSource::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsImportAssertion::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_IMPORT_NAMESPACE_CLAUSE => {
				if actual_len != 6usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [*]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![as]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyBinding::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![from])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsModuleSource::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsImportAssertion::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_INITIALIZER_CLAUSE => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [=]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_LABELED_STATEMENT => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == IDENT) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyStatement::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_LITERAL_EXPORT_NAME => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, IDENT | JS_STRING_LITERAL))
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_LITERAL_MEMBER_NAME => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, IDENT | JS_STRING_LITERAL | JS_NUMBER_LITERAL))
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_LOGICAL_EXPRESSION => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, T ! [??] | T ! [||] | T ! [&&]))
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_METHOD_CLASS_MEMBER => {
				if actual_len != 10usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, T![private] | T![protected] | T![public]))
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![static])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![abstract])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![async])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [*]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyClassMemberName::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeParams::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsParameters::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeAnnotation::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsFunctionBody::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_METHOD_OBJECT_MEMBER => {
				if actual_len != 7usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![async])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [*]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyObjectMemberName::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeParams::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsParameters::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeAnnotation::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsFunctionBody::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_MODULE => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == JS_SHEBANG)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsDirectiveList::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsModuleItemList::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![EOF]) == Some(false) {
					return false;
				}
				true
			}
			JS_MODULE_SOURCE => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == JS_STRING_LITERAL)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_NAME => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == IDENT) == Some(false) {
					return false;
				}
				true
			}
			JS_NAMED_IMPORT_SPECIFIER => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsLiteralExportName::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![as]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyBinding::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_NAMED_IMPORT_SPECIFIERS => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsNamedImportSpecifierList::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			JS_NAMESPACE_IMPORT_SPECIFIER => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [*]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![as]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyBinding::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_NULL_LITERAL_EXPRESSION => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![null])
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_NUMBER_LITERAL_EXPRESSION => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == JS_NUMBER_LITERAL)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_OBJECT_ASSIGNMENT_PATTERN => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsObjectAssignmentPatternPropertyList::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsName::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyAssignmentPattern::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsInitializerClause::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_REST => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T ! [...])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyAssignment::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyAssignment::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsInitializerClause::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_OBJECT_BINDING_PATTERN => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsObjectBindingPatternPropertyList::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			JS_OBJECT_BINDING_PATTERN_PROPERTY => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyObjectMemberName::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyBindingPattern::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsInitializerClause::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_OBJECT_BINDING_PATTERN_REST => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T ! [...])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyBinding::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyBinding::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsInitializerClause::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_OBJECT_EXPRESSION => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsObjectMemberList::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			JS_PARAMETERS => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsParameterList::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				true
			}
			JS_PARENTHESIZED_ASSIGNMENT => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyAssignment::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				true
			}
			JS_PARENTHESIZED_EXPRESSION => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				true
			}
			JS_POST_UPDATE_EXPRESSION => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyAssignment::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, T ! [++] | T ! [--]))
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_PRE_UPDATE_EXPRESSION => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, T ! [++] | T ! [--]))
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyAssignment::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_PRIVATE_CLASS_MEMBER_NAME => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [#]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == IDENT) == Some(false) {
					return false;
				}
				true
			}
			JS_PRIVATE_NAME => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [#]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == IDENT) == Some(false) {
					return false;
				}
				true
			}
			JS_PROPERTY_CLASS_MEMBER => {
				if actual_len != 11usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![declare])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, T![private] | T![protected] | T![public]))
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![static])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![readonly])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![abstract])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyClassMemberName::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [?]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![!]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeAnnotation::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsInitializerClause::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			JS_PROPERTY_OBJECT_MEMBER => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyObjectMemberName::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_REFERENCE_IDENTIFIER => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == IDENT) == Some(false) {
					return false;
				}
				true
			}
			JS_REGEX_LITERAL_EXPRESSION => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == JS_REGEX_LITERAL)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_REST_PARAMETER => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T ! [...])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyBindingPattern::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_RETURN_STATEMENT => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![return])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			JS_SCRIPT => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == JS_SHEBANG)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsDirectiveList::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsStatementList::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![EOF]) == Some(false) {
					return false;
				}
				true
			}
			JS_SEQUENCE_EXPRESSION => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [,]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_SETTER_CLASS_MEMBER => {
				if actual_len != 9usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, T![private] | T![protected] | T![public]))
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![static])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![abstract])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![set]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyClassMemberName::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyBindingPattern::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsFunctionBody::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_SETTER_OBJECT_MEMBER => {
				if actual_len != 6usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![set]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyObjectMemberName::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyBindingPattern::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsFunctionBody::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyBinding::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsReferenceIdentifier::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_SPREAD => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T ! [...])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_STATIC_MEMBER_ASSIGNMENT => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [.]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyName::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_STATIC_MEMBER_EXPRESSION => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, T ! [.] | T ! [?.]))
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyName::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_STRING_LITERAL_EXPRESSION => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == JS_STRING_LITERAL)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_SUPER_EXPRESSION => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![super])
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_SWITCH_STATEMENT => {
				if actual_len != 7usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![switch])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsSwitchCaseList::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			JS_THIS_EXPRESSION => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![this])
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_THROW_STATEMENT => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![throw])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			JS_TRY_FINALLY_STATEMENT => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![try]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsBlockStatement::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsCatchClause::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsFinallyClause::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_TRY_STATEMENT => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![try]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsBlockStatement::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsCatchClause::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_UNARY_EXPRESSION => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| {
					matches!(
						actual,
						T![delete] | T![void] | T![typeof] | T ! [+] | T ! [-] | T ! [~] | T![!]
					)
				}) == Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_VARIABLE_DECLARATION => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyBindingPattern::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![!]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeAnnotation::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsInitializerClause::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_VARIABLE_DECLARATIONS => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, T![var] | T![const] | T![let]))
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsVariableDeclarationList::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			JS_VARIABLE_STATEMENT => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsVariableDeclarations::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			JS_WHILE_STATEMENT => {
				if actual_len != 5usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![while])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyStatement::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_WITH_STATEMENT => {
				if actual_len != 5usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![with])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyStatement::can_cast) == Some(false) {
					return false;
				}
				true
			}
			JS_YIELD_EXPRESSION => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![yield])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [*]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			NEW_EXPR => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![new]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeArgs::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsCallArguments::can_cast) == Some(false) {
					return false;
				}
				true
			}
			NEW_TARGET => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![new]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [.]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![target])
					== Some(false)
				{
					return false;
				}
				true
			}
			SPECIFIER => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsName::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![as]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsName::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TEMPLATE => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['`']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(TemplateElementList::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['`']) == Some(false) {
					return false;
				}
				true
			}
			TEMPLATE_CHUNK_ELEMENT => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == TEMPLATE_CHUNK)
					== Some(false)
				{
					return false;
				}
				true
			}
			TEMPLATE_ELEMENT => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == DOLLAR_CURLY)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			TS_ANY => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![any]) == Some(false) {
					return false;
				}
				true
			}
			TS_ARRAY => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['[']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![']']) == Some(false) {
					return false;
				}
				true
			}
			TS_ASSERTION => {
				if actual_len != 5usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [<]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [>]) == Some(false) {
					return false;
				}
				true
			}
			TS_BIGINT => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_BOOLEAN => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_CALL_SIGNATURE_DECL => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeParams::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsParameters::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_CONDITIONAL_TYPE => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [?]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsExtends::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_CONST_ASSERTION => {
				if actual_len != 5usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [<]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![const])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [>]) == Some(false) {
					return false;
				}
				true
			}
			TS_CONSTRAINT => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![extends])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_CONSTRUCT_SIGNATURE_DECL => {
				if actual_len != 5usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![new]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeParams::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsParameters::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_CONSTRUCTOR_PARAM => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| matches!(actual, T![private] | T![protected] | T![public]))
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![readonly])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(JsAnyBindingPattern::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			TS_CONSTRUCTOR_TYPE => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![new]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsParameters::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_DEFAULT => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [=]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_ENUM => {
				if actual_len != 6usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![const])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![enum])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsEnumMemberList::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			TS_ENUM_MEMBER => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [=]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_EXPORT_ASSIGNMENT => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![export])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [=]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			TS_EXPR_WITH_TYPE_ARGS => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsEntityName::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeArgs::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_EXTENDS => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![extends])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_EXTERNAL_MODULE_REF => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![require])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == JS_STRING_LITERAL)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				true
			}
			TS_FN_TYPE => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsParameters::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T ! [=>])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_IMPLEMENTS_CLAUSE => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![implements])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeList::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_IMPORT => {
				if actual_len != 6usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![import])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeArgs::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [.]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsEntityName::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				true
			}
			TS_IMPORT_EQUALS_DECL => {
				if actual_len != 6usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![import])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![export])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [=]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsModuleRef::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			TS_INDEX_SIGNATURE => {
				if actual_len != 6usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![readonly])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['[']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyBinding::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![']']) == Some(false) {
					return false;
				}
				true
			}
			TS_INDEXED_ARRAY => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['[']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![']']) == Some(false) {
					return false;
				}
				true
			}
			TS_INFER => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![infer])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_INTERFACE_DECL => {
				if actual_len != 8usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![declare])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![interface])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeParams::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![extends])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(TsExprWithTypeArgs::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeElement::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			TS_INTERSECTION => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeList::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_LITERAL => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_MAPPED_TYPE => {
				if actual_len != 10usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(TsMappedTypeReadonly::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [-]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [+]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [?]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(TsMappedTypeParam::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			TS_MAPPED_TYPE_PARAM => {
				if actual_len != 5usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['[']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeName::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![']']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_MAPPED_TYPE_READONLY => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [-]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [+]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![readonly])
					== Some(false)
				{
					return false;
				}
				true
			}
			TS_METHOD_SIGNATURE => {
				if actual_len != 7usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![readonly])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeParams::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsParameters::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [?]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_MODULE_BLOCK => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyStatement::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			TS_MODULE_DECL => {
				if actual_len != 6usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![declare])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![global])
					== Some(false)
				{
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![module])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [.]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsNamespaceBody::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_NAMESPACE_DECL => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![declare])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [.]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsNamespaceBody::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_NAMESPACE_EXPORT_DECL => {
				if actual_len != 5usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![export])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![as]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![namespace])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [;]) == Some(false) {
					return false;
				}
				true
			}
			TS_NEVER => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![never])
					== Some(false)
				{
					return false;
				}
				true
			}
			TS_NON_NULL => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![!]) == Some(false) {
					return false;
				}
				true
			}
			TS_NULL => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![null])
					== Some(false)
				{
					return false;
				}
				true
			}
			TS_NUMBER => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_OBJECT => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_OBJECT_TYPE => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['{']) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(TsObjectMemberList::can_cast)
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			TS_PAREN => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['(']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![')']) == Some(false) {
					return false;
				}
				true
			}
			TS_PREDICATE => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsThisOrMore::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_PROPERTY_SIGNATURE => {
				if actual_len != 5usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![readonly])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(JsAnyExpression::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [?]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_QUALIFIED_PATH => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsEntityName::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [.]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeName::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_STRING => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_SYMBOL => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_TEMPLATE => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(TsTemplateElement::can_cast)
					== Some(false)
				{
					return false;
				}
				true
			}
			TS_TEMPLATE_ELEMENT => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['}']) == Some(false) {
					return false;
				}
				true
			}
			TS_THIS => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![this])
					== Some(false)
				{
					return false;
				}
				true
			}
			TS_TUPLE => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T!['[']) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTupleElement::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T![']']) == Some(false) {
					return false;
				}
				true
			}
			TS_TUPLE_ELEMENT => {
				if actual_len != 5usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [?]) == Some(false) {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T ! [...])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_TYPE_ALIAS_DECL => {
				if actual_len != 4usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![type])
					== Some(false)
				{
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeParams::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [=]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_TYPE_ANNOTATION => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [:]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_TYPE_ARGS => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [<]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeArgList::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [>]) == Some(false) {
					return false;
				}
				true
			}
			TS_TYPE_NAME => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_TYPE_OPERATOR => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsType::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_TYPE_PARAM => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(Ident::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsConstraint::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsDefault::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_TYPE_PARAMS => {
				if actual_len != 3usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [<]) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeParam::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(|actual| actual == T ! [>]) == Some(false) {
					return false;
				}
				true
			}
			TS_TYPE_REF => {
				if actual_len != 2usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsEntityName::can_cast) == Some(false) {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeArgs::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_UNDEFINED => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![undefined])
					== Some(false)
				{
					return false;
				}
				true
			}
			TS_UNION => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds.next().unwrap().map(TsTypeList::can_cast) == Some(false) {
					return false;
				}
				true
			}
			TS_UNKNOWN => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![unknown])
					== Some(false)
				{
					return false;
				}
				true
			}
			TS_VOID => {
				if actual_len != 1usize {
					return false;
				}
				if actual_kinds
					.next()
					.unwrap()
					.map(|actual| actual == T![void])
					== Some(false)
				{
					return false;
				}
				true
			}
			EXPORT_NAMED_SPECIFIER_LIST => {
				Self::fits_separated_list_shape(Specifier::can_cast, T ! [,], true, actual_kinds)
			}
			JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST => Self::fits_separated_list_shape(
				JsAnyArrayAssignmentPatternElement::can_cast,
				T ! [,],
				true,
				actual_kinds,
			),
			JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST => Self::fits_separated_list_shape(
				JsAnyArrayBindingPatternElement::can_cast,
				T ! [,],
				true,
				actual_kinds,
			),
			JS_ARRAY_ELEMENT_LIST => Self::fits_separated_list_shape(
				JsAnyArrayElement::can_cast,
				T ! [,],
				true,
				actual_kinds,
			),
			JS_CALL_ARGUMENT_LIST => Self::fits_separated_list_shape(
				JsAnyExpression::can_cast,
				T ! [,],
				true,
				actual_kinds,
			),
			JS_CLASS_MEMBER_LIST => Self::fits_list_shape(JsAnyClassMember::can_cast, actual_kinds),
			JS_CONSTRUCTOR_PARAMETER_LIST => Self::fits_separated_list_shape(
				JsAnyConstructorParameter::can_cast,
				T ! [,],
				true,
				actual_kinds,
			),
			JS_DIRECTIVE_LIST => Self::fits_list_shape(JsDirective::can_cast, actual_kinds),
			JS_IMPORT_ASSERTION_ENTRY_LIST => Self::fits_separated_list_shape(
				JsAnyImportAssertionEntry::can_cast,
				T ! [,],
				true,
				actual_kinds,
			),
			JS_MODULE_ITEM_LIST => Self::fits_list_shape(JsAnyModuleItem::can_cast, actual_kinds),
			JS_NAMED_IMPORT_SPECIFIER_LIST => Self::fits_separated_list_shape(
				JsAnyNamedImportSpecifier::can_cast,
				T ! [,],
				true,
				actual_kinds,
			),
			JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST => Self::fits_separated_list_shape(
				JsAnyObjectAssignmentPatternMember::can_cast,
				T ! [,],
				true,
				actual_kinds,
			),
			JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST => Self::fits_separated_list_shape(
				JsAnyObjectBindingPatternMember::can_cast,
				T ! [,],
				true,
				actual_kinds,
			),
			JS_OBJECT_MEMBER_LIST => Self::fits_separated_list_shape(
				JsAnyObjectMember::can_cast,
				T ! [,],
				true,
				actual_kinds,
			),
			JS_PARAMETER_LIST => Self::fits_separated_list_shape(
				JsAnyParameter::can_cast,
				T ! [,],
				true,
				actual_kinds,
			),
			JS_STATEMENT_LIST => Self::fits_list_shape(JsAnyStatement::can_cast, actual_kinds),
			JS_SWITCH_CASE_LIST => Self::fits_list_shape(JsAnySwitchClause::can_cast, actual_kinds),
			JS_VARIABLE_DECLARATION_LIST => Self::fits_separated_list_shape(
				JsVariableDeclaration::can_cast,
				T ! [,],
				false,
				actual_kinds,
			),
			TEMPLATE_ELEMENT_LIST => {
				Self::fits_list_shape(AnyTemplateElement::can_cast, actual_kinds)
			}
			TS_ENUM_MEMBER_LIST => Self::fits_list_shape(TsEnumMember::can_cast, actual_kinds),
			TS_OBJECT_MEMBER_LIST => Self::fits_list_shape(TsTypeElement::can_cast, actual_kinds),
			TS_TYPE_ARG_LIST => {
				Self::fits_separated_list_shape(TsType::can_cast, T ! [,], false, actual_kinds)
			}
			TS_TYPE_LIST => Self::fits_separated_list_shape(
				TsExprWithTypeArgs::can_cast,
				T ! [,],
				false,
				actual_kinds,
			),
			TS_TYPE_PARAM_LIST => {
				Self::fits_separated_list_shape(TsTypeParam::can_cast, T ! [,], false, actual_kinds)
			}
			_ => unreachable!("Is {:?} a token?", kind),
		}
	}
}
