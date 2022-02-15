//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](SyntaxNode::kind)"]
#[doc = r" of the provided [SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" The macro accepts an optional fallback branch wich defaults to"]
#[doc = r" `unreachable!()` as the only SyntaxKind variants not covered by"]
#[doc = r" this macro are token kinds that should not be used to construct"]
#[doc = r" a SyntaxNode."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r#" map_syntax_node!(syntax_node, node => Ok(node.format()), _ => Err("invalid node kind"))"#]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
$crate :: map_syntax_node ! ($node , $pattern => $body , _ => unreachable ! ())
    };
    ($ node : expr , $ pattern : pat => $ body : expr , $ fallback : pat => $ default : expr) => {
        match $node {
            node => match $crate::SyntaxNode::kind(&node) {
                $crate::JsSyntaxKind::IMPORT_META => {
                    let $pattern = unsafe { $crate::ast::ImportMeta::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN => {
                    let $pattern =
                        unsafe { $crate::ast::JsArrayAssignmentPattern::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => {
                    let $pattern = unsafe {
                        $crate::ast::JsArrayAssignmentPatternRestElement::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_BINDING_PATTERN => {
                    let $pattern =
                        unsafe { $crate::ast::JsArrayBindingPattern::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => {
                    let $pattern = unsafe {
                        $crate::ast::JsArrayBindingPatternRestElement::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_EXPRESSION => {
                    let $pattern = unsafe { $crate::ast::JsArrayExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_HOLE => {
                    let $pattern = unsafe { $crate::ast::JsArrayHole::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsArrowFunctionExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsAssignmentExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ASSIGNMENT_WITH_DEFAULT => {
                    let $pattern =
                        unsafe { $crate::ast::JsAssignmentWithDefault::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_AWAIT_EXPRESSION => {
                    let $pattern = unsafe { $crate::ast::JsAwaitExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BIG_INT_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsBigIntLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BINARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::ast::JsBinaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BINDING_PATTERN_WITH_DEFAULT => {
                    let $pattern =
                        unsafe { $crate::ast::JsBindingPatternWithDefault::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BLOCK_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsBlockStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsBooleanLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BREAK_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsBreakStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CALL_ARGUMENTS => {
                    let $pattern = unsafe { $crate::ast::JsCallArguments::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CALL_EXPRESSION => {
                    let $pattern = unsafe { $crate::ast::JsCallExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CASE_CLAUSE => {
                    let $pattern = unsafe { $crate::ast::JsCaseClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CATCH_CLAUSE => {
                    let $pattern = unsafe { $crate::ast::JsCatchClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CATCH_DECLARATION => {
                    let $pattern = unsafe { $crate::ast::JsCatchDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CLASS_DECLARATION => {
                    let $pattern = unsafe { $crate::ast::JsClassDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CLASS_EXPRESSION => {
                    let $pattern = unsafe { $crate::ast::JsClassExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                    let $pattern =
                        unsafe { $crate::ast::JsComputedMemberAssignment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsComputedMemberExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_COMPUTED_MEMBER_NAME => {
                    let $pattern =
                        unsafe { $crate::ast::JsComputedMemberName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsConditionalExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { $crate::ast::JsConstructorClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CONSTRUCTOR_PARAMETERS => {
                    let $pattern =
                        unsafe { $crate::ast::JsConstructorParameters::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CONTINUE_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsContinueStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_DEBUGGER_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsDebuggerStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_DEFAULT_CLAUSE => {
                    let $pattern = unsafe { $crate::ast::JsDefaultClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_DEFAULT_IMPORT_SPECIFIER => {
                    let $pattern =
                        unsafe { $crate::ast::JsDefaultImportSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_DIRECTIVE => {
                    let $pattern = unsafe { $crate::ast::JsDirective::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_DO_WHILE_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsDoWhileStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ELSE_CLAUSE => {
                    let $pattern = unsafe { $crate::ast::JsElseClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EMPTY_CLASS_MEMBER => {
                    let $pattern = unsafe { $crate::ast::JsEmptyClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EMPTY_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsEmptyStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT => {
                    let $pattern = unsafe { $crate::ast::JsExport::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_AS_CLAUSE => {
                    let $pattern = unsafe { $crate::ast::JsExportAsClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_DEFAULT_CLASS_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::ast::JsExportDefaultClassClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE => {
                    let $pattern = unsafe {
                        $crate::ast::JsExportDefaultExpressionClause::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_DEFAULT_FUNCTION_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::ast::JsExportDefaultFunctionClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_FROM_CLAUSE => {
                    let $pattern = unsafe { $crate::ast::JsExportFromClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_NAMED_CLAUSE => {
                    let $pattern = unsafe { $crate::ast::JsExportNamedClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_NAMED_FROM_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::ast::JsExportNamedFromClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER => {
                    let $pattern =
                        unsafe { $crate::ast::JsExportNamedFromSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_NAMED_SHORTHAND_SPECIFIER => {
                    let $pattern = unsafe {
                        $crate::ast::JsExportNamedShorthandSpecifier::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER => {
                    let $pattern =
                        unsafe { $crate::ast::JsExportNamedSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPRESSION_SNIPPED => {
                    let $pattern = unsafe { $crate::ast::JsExpressionSnipped::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPRESSION_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::ast::JsExpressionStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXTENDS_CLAUSE => {
                    let $pattern = unsafe { $crate::ast::JsExtendsClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FINALLY_CLAUSE => {
                    let $pattern = unsafe { $crate::ast::JsFinallyClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FOR_IN_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsForInStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FOR_OF_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsForOfStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FOR_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsForStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FOR_VARIABLE_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::ast::JsForVariableDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FORMAL_PARAMETER => {
                    let $pattern = unsafe { $crate::ast::JsFormalParameter::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FUNCTION_BODY => {
                    let $pattern = unsafe { $crate::ast::JsFunctionBody::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FUNCTION_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::ast::JsFunctionDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FUNCTION_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsFunctionExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_GETTER_CLASS_MEMBER => {
                    let $pattern = unsafe { $crate::ast::JsGetterClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_GETTER_OBJECT_MEMBER => {
                    let $pattern =
                        unsafe { $crate::ast::JsGetterObjectMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT => {
                    let $pattern =
                        unsafe { $crate::ast::JsIdentifierAssignment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IDENTIFIER_BINDING => {
                    let $pattern = unsafe { $crate::ast::JsIdentifierBinding::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IDENTIFIER_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsIdentifierExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IF_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsIfStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT => {
                    let $pattern = unsafe { $crate::ast::JsImport::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_ASSERTION => {
                    let $pattern = unsafe { $crate::ast::JsImportAssertion::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY => {
                    let $pattern =
                        unsafe { $crate::ast::JsImportAssertionEntry::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_BARE_CLAUSE => {
                    let $pattern = unsafe { $crate::ast::JsImportBareClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_CALL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsImportCallExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::ast::JsImportDefaultClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE => {
                    let $pattern = unsafe { $crate::ast::JsImportNamedClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::ast::JsImportNamespaceClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IN_EXPRESSION => {
                    let $pattern = unsafe { $crate::ast::JsInExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
                    let $pattern = unsafe { $crate::ast::JsInitializerClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_INSTANCEOF_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsInstanceofExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_LABELED_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsLabeledStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_LITERAL_EXPORT_NAME => {
                    let $pattern = unsafe { $crate::ast::JsLiteralExportName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_LITERAL_MEMBER_NAME => {
                    let $pattern = unsafe { $crate::ast::JsLiteralMemberName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_LOGICAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::ast::JsLogicalExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_METHOD_CLASS_MEMBER => {
                    let $pattern = unsafe { $crate::ast::JsMethodClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_METHOD_OBJECT_MEMBER => {
                    let $pattern =
                        unsafe { $crate::ast::JsMethodObjectMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_MODULE => {
                    let $pattern = unsafe { $crate::ast::JsModule::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_MODULE_SOURCE => {
                    let $pattern = unsafe { $crate::ast::JsModuleSource::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NAME => {
                    let $pattern = unsafe { $crate::ast::JsName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER => {
                    let $pattern =
                        unsafe { $crate::ast::JsNamedImportSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIERS => {
                    let $pattern =
                        unsafe { $crate::ast::JsNamedImportSpecifiers::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NAMESPACE_IMPORT_SPECIFIER => {
                    let $pattern =
                        unsafe { $crate::ast::JsNamespaceImportSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NEW_EXPRESSION => {
                    let $pattern = unsafe { $crate::ast::JsNewExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NULL_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsNullLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsNumberLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN => {
                    let $pattern =
                        unsafe { $crate::ast::JsObjectAssignmentPattern::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => {
                    let $pattern = unsafe {
                        $crate::ast::JsObjectAssignmentPatternProperty::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_REST => {
                    let $pattern =
                        unsafe { $crate::ast::JsObjectAssignmentPatternRest::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => {
                    let $pattern = unsafe {
                        $crate::ast::JsObjectAssignmentPatternShorthandProperty::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN => {
                    let $pattern =
                        unsafe { $crate::ast::JsObjectBindingPattern::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY => {
                    let $pattern =
                        unsafe { $crate::ast::JsObjectBindingPatternProperty::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST => {
                    let $pattern =
                        unsafe { $crate::ast::JsObjectBindingPatternRest::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => {
                    let $pattern = unsafe {
                        $crate::ast::JsObjectBindingPatternShorthandProperty::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_EXPRESSION => {
                    let $pattern = unsafe { $crate::ast::JsObjectExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PARAMETERS => {
                    let $pattern = unsafe { $crate::ast::JsParameters::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PARENTHESIZED_ASSIGNMENT => {
                    let $pattern =
                        unsafe { $crate::ast::JsParenthesizedAssignment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsParenthesizedExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_POST_UPDATE_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsPostUpdateExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsPreUpdateExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PRIVATE_CLASS_MEMBER_NAME => {
                    let $pattern =
                        unsafe { $crate::ast::JsPrivateClassMemberName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PRIVATE_NAME => {
                    let $pattern = unsafe { $crate::ast::JsPrivateName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { $crate::ast::JsPropertyClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER => {
                    let $pattern =
                        unsafe { $crate::ast::JsPropertyObjectMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_REFERENCE_IDENTIFIER => {
                    let $pattern =
                        unsafe { $crate::ast::JsReferenceIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_REGEX_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsRegexLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_REST_PARAMETER => {
                    let $pattern = unsafe { $crate::ast::JsRestParameter::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_RETURN_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsReturnStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SCRIPT => {
                    let $pattern = unsafe { $crate::ast::JsScript::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsSequenceExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SETTER_CLASS_MEMBER => {
                    let $pattern = unsafe { $crate::ast::JsSetterClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SETTER_OBJECT_MEMBER => {
                    let $pattern =
                        unsafe { $crate::ast::JsSetterObjectMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
                    let $pattern = unsafe {
                        $crate::ast::JsShorthandNamedImportSpecifier::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
                    let $pattern = unsafe {
                        $crate::ast::JsShorthandPropertyObjectMember::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_SPREAD => {
                    let $pattern = unsafe { $crate::ast::JsSpread::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER => {
                    let $pattern = unsafe {
                        $crate::ast::JsStaticInitializationBlockClassMember::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => {
                    let $pattern =
                        unsafe { $crate::ast::JsStaticMemberAssignment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsStaticMemberExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::JsStringLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SUPER_EXPRESSION => {
                    let $pattern = unsafe { $crate::ast::JsSuperExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SWITCH_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsSwitchStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_TEMPLATE => {
                    let $pattern = unsafe { $crate::ast::JsTemplate::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_TEMPLATE_CHUNK_ELEMENT => {
                    let $pattern =
                        unsafe { $crate::ast::JsTemplateChunkElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_TEMPLATE_ELEMENT => {
                    let $pattern = unsafe { $crate::ast::JsTemplateElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_THIS_EXPRESSION => {
                    let $pattern = unsafe { $crate::ast::JsThisExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_THROW_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsThrowStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_TRY_FINALLY_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::ast::JsTryFinallyStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_TRY_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsTryStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_UNARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::ast::JsUnaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_VARIABLE_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::ast::JsVariableDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_VARIABLE_DECLARATION_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::ast::JsVariableDeclarationClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_VARIABLE_DECLARATOR => {
                    let $pattern =
                        unsafe { $crate::ast::JsVariableDeclarator::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_VARIABLE_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsVariableStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_WHILE_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsWhileStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_WITH_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsWithStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_YIELD_ARGUMENT => {
                    let $pattern = unsafe { $crate::ast::JsYieldArgument::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_YIELD_EXPRESSION => {
                    let $pattern = unsafe { $crate::ast::JsYieldExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::NEW_TARGET => {
                    let $pattern = unsafe { $crate::ast::NewTarget::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_ANY_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsAnyType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_ARRAY_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsArrayType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_AS_EXPRESSION => {
                    let $pattern = unsafe { $crate::ast::TsAsExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_BIG_INT_LITERAL_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsBigIntLiteralType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_BIGINT_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsBigintType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_BOOLEAN_LITERAL_TYPE => {
                    let $pattern =
                        unsafe { $crate::ast::TsBooleanLiteralType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_BOOLEAN_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsBooleanType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_CALL_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { $crate::ast::TsCallSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_CONDITIONAL_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsConditionalType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { $crate::ast::TsConstructSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_CONSTRUCTOR_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsConstructorType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::ast::TsDeclareFunctionDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_DECLARE_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::TsDeclareStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_DEFAULT_TYPE_CLAUSE => {
                    let $pattern = unsafe { $crate::ast::TsDefaultTypeClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_DEFINITE_PROPERTY_ANNOTATION => {
                    let $pattern =
                        unsafe { $crate::ast::TsDefinitePropertyAnnotation::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_DEFINITE_VARIABLE_ANNOTATION => {
                    let $pattern =
                        unsafe { $crate::ast::TsDefiniteVariableAnnotation::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY => {
                    let $pattern = unsafe {
                        $crate::ast::TsEmptyExternalModuleDeclarationBody::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::TS_ENUM_DECLARATION => {
                    let $pattern = unsafe { $crate::ast::TsEnumDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_ENUM_MEMBER => {
                    let $pattern = unsafe { $crate::ast::TsEnumMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_EXTENDS_CLAUSE => {
                    let $pattern = unsafe { $crate::ast::TsExtendsClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::ast::TsExternalModuleDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_EXTERNAL_MODULE_REF => {
                    let $pattern = unsafe { $crate::ast::TsExternalModuleRef::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_FUNCTION_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsFunctionType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_GETTER_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { $crate::ast::TsGetterSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_GLOBAL_DECLARATION => {
                    let $pattern = unsafe { $crate::ast::TsGlobalDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_IDENTIFIER_BINDING => {
                    let $pattern = unsafe { $crate::ast::TsIdentifierBinding::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_IMPLEMENTS_CLAUSE => {
                    let $pattern = unsafe { $crate::ast::TsImplementsClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_IMPORT_EQUALS_DECL => {
                    let $pattern = unsafe { $crate::ast::TsImportEqualsDecl::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_IMPORT_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsImportType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_IMPORT_TYPE_QUALIFIER => {
                    let $pattern =
                        unsafe { $crate::ast::TsImportTypeQualifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INDEX_SIGNATURE_PARAMETER => {
                    let $pattern =
                        unsafe { $crate::ast::TsIndexSignatureParameter::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INDEX_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { $crate::ast::TsIndexSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INDEXED_ACCESS_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsIndexedAccessType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INFER_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsInferType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INTERFACE_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::ast::TsInterfaceDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INTERSECTION_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsIntersectionType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_MAPPED_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsMappedType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_MAPPED_TYPE_AS_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::ast::TsMappedTypeAsClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE => {
                    let $pattern = unsafe {
                        $crate::ast::TsMappedTypeOptionalModifierClause::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE => {
                    let $pattern = unsafe {
                        $crate::ast::TsMappedTypeReadonlyModifierClause::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::TS_METHOD_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { $crate::ast::TsMethodSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_MODULE_BLOCK => {
                    let $pattern = unsafe { $crate::ast::TsModuleBlock::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_MODULE_DECLARATION => {
                    let $pattern = unsafe { $crate::ast::TsModuleDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NAME_WITH_TYPE_ARGUMENTS => {
                    let $pattern =
                        unsafe { $crate::ast::TsNameWithTypeArguments::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NAMED_TUPLE_TYPE_ELEMENT => {
                    let $pattern =
                        unsafe { $crate::ast::TsNamedTupleTypeElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NEVER_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsNeverType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT => {
                    let $pattern =
                        unsafe { $crate::ast::TsNonNullAssertionAssignment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::TsNonNullAssertionExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NON_PRIMITIVE_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsNonPrimitiveType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NULL_LITERAL_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsNullLiteralType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NUMBER_LITERAL_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsNumberLiteralType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NUMBER_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsNumberType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_OBJECT_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsObjectType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_OPTIONAL_PROPERTY_ANNOTATION => {
                    let $pattern =
                        unsafe { $crate::ast::TsOptionalPropertyAnnotation::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_OPTIONAL_TUPLE_TYPE_ELEMENT => {
                    let $pattern =
                        unsafe { $crate::ast::TsOptionalTupleTypeElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_PARENTHESIZED_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsParenthesizedType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_PROPERTY_PARAMETER => {
                    let $pattern = unsafe { $crate::ast::TsPropertyParameter::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_PROPERTY_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { $crate::ast::TsPropertySignatureTypeMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_QUALIFIED_MODULE_NAME => {
                    let $pattern =
                        unsafe { $crate::ast::TsQualifiedModuleName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_QUALIFIED_NAME => {
                    let $pattern = unsafe { $crate::ast::TsQualifiedName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_READONLY_PROPERTY_PARAMETER => {
                    let $pattern =
                        unsafe { $crate::ast::TsReadonlyPropertyParameter::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_REFERENCE_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsReferenceType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_REST_TUPLE_TYPE_ELEMENT => {
                    let $pattern =
                        unsafe { $crate::ast::TsRestTupleTypeElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_RETURN_TYPE_ANNOTATION => {
                    let $pattern =
                        unsafe { $crate::ast::TsReturnTypeAnnotation::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_SETTER_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { $crate::ast::TsSetterSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_STRING_LITERAL_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsStringLiteralType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_STRING_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsStringType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_SYMBOL_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsSymbolType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TEMPLATE_CHUNK_ELEMENT => {
                    let $pattern =
                        unsafe { $crate::ast::TsTemplateChunkElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TEMPLATE_ELEMENT => {
                    let $pattern = unsafe { $crate::ast::TsTemplateElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TEMPLATE_LITERAL_TYPE => {
                    let $pattern =
                        unsafe { $crate::ast::TsTemplateLiteralType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_THIS_PARAMETER => {
                    let $pattern = unsafe { $crate::ast::TsThisParameter::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_THIS_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsThisType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TUPLE_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsTupleType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::ast::TsTypeAliasDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_ANNOTATION => {
                    let $pattern = unsafe { $crate::ast::TsTypeAnnotation::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_ARGUMENTS => {
                    let $pattern = unsafe { $crate::ast::TsTypeArguments::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::ast::TsTypeAssertionExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_CONSTRAINT_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::ast::TsTypeConstraintClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_OPERATOR_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsTypeOperatorType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_PARAMETER => {
                    let $pattern = unsafe { $crate::ast::TsTypeParameter::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_PARAMETER_NAME => {
                    let $pattern = unsafe { $crate::ast::TsTypeParameterName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_PARAMETERS => {
                    let $pattern = unsafe { $crate::ast::TsTypeParameters::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_PREDICATE => {
                    let $pattern = unsafe { $crate::ast::TsTypePredicate::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPEOF_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsTypeofType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_UNDEFINED_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsUndefinedType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_UNION_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsUnionType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_UNKNOWN_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsUnknownType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_VOID_TYPE => {
                    let $pattern = unsafe { $crate::ast::TsVoidType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_UNKNOWN => {
                    let $pattern = unsafe { $crate::ast::JsUnknown::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_UNKNOWN_ASSIGNMENT => {
                    let $pattern = unsafe { $crate::ast::JsUnknownAssignment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_UNKNOWN_BINDING => {
                    let $pattern = unsafe { $crate::ast::JsUnknownBinding::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_UNKNOWN_EXPRESSION => {
                    let $pattern = unsafe { $crate::ast::JsUnknownExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_UNKNOWN_IMPORT_ASSERTION_ENTRY => {
                    let $pattern =
                        unsafe { $crate::ast::JsUnknownImportAssertionEntry::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_UNKNOWN_MEMBER => {
                    let $pattern = unsafe { $crate::ast::JsUnknownMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_UNKNOWN_NAMED_IMPORT_SPECIFIER => {
                    let $pattern =
                        unsafe { $crate::ast::JsUnknownNamedImportSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_UNKNOWN_PARAMETER => {
                    let $pattern = unsafe { $crate::ast::JsUnknownParameter::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_UNKNOWN_STATEMENT => {
                    let $pattern = unsafe { $crate::ast::JsUnknownStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST => {
                    let $pattern = unsafe {
                        $crate::ast::JsArrayAssignmentPatternElementList::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST => {
                    let $pattern = unsafe {
                        $crate::ast::JsArrayBindingPatternElementList::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::ast::JsArrayElementList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CALL_ARGUMENT_LIST => {
                    let $pattern = unsafe { $crate::ast::JsCallArgumentList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CLASS_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::ast::JsClassMemberList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CONSTRUCTOR_PARAMETER_LIST => {
                    let $pattern =
                        unsafe { $crate::ast::JsConstructorParameterList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_DIRECTIVE_LIST => {
                    let $pattern = unsafe { $crate::ast::JsDirectiveList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER_LIST => {
                    let $pattern =
                        unsafe { $crate::ast::JsExportNamedFromSpecifierList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER_LIST => {
                    let $pattern =
                        unsafe { $crate::ast::JsExportNamedSpecifierList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY_LIST => {
                    let $pattern =
                        unsafe { $crate::ast::JsImportAssertionEntryList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_MODULE_ITEM_LIST => {
                    let $pattern = unsafe { $crate::ast::JsModuleItemList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER_LIST => {
                    let $pattern =
                        unsafe { $crate::ast::JsNamedImportSpecifierList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST => {
                    let $pattern = unsafe {
                        $crate::ast::JsObjectAssignmentPatternPropertyList::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST => {
                    let $pattern = unsafe {
                        $crate::ast::JsObjectBindingPatternPropertyList::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::ast::JsObjectMemberList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PARAMETER_LIST => {
                    let $pattern = unsafe { $crate::ast::JsParameterList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_STATEMENT_LIST => {
                    let $pattern = unsafe { $crate::ast::JsStatementList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SWITCH_CASE_LIST => {
                    let $pattern = unsafe { $crate::ast::JsSwitchCaseList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_TEMPLATE_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { $crate::ast::JsTemplateElementList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_VARIABLE_DECLARATOR_LIST => {
                    let $pattern =
                        unsafe { $crate::ast::JsVariableDeclaratorList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_ENUM_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::ast::TsEnumMemberList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INTERSECTION_TYPE_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { $crate::ast::TsIntersectionTypeElementList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TEMPLATE_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { $crate::ast::TsTemplateElementList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TUPLE_TYPE_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { $crate::ast::TsTupleTypeElementList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_ARGUMENT_LIST => {
                    let $pattern = unsafe { $crate::ast::TsTypeArgumentList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_LIST => {
                    let $pattern = unsafe { $crate::ast::TsTypeList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::ast::TsTypeMemberList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_PARAMETER_LIST => {
                    let $pattern = unsafe { $crate::ast::TsTypeParameterList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST => {
                    let $pattern =
                        unsafe { $crate::ast::TsUnionTypeVariantList::new_unchecked(node) };
                    $body
                }
                $fallback => $default,
            },
        }
    };
}
