//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{ast::*, JsLanguage, SyntaxKind::*, T};
use rome_rowan::AstTreeShape;
impl AstTreeShape for JsLanguage {
	fn validate_slot(parent: Self::Kind, index: usize, value: Option<Self::Kind>) -> bool {
		match parent {
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
			CALL_EXPR => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				1usize => value.is_none() || TsTypeArgs::can_cast(value.unwrap()),
				2usize => value.is_none() || JsCallArguments::can_cast(value.unwrap()),
				_ => false,
			},
			EXPORT_DECL => match index {
				0usize => value.is_none() || value.unwrap() == T![export],
				1usize => value.is_none() || value.unwrap() == T![type],
				2usize => value.is_none() || JsAnyExportDeclaration::can_cast(value.unwrap()),
				_ => false,
			},
			EXPORT_DEFAULT_DECL => match index {
				0usize => value.is_none() || value.unwrap() == T![export],
				1usize => value.is_none() || value.unwrap() == T![default],
				2usize => value.is_none() || value.unwrap() == T![type],
				3usize => value.is_none() || DefaultDecl::can_cast(value.unwrap()),
				_ => false,
			},
			EXPORT_DEFAULT_EXPR => match index {
				0usize => value.is_none() || value.unwrap() == T![export],
				1usize => value.is_none() || value.unwrap() == T![type],
				2usize => value.is_none() || value.unwrap() == T![default],
				3usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			EXPORT_NAMED => match index {
				0usize => value.is_none() || value.unwrap() == T!['{'],
				1usize => value.is_none() || ExportNamedSpecifierList::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T!['}'],
				3usize => value.is_none() || value.unwrap() == T![from],
				4usize => value.is_none() || value.unwrap() == JS_STRING_LITERAL,
				5usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			EXPORT_WILDCARD => match index {
				0usize => value.is_none() || value.unwrap() == T![export],
				1usize => value.is_none() || value.unwrap() == T![type],
				2usize => value.is_none() || value.unwrap() == T ! [*],
				3usize => value.is_none() || value.unwrap() == T![as],
				4usize => value.is_none() || Ident::can_cast(value.unwrap()),
				5usize => value.is_none() || value.unwrap() == T![from],
				6usize => value.is_none() || value.unwrap() == JS_STRING_LITERAL,
				_ => false,
			},
			FOR_STMT => match index {
				0usize => value.is_none() || value.unwrap() == T![for],
				1usize => value.is_none() || value.unwrap() == T!['('],
				2usize => value.is_none() || JsAnyForInitializer::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T ! [;],
				4usize => value.is_none() || ForStmtTest::can_cast(value.unwrap()),
				5usize => value.is_none() || value.unwrap() == T ! [;],
				6usize => value.is_none() || ForStmtUpdate::can_cast(value.unwrap()),
				7usize => value.is_none() || value.unwrap() == T![')'],
				8usize => value.is_none() || JsAnyStatement::can_cast(value.unwrap()),
				_ => false,
			},
			FOR_STMT_TEST => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			FOR_STMT_UPDATE => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			IDENT => match index {
				0usize => value.is_none() || value.unwrap() == IDENT,
				_ => false,
			},
			IMPORT_META => match index {
				0usize => value.is_none() || value.unwrap() == T![import],
				1usize => value.is_none() || value.unwrap() == T ! [.],
				2usize => value.is_none() || value.unwrap() == T![meta],
				_ => false,
			},
			JS_ARRAY_ASSIGNMENT_PATTERN => match index {
				0usize => value.is_none() || value.unwrap() == T!['['],
				1usize => {
					value.is_none() || JsArrayAssignmentPatternElementList::can_cast(value.unwrap())
				}
				2usize => value.is_none() || value.unwrap() == T![']'],
				_ => false,
			},
			JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T ! [...],
				1usize => value.is_none() || JsAnyAssignmentPattern::can_cast(value.unwrap()),
				_ => false,
			},
			JS_ARRAY_BINDING_PATTERN => match index {
				0usize => value.is_none() || value.unwrap() == T!['['],
				1usize => {
					value.is_none() || JsArrayBindingPatternElementList::can_cast(value.unwrap())
				}
				2usize => value.is_none() || value.unwrap() == T![']'],
				_ => false,
			},
			JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T ! [...],
				1usize => value.is_none() || JsAnyBindingPattern::can_cast(value.unwrap()),
				_ => false,
			},
			JS_ARRAY_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == T!['['],
				1usize => value.is_none() || JsArrayElementList::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T![']'],
				_ => false,
			},
			JS_ARRAY_HOLE => false,
			JS_ARROW_FUNCTION_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == T![async],
				1usize => value.is_none() || TsTypeParams::can_cast(value.unwrap()),
				2usize => value.is_none() || JsAnyArrowFunctionParameters::can_cast(value.unwrap()),
				3usize => value.is_none() || TsTypeAnnotation::can_cast(value.unwrap()),
				4usize => value.is_none() || value.unwrap() == T ! [=>],
				5usize => value.is_none() || JsAnyArrowFunctionBody::can_cast(value.unwrap()),
				_ => false,
			},
			JS_ASSIGNMENT_EXPRESSION => match index {
				0usize => value.is_none() || JsAnyAssignmentPattern::can_cast(value.unwrap()),
				1usize => {
					value.is_none()
						|| matches!(
							value.unwrap(),
							T ! [=]
								| T ! [+=] | T ! [-=] | T ! [*=] | T ! [/=]
								| T ! [%=] | T ! [**=] | T ! [>>=]
								| T ! [<<=] | T ! [>>>=] | T ! [&=]
								| T ! [|=] | T ! [^=] | T ! [&&=]
								| T ! [||=] | T ! [??=]
						)
				}
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			JS_ASSIGNMENT_WITH_DEFAULT => match index {
				0usize => value.is_none() || JsAnyAssignmentPattern::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [=],
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			JS_AWAIT_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == T![await],
				1usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			JS_BIG_INT_LITERAL_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == JS_BIG_INT_LITERAL,
				_ => false,
			},
			JS_BINARY_EXPRESSION => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				1usize => {
					value.is_none()
						|| matches!(
							value.unwrap(),
							T ! [<]
								| T ! [>] | T ! [<=] | T ! [>=] | T ! [==]
								| T ! [===] | T ! [!=] | T ! [!==]
								| T ! [+] | T ! [-] | T ! [*] | T ! [/]
								| T ! [%] | T ! [**] | T ! [<<] | T ! [>>]
								| T ! [>>>] | T ! [&] | T ! [|] | T ! [^]
								| T![in] | T![instanceof]
						)
				}
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			JS_BINDING_PATTERN_WITH_DEFAULT => match index {
				0usize => value.is_none() || JsAnyBindingPattern::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [=],
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			JS_BLOCK_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T!['{'],
				1usize => value.is_none() || JsStatementList::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			JS_BOOLEAN_LITERAL_EXPRESSION => match index {
				0usize => value.is_none() || matches!(value.unwrap(), T![true] | T![false]),
				_ => false,
			},
			JS_BREAK_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T![break],
				1usize => value.is_none() || value.unwrap() == IDENT,
				2usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			JS_CALL_ARGUMENTS => match index {
				0usize => value.is_none() || value.unwrap() == T!['('],
				1usize => value.is_none() || JsCallArgumentList::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T![')'],
				_ => false,
			},
			JS_CASE_CLAUSE => match index {
				0usize => value.is_none() || value.unwrap() == T![case],
				1usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T ! [:],
				3usize => value.is_none() || JsStatementList::can_cast(value.unwrap()),
				_ => false,
			},
			JS_CATCH_CLAUSE => match index {
				0usize => value.is_none() || value.unwrap() == T![catch],
				1usize => value.is_none() || JsCatchDeclaration::can_cast(value.unwrap()),
				2usize => value.is_none() || JsBlockStatement::can_cast(value.unwrap()),
				_ => false,
			},
			JS_CATCH_DECLARATION => match index {
				0usize => value.is_none() || value.unwrap() == T!['('],
				1usize => value.is_none() || JsAnyBindingPattern::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T![')'],
				_ => false,
			},
			JS_CLASS_DECLARATION => match index {
				0usize => value.is_none() || value.unwrap() == T![class],
				1usize => value.is_none() || JsAnyBinding::can_cast(value.unwrap()),
				2usize => value.is_none() || JsExtendsClause::can_cast(value.unwrap()),
				3usize => value.is_none() || TsImplementsClause::can_cast(value.unwrap()),
				4usize => value.is_none() || value.unwrap() == T!['{'],
				5usize => value.is_none() || JsClassMemberList::can_cast(value.unwrap()),
				6usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			JS_CLASS_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == T![class],
				1usize => value.is_none() || JsAnyBinding::can_cast(value.unwrap()),
				2usize => value.is_none() || JsExtendsClause::can_cast(value.unwrap()),
				3usize => value.is_none() || TsImplementsClause::can_cast(value.unwrap()),
				4usize => value.is_none() || value.unwrap() == T!['{'],
				5usize => value.is_none() || JsClassMemberList::can_cast(value.unwrap()),
				6usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			JS_COMPUTED_MEMBER_ASSIGNMENT => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T!['['],
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T![']'],
				_ => false,
			},
			JS_COMPUTED_MEMBER_EXPRESSION => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [?.],
				2usize => value.is_none() || value.unwrap() == T!['['],
				3usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				4usize => value.is_none() || value.unwrap() == T![']'],
				_ => false,
			},
			JS_COMPUTED_MEMBER_NAME => match index {
				0usize => value.is_none() || value.unwrap() == T!['['],
				1usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T![']'],
				_ => false,
			},
			JS_CONDITIONAL_EXPRESSION => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [?],
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T ! [:],
				4usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			JS_CONSTRUCTOR_CLASS_MEMBER => match index {
				0usize => {
					value.is_none()
						|| matches!(value.unwrap(), T![private] | T![protected] | T![public])
				}
				1usize => value.is_none() || JsLiteralMemberName::can_cast(value.unwrap()),
				2usize => value.is_none() || TsTypeParams::can_cast(value.unwrap()),
				3usize => value.is_none() || JsConstructorParameters::can_cast(value.unwrap()),
				4usize => value.is_none() || JsFunctionBody::can_cast(value.unwrap()),
				_ => false,
			},
			JS_CONSTRUCTOR_PARAMETERS => match index {
				0usize => value.is_none() || value.unwrap() == T!['('],
				1usize => value.is_none() || JsConstructorParameterList::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T![')'],
				_ => false,
			},
			JS_CONTINUE_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T![continue],
				1usize => value.is_none() || value.unwrap() == IDENT,
				2usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			JS_DEBUGGER_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T![debugger],
				1usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			JS_DEFAULT_CLAUSE => match index {
				0usize => value.is_none() || value.unwrap() == T![default],
				1usize => value.is_none() || value.unwrap() == T ! [:],
				2usize => value.is_none() || JsStatementList::can_cast(value.unwrap()),
				_ => false,
			},
			JS_DEFAULT_IMPORT_SPECIFIER => match index {
				0usize => value.is_none() || JsAnyBinding::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [,],
				_ => false,
			},
			JS_DIRECTIVE => match index {
				0usize => value.is_none() || value.unwrap() == JS_STRING_LITERAL,
				1usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			JS_DO_WHILE_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T![do],
				1usize => value.is_none() || JsAnyStatement::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T![while],
				3usize => value.is_none() || value.unwrap() == T!['('],
				4usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				5usize => value.is_none() || value.unwrap() == T![')'],
				6usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			JS_ELSE_CLAUSE => match index {
				0usize => value.is_none() || value.unwrap() == T![else],
				1usize => value.is_none() || JsAnyStatement::can_cast(value.unwrap()),
				_ => false,
			},
			JS_EMPTY_CLASS_MEMBER => match index {
				0usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			JS_EMPTY_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			JS_EXPRESSION_SNIPPED => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T![EOF],
				_ => false,
			},
			JS_EXPRESSION_STATEMENT => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			JS_EXTENDS_CLAUSE => match index {
				0usize => value.is_none() || value.unwrap() == T![extends],
				1usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			JS_FINALLY_CLAUSE => match index {
				0usize => value.is_none() || value.unwrap() == T![finally],
				1usize => value.is_none() || JsBlockStatement::can_cast(value.unwrap()),
				_ => false,
			},
			JS_FOR_IN_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T![for],
				1usize => value.is_none() || value.unwrap() == T!['('],
				2usize => value.is_none() || JsAnyForInOrOfInitializer::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T![in],
				4usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				5usize => value.is_none() || value.unwrap() == T![')'],
				6usize => value.is_none() || JsAnyStatement::can_cast(value.unwrap()),
				_ => false,
			},
			JS_FOR_OF_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T![for],
				1usize => value.is_none() || value.unwrap() == T![await],
				2usize => value.is_none() || value.unwrap() == T!['('],
				3usize => value.is_none() || JsAnyForInOrOfInitializer::can_cast(value.unwrap()),
				4usize => value.is_none() || value.unwrap() == T![of],
				5usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				6usize => value.is_none() || value.unwrap() == T![')'],
				7usize => value.is_none() || JsAnyStatement::can_cast(value.unwrap()),
				_ => false,
			},
			JS_FOR_VARIABLE_DECLARATION => match index {
				0usize => {
					value.is_none() || matches!(value.unwrap(), T![var] | T![let] | T![const])
				}
				1usize => value.is_none() || JsVariableDeclaration::can_cast(value.unwrap()),
				_ => false,
			},
			JS_FUNCTION_BODY => match index {
				0usize => value.is_none() || value.unwrap() == T!['{'],
				1usize => value.is_none() || JsDirectiveList::can_cast(value.unwrap()),
				2usize => value.is_none() || JsStatementList::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			JS_FUNCTION_DECLARATION => match index {
				0usize => value.is_none() || value.unwrap() == T![async],
				1usize => value.is_none() || value.unwrap() == T![function],
				2usize => value.is_none() || value.unwrap() == T ! [*],
				3usize => value.is_none() || JsAnyBinding::can_cast(value.unwrap()),
				4usize => value.is_none() || TsTypeParams::can_cast(value.unwrap()),
				5usize => value.is_none() || JsParameters::can_cast(value.unwrap()),
				6usize => value.is_none() || TsTypeAnnotation::can_cast(value.unwrap()),
				7usize => value.is_none() || JsFunctionBody::can_cast(value.unwrap()),
				_ => false,
			},
			JS_FUNCTION_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == T![async],
				1usize => value.is_none() || value.unwrap() == T![function],
				2usize => value.is_none() || value.unwrap() == T ! [*],
				3usize => value.is_none() || JsAnyBinding::can_cast(value.unwrap()),
				4usize => value.is_none() || TsTypeParams::can_cast(value.unwrap()),
				5usize => value.is_none() || JsParameters::can_cast(value.unwrap()),
				6usize => value.is_none() || TsTypeAnnotation::can_cast(value.unwrap()),
				7usize => value.is_none() || JsFunctionBody::can_cast(value.unwrap()),
				_ => false,
			},
			JS_GETTER_CLASS_MEMBER => match index {
				0usize => {
					value.is_none()
						|| matches!(value.unwrap(), T![private] | T![protected] | T![public])
				}
				1usize => value.is_none() || value.unwrap() == T![static],
				2usize => value.is_none() || value.unwrap() == T![abstract],
				3usize => value.is_none() || value.unwrap() == T![get],
				4usize => value.is_none() || JsAnyClassMemberName::can_cast(value.unwrap()),
				5usize => value.is_none() || value.unwrap() == T!['('],
				6usize => value.is_none() || value.unwrap() == T![')'],
				7usize => value.is_none() || TsTypeAnnotation::can_cast(value.unwrap()),
				8usize => value.is_none() || JsFunctionBody::can_cast(value.unwrap()),
				_ => false,
			},
			JS_GETTER_OBJECT_MEMBER => match index {
				0usize => value.is_none() || value.unwrap() == T![get],
				1usize => value.is_none() || JsAnyObjectMemberName::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T!['('],
				3usize => value.is_none() || value.unwrap() == T![')'],
				4usize => value.is_none() || TsTypeAnnotation::can_cast(value.unwrap()),
				5usize => value.is_none() || JsFunctionBody::can_cast(value.unwrap()),
				_ => false,
			},
			JS_IDENTIFIER_ASSIGNMENT => match index {
				0usize => value.is_none() || value.unwrap() == IDENT,
				_ => false,
			},
			JS_IDENTIFIER_BINDING => match index {
				0usize => value.is_none() || value.unwrap() == IDENT,
				_ => false,
			},
			JS_IDENTIFIER_EXPRESSION => match index {
				0usize => value.is_none() || JsReferenceIdentifier::can_cast(value.unwrap()),
				_ => false,
			},
			JS_IF_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T![if],
				1usize => value.is_none() || value.unwrap() == T!['('],
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T![')'],
				4usize => value.is_none() || JsAnyStatement::can_cast(value.unwrap()),
				5usize => value.is_none() || JsElseClause::can_cast(value.unwrap()),
				_ => false,
			},
			JS_IMPORT => match index {
				0usize => value.is_none() || value.unwrap() == T![import],
				1usize => value.is_none() || AnyJsImportClause::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			JS_IMPORT_ASSERTION => match index {
				0usize => value.is_none() || value.unwrap() == T![assert],
				1usize => value.is_none() || value.unwrap() == T!['{'],
				2usize => value.is_none() || JsImportAssertionEntryList::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			JS_IMPORT_ASSERTION_ENTRY => match index {
				0usize => value.is_none() || matches!(value.unwrap(), IDENT | JS_STRING_LITERAL),
				1usize => value.is_none() || value.unwrap() == T ! [:],
				2usize => value.is_none() || value.unwrap() == JS_STRING_LITERAL,
				_ => false,
			},
			JS_IMPORT_BARE_CLAUSE => match index {
				0usize => value.is_none() || JsModuleSource::can_cast(value.unwrap()),
				1usize => value.is_none() || JsImportAssertion::can_cast(value.unwrap()),
				_ => false,
			},
			JS_IMPORT_CALL_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == T![import],
				1usize => value.is_none() || value.unwrap() == T!['('],
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T![')'],
				_ => false,
			},
			JS_IMPORT_DEFAULT_CLAUSE => match index {
				0usize => value.is_none() || JsAnyBinding::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T![from],
				2usize => value.is_none() || JsModuleSource::can_cast(value.unwrap()),
				3usize => value.is_none() || JsImportAssertion::can_cast(value.unwrap()),
				_ => false,
			},
			JS_IMPORT_NAMED_CLAUSE => match index {
				0usize => value.is_none() || JsDefaultImportSpecifier::can_cast(value.unwrap()),
				1usize => value.is_none() || JsAnyNamedImport::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T![from],
				3usize => value.is_none() || JsModuleSource::can_cast(value.unwrap()),
				4usize => value.is_none() || JsImportAssertion::can_cast(value.unwrap()),
				_ => false,
			},
			JS_IMPORT_NAMESPACE_CLAUSE => match index {
				0usize => value.is_none() || value.unwrap() == T ! [*],
				1usize => value.is_none() || value.unwrap() == T![as],
				2usize => value.is_none() || JsAnyBinding::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T![from],
				4usize => value.is_none() || JsModuleSource::can_cast(value.unwrap()),
				5usize => value.is_none() || JsImportAssertion::can_cast(value.unwrap()),
				_ => false,
			},
			JS_INITIALIZER_CLAUSE => match index {
				0usize => value.is_none() || value.unwrap() == T ! [=],
				1usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			JS_LABELED_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == IDENT,
				1usize => value.is_none() || value.unwrap() == T ! [:],
				2usize => value.is_none() || JsAnyStatement::can_cast(value.unwrap()),
				_ => false,
			},
			JS_LITERAL_EXPORT_NAME => match index {
				0usize => value.is_none() || matches!(value.unwrap(), IDENT | JS_STRING_LITERAL),
				_ => false,
			},
			JS_LITERAL_MEMBER_NAME => match index {
				0usize => {
					value.is_none()
						|| matches!(
							value.unwrap(),
							IDENT | JS_STRING_LITERAL | JS_NUMBER_LITERAL
						)
				}
				_ => false,
			},
			JS_LOGICAL_EXPRESSION => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				1usize => {
					value.is_none() || matches!(value.unwrap(), T ! [??] | T ! [||] | T ! [&&])
				}
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			JS_METHOD_CLASS_MEMBER => match index {
				0usize => {
					value.is_none()
						|| matches!(value.unwrap(), T![private] | T![protected] | T![public])
				}
				1usize => value.is_none() || value.unwrap() == T![static],
				2usize => value.is_none() || value.unwrap() == T![abstract],
				3usize => value.is_none() || value.unwrap() == T![async],
				4usize => value.is_none() || value.unwrap() == T ! [*],
				5usize => value.is_none() || JsAnyClassMemberName::can_cast(value.unwrap()),
				6usize => value.is_none() || TsTypeParams::can_cast(value.unwrap()),
				7usize => value.is_none() || JsParameters::can_cast(value.unwrap()),
				8usize => value.is_none() || TsTypeAnnotation::can_cast(value.unwrap()),
				9usize => value.is_none() || JsFunctionBody::can_cast(value.unwrap()),
				_ => false,
			},
			JS_METHOD_OBJECT_MEMBER => match index {
				0usize => value.is_none() || value.unwrap() == T![async],
				1usize => value.is_none() || value.unwrap() == T ! [*],
				2usize => value.is_none() || JsAnyObjectMemberName::can_cast(value.unwrap()),
				3usize => value.is_none() || TsTypeParams::can_cast(value.unwrap()),
				4usize => value.is_none() || JsParameters::can_cast(value.unwrap()),
				5usize => value.is_none() || TsTypeAnnotation::can_cast(value.unwrap()),
				6usize => value.is_none() || JsFunctionBody::can_cast(value.unwrap()),
				_ => false,
			},
			JS_MODULE => match index {
				0usize => value.is_none() || value.unwrap() == JS_SHEBANG,
				1usize => value.is_none() || JsDirectiveList::can_cast(value.unwrap()),
				2usize => value.is_none() || JsModuleItemList::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T![EOF],
				_ => false,
			},
			JS_MODULE_SOURCE => match index {
				0usize => value.is_none() || value.unwrap() == JS_STRING_LITERAL,
				_ => false,
			},
			JS_NAME => match index {
				0usize => value.is_none() || value.unwrap() == IDENT,
				_ => false,
			},
			JS_NAMED_IMPORT_SPECIFIER => match index {
				0usize => value.is_none() || JsLiteralExportName::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T![as],
				2usize => value.is_none() || JsAnyBinding::can_cast(value.unwrap()),
				_ => false,
			},
			JS_NAMED_IMPORT_SPECIFIERS => match index {
				0usize => value.is_none() || value.unwrap() == T!['{'],
				1usize => value.is_none() || JsNamedImportSpecifierList::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			JS_NAMESPACE_IMPORT_SPECIFIER => match index {
				0usize => value.is_none() || value.unwrap() == T ! [*],
				1usize => value.is_none() || value.unwrap() == T![as],
				2usize => value.is_none() || JsAnyBinding::can_cast(value.unwrap()),
				_ => false,
			},
			JS_NULL_LITERAL_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == T![null],
				_ => false,
			},
			JS_NUMBER_LITERAL_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == JS_NUMBER_LITERAL,
				_ => false,
			},
			JS_OBJECT_ASSIGNMENT_PATTERN => match index {
				0usize => value.is_none() || value.unwrap() == T!['{'],
				1usize => {
					value.is_none()
						|| JsObjectAssignmentPatternPropertyList::can_cast(value.unwrap())
				}
				2usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => match index {
				0usize => value.is_none() || JsName::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [:],
				2usize => value.is_none() || JsAnyAssignmentPattern::can_cast(value.unwrap()),
				3usize => value.is_none() || JsInitializerClause::can_cast(value.unwrap()),
				_ => false,
			},
			JS_OBJECT_ASSIGNMENT_PATTERN_REST => match index {
				0usize => value.is_none() || value.unwrap() == T ! [...],
				1usize => value.is_none() || JsAnyAssignment::can_cast(value.unwrap()),
				_ => false,
			},
			JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => match index {
				0usize => value.is_none() || JsAnyAssignment::can_cast(value.unwrap()),
				1usize => value.is_none() || JsInitializerClause::can_cast(value.unwrap()),
				_ => false,
			},
			JS_OBJECT_BINDING_PATTERN => match index {
				0usize => value.is_none() || value.unwrap() == T!['{'],
				1usize => {
					value.is_none() || JsObjectBindingPatternPropertyList::can_cast(value.unwrap())
				}
				2usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			JS_OBJECT_BINDING_PATTERN_PROPERTY => match index {
				0usize => value.is_none() || JsAnyObjectMemberName::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [:],
				2usize => value.is_none() || JsAnyBindingPattern::can_cast(value.unwrap()),
				3usize => value.is_none() || JsInitializerClause::can_cast(value.unwrap()),
				_ => false,
			},
			JS_OBJECT_BINDING_PATTERN_REST => match index {
				0usize => value.is_none() || value.unwrap() == T ! [...],
				1usize => value.is_none() || JsAnyBinding::can_cast(value.unwrap()),
				_ => false,
			},
			JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => match index {
				0usize => value.is_none() || JsAnyBinding::can_cast(value.unwrap()),
				1usize => value.is_none() || JsInitializerClause::can_cast(value.unwrap()),
				_ => false,
			},
			JS_OBJECT_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == T!['{'],
				1usize => value.is_none() || JsObjectMemberList::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			JS_PARAMETERS => match index {
				0usize => value.is_none() || value.unwrap() == T!['('],
				1usize => value.is_none() || JsParameterList::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T![')'],
				_ => false,
			},
			JS_PARENTHESIZED_ASSIGNMENT => match index {
				0usize => value.is_none() || value.unwrap() == T!['('],
				1usize => value.is_none() || JsAnyAssignment::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T![')'],
				_ => false,
			},
			JS_PARENTHESIZED_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == T!['('],
				1usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T![')'],
				_ => false,
			},
			JS_POST_UPDATE_EXPRESSION => match index {
				0usize => value.is_none() || JsAnyAssignment::can_cast(value.unwrap()),
				1usize => value.is_none() || matches!(value.unwrap(), T ! [++] | T ! [--]),
				_ => false,
			},
			JS_PRE_UPDATE_EXPRESSION => match index {
				0usize => value.is_none() || matches!(value.unwrap(), T ! [++] | T ! [--]),
				1usize => value.is_none() || JsAnyAssignment::can_cast(value.unwrap()),
				_ => false,
			},
			JS_PRIVATE_CLASS_MEMBER_NAME => match index {
				0usize => value.is_none() || value.unwrap() == T ! [#],
				1usize => value.is_none() || value.unwrap() == IDENT,
				_ => false,
			},
			JS_PRIVATE_NAME => match index {
				0usize => value.is_none() || value.unwrap() == T ! [#],
				1usize => value.is_none() || value.unwrap() == IDENT,
				_ => false,
			},
			JS_PROPERTY_CLASS_MEMBER => match index {
				0usize => value.is_none() || value.unwrap() == T![declare],
				1usize => {
					value.is_none()
						|| matches!(value.unwrap(), T![private] | T![protected] | T![public])
				}
				2usize => value.is_none() || value.unwrap() == T![static],
				3usize => value.is_none() || value.unwrap() == T![readonly],
				4usize => value.is_none() || value.unwrap() == T![abstract],
				5usize => value.is_none() || JsAnyClassMemberName::can_cast(value.unwrap()),
				6usize => value.is_none() || value.unwrap() == T ! [?],
				7usize => value.is_none() || value.unwrap() == T![!],
				8usize => value.is_none() || TsTypeAnnotation::can_cast(value.unwrap()),
				9usize => value.is_none() || JsInitializerClause::can_cast(value.unwrap()),
				10usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			JS_PROPERTY_OBJECT_MEMBER => match index {
				0usize => value.is_none() || JsAnyObjectMemberName::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [:],
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			JS_REFERENCE_IDENTIFIER => match index {
				0usize => value.is_none() || value.unwrap() == IDENT,
				_ => false,
			},
			JS_REGEX_LITERAL_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == JS_REGEX_LITERAL,
				_ => false,
			},
			JS_REST_PARAMETER => match index {
				0usize => value.is_none() || value.unwrap() == T ! [...],
				1usize => value.is_none() || JsAnyBindingPattern::can_cast(value.unwrap()),
				_ => false,
			},
			JS_RETURN_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T![return],
				1usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			JS_SCRIPT => match index {
				0usize => value.is_none() || value.unwrap() == JS_SHEBANG,
				1usize => value.is_none() || JsDirectiveList::can_cast(value.unwrap()),
				2usize => value.is_none() || JsStatementList::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T![EOF],
				_ => false,
			},
			JS_SEQUENCE_EXPRESSION => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [,],
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			JS_SETTER_CLASS_MEMBER => match index {
				0usize => {
					value.is_none()
						|| matches!(value.unwrap(), T![private] | T![protected] | T![public])
				}
				1usize => value.is_none() || value.unwrap() == T![static],
				2usize => value.is_none() || value.unwrap() == T![abstract],
				3usize => value.is_none() || value.unwrap() == T![set],
				4usize => value.is_none() || JsAnyClassMemberName::can_cast(value.unwrap()),
				5usize => value.is_none() || value.unwrap() == T!['('],
				6usize => value.is_none() || JsAnyBindingPattern::can_cast(value.unwrap()),
				7usize => value.is_none() || value.unwrap() == T![')'],
				8usize => value.is_none() || JsFunctionBody::can_cast(value.unwrap()),
				_ => false,
			},
			JS_SETTER_OBJECT_MEMBER => match index {
				0usize => value.is_none() || value.unwrap() == T![set],
				1usize => value.is_none() || JsAnyObjectMemberName::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T!['('],
				3usize => value.is_none() || JsAnyBindingPattern::can_cast(value.unwrap()),
				4usize => value.is_none() || value.unwrap() == T![')'],
				5usize => value.is_none() || JsFunctionBody::can_cast(value.unwrap()),
				_ => false,
			},
			JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => match index {
				0usize => value.is_none() || JsAnyBinding::can_cast(value.unwrap()),
				_ => false,
			},
			JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => match index {
				0usize => value.is_none() || JsReferenceIdentifier::can_cast(value.unwrap()),
				_ => false,
			},
			JS_SPREAD => match index {
				0usize => value.is_none() || value.unwrap() == T ! [...],
				1usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			JS_STATIC_MEMBER_ASSIGNMENT => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [.],
				2usize => value.is_none() || JsAnyName::can_cast(value.unwrap()),
				_ => false,
			},
			JS_STATIC_MEMBER_EXPRESSION => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				1usize => value.is_none() || matches!(value.unwrap(), T ! [.] | T ! [?.]),
				2usize => value.is_none() || JsAnyName::can_cast(value.unwrap()),
				_ => false,
			},
			JS_STRING_LITERAL_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == JS_STRING_LITERAL,
				_ => false,
			},
			JS_SUPER_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == T![super],
				_ => false,
			},
			JS_SWITCH_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T![switch],
				1usize => value.is_none() || value.unwrap() == T!['('],
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T![')'],
				4usize => value.is_none() || value.unwrap() == T!['{'],
				5usize => value.is_none() || JsSwitchCaseList::can_cast(value.unwrap()),
				6usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			JS_THIS_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == T![this],
				_ => false,
			},
			JS_THROW_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T![throw],
				1usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			JS_TRY_FINALLY_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T![try],
				1usize => value.is_none() || JsBlockStatement::can_cast(value.unwrap()),
				2usize => value.is_none() || JsCatchClause::can_cast(value.unwrap()),
				3usize => value.is_none() || JsFinallyClause::can_cast(value.unwrap()),
				_ => false,
			},
			JS_TRY_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T![try],
				1usize => value.is_none() || JsBlockStatement::can_cast(value.unwrap()),
				2usize => value.is_none() || JsCatchClause::can_cast(value.unwrap()),
				_ => false,
			},
			JS_UNARY_EXPRESSION => match index {
				0usize => {
					value.is_none()
						|| matches!(
							value.unwrap(),
							T![delete]
								| T![void] | T![typeof] | T ! [+]
								| T ! [-] | T ! [~] | T![!]
						)
				}
				1usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			JS_VARIABLE_DECLARATION => match index {
				0usize => value.is_none() || JsAnyBindingPattern::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T![!],
				2usize => value.is_none() || TsTypeAnnotation::can_cast(value.unwrap()),
				3usize => value.is_none() || JsInitializerClause::can_cast(value.unwrap()),
				_ => false,
			},
			JS_VARIABLE_DECLARATIONS => match index {
				0usize => {
					value.is_none() || matches!(value.unwrap(), T![var] | T![const] | T![let])
				}
				1usize => value.is_none() || JsVariableDeclarationList::can_cast(value.unwrap()),
				_ => false,
			},
			JS_VARIABLE_STATEMENT => match index {
				0usize => value.is_none() || JsVariableDeclarations::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			JS_WHILE_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T![while],
				1usize => value.is_none() || value.unwrap() == T!['('],
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T![')'],
				4usize => value.is_none() || JsAnyStatement::can_cast(value.unwrap()),
				_ => false,
			},
			JS_WITH_STATEMENT => match index {
				0usize => value.is_none() || value.unwrap() == T![with],
				1usize => value.is_none() || value.unwrap() == T!['('],
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T![')'],
				4usize => value.is_none() || JsAnyStatement::can_cast(value.unwrap()),
				_ => false,
			},
			JS_YIELD_EXPRESSION => match index {
				0usize => value.is_none() || value.unwrap() == T![yield],
				1usize => value.is_none() || value.unwrap() == T ! [*],
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			NEW_EXPR => match index {
				0usize => value.is_none() || value.unwrap() == T![new],
				1usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				2usize => value.is_none() || TsTypeArgs::can_cast(value.unwrap()),
				3usize => value.is_none() || JsCallArguments::can_cast(value.unwrap()),
				_ => false,
			},
			NEW_TARGET => match index {
				0usize => value.is_none() || value.unwrap() == T![new],
				1usize => value.is_none() || value.unwrap() == T ! [.],
				2usize => value.is_none() || value.unwrap() == T![target],
				_ => false,
			},
			SPECIFIER => match index {
				0usize => value.is_none() || JsName::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T![as],
				2usize => value.is_none() || JsName::can_cast(value.unwrap()),
				_ => false,
			},
			TEMPLATE => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T!['`'],
				2usize => value.is_none() || TemplateElementList::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T!['`'],
				_ => false,
			},
			TEMPLATE_CHUNK_ELEMENT => match index {
				0usize => value.is_none() || value.unwrap() == TEMPLATE_CHUNK,
				_ => false,
			},
			TEMPLATE_ELEMENT => match index {
				0usize => value.is_none() || value.unwrap() == DOLLAR_CURLY,
				1usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			TS_ANY => match index {
				0usize => value.is_none() || value.unwrap() == T![any],
				_ => false,
			},
			TS_ARRAY => match index {
				0usize => value.is_none() || value.unwrap() == T!['['],
				1usize => value.is_none() || TsType::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T![']'],
				_ => false,
			},
			TS_ASSERTION => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				1usize => value.is_none() || Ident::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T ! [<],
				3usize => value.is_none() || TsType::can_cast(value.unwrap()),
				4usize => value.is_none() || value.unwrap() == T ! [>],
				_ => false,
			},
			TS_BIGINT => match index {
				0usize => value.is_none() || Ident::can_cast(value.unwrap()),
				_ => false,
			},
			TS_BOOLEAN => match index {
				0usize => value.is_none() || Ident::can_cast(value.unwrap()),
				_ => false,
			},
			TS_CALL_SIGNATURE_DECL => match index {
				0usize => value.is_none() || TsTypeParams::can_cast(value.unwrap()),
				1usize => value.is_none() || JsParameters::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T ! [:],
				3usize => value.is_none() || TsType::can_cast(value.unwrap()),
				_ => false,
			},
			TS_CONDITIONAL_TYPE => match index {
				0usize => value.is_none() || TsType::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [?],
				2usize => value.is_none() || value.unwrap() == T ! [:],
				3usize => value.is_none() || TsExtends::can_cast(value.unwrap()),
				_ => false,
			},
			TS_CONST_ASSERTION => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				1usize => value.is_none() || Ident::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T ! [<],
				3usize => value.is_none() || value.unwrap() == T![const],
				4usize => value.is_none() || value.unwrap() == T ! [>],
				_ => false,
			},
			TS_CONSTRAINT => match index {
				0usize => value.is_none() || value.unwrap() == T![extends],
				1usize => value.is_none() || TsType::can_cast(value.unwrap()),
				_ => false,
			},
			TS_CONSTRUCT_SIGNATURE_DECL => match index {
				0usize => value.is_none() || value.unwrap() == T![new],
				1usize => value.is_none() || TsTypeParams::can_cast(value.unwrap()),
				2usize => value.is_none() || JsParameters::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T ! [:],
				4usize => value.is_none() || TsType::can_cast(value.unwrap()),
				_ => false,
			},
			TS_CONSTRUCTOR_PARAM => match index {
				0usize => {
					value.is_none()
						|| matches!(value.unwrap(), T![private] | T![protected] | T![public])
				}
				1usize => value.is_none() || value.unwrap() == T![readonly],
				2usize => value.is_none() || JsAnyBindingPattern::can_cast(value.unwrap()),
				_ => false,
			},
			TS_CONSTRUCTOR_TYPE => match index {
				0usize => value.is_none() || value.unwrap() == T![new],
				1usize => value.is_none() || JsParameters::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T ! [:],
				3usize => value.is_none() || TsType::can_cast(value.unwrap()),
				_ => false,
			},
			TS_DEFAULT => match index {
				0usize => value.is_none() || value.unwrap() == T ! [=],
				1usize => value.is_none() || TsType::can_cast(value.unwrap()),
				_ => false,
			},
			TS_ENUM => match index {
				0usize => value.is_none() || value.unwrap() == T![const],
				1usize => value.is_none() || value.unwrap() == T![enum],
				2usize => value.is_none() || Ident::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T!['{'],
				4usize => value.is_none() || TsEnumMemberList::can_cast(value.unwrap()),
				5usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			TS_ENUM_MEMBER => match index {
				0usize => value.is_none() || Ident::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [=],
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				_ => false,
			},
			TS_EXPORT_ASSIGNMENT => match index {
				0usize => value.is_none() || value.unwrap() == T![export],
				1usize => value.is_none() || value.unwrap() == T ! [=],
				2usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			TS_EXPR_WITH_TYPE_ARGS => match index {
				0usize => value.is_none() || TsEntityName::can_cast(value.unwrap()),
				1usize => value.is_none() || TsTypeArgs::can_cast(value.unwrap()),
				_ => false,
			},
			TS_EXTENDS => match index {
				0usize => value.is_none() || value.unwrap() == T![extends],
				1usize => value.is_none() || TsType::can_cast(value.unwrap()),
				_ => false,
			},
			TS_EXTERNAL_MODULE_REF => match index {
				0usize => value.is_none() || value.unwrap() == T![require],
				1usize => value.is_none() || value.unwrap() == T!['('],
				2usize => value.is_none() || value.unwrap() == JS_STRING_LITERAL,
				3usize => value.is_none() || value.unwrap() == T![')'],
				_ => false,
			},
			TS_FN_TYPE => match index {
				0usize => value.is_none() || JsParameters::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [=>],
				2usize => value.is_none() || TsType::can_cast(value.unwrap()),
				_ => false,
			},
			TS_IMPLEMENTS_CLAUSE => match index {
				0usize => value.is_none() || value.unwrap() == T![implements],
				1usize => value.is_none() || TsTypeList::can_cast(value.unwrap()),
				_ => false,
			},
			TS_IMPORT => match index {
				0usize => value.is_none() || value.unwrap() == T![import],
				1usize => value.is_none() || TsTypeArgs::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T ! [.],
				3usize => value.is_none() || value.unwrap() == T!['('],
				4usize => value.is_none() || TsEntityName::can_cast(value.unwrap()),
				5usize => value.is_none() || value.unwrap() == T![')'],
				_ => false,
			},
			TS_IMPORT_EQUALS_DECL => match index {
				0usize => value.is_none() || value.unwrap() == T![import],
				1usize => value.is_none() || value.unwrap() == T![export],
				2usize => value.is_none() || Ident::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T ! [=],
				4usize => value.is_none() || TsModuleRef::can_cast(value.unwrap()),
				5usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			TS_INDEX_SIGNATURE => match index {
				0usize => value.is_none() || value.unwrap() == T![readonly],
				1usize => value.is_none() || value.unwrap() == T!['['],
				2usize => value.is_none() || JsAnyBinding::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T ! [:],
				4usize => value.is_none() || TsType::can_cast(value.unwrap()),
				5usize => value.is_none() || value.unwrap() == T![']'],
				_ => false,
			},
			TS_INDEXED_ARRAY => match index {
				0usize => value.is_none() || value.unwrap() == T!['['],
				1usize => value.is_none() || TsType::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T![']'],
				_ => false,
			},
			TS_INFER => match index {
				0usize => value.is_none() || value.unwrap() == T![infer],
				1usize => value.is_none() || Ident::can_cast(value.unwrap()),
				_ => false,
			},
			TS_INTERFACE_DECL => match index {
				0usize => value.is_none() || value.unwrap() == T![declare],
				1usize => value.is_none() || value.unwrap() == T![interface],
				2usize => value.is_none() || TsTypeParams::can_cast(value.unwrap()),
				3usize => value.is_none() || value.unwrap() == T![extends],
				4usize => value.is_none() || TsExprWithTypeArgs::can_cast(value.unwrap()),
				5usize => value.is_none() || value.unwrap() == T!['{'],
				6usize => value.is_none() || TsTypeElement::can_cast(value.unwrap()),
				7usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			TS_INTERSECTION => match index {
				0usize => value.is_none() || TsTypeList::can_cast(value.unwrap()),
				_ => false,
			},
			TS_LITERAL => match index {
				0usize => value.is_none() || Ident::can_cast(value.unwrap()),
				_ => false,
			},
			TS_MAPPED_TYPE => match index {
				0usize => value.is_none() || value.unwrap() == T!['{'],
				1usize => value.is_none() || TsMappedTypeReadonly::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T ! [-],
				3usize => value.is_none() || value.unwrap() == T ! [+],
				4usize => value.is_none() || value.unwrap() == T ! [?],
				5usize => value.is_none() || TsMappedTypeParam::can_cast(value.unwrap()),
				6usize => value.is_none() || value.unwrap() == T ! [:],
				7usize => value.is_none() || TsType::can_cast(value.unwrap()),
				8usize => value.is_none() || value.unwrap() == T!['}'],
				9usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			TS_MAPPED_TYPE_PARAM => match index {
				0usize => value.is_none() || value.unwrap() == T!['['],
				1usize => value.is_none() || TsTypeName::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T![']'],
				3usize => value.is_none() || Ident::can_cast(value.unwrap()),
				4usize => value.is_none() || TsType::can_cast(value.unwrap()),
				_ => false,
			},
			TS_MAPPED_TYPE_READONLY => match index {
				0usize => value.is_none() || value.unwrap() == T ! [-],
				1usize => value.is_none() || value.unwrap() == T ! [+],
				2usize => value.is_none() || value.unwrap() == T![readonly],
				_ => false,
			},
			TS_METHOD_SIGNATURE => match index {
				0usize => value.is_none() || value.unwrap() == T![readonly],
				1usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				2usize => value.is_none() || TsTypeParams::can_cast(value.unwrap()),
				3usize => value.is_none() || JsParameters::can_cast(value.unwrap()),
				4usize => value.is_none() || value.unwrap() == T ! [?],
				5usize => value.is_none() || value.unwrap() == T ! [:],
				6usize => value.is_none() || TsType::can_cast(value.unwrap()),
				_ => false,
			},
			TS_MODULE_BLOCK => match index {
				0usize => value.is_none() || value.unwrap() == T!['{'],
				1usize => value.is_none() || JsAnyStatement::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			TS_MODULE_DECL => match index {
				0usize => value.is_none() || value.unwrap() == T![declare],
				1usize => value.is_none() || value.unwrap() == T![global],
				2usize => value.is_none() || value.unwrap() == T![module],
				3usize => value.is_none() || value.unwrap() == T ! [.],
				4usize => value.is_none() || Ident::can_cast(value.unwrap()),
				5usize => value.is_none() || TsNamespaceBody::can_cast(value.unwrap()),
				_ => false,
			},
			TS_NAMESPACE_DECL => match index {
				0usize => value.is_none() || value.unwrap() == T![declare],
				1usize => value.is_none() || Ident::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T ! [.],
				3usize => value.is_none() || TsNamespaceBody::can_cast(value.unwrap()),
				_ => false,
			},
			TS_NAMESPACE_EXPORT_DECL => match index {
				0usize => value.is_none() || value.unwrap() == T![export],
				1usize => value.is_none() || value.unwrap() == T![as],
				2usize => value.is_none() || value.unwrap() == T![namespace],
				3usize => value.is_none() || Ident::can_cast(value.unwrap()),
				4usize => value.is_none() || value.unwrap() == T ! [;],
				_ => false,
			},
			TS_NEVER => match index {
				0usize => value.is_none() || value.unwrap() == T![never],
				_ => false,
			},
			TS_NON_NULL => match index {
				0usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T![!],
				_ => false,
			},
			TS_NULL => match index {
				0usize => value.is_none() || value.unwrap() == T![null],
				_ => false,
			},
			TS_NUMBER => match index {
				0usize => value.is_none() || Ident::can_cast(value.unwrap()),
				_ => false,
			},
			TS_OBJECT => match index {
				0usize => value.is_none() || Ident::can_cast(value.unwrap()),
				_ => false,
			},
			TS_OBJECT_TYPE => match index {
				0usize => value.is_none() || value.unwrap() == T!['{'],
				1usize => value.is_none() || TsObjectMemberList::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			TS_PAREN => match index {
				0usize => value.is_none() || value.unwrap() == T!['('],
				1usize => value.is_none() || TsType::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T![')'],
				_ => false,
			},
			TS_PREDICATE => match index {
				0usize => value.is_none() || TsThisOrMore::can_cast(value.unwrap()),
				1usize => value.is_none() || TsType::can_cast(value.unwrap()),
				_ => false,
			},
			TS_PROPERTY_SIGNATURE => match index {
				0usize => value.is_none() || value.unwrap() == T![readonly],
				1usize => value.is_none() || JsAnyExpression::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T ! [?],
				3usize => value.is_none() || value.unwrap() == T ! [:],
				4usize => value.is_none() || TsType::can_cast(value.unwrap()),
				_ => false,
			},
			TS_QUALIFIED_PATH => match index {
				0usize => value.is_none() || TsEntityName::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [.],
				2usize => value.is_none() || TsTypeName::can_cast(value.unwrap()),
				_ => false,
			},
			TS_STRING => match index {
				0usize => value.is_none() || Ident::can_cast(value.unwrap()),
				_ => false,
			},
			TS_SYMBOL => match index {
				0usize => value.is_none() || Ident::can_cast(value.unwrap()),
				_ => false,
			},
			TS_TEMPLATE => match index {
				0usize => value.is_none() || TsTemplateElement::can_cast(value.unwrap()),
				_ => false,
			},
			TS_TEMPLATE_ELEMENT => match index {
				0usize => value.is_none() || TsType::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T!['}'],
				_ => false,
			},
			TS_THIS => match index {
				0usize => value.is_none() || value.unwrap() == T![this],
				_ => false,
			},
			TS_TUPLE => match index {
				0usize => value.is_none() || value.unwrap() == T!['['],
				1usize => value.is_none() || TsTupleElement::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T![']'],
				_ => false,
			},
			TS_TUPLE_ELEMENT => match index {
				0usize => value.is_none() || Ident::can_cast(value.unwrap()),
				1usize => value.is_none() || value.unwrap() == T ! [:],
				2usize => value.is_none() || value.unwrap() == T ! [?],
				3usize => value.is_none() || value.unwrap() == T ! [...],
				4usize => value.is_none() || TsType::can_cast(value.unwrap()),
				_ => false,
			},
			TS_TYPE_ALIAS_DECL => match index {
				0usize => value.is_none() || value.unwrap() == T![type],
				1usize => value.is_none() || TsTypeParams::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T ! [=],
				3usize => value.is_none() || TsType::can_cast(value.unwrap()),
				_ => false,
			},
			TS_TYPE_ANNOTATION => match index {
				0usize => value.is_none() || value.unwrap() == T ! [:],
				1usize => value.is_none() || TsType::can_cast(value.unwrap()),
				_ => false,
			},
			TS_TYPE_ARGS => match index {
				0usize => value.is_none() || value.unwrap() == T ! [<],
				1usize => value.is_none() || TsTypeArgList::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T ! [>],
				_ => false,
			},
			TS_TYPE_NAME => match index {
				0usize => value.is_none() || Ident::can_cast(value.unwrap()),
				_ => false,
			},
			TS_TYPE_OPERATOR => match index {
				0usize => value.is_none() || TsType::can_cast(value.unwrap()),
				_ => false,
			},
			TS_TYPE_PARAM => match index {
				0usize => value.is_none() || Ident::can_cast(value.unwrap()),
				1usize => value.is_none() || TsConstraint::can_cast(value.unwrap()),
				2usize => value.is_none() || TsDefault::can_cast(value.unwrap()),
				_ => false,
			},
			TS_TYPE_PARAMS => match index {
				0usize => value.is_none() || value.unwrap() == T ! [<],
				1usize => value.is_none() || TsTypeParam::can_cast(value.unwrap()),
				2usize => value.is_none() || value.unwrap() == T ! [>],
				_ => false,
			},
			TS_TYPE_REF => match index {
				0usize => value.is_none() || TsEntityName::can_cast(value.unwrap()),
				1usize => value.is_none() || TsTypeArgs::can_cast(value.unwrap()),
				_ => false,
			},
			TS_UNDEFINED => match index {
				0usize => value.is_none() || value.unwrap() == T![undefined],
				_ => false,
			},
			TS_UNION => match index {
				0usize => value.is_none() || TsTypeList::can_cast(value.unwrap()),
				_ => false,
			},
			TS_UNKNOWN => match index {
				0usize => value.is_none() || value.unwrap() == T![unknown],
				_ => false,
			},
			TS_VOID => match index {
				0usize => value.is_none() || value.unwrap() == T![void],
				_ => false,
			},
			EXPORT_NAMED_SPECIFIER_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || Specifier::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || JsAnyArrayAssignmentPatternElement::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || JsAnyArrayBindingPatternElement::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			JS_ARRAY_ELEMENT_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || JsAnyArrayElement::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			JS_CALL_ARGUMENT_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || JsAnyExpression::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			JS_CLASS_MEMBER_LIST => value.is_none() || JsAnyClassMember::can_cast(value.unwrap()),
			JS_CONSTRUCTOR_PARAMETER_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || JsAnyConstructorParameter::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			JS_DIRECTIVE_LIST => value.is_none() || JsDirective::can_cast(value.unwrap()),
			JS_IMPORT_ASSERTION_ENTRY_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || JsAnyImportAssertionEntry::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			JS_MODULE_ITEM_LIST => value.is_none() || JsAnyModuleItem::can_cast(value.unwrap()),
			JS_NAMED_IMPORT_SPECIFIER_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || JsAnyNamedImportSpecifier::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || JsAnyObjectAssignmentPatternMember::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || JsAnyObjectBindingPatternMember::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			JS_OBJECT_MEMBER_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || JsAnyObjectMember::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			JS_PARAMETER_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || JsAnyParameter::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			JS_STATEMENT_LIST => value.is_none() || JsAnyStatement::can_cast(value.unwrap()),
			JS_SWITCH_CASE_LIST => value.is_none() || JsAnySwitchClause::can_cast(value.unwrap()),
			JS_VARIABLE_DECLARATION_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || JsVariableDeclaration::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			TEMPLATE_ELEMENT_LIST => {
				value.is_none() || AnyTemplateElement::can_cast(value.unwrap())
			}
			TS_ENUM_MEMBER_LIST => value.is_none() || TsEnumMember::can_cast(value.unwrap()),
			TS_OBJECT_MEMBER_LIST => value.is_none() || TsTypeElement::can_cast(value.unwrap()),
			TS_TYPE_ARG_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || TsType::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			TS_TYPE_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || TsExprWithTypeArgs::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			TS_TYPE_PARAM_LIST => {
				let expects_element = index % 2 == 0;
				if expects_element {
					value.is_none() || TsTypeParam::can_cast(value.unwrap())
				} else {
					value.is_none() || value.unwrap() == T ! [,]
				}
			}
			_ => unreachable!("Is {:?} a token?", parent),
		}
	}
	fn validate_end(parent: Self::Kind, actual_len: usize) -> bool {
		match parent {
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
			CALL_EXPR => actual_len == 3usize,
			EXPORT_DECL => actual_len == 3usize,
			EXPORT_DEFAULT_DECL => actual_len == 4usize,
			EXPORT_DEFAULT_EXPR => actual_len == 4usize,
			EXPORT_NAMED => actual_len == 6usize,
			EXPORT_WILDCARD => actual_len == 7usize,
			FOR_STMT => actual_len == 9usize,
			FOR_STMT_TEST => actual_len == 1usize,
			FOR_STMT_UPDATE => actual_len == 1usize,
			IDENT => actual_len == 1usize,
			IMPORT_META => actual_len == 3usize,
			JS_ARRAY_ASSIGNMENT_PATTERN => actual_len == 3usize,
			JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => actual_len == 2usize,
			JS_ARRAY_BINDING_PATTERN => actual_len == 3usize,
			JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => actual_len == 2usize,
			JS_ARRAY_EXPRESSION => actual_len == 3usize,
			JS_ARRAY_HOLE => actual_len == 0usize,
			JS_ARROW_FUNCTION_EXPRESSION => actual_len == 6usize,
			JS_ASSIGNMENT_EXPRESSION => actual_len == 3usize,
			JS_ASSIGNMENT_WITH_DEFAULT => actual_len == 3usize,
			JS_AWAIT_EXPRESSION => actual_len == 2usize,
			JS_BIG_INT_LITERAL_EXPRESSION => actual_len == 1usize,
			JS_BINARY_EXPRESSION => actual_len == 3usize,
			JS_BINDING_PATTERN_WITH_DEFAULT => actual_len == 3usize,
			JS_BLOCK_STATEMENT => actual_len == 3usize,
			JS_BOOLEAN_LITERAL_EXPRESSION => actual_len == 1usize,
			JS_BREAK_STATEMENT => actual_len == 3usize,
			JS_CALL_ARGUMENTS => actual_len == 3usize,
			JS_CASE_CLAUSE => actual_len == 4usize,
			JS_CATCH_CLAUSE => actual_len == 3usize,
			JS_CATCH_DECLARATION => actual_len == 3usize,
			JS_CLASS_DECLARATION => actual_len == 7usize,
			JS_CLASS_EXPRESSION => actual_len == 7usize,
			JS_COMPUTED_MEMBER_ASSIGNMENT => actual_len == 4usize,
			JS_COMPUTED_MEMBER_EXPRESSION => actual_len == 5usize,
			JS_COMPUTED_MEMBER_NAME => actual_len == 3usize,
			JS_CONDITIONAL_EXPRESSION => actual_len == 5usize,
			JS_CONSTRUCTOR_CLASS_MEMBER => actual_len == 5usize,
			JS_CONSTRUCTOR_PARAMETERS => actual_len == 3usize,
			JS_CONTINUE_STATEMENT => actual_len == 3usize,
			JS_DEBUGGER_STATEMENT => actual_len == 2usize,
			JS_DEFAULT_CLAUSE => actual_len == 3usize,
			JS_DEFAULT_IMPORT_SPECIFIER => actual_len == 2usize,
			JS_DIRECTIVE => actual_len == 2usize,
			JS_DO_WHILE_STATEMENT => actual_len == 7usize,
			JS_ELSE_CLAUSE => actual_len == 2usize,
			JS_EMPTY_CLASS_MEMBER => actual_len == 1usize,
			JS_EMPTY_STATEMENT => actual_len == 1usize,
			JS_EXPRESSION_SNIPPED => actual_len == 2usize,
			JS_EXPRESSION_STATEMENT => actual_len == 2usize,
			JS_EXTENDS_CLAUSE => actual_len == 2usize,
			JS_FINALLY_CLAUSE => actual_len == 2usize,
			JS_FOR_IN_STATEMENT => actual_len == 7usize,
			JS_FOR_OF_STATEMENT => actual_len == 8usize,
			JS_FOR_VARIABLE_DECLARATION => actual_len == 2usize,
			JS_FUNCTION_BODY => actual_len == 4usize,
			JS_FUNCTION_DECLARATION => actual_len == 8usize,
			JS_FUNCTION_EXPRESSION => actual_len == 8usize,
			JS_GETTER_CLASS_MEMBER => actual_len == 9usize,
			JS_GETTER_OBJECT_MEMBER => actual_len == 6usize,
			JS_IDENTIFIER_ASSIGNMENT => actual_len == 1usize,
			JS_IDENTIFIER_BINDING => actual_len == 1usize,
			JS_IDENTIFIER_EXPRESSION => actual_len == 1usize,
			JS_IF_STATEMENT => actual_len == 6usize,
			JS_IMPORT => actual_len == 3usize,
			JS_IMPORT_ASSERTION => actual_len == 4usize,
			JS_IMPORT_ASSERTION_ENTRY => actual_len == 3usize,
			JS_IMPORT_BARE_CLAUSE => actual_len == 2usize,
			JS_IMPORT_CALL_EXPRESSION => actual_len == 4usize,
			JS_IMPORT_DEFAULT_CLAUSE => actual_len == 4usize,
			JS_IMPORT_NAMED_CLAUSE => actual_len == 5usize,
			JS_IMPORT_NAMESPACE_CLAUSE => actual_len == 6usize,
			JS_INITIALIZER_CLAUSE => actual_len == 2usize,
			JS_LABELED_STATEMENT => actual_len == 3usize,
			JS_LITERAL_EXPORT_NAME => actual_len == 1usize,
			JS_LITERAL_MEMBER_NAME => actual_len == 1usize,
			JS_LOGICAL_EXPRESSION => actual_len == 3usize,
			JS_METHOD_CLASS_MEMBER => actual_len == 10usize,
			JS_METHOD_OBJECT_MEMBER => actual_len == 7usize,
			JS_MODULE => actual_len == 4usize,
			JS_MODULE_SOURCE => actual_len == 1usize,
			JS_NAME => actual_len == 1usize,
			JS_NAMED_IMPORT_SPECIFIER => actual_len == 3usize,
			JS_NAMED_IMPORT_SPECIFIERS => actual_len == 3usize,
			JS_NAMESPACE_IMPORT_SPECIFIER => actual_len == 3usize,
			JS_NULL_LITERAL_EXPRESSION => actual_len == 1usize,
			JS_NUMBER_LITERAL_EXPRESSION => actual_len == 1usize,
			JS_OBJECT_ASSIGNMENT_PATTERN => actual_len == 3usize,
			JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => actual_len == 4usize,
			JS_OBJECT_ASSIGNMENT_PATTERN_REST => actual_len == 2usize,
			JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => actual_len == 2usize,
			JS_OBJECT_BINDING_PATTERN => actual_len == 3usize,
			JS_OBJECT_BINDING_PATTERN_PROPERTY => actual_len == 4usize,
			JS_OBJECT_BINDING_PATTERN_REST => actual_len == 2usize,
			JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => actual_len == 2usize,
			JS_OBJECT_EXPRESSION => actual_len == 3usize,
			JS_PARAMETERS => actual_len == 3usize,
			JS_PARENTHESIZED_ASSIGNMENT => actual_len == 3usize,
			JS_PARENTHESIZED_EXPRESSION => actual_len == 3usize,
			JS_POST_UPDATE_EXPRESSION => actual_len == 2usize,
			JS_PRE_UPDATE_EXPRESSION => actual_len == 2usize,
			JS_PRIVATE_CLASS_MEMBER_NAME => actual_len == 2usize,
			JS_PRIVATE_NAME => actual_len == 2usize,
			JS_PROPERTY_CLASS_MEMBER => actual_len == 11usize,
			JS_PROPERTY_OBJECT_MEMBER => actual_len == 3usize,
			JS_REFERENCE_IDENTIFIER => actual_len == 1usize,
			JS_REGEX_LITERAL_EXPRESSION => actual_len == 1usize,
			JS_REST_PARAMETER => actual_len == 2usize,
			JS_RETURN_STATEMENT => actual_len == 3usize,
			JS_SCRIPT => actual_len == 4usize,
			JS_SEQUENCE_EXPRESSION => actual_len == 3usize,
			JS_SETTER_CLASS_MEMBER => actual_len == 9usize,
			JS_SETTER_OBJECT_MEMBER => actual_len == 6usize,
			JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => actual_len == 1usize,
			JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => actual_len == 1usize,
			JS_SPREAD => actual_len == 2usize,
			JS_STATIC_MEMBER_ASSIGNMENT => actual_len == 3usize,
			JS_STATIC_MEMBER_EXPRESSION => actual_len == 3usize,
			JS_STRING_LITERAL_EXPRESSION => actual_len == 1usize,
			JS_SUPER_EXPRESSION => actual_len == 1usize,
			JS_SWITCH_STATEMENT => actual_len == 7usize,
			JS_THIS_EXPRESSION => actual_len == 1usize,
			JS_THROW_STATEMENT => actual_len == 3usize,
			JS_TRY_FINALLY_STATEMENT => actual_len == 4usize,
			JS_TRY_STATEMENT => actual_len == 3usize,
			JS_UNARY_EXPRESSION => actual_len == 2usize,
			JS_VARIABLE_DECLARATION => actual_len == 4usize,
			JS_VARIABLE_DECLARATIONS => actual_len == 2usize,
			JS_VARIABLE_STATEMENT => actual_len == 2usize,
			JS_WHILE_STATEMENT => actual_len == 5usize,
			JS_WITH_STATEMENT => actual_len == 5usize,
			JS_YIELD_EXPRESSION => actual_len == 3usize,
			NEW_EXPR => actual_len == 4usize,
			NEW_TARGET => actual_len == 3usize,
			SPECIFIER => actual_len == 3usize,
			TEMPLATE => actual_len == 4usize,
			TEMPLATE_CHUNK_ELEMENT => actual_len == 1usize,
			TEMPLATE_ELEMENT => actual_len == 3usize,
			TS_ANY => actual_len == 1usize,
			TS_ARRAY => actual_len == 3usize,
			TS_ASSERTION => actual_len == 5usize,
			TS_BIGINT => actual_len == 1usize,
			TS_BOOLEAN => actual_len == 1usize,
			TS_CALL_SIGNATURE_DECL => actual_len == 4usize,
			TS_CONDITIONAL_TYPE => actual_len == 4usize,
			TS_CONST_ASSERTION => actual_len == 5usize,
			TS_CONSTRAINT => actual_len == 2usize,
			TS_CONSTRUCT_SIGNATURE_DECL => actual_len == 5usize,
			TS_CONSTRUCTOR_PARAM => actual_len == 3usize,
			TS_CONSTRUCTOR_TYPE => actual_len == 4usize,
			TS_DEFAULT => actual_len == 2usize,
			TS_ENUM => actual_len == 6usize,
			TS_ENUM_MEMBER => actual_len == 3usize,
			TS_EXPORT_ASSIGNMENT => actual_len == 4usize,
			TS_EXPR_WITH_TYPE_ARGS => actual_len == 2usize,
			TS_EXTENDS => actual_len == 2usize,
			TS_EXTERNAL_MODULE_REF => actual_len == 4usize,
			TS_FN_TYPE => actual_len == 3usize,
			TS_IMPLEMENTS_CLAUSE => actual_len == 2usize,
			TS_IMPORT => actual_len == 6usize,
			TS_IMPORT_EQUALS_DECL => actual_len == 6usize,
			TS_INDEX_SIGNATURE => actual_len == 6usize,
			TS_INDEXED_ARRAY => actual_len == 3usize,
			TS_INFER => actual_len == 2usize,
			TS_INTERFACE_DECL => actual_len == 8usize,
			TS_INTERSECTION => actual_len == 1usize,
			TS_LITERAL => actual_len == 1usize,
			TS_MAPPED_TYPE => actual_len == 10usize,
			TS_MAPPED_TYPE_PARAM => actual_len == 5usize,
			TS_MAPPED_TYPE_READONLY => actual_len == 3usize,
			TS_METHOD_SIGNATURE => actual_len == 7usize,
			TS_MODULE_BLOCK => actual_len == 3usize,
			TS_MODULE_DECL => actual_len == 6usize,
			TS_NAMESPACE_DECL => actual_len == 4usize,
			TS_NAMESPACE_EXPORT_DECL => actual_len == 5usize,
			TS_NEVER => actual_len == 1usize,
			TS_NON_NULL => actual_len == 2usize,
			TS_NULL => actual_len == 1usize,
			TS_NUMBER => actual_len == 1usize,
			TS_OBJECT => actual_len == 1usize,
			TS_OBJECT_TYPE => actual_len == 3usize,
			TS_PAREN => actual_len == 3usize,
			TS_PREDICATE => actual_len == 2usize,
			TS_PROPERTY_SIGNATURE => actual_len == 5usize,
			TS_QUALIFIED_PATH => actual_len == 3usize,
			TS_STRING => actual_len == 1usize,
			TS_SYMBOL => actual_len == 1usize,
			TS_TEMPLATE => actual_len == 1usize,
			TS_TEMPLATE_ELEMENT => actual_len == 2usize,
			TS_THIS => actual_len == 1usize,
			TS_TUPLE => actual_len == 3usize,
			TS_TUPLE_ELEMENT => actual_len == 5usize,
			TS_TYPE_ALIAS_DECL => actual_len == 4usize,
			TS_TYPE_ANNOTATION => actual_len == 2usize,
			TS_TYPE_ARGS => actual_len == 3usize,
			TS_TYPE_NAME => actual_len == 1usize,
			TS_TYPE_OPERATOR => actual_len == 1usize,
			TS_TYPE_PARAM => actual_len == 3usize,
			TS_TYPE_PARAMS => actual_len == 3usize,
			TS_TYPE_REF => actual_len == 2usize,
			TS_UNDEFINED => actual_len == 1usize,
			TS_UNION => actual_len == 1usize,
			TS_UNKNOWN => actual_len == 1usize,
			TS_VOID => actual_len == 1usize,
			EXPORT_NAMED_SPECIFIER_LIST => true,
			JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST => true,
			JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST => true,
			JS_ARRAY_ELEMENT_LIST => true,
			JS_CALL_ARGUMENT_LIST => true,
			JS_CLASS_MEMBER_LIST => true,
			JS_CONSTRUCTOR_PARAMETER_LIST => true,
			JS_DIRECTIVE_LIST => true,
			JS_IMPORT_ASSERTION_ENTRY_LIST => true,
			JS_MODULE_ITEM_LIST => true,
			JS_NAMED_IMPORT_SPECIFIER_LIST => true,
			JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST => true,
			JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST => true,
			JS_OBJECT_MEMBER_LIST => true,
			JS_PARAMETER_LIST => true,
			JS_STATEMENT_LIST => true,
			JS_SWITCH_CASE_LIST => true,
			JS_VARIABLE_DECLARATION_LIST => actual_len == 0 || actual_len % 2 == 1,
			TEMPLATE_ELEMENT_LIST => true,
			TS_ENUM_MEMBER_LIST => true,
			TS_OBJECT_MEMBER_LIST => true,
			TS_TYPE_ARG_LIST => actual_len == 0 || actual_len % 2 == 1,
			TS_TYPE_LIST => actual_len == 0 || actual_len % 2 == 1,
			TS_TYPE_PARAM_LIST => actual_len == 0 || actual_len % 2 == 1,
			_ => unreachable!("Is {:?} a token?", parent),
		}
	}
}
