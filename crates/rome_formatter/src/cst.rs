use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyNamedImportSpecifier::JsNamedImportSpecifier;
use rslint_parser::ast::{
    ImportMeta, JsArrayBindingPattern, JsArrayExpression, JsArrowFunctionExpression,
    JsBlockStatement, JsBooleanLiteralExpression, JsCallArguments, JsCallExpression, JsCaseClause,
    JsCatchClause, JsClassStatement, JsConstructorParameters, JsContinueStatement,
    JsDebuggerStatement, JsDefaultClause, JsDefaultImportSpecifier, JsDoWhileStatement,
    JsEmptyStatement, JsExpressionStatement, JsFinallyClause, JsForInStatement, JsForStatement,
    JsFunctionStatement, JsGetterClassMember, JsIdentifierBinding, JsIdentifierExpression,
    JsIfStatement, JsImport, JsImportAssertion, JsImportAssertionEntry, JsImportBareClause,
    JsImportCallExpression, JsImportDefaultClause, JsImportNamedClause, JsImportNamespaceClause,
    JsLabeledStatement, JsLiteralExportName, JsModule, JsModuleSource, JsNamedImportSpecifier,
    JsNamedImportSpecifiers, JsNamespaceImportSpecifier, JsNullLiteralExpression,
    JsNumberLiteralExpression, JsObjectExpression, JsParameters, JsPropertyClassMember,
    JsPropertyObjectMember, JsReturnStatement, JsScript, JsSequenceExpression, JsSetterClassMember,
    JsShorthandNamedImportSpecifier, JsShorthandPropertyObjectMember, JsSpread, JsStatementList,
    JsStaticInitializationBlockClassMember, JsStringLiteralExpression, JsSwitchStatement,
    JsTemplate, JsTemplateChunkElement, JsTemplateElement, JsTryStatement, JsUnknownAssignment,
    JsUnknownBinding, JsUnknownExpression, JsUnknownImportAssertionEntry, JsUnknownMember,
    JsUnknownNamedImportSpecifier, JsUnknownParameter, JsUnknownStatement, JsVariableDeclaration,
    JsVariableDeclarations, JsVariableStatement, JsWhileStatement, JsWithStatement,
};
use rslint_parser::{AstNode, JsSyntaxKind, SyntaxNode};

impl ToFormatElement for SyntaxNode {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self.kind() {
            JsSyntaxKind::JS_ARRAY_EXPRESSION => JsArrayExpression::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                JsArrowFunctionExpression::cast(self.clone())
                    .unwrap()
                    .to_format_element(formatter)
            }
            JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION => {
                JsBooleanLiteralExpression::cast(self.clone())
                    .unwrap()
                    .to_format_element(formatter)
            }
            JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION => {
                JsStringLiteralExpression::cast(self.clone())
                    .unwrap()
                    .to_format_element(formatter)
            }
            JsSyntaxKind::JS_IDENTIFIER_BINDING => JsIdentifierBinding::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_NULL_LITERAL_EXPRESSION => JsNullLiteralExpression::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION => {
                JsNumberLiteralExpression::cast(self.clone())
                    .unwrap()
                    .to_format_element(formatter)
            }
            JsSyntaxKind::JS_IDENTIFIER_EXPRESSION => JsIdentifierExpression::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_PARAMETERS => JsParameters::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_SCRIPT => JsScript::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_MODULE => JsModule::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_SPREAD => JsSpread::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_VARIABLE_STATEMENT => JsVariableStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_VARIABLE_DECLARATION => JsVariableDeclaration::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_FUNCTION_STATEMENT => JsFunctionStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION => JsSequenceExpression::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_BLOCK_STATEMENT => JsBlockStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_EXPRESSION_STATEMENT => JsExpressionStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_RETURN_STATEMENT => JsReturnStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_IF_STATEMENT => JsIfStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_FOR_STATEMENT => JsForStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_EMPTY_STATEMENT => JsEmptyStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
                JsShorthandPropertyObjectMember::cast(self.clone())
                    .unwrap()
                    .to_format_element(formatter)
            }
            JsSyntaxKind::JS_OBJECT_EXPRESSION => JsObjectExpression::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_WHILE_STATEMENT => JsWhileStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_DO_WHILE_STATEMENT => JsDoWhileStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_SWITCH_STATEMENT => JsSwitchStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_DEFAULT_CLAUSE => JsDefaultClause::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_CASE_CLAUSE => JsCaseClause::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_CONTINUE_STATEMENT => JsContinueStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_LABELED_STATEMENT => JsLabeledStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_TRY_STATEMENT => JsTryStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_FINALLY_CLAUSE => JsFinallyClause::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_CATCH_CLAUSE => JsCatchClause::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_WITH_STATEMENT => JsWithStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_DEBUGGER_STATEMENT => JsDebuggerStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_FOR_IN_STATEMENT => JsForInStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_ARRAY_BINDING_PATTERN => JsArrayBindingPattern::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_CALL_EXPRESSION => JsCallExpression::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_CALL_ARGUMENTS => JsCallArguments::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER => JsPropertyObjectMember::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_CLASS_STATEMENT => JsClassStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_CONSTRUCTOR_PARAMETERS => JsConstructorParameters::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_GETTER_CLASS_MEMBER => JsGetterClassMember::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_SETTER_CLASS_MEMBER => JsSetterClassMember::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER => JsPropertyClassMember::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_DEFAULT_IMPORT_SPECIFIER => {
                JsDefaultImportSpecifier::cast(self.clone())
                    .unwrap()
                    .to_format_element(formatter)
            }
            JsSyntaxKind::JS_UNKNOWN_BINDING => JsUnknownBinding::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_UNKNOWN_MEMBER => JsUnknownMember::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_UNKNOWN_STATEMENT => JsUnknownStatement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_UNKNOWN_EXPRESSION => JsUnknownExpression::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_UNKNOWN_ASSIGNMENT => JsUnknownAssignment::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_UNKNOWN_PARAMETER => JsUnknownParameter::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),

            JsSyntaxKind::JS_UNKNOWN_IMPORT_ASSERTION_ENTRY => {
                JsUnknownImportAssertionEntry::cast(self.clone())
                    .unwrap()
                    .to_format_element(formatter)
            }
            JsSyntaxKind::JS_UNKNOWN_NAMED_IMPORT_SPECIFIER => {
                JsUnknownNamedImportSpecifier::cast(self.clone())
                    .unwrap()
                    .to_format_element(formatter)
            }
            JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER => {
                JsStaticInitializationBlockClassMember::cast(self.clone())
                    .unwrap()
                    .to_format_element(formatter)
            }
            JsSyntaxKind::JS_TEMPLATE => JsTemplate::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),

            JsSyntaxKind::JS_TEMPLATE_ELEMENT => JsTemplateElement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_TEMPLATE_CHUNK_ELEMENT => JsTemplateChunkElement::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_IMPORT => JsImport::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_IMPORT_BARE_CLAUSE => JsImportBareClause::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_MODULE_SOURCE => JsModuleSource::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_IMPORT_ASSERTION => JsImportAssertion::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE => JsImportNamedClause::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_STATEMENT_LIST => {
                Ok(formatter.format_list(JsStatementList::cast(self.clone()).unwrap()))
            }
            JsSyntaxKind::JS_VARIABLE_DECLARATIONS => JsVariableDeclarations::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY => JsImportAssertionEntry::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),

            JsSyntaxKind::JS_IMPORT_CALL_EXPRESSION => JsImportCallExpression::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE => JsImportDefaultClause::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_LITERAL_EXPORT_NAME => JsLiteralExportName::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE => JsImportNamespaceClause::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_NAMESPACE_IMPORT_SPECIFIER => {
                JsNamespaceImportSpecifier::cast(self.clone())
                    .unwrap()
                    .to_format_element(formatter)
            }
            JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER => JsNamedImportSpecifier::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIERS => JsNamedImportSpecifiers::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
                JsShorthandNamedImportSpecifier::cast(self.clone())
                    .unwrap()
                    .to_format_element(formatter)
            }
            JsSyntaxKind::IMPORT_META => ImportMeta::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_EXPORT => JsExport::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_EXPORT_CLASS_CLAUSE => JsExportClassClause::cast(self.clone())
                .unwrap()
                .to_format_element(formatter),
            JsSyntaxKind::JS_EXPORT_DEFAULT_CLASS_CLAUSE => {
                JsExportDefaultClassClause::cast(self.clone())
                    .unwrap()
                    .to_format_element(formatter)
            }

            _ => todo!(
                "Implement formatting for the {:?} syntax kind.",
                self.kind()
            ),
        }
    }
}
