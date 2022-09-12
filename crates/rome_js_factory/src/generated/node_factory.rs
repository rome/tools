//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use rome_js_syntax::{
    JsSyntaxElement as SyntaxElement, JsSyntaxNode as SyntaxNode, JsSyntaxToken as SyntaxToken, *,
};
use rome_rowan::AstNode;
pub fn import_meta(
    import_token: SyntaxToken,
    dot_token: SyntaxToken,
    meta_token: SyntaxToken,
) -> ImportMeta {
    ImportMeta::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::IMPORT_META,
        [
            Some(SyntaxElement::Token(import_token)),
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Token(meta_token)),
        ],
    ))
}
pub fn js_array_assignment_pattern(
    l_brack_token: SyntaxToken,
    elements: JsArrayAssignmentPatternElementList,
    r_brack_token: SyntaxToken,
) -> JsArrayAssignmentPattern {
    JsArrayAssignmentPattern::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(elements.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn js_array_assignment_pattern_rest_element(
    dotdotdot_token: SyntaxToken,
    pattern: JsAnyAssignmentPattern,
) -> JsArrayAssignmentPatternRestElement {
    JsArrayAssignmentPatternRestElement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT,
        [
            Some(SyntaxElement::Token(dotdotdot_token)),
            Some(SyntaxElement::Node(pattern.into_syntax())),
        ],
    ))
}
pub fn js_array_binding_pattern(
    l_brack_token: SyntaxToken,
    elements: JsArrayBindingPatternElementList,
    r_brack_token: SyntaxToken,
) -> JsArrayBindingPattern {
    JsArrayBindingPattern::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_ARRAY_BINDING_PATTERN,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(elements.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn js_array_binding_pattern_rest_element(
    dotdotdot_token: SyntaxToken,
    pattern: JsAnyBindingPattern,
) -> JsArrayBindingPatternRestElement {
    JsArrayBindingPatternRestElement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_REST_ELEMENT,
        [
            Some(SyntaxElement::Token(dotdotdot_token)),
            Some(SyntaxElement::Node(pattern.into_syntax())),
        ],
    ))
}
pub fn js_array_expression(
    l_brack_token: SyntaxToken,
    elements: JsArrayElementList,
    r_brack_token: SyntaxToken,
) -> JsArrayExpression {
    JsArrayExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_ARRAY_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(elements.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn js_array_hole() -> JsArrayHole {
    JsArrayHole::unwrap_cast(SyntaxNode::new_detached(JsSyntaxKind::JS_ARRAY_HOLE, []))
}
pub fn js_arrow_function_expression(
    parameters: JsAnyArrowFunctionParameters,
    fat_arrow_token: SyntaxToken,
    body: JsAnyFunctionBody,
) -> JsArrowFunctionExpressionBuilder {
    JsArrowFunctionExpressionBuilder {
        parameters,
        fat_arrow_token,
        body,
        async_token: None,
        type_parameters: None,
        return_type_annotation: None,
    }
}
pub struct JsArrowFunctionExpressionBuilder {
    parameters: JsAnyArrowFunctionParameters,
    fat_arrow_token: SyntaxToken,
    body: JsAnyFunctionBody,
    async_token: Option<SyntaxToken>,
    type_parameters: Option<TsTypeParameters>,
    return_type_annotation: Option<TsReturnTypeAnnotation>,
}
impl JsArrowFunctionExpressionBuilder {
    pub fn with_async_token(mut self, async_token: SyntaxToken) -> Self {
        self.async_token = Some(async_token);
        self
    }
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_return_type_annotation(
        mut self,
        return_type_annotation: TsReturnTypeAnnotation,
    ) -> Self {
        self.return_type_annotation = Some(return_type_annotation);
        self
    }
    pub fn build(self) -> JsArrowFunctionExpression {
        JsArrowFunctionExpression::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION,
            [
                self.async_token.map(|token| SyntaxElement::Token(token)),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                self.return_type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.fat_arrow_token)),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn js_assignment_expression(
    left: JsAnyAssignmentPattern,
    operator_token_token: SyntaxToken,
    right: JsAnyExpression,
) -> JsAssignmentExpression {
    JsAssignmentExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn js_assignment_with_default(
    pattern: JsAnyAssignmentPattern,
    eq_token: SyntaxToken,
    default: JsAnyExpression,
) -> JsAssignmentWithDefault {
    JsAssignmentWithDefault::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_ASSIGNMENT_WITH_DEFAULT,
        [
            Some(SyntaxElement::Node(pattern.into_syntax())),
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(default.into_syntax())),
        ],
    ))
}
pub fn js_await_expression(
    await_token: SyntaxToken,
    argument: JsAnyExpression,
) -> JsAwaitExpression {
    JsAwaitExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_AWAIT_EXPRESSION,
        [
            Some(SyntaxElement::Token(await_token)),
            Some(SyntaxElement::Node(argument.into_syntax())),
        ],
    ))
}
pub fn js_big_int_literal_expression(value_token: SyntaxToken) -> JsBigIntLiteralExpression {
    JsBigIntLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_BIG_INT_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn js_binary_expression(
    left: JsAnyExpression,
    operator_token_token: SyntaxToken,
    right: JsAnyExpression,
) -> JsBinaryExpression {
    JsBinaryExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_BINARY_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn js_binding_pattern_with_default(
    pattern: JsAnyBindingPattern,
    eq_token: SyntaxToken,
    default: JsAnyExpression,
) -> JsBindingPatternWithDefault {
    JsBindingPatternWithDefault::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_BINDING_PATTERN_WITH_DEFAULT,
        [
            Some(SyntaxElement::Node(pattern.into_syntax())),
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(default.into_syntax())),
        ],
    ))
}
pub fn js_block_statement(
    l_curly_token: SyntaxToken,
    statements: JsStatementList,
    r_curly_token: SyntaxToken,
) -> JsBlockStatement {
    JsBlockStatement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_BLOCK_STATEMENT,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(statements.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn js_boolean_literal_expression(value_token_token: SyntaxToken) -> JsBooleanLiteralExpression {
    JsBooleanLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token_token))],
    ))
}
pub fn js_break_statement(break_token: SyntaxToken) -> JsBreakStatementBuilder {
    JsBreakStatementBuilder {
        break_token,
        label_token: None,
        semicolon_token: None,
    }
}
pub struct JsBreakStatementBuilder {
    break_token: SyntaxToken,
    label_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl JsBreakStatementBuilder {
    pub fn with_label_token(mut self, label_token: SyntaxToken) -> Self {
        self.label_token = Some(label_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsBreakStatement {
        JsBreakStatement::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_BREAK_STATEMENT,
            [
                Some(SyntaxElement::Token(self.break_token)),
                self.label_token.map(|token| SyntaxElement::Token(token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_call_arguments(
    l_paren_token: SyntaxToken,
    args: JsCallArgumentList,
    r_paren_token: SyntaxToken,
) -> JsCallArguments {
    JsCallArguments::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_CALL_ARGUMENTS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(args.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn js_call_expression(
    callee: JsAnyExpression,
    arguments: JsCallArguments,
) -> JsCallExpressionBuilder {
    JsCallExpressionBuilder {
        callee,
        arguments,
        optional_chain_token: None,
        type_arguments: None,
    }
}
pub struct JsCallExpressionBuilder {
    callee: JsAnyExpression,
    arguments: JsCallArguments,
    optional_chain_token: Option<SyntaxToken>,
    type_arguments: Option<TsTypeArguments>,
}
impl JsCallExpressionBuilder {
    pub fn with_optional_chain_token(mut self, optional_chain_token: SyntaxToken) -> Self {
        self.optional_chain_token = Some(optional_chain_token);
        self
    }
    pub fn with_type_arguments(mut self, type_arguments: TsTypeArguments) -> Self {
        self.type_arguments = Some(type_arguments);
        self
    }
    pub fn build(self) -> JsCallExpression {
        JsCallExpression::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_CALL_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.callee.into_syntax())),
                self.optional_chain_token
                    .map(|token| SyntaxElement::Token(token)),
                self.type_arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.arguments.into_syntax())),
            ],
        ))
    }
}
pub fn js_case_clause(
    case_token: SyntaxToken,
    test: JsAnyExpression,
    colon_token: SyntaxToken,
    consequent: JsStatementList,
) -> JsCaseClause {
    JsCaseClause::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_CASE_CLAUSE,
        [
            Some(SyntaxElement::Token(case_token)),
            Some(SyntaxElement::Node(test.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(consequent.into_syntax())),
        ],
    ))
}
pub fn js_catch_clause(catch_token: SyntaxToken, body: JsBlockStatement) -> JsCatchClauseBuilder {
    JsCatchClauseBuilder {
        catch_token,
        body,
        declaration: None,
    }
}
pub struct JsCatchClauseBuilder {
    catch_token: SyntaxToken,
    body: JsBlockStatement,
    declaration: Option<JsCatchDeclaration>,
}
impl JsCatchClauseBuilder {
    pub fn with_declaration(mut self, declaration: JsCatchDeclaration) -> Self {
        self.declaration = Some(declaration);
        self
    }
    pub fn build(self) -> JsCatchClause {
        JsCatchClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_CATCH_CLAUSE,
            [
                Some(SyntaxElement::Token(self.catch_token)),
                self.declaration
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn js_catch_declaration(
    l_paren_token: SyntaxToken,
    binding: JsAnyBindingPattern,
    r_paren_token: SyntaxToken,
) -> JsCatchDeclarationBuilder {
    JsCatchDeclarationBuilder {
        l_paren_token,
        binding,
        r_paren_token,
        type_annotation: None,
    }
}
pub struct JsCatchDeclarationBuilder {
    l_paren_token: SyntaxToken,
    binding: JsAnyBindingPattern,
    r_paren_token: SyntaxToken,
    type_annotation: Option<TsTypeAnnotation>,
}
impl JsCatchDeclarationBuilder {
    pub fn with_type_annotation(mut self, type_annotation: TsTypeAnnotation) -> Self {
        self.type_annotation = Some(type_annotation);
        self
    }
    pub fn build(self) -> JsCatchDeclaration {
        JsCatchDeclaration::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_CATCH_DECLARATION,
            [
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.binding.into_syntax())),
                self.type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn js_class_declaration(
    class_token: SyntaxToken,
    id: JsAnyBinding,
    l_curly_token: SyntaxToken,
    members: JsClassMemberList,
    r_curly_token: SyntaxToken,
) -> JsClassDeclarationBuilder {
    JsClassDeclarationBuilder {
        class_token,
        id,
        l_curly_token,
        members,
        r_curly_token,
        abstract_token: None,
        type_parameters: None,
        extends_clause: None,
        implements_clause: None,
    }
}
pub struct JsClassDeclarationBuilder {
    class_token: SyntaxToken,
    id: JsAnyBinding,
    l_curly_token: SyntaxToken,
    members: JsClassMemberList,
    r_curly_token: SyntaxToken,
    abstract_token: Option<SyntaxToken>,
    type_parameters: Option<TsTypeParameters>,
    extends_clause: Option<JsExtendsClause>,
    implements_clause: Option<TsImplementsClause>,
}
impl JsClassDeclarationBuilder {
    pub fn with_abstract_token(mut self, abstract_token: SyntaxToken) -> Self {
        self.abstract_token = Some(abstract_token);
        self
    }
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_extends_clause(mut self, extends_clause: JsExtendsClause) -> Self {
        self.extends_clause = Some(extends_clause);
        self
    }
    pub fn with_implements_clause(mut self, implements_clause: TsImplementsClause) -> Self {
        self.implements_clause = Some(implements_clause);
        self
    }
    pub fn build(self) -> JsClassDeclaration {
        JsClassDeclaration::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_CLASS_DECLARATION,
            [
                self.abstract_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.class_token)),
                Some(SyntaxElement::Node(self.id.into_syntax())),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.extends_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.implements_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.l_curly_token)),
                Some(SyntaxElement::Node(self.members.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn js_class_export_default_declaration(
    class_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    members: JsClassMemberList,
    r_curly_token: SyntaxToken,
) -> JsClassExportDefaultDeclarationBuilder {
    JsClassExportDefaultDeclarationBuilder {
        class_token,
        l_curly_token,
        members,
        r_curly_token,
        abstract_token: None,
        id: None,
        type_parameters: None,
        extends_clause: None,
        implements_clause: None,
    }
}
pub struct JsClassExportDefaultDeclarationBuilder {
    class_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    members: JsClassMemberList,
    r_curly_token: SyntaxToken,
    abstract_token: Option<SyntaxToken>,
    id: Option<JsAnyBinding>,
    type_parameters: Option<TsTypeParameters>,
    extends_clause: Option<JsExtendsClause>,
    implements_clause: Option<TsImplementsClause>,
}
impl JsClassExportDefaultDeclarationBuilder {
    pub fn with_abstract_token(mut self, abstract_token: SyntaxToken) -> Self {
        self.abstract_token = Some(abstract_token);
        self
    }
    pub fn with_id(mut self, id: JsAnyBinding) -> Self {
        self.id = Some(id);
        self
    }
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_extends_clause(mut self, extends_clause: JsExtendsClause) -> Self {
        self.extends_clause = Some(extends_clause);
        self
    }
    pub fn with_implements_clause(mut self, implements_clause: TsImplementsClause) -> Self {
        self.implements_clause = Some(implements_clause);
        self
    }
    pub fn build(self) -> JsClassExportDefaultDeclaration {
        JsClassExportDefaultDeclaration::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_CLASS_EXPORT_DEFAULT_DECLARATION,
            [
                self.abstract_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.class_token)),
                self.id
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.extends_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.implements_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.l_curly_token)),
                Some(SyntaxElement::Node(self.members.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn js_class_expression(
    class_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    members: JsClassMemberList,
    r_curly_token: SyntaxToken,
) -> JsClassExpressionBuilder {
    JsClassExpressionBuilder {
        class_token,
        l_curly_token,
        members,
        r_curly_token,
        id: None,
        type_parameters: None,
        extends_clause: None,
        implements_clause: None,
    }
}
pub struct JsClassExpressionBuilder {
    class_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    members: JsClassMemberList,
    r_curly_token: SyntaxToken,
    id: Option<JsAnyBinding>,
    type_parameters: Option<TsTypeParameters>,
    extends_clause: Option<JsExtendsClause>,
    implements_clause: Option<TsImplementsClause>,
}
impl JsClassExpressionBuilder {
    pub fn with_id(mut self, id: JsAnyBinding) -> Self {
        self.id = Some(id);
        self
    }
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_extends_clause(mut self, extends_clause: JsExtendsClause) -> Self {
        self.extends_clause = Some(extends_clause);
        self
    }
    pub fn with_implements_clause(mut self, implements_clause: TsImplementsClause) -> Self {
        self.implements_clause = Some(implements_clause);
        self
    }
    pub fn build(self) -> JsClassExpression {
        JsClassExpression::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_CLASS_EXPRESSION,
            [
                Some(SyntaxElement::Token(self.class_token)),
                self.id
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.extends_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.implements_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.l_curly_token)),
                Some(SyntaxElement::Node(self.members.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn js_computed_member_assignment(
    object: JsAnyExpression,
    l_brack_token: SyntaxToken,
    member: JsAnyExpression,
    r_brack_token: SyntaxToken,
) -> JsComputedMemberAssignment {
    JsComputedMemberAssignment::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT,
        [
            Some(SyntaxElement::Node(object.into_syntax())),
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(member.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn js_computed_member_expression(
    object: JsAnyExpression,
    l_brack_token: SyntaxToken,
    member: JsAnyExpression,
    r_brack_token: SyntaxToken,
) -> JsComputedMemberExpressionBuilder {
    JsComputedMemberExpressionBuilder {
        object,
        l_brack_token,
        member,
        r_brack_token,
        optional_chain_token: None,
    }
}
pub struct JsComputedMemberExpressionBuilder {
    object: JsAnyExpression,
    l_brack_token: SyntaxToken,
    member: JsAnyExpression,
    r_brack_token: SyntaxToken,
    optional_chain_token: Option<SyntaxToken>,
}
impl JsComputedMemberExpressionBuilder {
    pub fn with_optional_chain_token(mut self, optional_chain_token: SyntaxToken) -> Self {
        self.optional_chain_token = Some(optional_chain_token);
        self
    }
    pub fn build(self) -> JsComputedMemberExpression {
        JsComputedMemberExpression::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.object.into_syntax())),
                self.optional_chain_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Node(self.member.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
            ],
        ))
    }
}
pub fn js_computed_member_name(
    l_brack_token: SyntaxToken,
    expression: JsAnyExpression,
    r_brack_token: SyntaxToken,
) -> JsComputedMemberName {
    JsComputedMemberName::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_COMPUTED_MEMBER_NAME,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn js_conditional_expression(
    test: JsAnyExpression,
    question_mark_token: SyntaxToken,
    consequent: JsAnyExpression,
    colon_token: SyntaxToken,
    alternate: JsAnyExpression,
) -> JsConditionalExpression {
    JsConditionalExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_CONDITIONAL_EXPRESSION,
        [
            Some(SyntaxElement::Node(test.into_syntax())),
            Some(SyntaxElement::Token(question_mark_token)),
            Some(SyntaxElement::Node(consequent.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(alternate.into_syntax())),
        ],
    ))
}
pub fn js_constructor_class_member(
    modifiers: JsConstructorModifierList,
    name: JsLiteralMemberName,
    parameters: JsConstructorParameters,
    body: JsFunctionBody,
) -> JsConstructorClassMember {
    JsConstructorClassMember::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER,
        [
            Some(SyntaxElement::Node(modifiers.into_syntax())),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(parameters.into_syntax())),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn js_constructor_parameters(
    l_paren_token: SyntaxToken,
    parameters: JsConstructorParameterList,
    r_paren_token: SyntaxToken,
) -> JsConstructorParameters {
    JsConstructorParameters::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_CONSTRUCTOR_PARAMETERS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(parameters.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn js_continue_statement(continue_token: SyntaxToken) -> JsContinueStatementBuilder {
    JsContinueStatementBuilder {
        continue_token,
        label_token: None,
        semicolon_token: None,
    }
}
pub struct JsContinueStatementBuilder {
    continue_token: SyntaxToken,
    label_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl JsContinueStatementBuilder {
    pub fn with_label_token(mut self, label_token: SyntaxToken) -> Self {
        self.label_token = Some(label_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsContinueStatement {
        JsContinueStatement::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_CONTINUE_STATEMENT,
            [
                Some(SyntaxElement::Token(self.continue_token)),
                self.label_token.map(|token| SyntaxElement::Token(token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_debugger_statement(debugger_token: SyntaxToken) -> JsDebuggerStatementBuilder {
    JsDebuggerStatementBuilder {
        debugger_token,
        semicolon_token: None,
    }
}
pub struct JsDebuggerStatementBuilder {
    debugger_token: SyntaxToken,
    semicolon_token: Option<SyntaxToken>,
}
impl JsDebuggerStatementBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsDebuggerStatement {
        JsDebuggerStatement::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_DEBUGGER_STATEMENT,
            [
                Some(SyntaxElement::Token(self.debugger_token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_default_clause(
    default_token: SyntaxToken,
    colon_token: SyntaxToken,
    consequent: JsStatementList,
) -> JsDefaultClause {
    JsDefaultClause::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_DEFAULT_CLAUSE,
        [
            Some(SyntaxElement::Token(default_token)),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(consequent.into_syntax())),
        ],
    ))
}
pub fn js_default_import_specifier(
    local_name: JsAnyBinding,
    trailing_comma_token: SyntaxToken,
) -> JsDefaultImportSpecifier {
    JsDefaultImportSpecifier::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_DEFAULT_IMPORT_SPECIFIER,
        [
            Some(SyntaxElement::Node(local_name.into_syntax())),
            Some(SyntaxElement::Token(trailing_comma_token)),
        ],
    ))
}
pub fn js_directive(value_token: SyntaxToken) -> JsDirectiveBuilder {
    JsDirectiveBuilder {
        value_token,
        semicolon_token: None,
    }
}
pub struct JsDirectiveBuilder {
    value_token: SyntaxToken,
    semicolon_token: Option<SyntaxToken>,
}
impl JsDirectiveBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsDirective {
        JsDirective::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_DIRECTIVE,
            [
                Some(SyntaxElement::Token(self.value_token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_do_while_statement(
    do_token: SyntaxToken,
    body: JsAnyStatement,
    while_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    test: JsAnyExpression,
    r_paren_token: SyntaxToken,
) -> JsDoWhileStatementBuilder {
    JsDoWhileStatementBuilder {
        do_token,
        body,
        while_token,
        l_paren_token,
        test,
        r_paren_token,
        semicolon_token: None,
    }
}
pub struct JsDoWhileStatementBuilder {
    do_token: SyntaxToken,
    body: JsAnyStatement,
    while_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    test: JsAnyExpression,
    r_paren_token: SyntaxToken,
    semicolon_token: Option<SyntaxToken>,
}
impl JsDoWhileStatementBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsDoWhileStatement {
        JsDoWhileStatement::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_DO_WHILE_STATEMENT,
            [
                Some(SyntaxElement::Token(self.do_token)),
                Some(SyntaxElement::Node(self.body.into_syntax())),
                Some(SyntaxElement::Token(self.while_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.test.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_else_clause(else_token: SyntaxToken, alternate: JsAnyStatement) -> JsElseClause {
    JsElseClause::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_ELSE_CLAUSE,
        [
            Some(SyntaxElement::Token(else_token)),
            Some(SyntaxElement::Node(alternate.into_syntax())),
        ],
    ))
}
pub fn js_empty_class_member(semicolon_token: SyntaxToken) -> JsEmptyClassMember {
    JsEmptyClassMember::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_EMPTY_CLASS_MEMBER,
        [Some(SyntaxElement::Token(semicolon_token))],
    ))
}
pub fn js_empty_statement(semicolon_token: SyntaxToken) -> JsEmptyStatement {
    JsEmptyStatement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_EMPTY_STATEMENT,
        [Some(SyntaxElement::Token(semicolon_token))],
    ))
}
pub fn js_export(export_token: SyntaxToken, export_clause: JsAnyExportClause) -> JsExport {
    JsExport::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_EXPORT,
        [
            Some(SyntaxElement::Token(export_token)),
            Some(SyntaxElement::Node(export_clause.into_syntax())),
        ],
    ))
}
pub fn js_export_as_clause(
    as_token: SyntaxToken,
    exported_name: JsLiteralExportName,
) -> JsExportAsClause {
    JsExportAsClause::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_EXPORT_AS_CLAUSE,
        [
            Some(SyntaxElement::Token(as_token)),
            Some(SyntaxElement::Node(exported_name.into_syntax())),
        ],
    ))
}
pub fn js_export_default_declaration_clause(
    default_token: SyntaxToken,
    declaration: JsAnyExportDefaultDeclaration,
) -> JsExportDefaultDeclarationClauseBuilder {
    JsExportDefaultDeclarationClauseBuilder {
        default_token,
        declaration,
        semicolon_token: None,
    }
}
pub struct JsExportDefaultDeclarationClauseBuilder {
    default_token: SyntaxToken,
    declaration: JsAnyExportDefaultDeclaration,
    semicolon_token: Option<SyntaxToken>,
}
impl JsExportDefaultDeclarationClauseBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsExportDefaultDeclarationClause {
        JsExportDefaultDeclarationClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE,
            [
                Some(SyntaxElement::Token(self.default_token)),
                Some(SyntaxElement::Node(self.declaration.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_export_default_expression_clause(
    default_token: SyntaxToken,
    expression: JsAnyExpression,
) -> JsExportDefaultExpressionClauseBuilder {
    JsExportDefaultExpressionClauseBuilder {
        default_token,
        expression,
        semicolon_token: None,
    }
}
pub struct JsExportDefaultExpressionClauseBuilder {
    default_token: SyntaxToken,
    expression: JsAnyExpression,
    semicolon_token: Option<SyntaxToken>,
}
impl JsExportDefaultExpressionClauseBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsExportDefaultExpressionClause {
        JsExportDefaultExpressionClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE,
            [
                Some(SyntaxElement::Token(self.default_token)),
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_export_from_clause(
    star_token: SyntaxToken,
    from_token: SyntaxToken,
    source: JsModuleSource,
) -> JsExportFromClauseBuilder {
    JsExportFromClauseBuilder {
        star_token,
        from_token,
        source,
        export_as: None,
        assertion: None,
        semicolon_token: None,
    }
}
pub struct JsExportFromClauseBuilder {
    star_token: SyntaxToken,
    from_token: SyntaxToken,
    source: JsModuleSource,
    export_as: Option<JsExportAsClause>,
    assertion: Option<JsImportAssertion>,
    semicolon_token: Option<SyntaxToken>,
}
impl JsExportFromClauseBuilder {
    pub fn with_export_as(mut self, export_as: JsExportAsClause) -> Self {
        self.export_as = Some(export_as);
        self
    }
    pub fn with_assertion(mut self, assertion: JsImportAssertion) -> Self {
        self.assertion = Some(assertion);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsExportFromClause {
        JsExportFromClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_EXPORT_FROM_CLAUSE,
            [
                Some(SyntaxElement::Token(self.star_token)),
                self.export_as
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.from_token)),
                Some(SyntaxElement::Node(self.source.into_syntax())),
                self.assertion
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_export_named_clause(
    l_curly_token: SyntaxToken,
    specifiers: JsExportNamedSpecifierList,
    r_curly_token: SyntaxToken,
) -> JsExportNamedClauseBuilder {
    JsExportNamedClauseBuilder {
        l_curly_token,
        specifiers,
        r_curly_token,
        type_token: None,
        semicolon_token: None,
    }
}
pub struct JsExportNamedClauseBuilder {
    l_curly_token: SyntaxToken,
    specifiers: JsExportNamedSpecifierList,
    r_curly_token: SyntaxToken,
    type_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl JsExportNamedClauseBuilder {
    pub fn with_type_token(mut self, type_token: SyntaxToken) -> Self {
        self.type_token = Some(type_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsExportNamedClause {
        JsExportNamedClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_EXPORT_NAMED_CLAUSE,
            [
                self.type_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.l_curly_token)),
                Some(SyntaxElement::Node(self.specifiers.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_export_named_from_clause(
    l_curly_token: SyntaxToken,
    specifiers: JsExportNamedFromSpecifierList,
    r_curly_token: SyntaxToken,
    from_token: SyntaxToken,
    source: JsModuleSource,
) -> JsExportNamedFromClauseBuilder {
    JsExportNamedFromClauseBuilder {
        l_curly_token,
        specifiers,
        r_curly_token,
        from_token,
        source,
        type_token: None,
        assertion: None,
        semicolon_token: None,
    }
}
pub struct JsExportNamedFromClauseBuilder {
    l_curly_token: SyntaxToken,
    specifiers: JsExportNamedFromSpecifierList,
    r_curly_token: SyntaxToken,
    from_token: SyntaxToken,
    source: JsModuleSource,
    type_token: Option<SyntaxToken>,
    assertion: Option<JsImportAssertion>,
    semicolon_token: Option<SyntaxToken>,
}
impl JsExportNamedFromClauseBuilder {
    pub fn with_type_token(mut self, type_token: SyntaxToken) -> Self {
        self.type_token = Some(type_token);
        self
    }
    pub fn with_assertion(mut self, assertion: JsImportAssertion) -> Self {
        self.assertion = Some(assertion);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsExportNamedFromClause {
        JsExportNamedFromClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_EXPORT_NAMED_FROM_CLAUSE,
            [
                self.type_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.l_curly_token)),
                Some(SyntaxElement::Node(self.specifiers.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
                Some(SyntaxElement::Token(self.from_token)),
                Some(SyntaxElement::Node(self.source.into_syntax())),
                self.assertion
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_export_named_from_specifier(
    source_name: JsLiteralExportName,
) -> JsExportNamedFromSpecifierBuilder {
    JsExportNamedFromSpecifierBuilder {
        source_name,
        type_token: None,
        export_as: None,
    }
}
pub struct JsExportNamedFromSpecifierBuilder {
    source_name: JsLiteralExportName,
    type_token: Option<SyntaxToken>,
    export_as: Option<JsExportAsClause>,
}
impl JsExportNamedFromSpecifierBuilder {
    pub fn with_type_token(mut self, type_token: SyntaxToken) -> Self {
        self.type_token = Some(type_token);
        self
    }
    pub fn with_export_as(mut self, export_as: JsExportAsClause) -> Self {
        self.export_as = Some(export_as);
        self
    }
    pub fn build(self) -> JsExportNamedFromSpecifier {
        JsExportNamedFromSpecifier::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER,
            [
                self.type_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.source_name.into_syntax())),
                self.export_as
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn js_export_named_shorthand_specifier(
    name: JsReferenceIdentifier,
) -> JsExportNamedShorthandSpecifierBuilder {
    JsExportNamedShorthandSpecifierBuilder {
        name,
        type_token: None,
    }
}
pub struct JsExportNamedShorthandSpecifierBuilder {
    name: JsReferenceIdentifier,
    type_token: Option<SyntaxToken>,
}
impl JsExportNamedShorthandSpecifierBuilder {
    pub fn with_type_token(mut self, type_token: SyntaxToken) -> Self {
        self.type_token = Some(type_token);
        self
    }
    pub fn build(self) -> JsExportNamedShorthandSpecifier {
        JsExportNamedShorthandSpecifier::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_EXPORT_NAMED_SHORTHAND_SPECIFIER,
            [
                self.type_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
            ],
        ))
    }
}
pub fn js_export_named_specifier(
    local_name: JsReferenceIdentifier,
    as_token: SyntaxToken,
    exported_name: JsLiteralExportName,
) -> JsExportNamedSpecifierBuilder {
    JsExportNamedSpecifierBuilder {
        local_name,
        as_token,
        exported_name,
        type_token: None,
    }
}
pub struct JsExportNamedSpecifierBuilder {
    local_name: JsReferenceIdentifier,
    as_token: SyntaxToken,
    exported_name: JsLiteralExportName,
    type_token: Option<SyntaxToken>,
}
impl JsExportNamedSpecifierBuilder {
    pub fn with_type_token(mut self, type_token: SyntaxToken) -> Self {
        self.type_token = Some(type_token);
        self
    }
    pub fn build(self) -> JsExportNamedSpecifier {
        JsExportNamedSpecifier::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER,
            [
                self.type_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.local_name.into_syntax())),
                Some(SyntaxElement::Token(self.as_token)),
                Some(SyntaxElement::Node(self.exported_name.into_syntax())),
            ],
        ))
    }
}
pub fn js_expression_snipped(
    expression: JsAnyExpression,
    eof_token: SyntaxToken,
) -> JsExpressionSnipped {
    JsExpressionSnipped::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_EXPRESSION_SNIPPED,
        [
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(eof_token)),
        ],
    ))
}
pub fn js_expression_statement(expression: JsAnyExpression) -> JsExpressionStatementBuilder {
    JsExpressionStatementBuilder {
        expression,
        semicolon_token: None,
    }
}
pub struct JsExpressionStatementBuilder {
    expression: JsAnyExpression,
    semicolon_token: Option<SyntaxToken>,
}
impl JsExpressionStatementBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsExpressionStatement {
        JsExpressionStatement::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_EXPRESSION_STATEMENT,
            [
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_extends_clause(
    extends_token: SyntaxToken,
    super_class: JsAnyExpression,
) -> JsExtendsClauseBuilder {
    JsExtendsClauseBuilder {
        extends_token,
        super_class,
        type_arguments: None,
    }
}
pub struct JsExtendsClauseBuilder {
    extends_token: SyntaxToken,
    super_class: JsAnyExpression,
    type_arguments: Option<TsTypeArguments>,
}
impl JsExtendsClauseBuilder {
    pub fn with_type_arguments(mut self, type_arguments: TsTypeArguments) -> Self {
        self.type_arguments = Some(type_arguments);
        self
    }
    pub fn build(self) -> JsExtendsClause {
        JsExtendsClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_EXTENDS_CLAUSE,
            [
                Some(SyntaxElement::Token(self.extends_token)),
                Some(SyntaxElement::Node(self.super_class.into_syntax())),
                self.type_arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn js_finally_clause(finally_token: SyntaxToken, body: JsBlockStatement) -> JsFinallyClause {
    JsFinallyClause::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_FINALLY_CLAUSE,
        [
            Some(SyntaxElement::Token(finally_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn js_for_in_statement(
    for_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    initializer: JsAnyForInOrOfInitializer,
    in_token: SyntaxToken,
    expression: JsAnyExpression,
    r_paren_token: SyntaxToken,
    body: JsAnyStatement,
) -> JsForInStatement {
    JsForInStatement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_FOR_IN_STATEMENT,
        [
            Some(SyntaxElement::Token(for_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(initializer.into_syntax())),
            Some(SyntaxElement::Token(in_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn js_for_of_statement(
    for_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    initializer: JsAnyForInOrOfInitializer,
    of_token: SyntaxToken,
    expression: JsAnyExpression,
    r_paren_token: SyntaxToken,
    body: JsAnyStatement,
) -> JsForOfStatementBuilder {
    JsForOfStatementBuilder {
        for_token,
        l_paren_token,
        initializer,
        of_token,
        expression,
        r_paren_token,
        body,
        await_token: None,
    }
}
pub struct JsForOfStatementBuilder {
    for_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    initializer: JsAnyForInOrOfInitializer,
    of_token: SyntaxToken,
    expression: JsAnyExpression,
    r_paren_token: SyntaxToken,
    body: JsAnyStatement,
    await_token: Option<SyntaxToken>,
}
impl JsForOfStatementBuilder {
    pub fn with_await_token(mut self, await_token: SyntaxToken) -> Self {
        self.await_token = Some(await_token);
        self
    }
    pub fn build(self) -> JsForOfStatement {
        JsForOfStatement::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_FOR_OF_STATEMENT,
            [
                Some(SyntaxElement::Token(self.for_token)),
                self.await_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.initializer.into_syntax())),
                Some(SyntaxElement::Token(self.of_token)),
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn js_for_statement(
    for_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    first_semi_token: SyntaxToken,
    second_semi_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: JsAnyStatement,
) -> JsForStatementBuilder {
    JsForStatementBuilder {
        for_token,
        l_paren_token,
        first_semi_token,
        second_semi_token,
        r_paren_token,
        body,
        initializer: None,
        test: None,
        update: None,
    }
}
pub struct JsForStatementBuilder {
    for_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    first_semi_token: SyntaxToken,
    second_semi_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: JsAnyStatement,
    initializer: Option<JsAnyForInitializer>,
    test: Option<JsAnyExpression>,
    update: Option<JsAnyExpression>,
}
impl JsForStatementBuilder {
    pub fn with_initializer(mut self, initializer: JsAnyForInitializer) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn with_test(mut self, test: JsAnyExpression) -> Self {
        self.test = Some(test);
        self
    }
    pub fn with_update(mut self, update: JsAnyExpression) -> Self {
        self.update = Some(update);
        self
    }
    pub fn build(self) -> JsForStatement {
        JsForStatement::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_FOR_STATEMENT,
            [
                Some(SyntaxElement::Token(self.for_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.first_semi_token)),
                self.test
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.second_semi_token)),
                self.update
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn js_for_variable_declaration(
    kind_token_token: SyntaxToken,
    declarator: JsVariableDeclarator,
) -> JsForVariableDeclaration {
    JsForVariableDeclaration::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_FOR_VARIABLE_DECLARATION,
        [
            Some(SyntaxElement::Token(kind_token_token)),
            Some(SyntaxElement::Node(declarator.into_syntax())),
        ],
    ))
}
pub fn js_formal_parameter(binding: JsAnyBindingPattern) -> JsFormalParameterBuilder {
    JsFormalParameterBuilder {
        binding,
        question_mark_token: None,
        type_annotation: None,
        initializer: None,
    }
}
pub struct JsFormalParameterBuilder {
    binding: JsAnyBindingPattern,
    question_mark_token: Option<SyntaxToken>,
    type_annotation: Option<TsTypeAnnotation>,
    initializer: Option<JsInitializerClause>,
}
impl JsFormalParameterBuilder {
    pub fn with_question_mark_token(mut self, question_mark_token: SyntaxToken) -> Self {
        self.question_mark_token = Some(question_mark_token);
        self
    }
    pub fn with_type_annotation(mut self, type_annotation: TsTypeAnnotation) -> Self {
        self.type_annotation = Some(type_annotation);
        self
    }
    pub fn with_initializer(mut self, initializer: JsInitializerClause) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn build(self) -> JsFormalParameter {
        JsFormalParameter::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_FORMAL_PARAMETER,
            [
                Some(SyntaxElement::Node(self.binding.into_syntax())),
                self.question_mark_token
                    .map(|token| SyntaxElement::Token(token)),
                self.type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn js_function_body(
    l_curly_token: SyntaxToken,
    directives: JsDirectiveList,
    statements: JsStatementList,
    r_curly_token: SyntaxToken,
) -> JsFunctionBody {
    JsFunctionBody::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_FUNCTION_BODY,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(directives.into_syntax())),
            Some(SyntaxElement::Node(statements.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn js_function_declaration(
    function_token: SyntaxToken,
    id: JsAnyBinding,
    parameters: JsParameters,
    body: JsFunctionBody,
) -> JsFunctionDeclarationBuilder {
    JsFunctionDeclarationBuilder {
        function_token,
        id,
        parameters,
        body,
        async_token: None,
        star_token: None,
        type_parameters: None,
        return_type_annotation: None,
    }
}
pub struct JsFunctionDeclarationBuilder {
    function_token: SyntaxToken,
    id: JsAnyBinding,
    parameters: JsParameters,
    body: JsFunctionBody,
    async_token: Option<SyntaxToken>,
    star_token: Option<SyntaxToken>,
    type_parameters: Option<TsTypeParameters>,
    return_type_annotation: Option<TsReturnTypeAnnotation>,
}
impl JsFunctionDeclarationBuilder {
    pub fn with_async_token(mut self, async_token: SyntaxToken) -> Self {
        self.async_token = Some(async_token);
        self
    }
    pub fn with_star_token(mut self, star_token: SyntaxToken) -> Self {
        self.star_token = Some(star_token);
        self
    }
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_return_type_annotation(
        mut self,
        return_type_annotation: TsReturnTypeAnnotation,
    ) -> Self {
        self.return_type_annotation = Some(return_type_annotation);
        self
    }
    pub fn build(self) -> JsFunctionDeclaration {
        JsFunctionDeclaration::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_FUNCTION_DECLARATION,
            [
                self.async_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.function_token)),
                self.star_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.id.into_syntax())),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                self.return_type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn js_function_export_default_declaration(
    function_token: SyntaxToken,
    parameters: JsParameters,
    body: JsFunctionBody,
) -> JsFunctionExportDefaultDeclarationBuilder {
    JsFunctionExportDefaultDeclarationBuilder {
        function_token,
        parameters,
        body,
        async_token: None,
        star_token: None,
        id: None,
        type_parameters: None,
        return_type_annotation: None,
    }
}
pub struct JsFunctionExportDefaultDeclarationBuilder {
    function_token: SyntaxToken,
    parameters: JsParameters,
    body: JsFunctionBody,
    async_token: Option<SyntaxToken>,
    star_token: Option<SyntaxToken>,
    id: Option<JsAnyBinding>,
    type_parameters: Option<TsTypeParameters>,
    return_type_annotation: Option<TsReturnTypeAnnotation>,
}
impl JsFunctionExportDefaultDeclarationBuilder {
    pub fn with_async_token(mut self, async_token: SyntaxToken) -> Self {
        self.async_token = Some(async_token);
        self
    }
    pub fn with_star_token(mut self, star_token: SyntaxToken) -> Self {
        self.star_token = Some(star_token);
        self
    }
    pub fn with_id(mut self, id: JsAnyBinding) -> Self {
        self.id = Some(id);
        self
    }
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_return_type_annotation(
        mut self,
        return_type_annotation: TsReturnTypeAnnotation,
    ) -> Self {
        self.return_type_annotation = Some(return_type_annotation);
        self
    }
    pub fn build(self) -> JsFunctionExportDefaultDeclaration {
        JsFunctionExportDefaultDeclaration::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION,
            [
                self.async_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.function_token)),
                self.star_token.map(|token| SyntaxElement::Token(token)),
                self.id
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                self.return_type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn js_function_expression(
    function_token: SyntaxToken,
    parameters: JsParameters,
    body: JsFunctionBody,
) -> JsFunctionExpressionBuilder {
    JsFunctionExpressionBuilder {
        function_token,
        parameters,
        body,
        async_token: None,
        star_token: None,
        id: None,
        type_parameters: None,
        return_type_annotation: None,
    }
}
pub struct JsFunctionExpressionBuilder {
    function_token: SyntaxToken,
    parameters: JsParameters,
    body: JsFunctionBody,
    async_token: Option<SyntaxToken>,
    star_token: Option<SyntaxToken>,
    id: Option<JsAnyBinding>,
    type_parameters: Option<TsTypeParameters>,
    return_type_annotation: Option<TsReturnTypeAnnotation>,
}
impl JsFunctionExpressionBuilder {
    pub fn with_async_token(mut self, async_token: SyntaxToken) -> Self {
        self.async_token = Some(async_token);
        self
    }
    pub fn with_star_token(mut self, star_token: SyntaxToken) -> Self {
        self.star_token = Some(star_token);
        self
    }
    pub fn with_id(mut self, id: JsAnyBinding) -> Self {
        self.id = Some(id);
        self
    }
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_return_type_annotation(
        mut self,
        return_type_annotation: TsReturnTypeAnnotation,
    ) -> Self {
        self.return_type_annotation = Some(return_type_annotation);
        self
    }
    pub fn build(self) -> JsFunctionExpression {
        JsFunctionExpression::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_FUNCTION_EXPRESSION,
            [
                self.async_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.function_token)),
                self.star_token.map(|token| SyntaxElement::Token(token)),
                self.id
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                self.return_type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn js_getter_class_member(
    modifiers: JsMethodModifierList,
    get_token: SyntaxToken,
    name: JsAnyClassMemberName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: JsFunctionBody,
) -> JsGetterClassMemberBuilder {
    JsGetterClassMemberBuilder {
        modifiers,
        get_token,
        name,
        l_paren_token,
        r_paren_token,
        body,
        return_type: None,
    }
}
pub struct JsGetterClassMemberBuilder {
    modifiers: JsMethodModifierList,
    get_token: SyntaxToken,
    name: JsAnyClassMemberName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: JsFunctionBody,
    return_type: Option<TsTypeAnnotation>,
}
impl JsGetterClassMemberBuilder {
    pub fn with_return_type(mut self, return_type: TsTypeAnnotation) -> Self {
        self.return_type = Some(return_type);
        self
    }
    pub fn build(self) -> JsGetterClassMember {
        JsGetterClassMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_GETTER_CLASS_MEMBER,
            [
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                Some(SyntaxElement::Token(self.get_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.return_type
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn js_getter_object_member(
    get_token: SyntaxToken,
    name: JsAnyObjectMemberName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: JsFunctionBody,
) -> JsGetterObjectMemberBuilder {
    JsGetterObjectMemberBuilder {
        get_token,
        name,
        l_paren_token,
        r_paren_token,
        body,
        return_type: None,
    }
}
pub struct JsGetterObjectMemberBuilder {
    get_token: SyntaxToken,
    name: JsAnyObjectMemberName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: JsFunctionBody,
    return_type: Option<TsTypeAnnotation>,
}
impl JsGetterObjectMemberBuilder {
    pub fn with_return_type(mut self, return_type: TsTypeAnnotation) -> Self {
        self.return_type = Some(return_type);
        self
    }
    pub fn build(self) -> JsGetterObjectMember {
        JsGetterObjectMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_GETTER_OBJECT_MEMBER,
            [
                Some(SyntaxElement::Token(self.get_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.return_type
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn js_identifier_assignment(name_token: SyntaxToken) -> JsIdentifierAssignment {
    JsIdentifierAssignment::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT,
        [Some(SyntaxElement::Token(name_token))],
    ))
}
pub fn js_identifier_binding(name_token: SyntaxToken) -> JsIdentifierBinding {
    JsIdentifierBinding::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_IDENTIFIER_BINDING,
        [Some(SyntaxElement::Token(name_token))],
    ))
}
pub fn js_identifier_expression(name: JsReferenceIdentifier) -> JsIdentifierExpression {
    JsIdentifierExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_IDENTIFIER_EXPRESSION,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn js_if_statement(
    if_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    test: JsAnyExpression,
    r_paren_token: SyntaxToken,
    consequent: JsAnyStatement,
) -> JsIfStatementBuilder {
    JsIfStatementBuilder {
        if_token,
        l_paren_token,
        test,
        r_paren_token,
        consequent,
        else_clause: None,
    }
}
pub struct JsIfStatementBuilder {
    if_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    test: JsAnyExpression,
    r_paren_token: SyntaxToken,
    consequent: JsAnyStatement,
    else_clause: Option<JsElseClause>,
}
impl JsIfStatementBuilder {
    pub fn with_else_clause(mut self, else_clause: JsElseClause) -> Self {
        self.else_clause = Some(else_clause);
        self
    }
    pub fn build(self) -> JsIfStatement {
        JsIfStatement::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_IF_STATEMENT,
            [
                Some(SyntaxElement::Token(self.if_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.test.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                Some(SyntaxElement::Node(self.consequent.into_syntax())),
                self.else_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn js_import(import_token: SyntaxToken, import_clause: JsAnyImportClause) -> JsImportBuilder {
    JsImportBuilder {
        import_token,
        import_clause,
        semicolon_token: None,
    }
}
pub struct JsImportBuilder {
    import_token: SyntaxToken,
    import_clause: JsAnyImportClause,
    semicolon_token: Option<SyntaxToken>,
}
impl JsImportBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsImport {
        JsImport::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_IMPORT,
            [
                Some(SyntaxElement::Token(self.import_token)),
                Some(SyntaxElement::Node(self.import_clause.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_import_assertion(
    assert_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    assertions: JsImportAssertionEntryList,
    r_curly_token: SyntaxToken,
) -> JsImportAssertion {
    JsImportAssertion::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_IMPORT_ASSERTION,
        [
            Some(SyntaxElement::Token(assert_token)),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(assertions.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn js_import_assertion_entry(
    key_token: SyntaxToken,
    colon_token: SyntaxToken,
    value_token: SyntaxToken,
) -> JsImportAssertionEntry {
    JsImportAssertionEntry::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY,
        [
            Some(SyntaxElement::Token(key_token)),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Token(value_token)),
        ],
    ))
}
pub fn js_import_bare_clause(source: JsModuleSource) -> JsImportBareClauseBuilder {
    JsImportBareClauseBuilder {
        source,
        assertion: None,
    }
}
pub struct JsImportBareClauseBuilder {
    source: JsModuleSource,
    assertion: Option<JsImportAssertion>,
}
impl JsImportBareClauseBuilder {
    pub fn with_assertion(mut self, assertion: JsImportAssertion) -> Self {
        self.assertion = Some(assertion);
        self
    }
    pub fn build(self) -> JsImportBareClause {
        JsImportBareClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_IMPORT_BARE_CLAUSE,
            [
                Some(SyntaxElement::Node(self.source.into_syntax())),
                self.assertion
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn js_import_call_expression(
    import_token: SyntaxToken,
    arguments: JsCallArguments,
) -> JsImportCallExpression {
    JsImportCallExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_IMPORT_CALL_EXPRESSION,
        [
            Some(SyntaxElement::Token(import_token)),
            Some(SyntaxElement::Node(arguments.into_syntax())),
        ],
    ))
}
pub fn js_import_default_clause(
    local_name: JsAnyBinding,
    from_token: SyntaxToken,
    source: JsModuleSource,
) -> JsImportDefaultClauseBuilder {
    JsImportDefaultClauseBuilder {
        local_name,
        from_token,
        source,
        type_token: None,
        assertion: None,
    }
}
pub struct JsImportDefaultClauseBuilder {
    local_name: JsAnyBinding,
    from_token: SyntaxToken,
    source: JsModuleSource,
    type_token: Option<SyntaxToken>,
    assertion: Option<JsImportAssertion>,
}
impl JsImportDefaultClauseBuilder {
    pub fn with_type_token(mut self, type_token: SyntaxToken) -> Self {
        self.type_token = Some(type_token);
        self
    }
    pub fn with_assertion(mut self, assertion: JsImportAssertion) -> Self {
        self.assertion = Some(assertion);
        self
    }
    pub fn build(self) -> JsImportDefaultClause {
        JsImportDefaultClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE,
            [
                self.type_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.local_name.into_syntax())),
                Some(SyntaxElement::Token(self.from_token)),
                Some(SyntaxElement::Node(self.source.into_syntax())),
                self.assertion
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn js_import_named_clause(
    named_import: JsAnyNamedImport,
    from_token: SyntaxToken,
    source: JsModuleSource,
) -> JsImportNamedClauseBuilder {
    JsImportNamedClauseBuilder {
        named_import,
        from_token,
        source,
        type_token: None,
        default_specifier: None,
        assertion: None,
    }
}
pub struct JsImportNamedClauseBuilder {
    named_import: JsAnyNamedImport,
    from_token: SyntaxToken,
    source: JsModuleSource,
    type_token: Option<SyntaxToken>,
    default_specifier: Option<JsDefaultImportSpecifier>,
    assertion: Option<JsImportAssertion>,
}
impl JsImportNamedClauseBuilder {
    pub fn with_type_token(mut self, type_token: SyntaxToken) -> Self {
        self.type_token = Some(type_token);
        self
    }
    pub fn with_default_specifier(mut self, default_specifier: JsDefaultImportSpecifier) -> Self {
        self.default_specifier = Some(default_specifier);
        self
    }
    pub fn with_assertion(mut self, assertion: JsImportAssertion) -> Self {
        self.assertion = Some(assertion);
        self
    }
    pub fn build(self) -> JsImportNamedClause {
        JsImportNamedClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE,
            [
                self.type_token.map(|token| SyntaxElement::Token(token)),
                self.default_specifier
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.named_import.into_syntax())),
                Some(SyntaxElement::Token(self.from_token)),
                Some(SyntaxElement::Node(self.source.into_syntax())),
                self.assertion
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn js_import_namespace_clause(
    star_token: SyntaxToken,
    as_token: SyntaxToken,
    local_name: JsAnyBinding,
    from_token: SyntaxToken,
    source: JsModuleSource,
) -> JsImportNamespaceClauseBuilder {
    JsImportNamespaceClauseBuilder {
        star_token,
        as_token,
        local_name,
        from_token,
        source,
        type_token: None,
        assertion: None,
    }
}
pub struct JsImportNamespaceClauseBuilder {
    star_token: SyntaxToken,
    as_token: SyntaxToken,
    local_name: JsAnyBinding,
    from_token: SyntaxToken,
    source: JsModuleSource,
    type_token: Option<SyntaxToken>,
    assertion: Option<JsImportAssertion>,
}
impl JsImportNamespaceClauseBuilder {
    pub fn with_type_token(mut self, type_token: SyntaxToken) -> Self {
        self.type_token = Some(type_token);
        self
    }
    pub fn with_assertion(mut self, assertion: JsImportAssertion) -> Self {
        self.assertion = Some(assertion);
        self
    }
    pub fn build(self) -> JsImportNamespaceClause {
        JsImportNamespaceClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE,
            [
                self.type_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.star_token)),
                Some(SyntaxElement::Token(self.as_token)),
                Some(SyntaxElement::Node(self.local_name.into_syntax())),
                Some(SyntaxElement::Token(self.from_token)),
                Some(SyntaxElement::Node(self.source.into_syntax())),
                self.assertion
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn js_in_expression(
    property: JsAnyInProperty,
    in_token: SyntaxToken,
    object: JsAnyExpression,
) -> JsInExpression {
    JsInExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_IN_EXPRESSION,
        [
            Some(SyntaxElement::Node(property.into_syntax())),
            Some(SyntaxElement::Token(in_token)),
            Some(SyntaxElement::Node(object.into_syntax())),
        ],
    ))
}
pub fn js_initializer_clause(
    eq_token: SyntaxToken,
    expression: JsAnyExpression,
) -> JsInitializerClause {
    JsInitializerClause::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_INITIALIZER_CLAUSE,
        [
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
        ],
    ))
}
pub fn js_instanceof_expression(
    left: JsAnyExpression,
    instanceof_token: SyntaxToken,
    right: JsAnyExpression,
) -> JsInstanceofExpression {
    JsInstanceofExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_INSTANCEOF_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(instanceof_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn js_labeled_statement(
    label_token: SyntaxToken,
    colon_token: SyntaxToken,
    body: JsAnyStatement,
) -> JsLabeledStatement {
    JsLabeledStatement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_LABELED_STATEMENT,
        [
            Some(SyntaxElement::Token(label_token)),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn js_literal_export_name(value_token: SyntaxToken) -> JsLiteralExportName {
    JsLiteralExportName::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_LITERAL_EXPORT_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn js_literal_member_name(value_token: SyntaxToken) -> JsLiteralMemberName {
    JsLiteralMemberName::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_LITERAL_MEMBER_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn js_logical_expression(
    left: JsAnyExpression,
    operator_token_token: SyntaxToken,
    right: JsAnyExpression,
) -> JsLogicalExpression {
    JsLogicalExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_LOGICAL_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn js_method_class_member(
    modifiers: JsMethodModifierList,
    name: JsAnyClassMemberName,
    parameters: JsParameters,
    body: JsFunctionBody,
) -> JsMethodClassMemberBuilder {
    JsMethodClassMemberBuilder {
        modifiers,
        name,
        parameters,
        body,
        async_token: None,
        star_token: None,
        question_mark_token: None,
        type_parameters: None,
        return_type_annotation: None,
    }
}
pub struct JsMethodClassMemberBuilder {
    modifiers: JsMethodModifierList,
    name: JsAnyClassMemberName,
    parameters: JsParameters,
    body: JsFunctionBody,
    async_token: Option<SyntaxToken>,
    star_token: Option<SyntaxToken>,
    question_mark_token: Option<SyntaxToken>,
    type_parameters: Option<TsTypeParameters>,
    return_type_annotation: Option<TsReturnTypeAnnotation>,
}
impl JsMethodClassMemberBuilder {
    pub fn with_async_token(mut self, async_token: SyntaxToken) -> Self {
        self.async_token = Some(async_token);
        self
    }
    pub fn with_star_token(mut self, star_token: SyntaxToken) -> Self {
        self.star_token = Some(star_token);
        self
    }
    pub fn with_question_mark_token(mut self, question_mark_token: SyntaxToken) -> Self {
        self.question_mark_token = Some(question_mark_token);
        self
    }
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_return_type_annotation(
        mut self,
        return_type_annotation: TsReturnTypeAnnotation,
    ) -> Self {
        self.return_type_annotation = Some(return_type_annotation);
        self
    }
    pub fn build(self) -> JsMethodClassMember {
        JsMethodClassMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_METHOD_CLASS_MEMBER,
            [
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                self.async_token.map(|token| SyntaxElement::Token(token)),
                self.star_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.question_mark_token
                    .map(|token| SyntaxElement::Token(token)),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                self.return_type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn js_method_object_member(
    name: JsAnyObjectMemberName,
    parameters: JsParameters,
    body: JsFunctionBody,
) -> JsMethodObjectMemberBuilder {
    JsMethodObjectMemberBuilder {
        name,
        parameters,
        body,
        async_token: None,
        star_token: None,
        type_parameters: None,
        return_type_annotation: None,
    }
}
pub struct JsMethodObjectMemberBuilder {
    name: JsAnyObjectMemberName,
    parameters: JsParameters,
    body: JsFunctionBody,
    async_token: Option<SyntaxToken>,
    star_token: Option<SyntaxToken>,
    type_parameters: Option<TsTypeParameters>,
    return_type_annotation: Option<TsReturnTypeAnnotation>,
}
impl JsMethodObjectMemberBuilder {
    pub fn with_async_token(mut self, async_token: SyntaxToken) -> Self {
        self.async_token = Some(async_token);
        self
    }
    pub fn with_star_token(mut self, star_token: SyntaxToken) -> Self {
        self.star_token = Some(star_token);
        self
    }
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_return_type_annotation(
        mut self,
        return_type_annotation: TsReturnTypeAnnotation,
    ) -> Self {
        self.return_type_annotation = Some(return_type_annotation);
        self
    }
    pub fn build(self) -> JsMethodObjectMember {
        JsMethodObjectMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_METHOD_OBJECT_MEMBER,
            [
                self.async_token.map(|token| SyntaxElement::Token(token)),
                self.star_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                self.return_type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn js_module(
    directives: JsDirectiveList,
    items: JsModuleItemList,
    eof_token: SyntaxToken,
) -> JsModuleBuilder {
    JsModuleBuilder {
        directives,
        items,
        eof_token,
        interpreter_token: None,
    }
}
pub struct JsModuleBuilder {
    directives: JsDirectiveList,
    items: JsModuleItemList,
    eof_token: SyntaxToken,
    interpreter_token: Option<SyntaxToken>,
}
impl JsModuleBuilder {
    pub fn with_interpreter_token(mut self, interpreter_token: SyntaxToken) -> Self {
        self.interpreter_token = Some(interpreter_token);
        self
    }
    pub fn build(self) -> JsModule {
        JsModule::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_MODULE,
            [
                self.interpreter_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                Some(SyntaxElement::Node(self.items.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn js_module_source(value_token: SyntaxToken) -> JsModuleSource {
    JsModuleSource::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_MODULE_SOURCE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn js_name(value_token: SyntaxToken) -> JsName {
    JsName::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn js_named_import_specifier(
    name: JsLiteralExportName,
    as_token: SyntaxToken,
    local_name: JsAnyBinding,
) -> JsNamedImportSpecifierBuilder {
    JsNamedImportSpecifierBuilder {
        name,
        as_token,
        local_name,
        type_token: None,
    }
}
pub struct JsNamedImportSpecifierBuilder {
    name: JsLiteralExportName,
    as_token: SyntaxToken,
    local_name: JsAnyBinding,
    type_token: Option<SyntaxToken>,
}
impl JsNamedImportSpecifierBuilder {
    pub fn with_type_token(mut self, type_token: SyntaxToken) -> Self {
        self.type_token = Some(type_token);
        self
    }
    pub fn build(self) -> JsNamedImportSpecifier {
        JsNamedImportSpecifier::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER,
            [
                self.type_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.as_token)),
                Some(SyntaxElement::Node(self.local_name.into_syntax())),
            ],
        ))
    }
}
pub fn js_named_import_specifiers(
    l_curly_token: SyntaxToken,
    specifiers: JsNamedImportSpecifierList,
    r_curly_token: SyntaxToken,
) -> JsNamedImportSpecifiers {
    JsNamedImportSpecifiers::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIERS,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(specifiers.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn js_namespace_import_specifier(
    star_token: SyntaxToken,
    as_token: SyntaxToken,
    local_name: JsAnyBinding,
) -> JsNamespaceImportSpecifier {
    JsNamespaceImportSpecifier::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_NAMESPACE_IMPORT_SPECIFIER,
        [
            Some(SyntaxElement::Token(star_token)),
            Some(SyntaxElement::Token(as_token)),
            Some(SyntaxElement::Node(local_name.into_syntax())),
        ],
    ))
}
pub fn js_new_expression(
    new_token: SyntaxToken,
    callee: JsAnyExpression,
) -> JsNewExpressionBuilder {
    JsNewExpressionBuilder {
        new_token,
        callee,
        type_arguments: None,
        arguments: None,
    }
}
pub struct JsNewExpressionBuilder {
    new_token: SyntaxToken,
    callee: JsAnyExpression,
    type_arguments: Option<TsTypeArguments>,
    arguments: Option<JsCallArguments>,
}
impl JsNewExpressionBuilder {
    pub fn with_type_arguments(mut self, type_arguments: TsTypeArguments) -> Self {
        self.type_arguments = Some(type_arguments);
        self
    }
    pub fn with_arguments(mut self, arguments: JsCallArguments) -> Self {
        self.arguments = Some(arguments);
        self
    }
    pub fn build(self) -> JsNewExpression {
        JsNewExpression::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_NEW_EXPRESSION,
            [
                Some(SyntaxElement::Token(self.new_token)),
                Some(SyntaxElement::Node(self.callee.into_syntax())),
                self.type_arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn js_null_literal_expression(value_token: SyntaxToken) -> JsNullLiteralExpression {
    JsNullLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_NULL_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn js_number_literal_expression(value_token: SyntaxToken) -> JsNumberLiteralExpression {
    JsNumberLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn js_object_assignment_pattern(
    l_curly_token: SyntaxToken,
    properties: JsObjectAssignmentPatternPropertyList,
    r_curly_token: SyntaxToken,
) -> JsObjectAssignmentPattern {
    JsObjectAssignmentPattern::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(properties.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn js_object_assignment_pattern_property(
    member: JsAnyObjectMemberName,
    colon_token: SyntaxToken,
    pattern: JsAnyAssignmentPattern,
) -> JsObjectAssignmentPatternPropertyBuilder {
    JsObjectAssignmentPatternPropertyBuilder {
        member,
        colon_token,
        pattern,
        init: None,
    }
}
pub struct JsObjectAssignmentPatternPropertyBuilder {
    member: JsAnyObjectMemberName,
    colon_token: SyntaxToken,
    pattern: JsAnyAssignmentPattern,
    init: Option<JsInitializerClause>,
}
impl JsObjectAssignmentPatternPropertyBuilder {
    pub fn with_init(mut self, init: JsInitializerClause) -> Self {
        self.init = Some(init);
        self
    }
    pub fn build(self) -> JsObjectAssignmentPatternProperty {
        JsObjectAssignmentPatternProperty::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY,
            [
                Some(SyntaxElement::Node(self.member.into_syntax())),
                Some(SyntaxElement::Token(self.colon_token)),
                Some(SyntaxElement::Node(self.pattern.into_syntax())),
                self.init
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn js_object_assignment_pattern_rest(
    dotdotdot_token: SyntaxToken,
    target: JsAnyAssignment,
) -> JsObjectAssignmentPatternRest {
    JsObjectAssignmentPatternRest::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_REST,
        [
            Some(SyntaxElement::Token(dotdotdot_token)),
            Some(SyntaxElement::Node(target.into_syntax())),
        ],
    ))
}
pub fn js_object_assignment_pattern_shorthand_property(
    identifier: JsIdentifierAssignment,
) -> JsObjectAssignmentPatternShorthandPropertyBuilder {
    JsObjectAssignmentPatternShorthandPropertyBuilder {
        identifier,
        init: None,
    }
}
pub struct JsObjectAssignmentPatternShorthandPropertyBuilder {
    identifier: JsIdentifierAssignment,
    init: Option<JsInitializerClause>,
}
impl JsObjectAssignmentPatternShorthandPropertyBuilder {
    pub fn with_init(mut self, init: JsInitializerClause) -> Self {
        self.init = Some(init);
        self
    }
    pub fn build(self) -> JsObjectAssignmentPatternShorthandProperty {
        JsObjectAssignmentPatternShorthandProperty::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY,
            [
                Some(SyntaxElement::Node(self.identifier.into_syntax())),
                self.init
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn js_object_binding_pattern(
    l_curly_token: SyntaxToken,
    properties: JsObjectBindingPatternPropertyList,
    r_curly_token: SyntaxToken,
) -> JsObjectBindingPattern {
    JsObjectBindingPattern::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_OBJECT_BINDING_PATTERN,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(properties.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn js_object_binding_pattern_property(
    member: JsAnyObjectMemberName,
    colon_token: SyntaxToken,
    pattern: JsAnyBindingPattern,
) -> JsObjectBindingPatternPropertyBuilder {
    JsObjectBindingPatternPropertyBuilder {
        member,
        colon_token,
        pattern,
        init: None,
    }
}
pub struct JsObjectBindingPatternPropertyBuilder {
    member: JsAnyObjectMemberName,
    colon_token: SyntaxToken,
    pattern: JsAnyBindingPattern,
    init: Option<JsInitializerClause>,
}
impl JsObjectBindingPatternPropertyBuilder {
    pub fn with_init(mut self, init: JsInitializerClause) -> Self {
        self.init = Some(init);
        self
    }
    pub fn build(self) -> JsObjectBindingPatternProperty {
        JsObjectBindingPatternProperty::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY,
            [
                Some(SyntaxElement::Node(self.member.into_syntax())),
                Some(SyntaxElement::Token(self.colon_token)),
                Some(SyntaxElement::Node(self.pattern.into_syntax())),
                self.init
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn js_object_binding_pattern_rest(
    dotdotdot_token: SyntaxToken,
    binding: JsAnyBinding,
) -> JsObjectBindingPatternRest {
    JsObjectBindingPatternRest::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST,
        [
            Some(SyntaxElement::Token(dotdotdot_token)),
            Some(SyntaxElement::Node(binding.into_syntax())),
        ],
    ))
}
pub fn js_object_binding_pattern_shorthand_property(
    identifier: JsAnyBinding,
) -> JsObjectBindingPatternShorthandPropertyBuilder {
    JsObjectBindingPatternShorthandPropertyBuilder {
        identifier,
        init: None,
    }
}
pub struct JsObjectBindingPatternShorthandPropertyBuilder {
    identifier: JsAnyBinding,
    init: Option<JsInitializerClause>,
}
impl JsObjectBindingPatternShorthandPropertyBuilder {
    pub fn with_init(mut self, init: JsInitializerClause) -> Self {
        self.init = Some(init);
        self
    }
    pub fn build(self) -> JsObjectBindingPatternShorthandProperty {
        JsObjectBindingPatternShorthandProperty::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY,
            [
                Some(SyntaxElement::Node(self.identifier.into_syntax())),
                self.init
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn js_object_expression(
    l_curly_token: SyntaxToken,
    members: JsObjectMemberList,
    r_curly_token: SyntaxToken,
) -> JsObjectExpression {
    JsObjectExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_OBJECT_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(members.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn js_parameters(
    l_paren_token: SyntaxToken,
    items: JsParameterList,
    r_paren_token: SyntaxToken,
) -> JsParameters {
    JsParameters::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_PARAMETERS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn js_parenthesized_assignment(
    l_paren_token: SyntaxToken,
    assignment: JsAnyAssignment,
    r_paren_token: SyntaxToken,
) -> JsParenthesizedAssignment {
    JsParenthesizedAssignment::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_PARENTHESIZED_ASSIGNMENT,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(assignment.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn js_parenthesized_expression(
    l_paren_token: SyntaxToken,
    expression: JsAnyExpression,
    r_paren_token: SyntaxToken,
) -> JsParenthesizedExpression {
    JsParenthesizedExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn js_post_update_expression(
    operand: JsAnyAssignment,
    operator_token_token: SyntaxToken,
) -> JsPostUpdateExpression {
    JsPostUpdateExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_POST_UPDATE_EXPRESSION,
        [
            Some(SyntaxElement::Node(operand.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
        ],
    ))
}
pub fn js_pre_update_expression(
    operator_token_token: SyntaxToken,
    operand: JsAnyAssignment,
) -> JsPreUpdateExpression {
    JsPreUpdateExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION,
        [
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(operand.into_syntax())),
        ],
    ))
}
pub fn js_private_class_member_name(
    hash_token: SyntaxToken,
    id_token: SyntaxToken,
) -> JsPrivateClassMemberName {
    JsPrivateClassMemberName::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_PRIVATE_CLASS_MEMBER_NAME,
        [
            Some(SyntaxElement::Token(hash_token)),
            Some(SyntaxElement::Token(id_token)),
        ],
    ))
}
pub fn js_private_name(hash_token: SyntaxToken, value_token: SyntaxToken) -> JsPrivateName {
    JsPrivateName::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_PRIVATE_NAME,
        [
            Some(SyntaxElement::Token(hash_token)),
            Some(SyntaxElement::Token(value_token)),
        ],
    ))
}
pub fn js_property_class_member(
    modifiers: JsPropertyModifierList,
    name: JsAnyClassMemberName,
) -> JsPropertyClassMemberBuilder {
    JsPropertyClassMemberBuilder {
        modifiers,
        name,
        property_annotation: None,
        value: None,
        semicolon_token: None,
    }
}
pub struct JsPropertyClassMemberBuilder {
    modifiers: JsPropertyModifierList,
    name: JsAnyClassMemberName,
    property_annotation: Option<TsAnyPropertyAnnotation>,
    value: Option<JsInitializerClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl JsPropertyClassMemberBuilder {
    pub fn with_property_annotation(
        mut self,
        property_annotation: TsAnyPropertyAnnotation,
    ) -> Self {
        self.property_annotation = Some(property_annotation);
        self
    }
    pub fn with_value(mut self, value: JsInitializerClause) -> Self {
        self.value = Some(value);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsPropertyClassMember {
        JsPropertyClassMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER,
            [
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.property_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_property_object_member(
    name: JsAnyObjectMemberName,
    colon_token: SyntaxToken,
    value: JsAnyExpression,
) -> JsPropertyObjectMember {
    JsPropertyObjectMember::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn js_reference_identifier(value_token: SyntaxToken) -> JsReferenceIdentifier {
    JsReferenceIdentifier::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_REFERENCE_IDENTIFIER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn js_regex_literal_expression(value_token: SyntaxToken) -> JsRegexLiteralExpression {
    JsRegexLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_REGEX_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn js_rest_parameter(
    dotdotdot_token: SyntaxToken,
    binding: JsAnyBindingPattern,
) -> JsRestParameterBuilder {
    JsRestParameterBuilder {
        dotdotdot_token,
        binding,
        type_annotation: None,
    }
}
pub struct JsRestParameterBuilder {
    dotdotdot_token: SyntaxToken,
    binding: JsAnyBindingPattern,
    type_annotation: Option<TsTypeAnnotation>,
}
impl JsRestParameterBuilder {
    pub fn with_type_annotation(mut self, type_annotation: TsTypeAnnotation) -> Self {
        self.type_annotation = Some(type_annotation);
        self
    }
    pub fn build(self) -> JsRestParameter {
        JsRestParameter::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_REST_PARAMETER,
            [
                Some(SyntaxElement::Token(self.dotdotdot_token)),
                Some(SyntaxElement::Node(self.binding.into_syntax())),
                self.type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn js_return_statement(return_token: SyntaxToken) -> JsReturnStatementBuilder {
    JsReturnStatementBuilder {
        return_token,
        argument: None,
        semicolon_token: None,
    }
}
pub struct JsReturnStatementBuilder {
    return_token: SyntaxToken,
    argument: Option<JsAnyExpression>,
    semicolon_token: Option<SyntaxToken>,
}
impl JsReturnStatementBuilder {
    pub fn with_argument(mut self, argument: JsAnyExpression) -> Self {
        self.argument = Some(argument);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsReturnStatement {
        JsReturnStatement::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_RETURN_STATEMENT,
            [
                Some(SyntaxElement::Token(self.return_token)),
                self.argument
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_script(
    directives: JsDirectiveList,
    statements: JsStatementList,
    eof_token: SyntaxToken,
) -> JsScriptBuilder {
    JsScriptBuilder {
        directives,
        statements,
        eof_token,
        interpreter_token: None,
    }
}
pub struct JsScriptBuilder {
    directives: JsDirectiveList,
    statements: JsStatementList,
    eof_token: SyntaxToken,
    interpreter_token: Option<SyntaxToken>,
}
impl JsScriptBuilder {
    pub fn with_interpreter_token(mut self, interpreter_token: SyntaxToken) -> Self {
        self.interpreter_token = Some(interpreter_token);
        self
    }
    pub fn build(self) -> JsScript {
        JsScript::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_SCRIPT,
            [
                self.interpreter_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                Some(SyntaxElement::Node(self.statements.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn js_sequence_expression(
    left: JsAnyExpression,
    comma_token: SyntaxToken,
    right: JsAnyExpression,
) -> JsSequenceExpression {
    JsSequenceExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_SEQUENCE_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(comma_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn js_setter_class_member(
    modifiers: JsMethodModifierList,
    set_token: SyntaxToken,
    name: JsAnyClassMemberName,
    l_paren_token: SyntaxToken,
    parameter: JsAnyFormalParameter,
    r_paren_token: SyntaxToken,
    body: JsFunctionBody,
) -> JsSetterClassMember {
    JsSetterClassMember::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_SETTER_CLASS_MEMBER,
        [
            Some(SyntaxElement::Node(modifiers.into_syntax())),
            Some(SyntaxElement::Token(set_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(parameter.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn js_setter_object_member(
    set_token: SyntaxToken,
    name: JsAnyObjectMemberName,
    l_paren_token: SyntaxToken,
    parameter: JsAnyFormalParameter,
    r_paren_token: SyntaxToken,
    body: JsFunctionBody,
) -> JsSetterObjectMember {
    JsSetterObjectMember::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_SETTER_OBJECT_MEMBER,
        [
            Some(SyntaxElement::Token(set_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(parameter.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn js_shorthand_named_import_specifier(
    local_name: JsAnyBinding,
) -> JsShorthandNamedImportSpecifierBuilder {
    JsShorthandNamedImportSpecifierBuilder {
        local_name,
        type_token: None,
    }
}
pub struct JsShorthandNamedImportSpecifierBuilder {
    local_name: JsAnyBinding,
    type_token: Option<SyntaxToken>,
}
impl JsShorthandNamedImportSpecifierBuilder {
    pub fn with_type_token(mut self, type_token: SyntaxToken) -> Self {
        self.type_token = Some(type_token);
        self
    }
    pub fn build(self) -> JsShorthandNamedImportSpecifier {
        JsShorthandNamedImportSpecifier::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER,
            [
                self.type_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.local_name.into_syntax())),
            ],
        ))
    }
}
pub fn js_shorthand_property_object_member(
    name: JsReferenceIdentifier,
) -> JsShorthandPropertyObjectMember {
    JsShorthandPropertyObjectMember::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn js_spread(dotdotdot_token: SyntaxToken, argument: JsAnyExpression) -> JsSpread {
    JsSpread::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_SPREAD,
        [
            Some(SyntaxElement::Token(dotdotdot_token)),
            Some(SyntaxElement::Node(argument.into_syntax())),
        ],
    ))
}
pub fn js_static_initialization_block_class_member(
    static_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    statements: JsStatementList,
    r_curly_token: SyntaxToken,
) -> JsStaticInitializationBlockClassMember {
    JsStaticInitializationBlockClassMember::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER,
        [
            Some(SyntaxElement::Token(static_token)),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(statements.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn js_static_member_assignment(
    object: JsAnyExpression,
    dot_token: SyntaxToken,
    member: JsAnyName,
) -> JsStaticMemberAssignment {
    JsStaticMemberAssignment::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT,
        [
            Some(SyntaxElement::Node(object.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(member.into_syntax())),
        ],
    ))
}
pub fn js_static_member_expression(
    object: JsAnyExpression,
    operator_token_token: SyntaxToken,
    member: JsAnyName,
) -> JsStaticMemberExpression {
    JsStaticMemberExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION,
        [
            Some(SyntaxElement::Node(object.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(member.into_syntax())),
        ],
    ))
}
pub fn js_static_modifier(modifier_token: SyntaxToken) -> JsStaticModifier {
    JsStaticModifier::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_STATIC_MODIFIER,
        [Some(SyntaxElement::Token(modifier_token))],
    ))
}
pub fn js_string_literal_expression(value_token: SyntaxToken) -> JsStringLiteralExpression {
    JsStringLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn js_super_expression(super_token: SyntaxToken) -> JsSuperExpression {
    JsSuperExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_SUPER_EXPRESSION,
        [Some(SyntaxElement::Token(super_token))],
    ))
}
pub fn js_switch_statement(
    switch_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    discriminant: JsAnyExpression,
    r_paren_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    cases: JsSwitchCaseList,
    r_curly_token: SyntaxToken,
) -> JsSwitchStatement {
    JsSwitchStatement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_SWITCH_STATEMENT,
        [
            Some(SyntaxElement::Token(switch_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(discriminant.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(cases.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn js_template(
    l_tick_token: SyntaxToken,
    elements: JsTemplateElementList,
    r_tick_token: SyntaxToken,
) -> JsTemplateBuilder {
    JsTemplateBuilder {
        l_tick_token,
        elements,
        r_tick_token,
        tag: None,
        type_arguments: None,
    }
}
pub struct JsTemplateBuilder {
    l_tick_token: SyntaxToken,
    elements: JsTemplateElementList,
    r_tick_token: SyntaxToken,
    tag: Option<JsAnyExpression>,
    type_arguments: Option<TsTypeArguments>,
}
impl JsTemplateBuilder {
    pub fn with_tag(mut self, tag: JsAnyExpression) -> Self {
        self.tag = Some(tag);
        self
    }
    pub fn with_type_arguments(mut self, type_arguments: TsTypeArguments) -> Self {
        self.type_arguments = Some(type_arguments);
        self
    }
    pub fn build(self) -> JsTemplate {
        JsTemplate::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_TEMPLATE,
            [
                self.tag
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.type_arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.l_tick_token)),
                Some(SyntaxElement::Node(self.elements.into_syntax())),
                Some(SyntaxElement::Token(self.r_tick_token)),
            ],
        ))
    }
}
pub fn js_template_chunk_element(template_chunk_token: SyntaxToken) -> JsTemplateChunkElement {
    JsTemplateChunkElement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_TEMPLATE_CHUNK_ELEMENT,
        [Some(SyntaxElement::Token(template_chunk_token))],
    ))
}
pub fn js_template_element(
    dollar_curly_token: SyntaxToken,
    expression: JsAnyExpression,
    r_curly_token: SyntaxToken,
) -> JsTemplateElement {
    JsTemplateElement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_TEMPLATE_ELEMENT,
        [
            Some(SyntaxElement::Token(dollar_curly_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn js_this_expression(this_token: SyntaxToken) -> JsThisExpression {
    JsThisExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_THIS_EXPRESSION,
        [Some(SyntaxElement::Token(this_token))],
    ))
}
pub fn js_throw_statement(
    throw_token: SyntaxToken,
    argument: JsAnyExpression,
) -> JsThrowStatementBuilder {
    JsThrowStatementBuilder {
        throw_token,
        argument,
        semicolon_token: None,
    }
}
pub struct JsThrowStatementBuilder {
    throw_token: SyntaxToken,
    argument: JsAnyExpression,
    semicolon_token: Option<SyntaxToken>,
}
impl JsThrowStatementBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsThrowStatement {
        JsThrowStatement::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_THROW_STATEMENT,
            [
                Some(SyntaxElement::Token(self.throw_token)),
                Some(SyntaxElement::Node(self.argument.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_try_finally_statement(
    try_token: SyntaxToken,
    body: JsBlockStatement,
    finally_clause: JsFinallyClause,
) -> JsTryFinallyStatementBuilder {
    JsTryFinallyStatementBuilder {
        try_token,
        body,
        finally_clause,
        catch_clause: None,
    }
}
pub struct JsTryFinallyStatementBuilder {
    try_token: SyntaxToken,
    body: JsBlockStatement,
    finally_clause: JsFinallyClause,
    catch_clause: Option<JsCatchClause>,
}
impl JsTryFinallyStatementBuilder {
    pub fn with_catch_clause(mut self, catch_clause: JsCatchClause) -> Self {
        self.catch_clause = Some(catch_clause);
        self
    }
    pub fn build(self) -> JsTryFinallyStatement {
        JsTryFinallyStatement::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_TRY_FINALLY_STATEMENT,
            [
                Some(SyntaxElement::Token(self.try_token)),
                Some(SyntaxElement::Node(self.body.into_syntax())),
                self.catch_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.finally_clause.into_syntax())),
            ],
        ))
    }
}
pub fn js_try_statement(
    try_token: SyntaxToken,
    body: JsBlockStatement,
    catch_clause: JsCatchClause,
) -> JsTryStatement {
    JsTryStatement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_TRY_STATEMENT,
        [
            Some(SyntaxElement::Token(try_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
            Some(SyntaxElement::Node(catch_clause.into_syntax())),
        ],
    ))
}
pub fn js_unary_expression(
    operator_token_token: SyntaxToken,
    argument: JsAnyExpression,
) -> JsUnaryExpression {
    JsUnaryExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_UNARY_EXPRESSION,
        [
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(argument.into_syntax())),
        ],
    ))
}
pub fn js_variable_declaration(
    kind_token: SyntaxToken,
    declarators: JsVariableDeclaratorList,
) -> JsVariableDeclaration {
    JsVariableDeclaration::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_VARIABLE_DECLARATION,
        [
            Some(SyntaxElement::Token(kind_token)),
            Some(SyntaxElement::Node(declarators.into_syntax())),
        ],
    ))
}
pub fn js_variable_declaration_clause(
    declaration: JsVariableDeclaration,
) -> JsVariableDeclarationClauseBuilder {
    JsVariableDeclarationClauseBuilder {
        declaration,
        semicolon_token: None,
    }
}
pub struct JsVariableDeclarationClauseBuilder {
    declaration: JsVariableDeclaration,
    semicolon_token: Option<SyntaxToken>,
}
impl JsVariableDeclarationClauseBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsVariableDeclarationClause {
        JsVariableDeclarationClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_VARIABLE_DECLARATION_CLAUSE,
            [
                Some(SyntaxElement::Node(self.declaration.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_variable_declarator(id: JsAnyBindingPattern) -> JsVariableDeclaratorBuilder {
    JsVariableDeclaratorBuilder {
        id,
        variable_annotation: None,
        initializer: None,
    }
}
pub struct JsVariableDeclaratorBuilder {
    id: JsAnyBindingPattern,
    variable_annotation: Option<TsAnyVariableAnnotation>,
    initializer: Option<JsInitializerClause>,
}
impl JsVariableDeclaratorBuilder {
    pub fn with_variable_annotation(
        mut self,
        variable_annotation: TsAnyVariableAnnotation,
    ) -> Self {
        self.variable_annotation = Some(variable_annotation);
        self
    }
    pub fn with_initializer(mut self, initializer: JsInitializerClause) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn build(self) -> JsVariableDeclarator {
        JsVariableDeclarator::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_VARIABLE_DECLARATOR,
            [
                Some(SyntaxElement::Node(self.id.into_syntax())),
                self.variable_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn js_variable_statement(declaration: JsVariableDeclaration) -> JsVariableStatementBuilder {
    JsVariableStatementBuilder {
        declaration,
        semicolon_token: None,
    }
}
pub struct JsVariableStatementBuilder {
    declaration: JsVariableDeclaration,
    semicolon_token: Option<SyntaxToken>,
}
impl JsVariableStatementBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> JsVariableStatement {
        JsVariableStatement::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_VARIABLE_STATEMENT,
            [
                Some(SyntaxElement::Node(self.declaration.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn js_while_statement(
    while_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    test: JsAnyExpression,
    r_paren_token: SyntaxToken,
    body: JsAnyStatement,
) -> JsWhileStatement {
    JsWhileStatement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_WHILE_STATEMENT,
        [
            Some(SyntaxElement::Token(while_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(test.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn js_with_statement(
    with_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    object: JsAnyExpression,
    r_paren_token: SyntaxToken,
    body: JsAnyStatement,
) -> JsWithStatement {
    JsWithStatement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_WITH_STATEMENT,
        [
            Some(SyntaxElement::Token(with_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(object.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn js_yield_argument(expression: JsAnyExpression) -> JsYieldArgumentBuilder {
    JsYieldArgumentBuilder {
        expression,
        star_token: None,
    }
}
pub struct JsYieldArgumentBuilder {
    expression: JsAnyExpression,
    star_token: Option<SyntaxToken>,
}
impl JsYieldArgumentBuilder {
    pub fn with_star_token(mut self, star_token: SyntaxToken) -> Self {
        self.star_token = Some(star_token);
        self
    }
    pub fn build(self) -> JsYieldArgument {
        JsYieldArgument::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_YIELD_ARGUMENT,
            [
                self.star_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.expression.into_syntax())),
            ],
        ))
    }
}
pub fn js_yield_expression(yield_token: SyntaxToken) -> JsYieldExpressionBuilder {
    JsYieldExpressionBuilder {
        yield_token,
        argument: None,
    }
}
pub struct JsYieldExpressionBuilder {
    yield_token: SyntaxToken,
    argument: Option<JsYieldArgument>,
}
impl JsYieldExpressionBuilder {
    pub fn with_argument(mut self, argument: JsYieldArgument) -> Self {
        self.argument = Some(argument);
        self
    }
    pub fn build(self) -> JsYieldExpression {
        JsYieldExpression::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JS_YIELD_EXPRESSION,
            [
                Some(SyntaxElement::Token(self.yield_token)),
                self.argument
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn jsx_attribute(name: JsxAnyAttributeName) -> JsxAttributeBuilder {
    JsxAttributeBuilder {
        name,
        initializer: None,
    }
}
pub struct JsxAttributeBuilder {
    name: JsxAnyAttributeName,
    initializer: Option<JsxAttributeInitializerClause>,
}
impl JsxAttributeBuilder {
    pub fn with_initializer(mut self, initializer: JsxAttributeInitializerClause) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn build(self) -> JsxAttribute {
        JsxAttribute::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JSX_ATTRIBUTE,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn jsx_attribute_initializer_clause(
    eq_token: SyntaxToken,
    value: JsxAnyAttributeValue,
) -> JsxAttributeInitializerClause {
    JsxAttributeInitializerClause::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_ATTRIBUTE_INITIALIZER_CLAUSE,
        [
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn jsx_closing_element(
    l_angle_token: SyntaxToken,
    slash_token: SyntaxToken,
    name: JsxAnyElementName,
    r_angle_token: SyntaxToken,
) -> JsxClosingElement {
    JsxClosingElement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_CLOSING_ELEMENT,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Token(slash_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn jsx_closing_fragment(
    l_angle_token: SyntaxToken,
    slash_token: SyntaxToken,
    r_angle_token: SyntaxToken,
) -> JsxClosingFragment {
    JsxClosingFragment::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_CLOSING_FRAGMENT,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Token(slash_token)),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn jsx_element(
    opening_element: JsxOpeningElement,
    children: JsxChildList,
    closing_element: JsxClosingElement,
) -> JsxElement {
    JsxElement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_ELEMENT,
        [
            Some(SyntaxElement::Node(opening_element.into_syntax())),
            Some(SyntaxElement::Node(children.into_syntax())),
            Some(SyntaxElement::Node(closing_element.into_syntax())),
        ],
    ))
}
pub fn jsx_expression_attribute_value(
    l_curly_token: SyntaxToken,
    expression: JsAnyExpression,
    r_curly_token: SyntaxToken,
) -> JsxExpressionAttributeValue {
    JsxExpressionAttributeValue::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn jsx_expression_child(
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
) -> JsxExpressionChildBuilder {
    JsxExpressionChildBuilder {
        l_curly_token,
        r_curly_token,
        expression: None,
    }
}
pub struct JsxExpressionChildBuilder {
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
    expression: Option<JsAnyExpression>,
}
impl JsxExpressionChildBuilder {
    pub fn with_expression(mut self, expression: JsAnyExpression) -> Self {
        self.expression = Some(expression);
        self
    }
    pub fn build(self) -> JsxExpressionChild {
        JsxExpressionChild::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JSX_EXPRESSION_CHILD,
            [
                Some(SyntaxElement::Token(self.l_curly_token)),
                self.expression
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn jsx_fragment(
    opening_fragment: JsxOpeningFragment,
    children: JsxChildList,
    closing_fragment: JsxClosingFragment,
) -> JsxFragment {
    JsxFragment::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_FRAGMENT,
        [
            Some(SyntaxElement::Node(opening_fragment.into_syntax())),
            Some(SyntaxElement::Node(children.into_syntax())),
            Some(SyntaxElement::Node(closing_fragment.into_syntax())),
        ],
    ))
}
pub fn jsx_member_name(
    object: JsxAnyObjectName,
    dot_token: SyntaxToken,
    member: JsName,
) -> JsxMemberName {
    JsxMemberName::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_MEMBER_NAME,
        [
            Some(SyntaxElement::Node(object.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(member.into_syntax())),
        ],
    ))
}
pub fn jsx_name(value_token: SyntaxToken) -> JsxName {
    JsxName::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn jsx_namespace_name(
    namespace: JsxName,
    colon_token: SyntaxToken,
    name: JsxName,
) -> JsxNamespaceName {
    JsxNamespaceName::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_NAMESPACE_NAME,
        [
            Some(SyntaxElement::Node(namespace.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn jsx_opening_element(
    l_angle_token: SyntaxToken,
    name: JsxAnyElementName,
    attributes: JsxAttributeList,
    r_angle_token: SyntaxToken,
) -> JsxOpeningElementBuilder {
    JsxOpeningElementBuilder {
        l_angle_token,
        name,
        attributes,
        r_angle_token,
        type_arguments: None,
    }
}
pub struct JsxOpeningElementBuilder {
    l_angle_token: SyntaxToken,
    name: JsxAnyElementName,
    attributes: JsxAttributeList,
    r_angle_token: SyntaxToken,
    type_arguments: Option<TsTypeArguments>,
}
impl JsxOpeningElementBuilder {
    pub fn with_type_arguments(mut self, type_arguments: TsTypeArguments) -> Self {
        self.type_arguments = Some(type_arguments);
        self
    }
    pub fn build(self) -> JsxOpeningElement {
        JsxOpeningElement::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JSX_OPENING_ELEMENT,
            [
                Some(SyntaxElement::Token(self.l_angle_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.type_arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.attributes.into_syntax())),
                Some(SyntaxElement::Token(self.r_angle_token)),
            ],
        ))
    }
}
pub fn jsx_opening_fragment(
    l_angle_token: SyntaxToken,
    r_angle_token: SyntaxToken,
) -> JsxOpeningFragment {
    JsxOpeningFragment::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_OPENING_FRAGMENT,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn jsx_reference_identifier(value_token: SyntaxToken) -> JsxReferenceIdentifier {
    JsxReferenceIdentifier::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_REFERENCE_IDENTIFIER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn jsx_self_closing_element(
    l_angle_token: SyntaxToken,
    name: JsxAnyElementName,
    attributes: JsxAttributeList,
    slash_token: SyntaxToken,
    r_angle_token: SyntaxToken,
) -> JsxSelfClosingElementBuilder {
    JsxSelfClosingElementBuilder {
        l_angle_token,
        name,
        attributes,
        slash_token,
        r_angle_token,
        type_arguments: None,
    }
}
pub struct JsxSelfClosingElementBuilder {
    l_angle_token: SyntaxToken,
    name: JsxAnyElementName,
    attributes: JsxAttributeList,
    slash_token: SyntaxToken,
    r_angle_token: SyntaxToken,
    type_arguments: Option<TsTypeArguments>,
}
impl JsxSelfClosingElementBuilder {
    pub fn with_type_arguments(mut self, type_arguments: TsTypeArguments) -> Self {
        self.type_arguments = Some(type_arguments);
        self
    }
    pub fn build(self) -> JsxSelfClosingElement {
        JsxSelfClosingElement::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT,
            [
                Some(SyntaxElement::Token(self.l_angle_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.type_arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.attributes.into_syntax())),
                Some(SyntaxElement::Token(self.slash_token)),
                Some(SyntaxElement::Token(self.r_angle_token)),
            ],
        ))
    }
}
pub fn jsx_spread_attribute(
    l_curly_token: SyntaxToken,
    dotdotdot_token: SyntaxToken,
    argument: JsAnyExpression,
    r_curly_token: SyntaxToken,
) -> JsxSpreadAttribute {
    JsxSpreadAttribute::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_SPREAD_ATTRIBUTE,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Token(dotdotdot_token)),
            Some(SyntaxElement::Node(argument.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn jsx_spread_child(
    l_curly_token: SyntaxToken,
    dotdotdot_token: SyntaxToken,
    expression: JsAnyExpression,
    r_curly_token: SyntaxToken,
) -> JsxSpreadChild {
    JsxSpreadChild::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_SPREAD_CHILD,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Token(dotdotdot_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn jsx_string(value_token: SyntaxToken) -> JsxString {
    JsxString::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn jsx_tag_expression(tag: JsxAnyTag) -> JsxTagExpression {
    JsxTagExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_TAG_EXPRESSION,
        [Some(SyntaxElement::Node(tag.into_syntax()))],
    ))
}
pub fn jsx_text(value_token: SyntaxToken) -> JsxText {
    JsxText::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_TEXT,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn new_target(
    new_token: SyntaxToken,
    dot_token: SyntaxToken,
    target_token: SyntaxToken,
) -> NewTarget {
    NewTarget::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::NEW_TARGET,
        [
            Some(SyntaxElement::Token(new_token)),
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Token(target_token)),
        ],
    ))
}
pub fn ts_abstract_modifier(modifier_token: SyntaxToken) -> TsAbstractModifier {
    TsAbstractModifier::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_ABSTRACT_MODIFIER,
        [Some(SyntaxElement::Token(modifier_token))],
    ))
}
pub fn ts_accessibility_modifier(modifier_token_token: SyntaxToken) -> TsAccessibilityModifier {
    TsAccessibilityModifier::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_ACCESSIBILITY_MODIFIER,
        [Some(SyntaxElement::Token(modifier_token_token))],
    ))
}
pub fn ts_any_type(any_token: SyntaxToken) -> TsAnyType {
    TsAnyType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_ANY_TYPE,
        [Some(SyntaxElement::Token(any_token))],
    ))
}
pub fn ts_array_type(
    element_type: TsType,
    l_brack_token: SyntaxToken,
    r_brack_token: SyntaxToken,
) -> TsArrayType {
    TsArrayType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_ARRAY_TYPE,
        [
            Some(SyntaxElement::Node(element_type.into_syntax())),
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn ts_as_assignment(
    assignment: JsAnyAssignment,
    as_token: SyntaxToken,
    ty: TsType,
) -> TsAsAssignment {
    TsAsAssignment::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_AS_ASSIGNMENT,
        [
            Some(SyntaxElement::Node(assignment.into_syntax())),
            Some(SyntaxElement::Token(as_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn ts_as_expression(
    expression: JsAnyExpression,
    as_token: SyntaxToken,
    ty: TsType,
) -> TsAsExpression {
    TsAsExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_AS_EXPRESSION,
        [
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(as_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn ts_asserts_condition(is_token: SyntaxToken, ty: TsType) -> TsAssertsCondition {
    TsAssertsCondition::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_ASSERTS_CONDITION,
        [
            Some(SyntaxElement::Token(is_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn ts_asserts_return_type(
    asserts_token: SyntaxToken,
    parameter_name: TsAnyTypePredicateParameterName,
) -> TsAssertsReturnTypeBuilder {
    TsAssertsReturnTypeBuilder {
        asserts_token,
        parameter_name,
        predicate: None,
    }
}
pub struct TsAssertsReturnTypeBuilder {
    asserts_token: SyntaxToken,
    parameter_name: TsAnyTypePredicateParameterName,
    predicate: Option<TsAssertsCondition>,
}
impl TsAssertsReturnTypeBuilder {
    pub fn with_predicate(mut self, predicate: TsAssertsCondition) -> Self {
        self.predicate = Some(predicate);
        self
    }
    pub fn build(self) -> TsAssertsReturnType {
        TsAssertsReturnType::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_ASSERTS_RETURN_TYPE,
            [
                Some(SyntaxElement::Token(self.asserts_token)),
                Some(SyntaxElement::Node(self.parameter_name.into_syntax())),
                self.predicate
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn ts_big_int_literal_type(literal_token: SyntaxToken) -> TsBigIntLiteralTypeBuilder {
    TsBigIntLiteralTypeBuilder {
        literal_token,
        minus_token: None,
    }
}
pub struct TsBigIntLiteralTypeBuilder {
    literal_token: SyntaxToken,
    minus_token: Option<SyntaxToken>,
}
impl TsBigIntLiteralTypeBuilder {
    pub fn with_minus_token(mut self, minus_token: SyntaxToken) -> Self {
        self.minus_token = Some(minus_token);
        self
    }
    pub fn build(self) -> TsBigIntLiteralType {
        TsBigIntLiteralType::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_BIG_INT_LITERAL_TYPE,
            [
                self.minus_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.literal_token)),
            ],
        ))
    }
}
pub fn ts_bigint_type(bigint_token: SyntaxToken) -> TsBigintType {
    TsBigintType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_BIGINT_TYPE,
        [Some(SyntaxElement::Token(bigint_token))],
    ))
}
pub fn ts_boolean_literal_type(literal_token: SyntaxToken) -> TsBooleanLiteralType {
    TsBooleanLiteralType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_BOOLEAN_LITERAL_TYPE,
        [Some(SyntaxElement::Token(literal_token))],
    ))
}
pub fn ts_boolean_type(boolean_token: SyntaxToken) -> TsBooleanType {
    TsBooleanType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_BOOLEAN_TYPE,
        [Some(SyntaxElement::Token(boolean_token))],
    ))
}
pub fn ts_call_signature_type_member(parameters: JsParameters) -> TsCallSignatureTypeMemberBuilder {
    TsCallSignatureTypeMemberBuilder {
        parameters,
        type_parameters: None,
        return_type_annotation: None,
        separator_token_token: None,
    }
}
pub struct TsCallSignatureTypeMemberBuilder {
    parameters: JsParameters,
    type_parameters: Option<TsTypeParameters>,
    return_type_annotation: Option<TsReturnTypeAnnotation>,
    separator_token_token: Option<SyntaxToken>,
}
impl TsCallSignatureTypeMemberBuilder {
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_return_type_annotation(
        mut self,
        return_type_annotation: TsReturnTypeAnnotation,
    ) -> Self {
        self.return_type_annotation = Some(return_type_annotation);
        self
    }
    pub fn with_separator_token_token(mut self, separator_token_token: SyntaxToken) -> Self {
        self.separator_token_token = Some(separator_token_token);
        self
    }
    pub fn build(self) -> TsCallSignatureTypeMember {
        TsCallSignatureTypeMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_CALL_SIGNATURE_TYPE_MEMBER,
            [
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                self.return_type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.separator_token_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_conditional_type(
    check_type: TsType,
    extends_token: SyntaxToken,
    extends_type: TsType,
    question_mark_token: SyntaxToken,
    true_type: TsType,
    colon_token: SyntaxToken,
    false_type: TsType,
) -> TsConditionalType {
    TsConditionalType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_CONDITIONAL_TYPE,
        [
            Some(SyntaxElement::Node(check_type.into_syntax())),
            Some(SyntaxElement::Token(extends_token)),
            Some(SyntaxElement::Node(extends_type.into_syntax())),
            Some(SyntaxElement::Token(question_mark_token)),
            Some(SyntaxElement::Node(true_type.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(false_type.into_syntax())),
        ],
    ))
}
pub fn ts_construct_signature_type_member(
    new_token: SyntaxToken,
    parameters: JsParameters,
) -> TsConstructSignatureTypeMemberBuilder {
    TsConstructSignatureTypeMemberBuilder {
        new_token,
        parameters,
        type_parameters: None,
        type_annotation: None,
        separator_token_token: None,
    }
}
pub struct TsConstructSignatureTypeMemberBuilder {
    new_token: SyntaxToken,
    parameters: JsParameters,
    type_parameters: Option<TsTypeParameters>,
    type_annotation: Option<TsTypeAnnotation>,
    separator_token_token: Option<SyntaxToken>,
}
impl TsConstructSignatureTypeMemberBuilder {
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_type_annotation(mut self, type_annotation: TsTypeAnnotation) -> Self {
        self.type_annotation = Some(type_annotation);
        self
    }
    pub fn with_separator_token_token(mut self, separator_token_token: SyntaxToken) -> Self {
        self.separator_token_token = Some(separator_token_token);
        self
    }
    pub fn build(self) -> TsConstructSignatureTypeMember {
        TsConstructSignatureTypeMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER,
            [
                Some(SyntaxElement::Token(self.new_token)),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                self.type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.separator_token_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_constructor_signature_class_member(
    modifiers: JsConstructorModifierList,
    name: JsLiteralMemberName,
    parameters: JsConstructorParameters,
) -> TsConstructorSignatureClassMemberBuilder {
    TsConstructorSignatureClassMemberBuilder {
        modifiers,
        name,
        parameters,
        semicolon_token: None,
    }
}
pub struct TsConstructorSignatureClassMemberBuilder {
    modifiers: JsConstructorModifierList,
    name: JsLiteralMemberName,
    parameters: JsConstructorParameters,
    semicolon_token: Option<SyntaxToken>,
}
impl TsConstructorSignatureClassMemberBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> TsConstructorSignatureClassMember {
        TsConstructorSignatureClassMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER,
            [
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_constructor_type(
    new_token: SyntaxToken,
    parameters: JsParameters,
    fat_arrow_token: SyntaxToken,
    return_type: TsType,
) -> TsConstructorTypeBuilder {
    TsConstructorTypeBuilder {
        new_token,
        parameters,
        fat_arrow_token,
        return_type,
        abstract_token: None,
        type_parameters: None,
    }
}
pub struct TsConstructorTypeBuilder {
    new_token: SyntaxToken,
    parameters: JsParameters,
    fat_arrow_token: SyntaxToken,
    return_type: TsType,
    abstract_token: Option<SyntaxToken>,
    type_parameters: Option<TsTypeParameters>,
}
impl TsConstructorTypeBuilder {
    pub fn with_abstract_token(mut self, abstract_token: SyntaxToken) -> Self {
        self.abstract_token = Some(abstract_token);
        self
    }
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn build(self) -> TsConstructorType {
        TsConstructorType::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_CONSTRUCTOR_TYPE,
            [
                self.abstract_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.new_token)),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                Some(SyntaxElement::Token(self.fat_arrow_token)),
                Some(SyntaxElement::Node(self.return_type.into_syntax())),
            ],
        ))
    }
}
pub fn ts_declare_function_declaration(
    function_token: SyntaxToken,
    id: JsAnyBinding,
    parameters: JsParameters,
) -> TsDeclareFunctionDeclarationBuilder {
    TsDeclareFunctionDeclarationBuilder {
        function_token,
        id,
        parameters,
        async_token: None,
        type_parameters: None,
        return_type_annotation: None,
        semicolon_token: None,
    }
}
pub struct TsDeclareFunctionDeclarationBuilder {
    function_token: SyntaxToken,
    id: JsAnyBinding,
    parameters: JsParameters,
    async_token: Option<SyntaxToken>,
    type_parameters: Option<TsTypeParameters>,
    return_type_annotation: Option<TsReturnTypeAnnotation>,
    semicolon_token: Option<SyntaxToken>,
}
impl TsDeclareFunctionDeclarationBuilder {
    pub fn with_async_token(mut self, async_token: SyntaxToken) -> Self {
        self.async_token = Some(async_token);
        self
    }
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_return_type_annotation(
        mut self,
        return_type_annotation: TsReturnTypeAnnotation,
    ) -> Self {
        self.return_type_annotation = Some(return_type_annotation);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> TsDeclareFunctionDeclaration {
        TsDeclareFunctionDeclaration::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION,
            [
                self.async_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.function_token)),
                Some(SyntaxElement::Node(self.id.into_syntax())),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                self.return_type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_declare_modifier(modifier_token: SyntaxToken) -> TsDeclareModifier {
    TsDeclareModifier::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_DECLARE_MODIFIER,
        [Some(SyntaxElement::Token(modifier_token))],
    ))
}
pub fn ts_declare_statement(
    declare_token: SyntaxToken,
    declaration: JsAnyDeclarationClause,
) -> TsDeclareStatement {
    TsDeclareStatement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_DECLARE_STATEMENT,
        [
            Some(SyntaxElement::Token(declare_token)),
            Some(SyntaxElement::Node(declaration.into_syntax())),
        ],
    ))
}
pub fn ts_default_type_clause(eq_token: SyntaxToken, ty: TsType) -> TsDefaultTypeClause {
    TsDefaultTypeClause::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_DEFAULT_TYPE_CLAUSE,
        [
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn ts_definite_property_annotation(
    excl_token: SyntaxToken,
    type_annotation: TsTypeAnnotation,
) -> TsDefinitePropertyAnnotation {
    TsDefinitePropertyAnnotation::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_DEFINITE_PROPERTY_ANNOTATION,
        [
            Some(SyntaxElement::Token(excl_token)),
            Some(SyntaxElement::Node(type_annotation.into_syntax())),
        ],
    ))
}
pub fn ts_definite_variable_annotation(
    excl_token: SyntaxToken,
    type_annotation: TsTypeAnnotation,
) -> TsDefiniteVariableAnnotation {
    TsDefiniteVariableAnnotation::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_DEFINITE_VARIABLE_ANNOTATION,
        [
            Some(SyntaxElement::Token(excl_token)),
            Some(SyntaxElement::Node(type_annotation.into_syntax())),
        ],
    ))
}
pub fn ts_empty_external_module_declaration_body(
    semicolon_token: SyntaxToken,
) -> TsEmptyExternalModuleDeclarationBody {
    TsEmptyExternalModuleDeclarationBody::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY,
        [Some(SyntaxElement::Token(semicolon_token))],
    ))
}
pub fn ts_enum_declaration(
    enum_token: SyntaxToken,
    id: JsAnyBinding,
    l_curly_token: SyntaxToken,
    members: TsEnumMemberList,
    r_curly_token: SyntaxToken,
) -> TsEnumDeclarationBuilder {
    TsEnumDeclarationBuilder {
        enum_token,
        id,
        l_curly_token,
        members,
        r_curly_token,
        const_token: None,
    }
}
pub struct TsEnumDeclarationBuilder {
    enum_token: SyntaxToken,
    id: JsAnyBinding,
    l_curly_token: SyntaxToken,
    members: TsEnumMemberList,
    r_curly_token: SyntaxToken,
    const_token: Option<SyntaxToken>,
}
impl TsEnumDeclarationBuilder {
    pub fn with_const_token(mut self, const_token: SyntaxToken) -> Self {
        self.const_token = Some(const_token);
        self
    }
    pub fn build(self) -> TsEnumDeclaration {
        TsEnumDeclaration::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_ENUM_DECLARATION,
            [
                self.const_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.enum_token)),
                Some(SyntaxElement::Node(self.id.into_syntax())),
                Some(SyntaxElement::Token(self.l_curly_token)),
                Some(SyntaxElement::Node(self.members.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn ts_enum_member(name: JsAnyObjectMemberName) -> TsEnumMemberBuilder {
    TsEnumMemberBuilder {
        name,
        initializer: None,
    }
}
pub struct TsEnumMemberBuilder {
    name: JsAnyObjectMemberName,
    initializer: Option<JsInitializerClause>,
}
impl TsEnumMemberBuilder {
    pub fn with_initializer(mut self, initializer: JsInitializerClause) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn build(self) -> TsEnumMember {
        TsEnumMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_ENUM_MEMBER,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn ts_export_as_namespace_clause(
    as_token: SyntaxToken,
    namespace_token: SyntaxToken,
    name: JsName,
) -> TsExportAsNamespaceClauseBuilder {
    TsExportAsNamespaceClauseBuilder {
        as_token,
        namespace_token,
        name,
        semicolon_token: None,
    }
}
pub struct TsExportAsNamespaceClauseBuilder {
    as_token: SyntaxToken,
    namespace_token: SyntaxToken,
    name: JsName,
    semicolon_token: Option<SyntaxToken>,
}
impl TsExportAsNamespaceClauseBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> TsExportAsNamespaceClause {
        TsExportAsNamespaceClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_EXPORT_AS_NAMESPACE_CLAUSE,
            [
                Some(SyntaxElement::Token(self.as_token)),
                Some(SyntaxElement::Token(self.namespace_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_export_assignment_clause(
    eq_token: SyntaxToken,
    expression: JsAnyExpression,
) -> TsExportAssignmentClauseBuilder {
    TsExportAssignmentClauseBuilder {
        eq_token,
        expression,
        semicolon_token: None,
    }
}
pub struct TsExportAssignmentClauseBuilder {
    eq_token: SyntaxToken,
    expression: JsAnyExpression,
    semicolon_token: Option<SyntaxToken>,
}
impl TsExportAssignmentClauseBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> TsExportAssignmentClause {
        TsExportAssignmentClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_EXPORT_ASSIGNMENT_CLAUSE,
            [
                Some(SyntaxElement::Token(self.eq_token)),
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_export_declare_clause(
    declare_token: SyntaxToken,
    declaration: JsAnyDeclarationClause,
) -> TsExportDeclareClause {
    TsExportDeclareClause::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_EXPORT_DECLARE_CLAUSE,
        [
            Some(SyntaxElement::Token(declare_token)),
            Some(SyntaxElement::Node(declaration.into_syntax())),
        ],
    ))
}
pub fn ts_extends_clause(extends_token: SyntaxToken, types: TsTypeList) -> TsExtendsClause {
    TsExtendsClause::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_EXTENDS_CLAUSE,
        [
            Some(SyntaxElement::Token(extends_token)),
            Some(SyntaxElement::Node(types.into_syntax())),
        ],
    ))
}
pub fn ts_external_module_declaration(
    module_token: SyntaxToken,
    source: JsModuleSource,
) -> TsExternalModuleDeclarationBuilder {
    TsExternalModuleDeclarationBuilder {
        module_token,
        source,
        body: None,
    }
}
pub struct TsExternalModuleDeclarationBuilder {
    module_token: SyntaxToken,
    source: JsModuleSource,
    body: Option<TsAnyExternalModuleDeclarationBody>,
}
impl TsExternalModuleDeclarationBuilder {
    pub fn with_body(mut self, body: TsAnyExternalModuleDeclarationBody) -> Self {
        self.body = Some(body);
        self
    }
    pub fn build(self) -> TsExternalModuleDeclaration {
        TsExternalModuleDeclaration::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION,
            [
                Some(SyntaxElement::Token(self.module_token)),
                Some(SyntaxElement::Node(self.source.into_syntax())),
                self.body
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn ts_external_module_reference(
    require_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    source: JsModuleSource,
    r_paren_token: SyntaxToken,
) -> TsExternalModuleReference {
    TsExternalModuleReference::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_EXTERNAL_MODULE_REFERENCE,
        [
            Some(SyntaxElement::Token(require_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(source.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn ts_function_type(
    parameters: JsParameters,
    fat_arrow_token: SyntaxToken,
    return_type: TsAnyReturnType,
) -> TsFunctionTypeBuilder {
    TsFunctionTypeBuilder {
        parameters,
        fat_arrow_token,
        return_type,
        type_parameters: None,
    }
}
pub struct TsFunctionTypeBuilder {
    parameters: JsParameters,
    fat_arrow_token: SyntaxToken,
    return_type: TsAnyReturnType,
    type_parameters: Option<TsTypeParameters>,
}
impl TsFunctionTypeBuilder {
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn build(self) -> TsFunctionType {
        TsFunctionType::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_FUNCTION_TYPE,
            [
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                Some(SyntaxElement::Token(self.fat_arrow_token)),
                Some(SyntaxElement::Node(self.return_type.into_syntax())),
            ],
        ))
    }
}
pub fn ts_getter_signature_class_member(
    modifiers: TsMethodSignatureModifierList,
    get_token: SyntaxToken,
    name: JsAnyClassMemberName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
) -> TsGetterSignatureClassMemberBuilder {
    TsGetterSignatureClassMemberBuilder {
        modifiers,
        get_token,
        name,
        l_paren_token,
        r_paren_token,
        return_type: None,
        semicolon_token: None,
    }
}
pub struct TsGetterSignatureClassMemberBuilder {
    modifiers: TsMethodSignatureModifierList,
    get_token: SyntaxToken,
    name: JsAnyClassMemberName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    return_type: Option<TsTypeAnnotation>,
    semicolon_token: Option<SyntaxToken>,
}
impl TsGetterSignatureClassMemberBuilder {
    pub fn with_return_type(mut self, return_type: TsTypeAnnotation) -> Self {
        self.return_type = Some(return_type);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> TsGetterSignatureClassMember {
        TsGetterSignatureClassMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_GETTER_SIGNATURE_CLASS_MEMBER,
            [
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                Some(SyntaxElement::Token(self.get_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.return_type
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_getter_signature_type_member(
    get_token: SyntaxToken,
    name: JsAnyObjectMemberName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
) -> TsGetterSignatureTypeMemberBuilder {
    TsGetterSignatureTypeMemberBuilder {
        get_token,
        name,
        l_paren_token,
        r_paren_token,
        type_annotation: None,
        separator_token_token: None,
    }
}
pub struct TsGetterSignatureTypeMemberBuilder {
    get_token: SyntaxToken,
    name: JsAnyObjectMemberName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    type_annotation: Option<TsTypeAnnotation>,
    separator_token_token: Option<SyntaxToken>,
}
impl TsGetterSignatureTypeMemberBuilder {
    pub fn with_type_annotation(mut self, type_annotation: TsTypeAnnotation) -> Self {
        self.type_annotation = Some(type_annotation);
        self
    }
    pub fn with_separator_token_token(mut self, separator_token_token: SyntaxToken) -> Self {
        self.separator_token_token = Some(separator_token_token);
        self
    }
    pub fn build(self) -> TsGetterSignatureTypeMember {
        TsGetterSignatureTypeMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_GETTER_SIGNATURE_TYPE_MEMBER,
            [
                Some(SyntaxElement::Token(self.get_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.separator_token_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_global_declaration(
    global_token: SyntaxToken,
    body: TsModuleBlock,
) -> TsGlobalDeclaration {
    TsGlobalDeclaration::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_GLOBAL_DECLARATION,
        [
            Some(SyntaxElement::Token(global_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn ts_identifier_binding(name_token: SyntaxToken) -> TsIdentifierBinding {
    TsIdentifierBinding::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_IDENTIFIER_BINDING,
        [Some(SyntaxElement::Token(name_token))],
    ))
}
pub fn ts_implements_clause(
    implements_token: SyntaxToken,
    types: TsTypeList,
) -> TsImplementsClause {
    TsImplementsClause::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_IMPLEMENTS_CLAUSE,
        [
            Some(SyntaxElement::Token(implements_token)),
            Some(SyntaxElement::Node(types.into_syntax())),
        ],
    ))
}
pub fn ts_import_equals_declaration(
    import_token: SyntaxToken,
    id: JsAnyBinding,
    eq_token: SyntaxToken,
    module_reference: TsAnyModuleReference,
) -> TsImportEqualsDeclarationBuilder {
    TsImportEqualsDeclarationBuilder {
        import_token,
        id,
        eq_token,
        module_reference,
        type_token: None,
        semicolon_token: None,
    }
}
pub struct TsImportEqualsDeclarationBuilder {
    import_token: SyntaxToken,
    id: JsAnyBinding,
    eq_token: SyntaxToken,
    module_reference: TsAnyModuleReference,
    type_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl TsImportEqualsDeclarationBuilder {
    pub fn with_type_token(mut self, type_token: SyntaxToken) -> Self {
        self.type_token = Some(type_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> TsImportEqualsDeclaration {
        TsImportEqualsDeclaration::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_IMPORT_EQUALS_DECLARATION,
            [
                Some(SyntaxElement::Token(self.import_token)),
                self.type_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.id.into_syntax())),
                Some(SyntaxElement::Token(self.eq_token)),
                Some(SyntaxElement::Node(self.module_reference.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_import_type(
    import_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    argument_token: SyntaxToken,
    r_paren_token: SyntaxToken,
) -> TsImportTypeBuilder {
    TsImportTypeBuilder {
        import_token,
        l_paren_token,
        argument_token,
        r_paren_token,
        typeof_token: None,
        qualifier_clause: None,
        type_arguments: None,
    }
}
pub struct TsImportTypeBuilder {
    import_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    argument_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    typeof_token: Option<SyntaxToken>,
    qualifier_clause: Option<TsImportTypeQualifier>,
    type_arguments: Option<TsTypeArguments>,
}
impl TsImportTypeBuilder {
    pub fn with_typeof_token(mut self, typeof_token: SyntaxToken) -> Self {
        self.typeof_token = Some(typeof_token);
        self
    }
    pub fn with_qualifier_clause(mut self, qualifier_clause: TsImportTypeQualifier) -> Self {
        self.qualifier_clause = Some(qualifier_clause);
        self
    }
    pub fn with_type_arguments(mut self, type_arguments: TsTypeArguments) -> Self {
        self.type_arguments = Some(type_arguments);
        self
    }
    pub fn build(self) -> TsImportType {
        TsImportType::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_IMPORT_TYPE,
            [
                self.typeof_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.import_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Token(self.argument_token)),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.qualifier_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.type_arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn ts_import_type_qualifier(dot_token: SyntaxToken, right: TsAnyName) -> TsImportTypeQualifier {
    TsImportTypeQualifier::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_IMPORT_TYPE_QUALIFIER,
        [
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn ts_index_signature_class_member(
    modifiers: TsIndexSignatureModifierList,
    l_brack_token: SyntaxToken,
    parameter: TsIndexSignatureParameter,
    r_brack_token: SyntaxToken,
    type_annotation: TsTypeAnnotation,
) -> TsIndexSignatureClassMemberBuilder {
    TsIndexSignatureClassMemberBuilder {
        modifiers,
        l_brack_token,
        parameter,
        r_brack_token,
        type_annotation,
        semicolon_token: None,
    }
}
pub struct TsIndexSignatureClassMemberBuilder {
    modifiers: TsIndexSignatureModifierList,
    l_brack_token: SyntaxToken,
    parameter: TsIndexSignatureParameter,
    r_brack_token: SyntaxToken,
    type_annotation: TsTypeAnnotation,
    semicolon_token: Option<SyntaxToken>,
}
impl TsIndexSignatureClassMemberBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> TsIndexSignatureClassMember {
        TsIndexSignatureClassMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_INDEX_SIGNATURE_CLASS_MEMBER,
            [
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Node(self.parameter.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
                Some(SyntaxElement::Node(self.type_annotation.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_index_signature_parameter(
    binding: JsIdentifierBinding,
    type_annotation: TsTypeAnnotation,
) -> TsIndexSignatureParameter {
    TsIndexSignatureParameter::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_INDEX_SIGNATURE_PARAMETER,
        [
            Some(SyntaxElement::Node(binding.into_syntax())),
            Some(SyntaxElement::Node(type_annotation.into_syntax())),
        ],
    ))
}
pub fn ts_index_signature_type_member(
    l_brack_token: SyntaxToken,
    parameter: TsIndexSignatureParameter,
    r_brack_token: SyntaxToken,
    type_annotation: TsTypeAnnotation,
) -> TsIndexSignatureTypeMemberBuilder {
    TsIndexSignatureTypeMemberBuilder {
        l_brack_token,
        parameter,
        r_brack_token,
        type_annotation,
        readonly_token: None,
        separator_token_token: None,
    }
}
pub struct TsIndexSignatureTypeMemberBuilder {
    l_brack_token: SyntaxToken,
    parameter: TsIndexSignatureParameter,
    r_brack_token: SyntaxToken,
    type_annotation: TsTypeAnnotation,
    readonly_token: Option<SyntaxToken>,
    separator_token_token: Option<SyntaxToken>,
}
impl TsIndexSignatureTypeMemberBuilder {
    pub fn with_readonly_token(mut self, readonly_token: SyntaxToken) -> Self {
        self.readonly_token = Some(readonly_token);
        self
    }
    pub fn with_separator_token_token(mut self, separator_token_token: SyntaxToken) -> Self {
        self.separator_token_token = Some(separator_token_token);
        self
    }
    pub fn build(self) -> TsIndexSignatureTypeMember {
        TsIndexSignatureTypeMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_INDEX_SIGNATURE_TYPE_MEMBER,
            [
                self.readonly_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Node(self.parameter.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
                Some(SyntaxElement::Node(self.type_annotation.into_syntax())),
                self.separator_token_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_indexed_access_type(
    object_type: TsType,
    l_brack_token: SyntaxToken,
    index_type: TsType,
    r_brack_token: SyntaxToken,
) -> TsIndexedAccessType {
    TsIndexedAccessType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_INDEXED_ACCESS_TYPE,
        [
            Some(SyntaxElement::Node(object_type.into_syntax())),
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(index_type.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn ts_infer_type(infer_token: SyntaxToken, type_parameter: TsTypeParameterName) -> TsInferType {
    TsInferType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_INFER_TYPE,
        [
            Some(SyntaxElement::Token(infer_token)),
            Some(SyntaxElement::Node(type_parameter.into_syntax())),
        ],
    ))
}
pub fn ts_instantiation_expression(
    expression: JsAnyExpression,
    arguments: TsTypeArguments,
) -> TsInstantiationExpression {
    TsInstantiationExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_INSTANTIATION_EXPRESSION,
        [
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Node(arguments.into_syntax())),
        ],
    ))
}
pub fn ts_interface_declaration(
    interface_token: SyntaxToken,
    id: TsIdentifierBinding,
    l_curly_token: SyntaxToken,
    members: TsTypeMemberList,
    r_curly_token: SyntaxToken,
) -> TsInterfaceDeclarationBuilder {
    TsInterfaceDeclarationBuilder {
        interface_token,
        id,
        l_curly_token,
        members,
        r_curly_token,
        type_parameters: None,
        extends_clause: None,
    }
}
pub struct TsInterfaceDeclarationBuilder {
    interface_token: SyntaxToken,
    id: TsIdentifierBinding,
    l_curly_token: SyntaxToken,
    members: TsTypeMemberList,
    r_curly_token: SyntaxToken,
    type_parameters: Option<TsTypeParameters>,
    extends_clause: Option<TsExtendsClause>,
}
impl TsInterfaceDeclarationBuilder {
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_extends_clause(mut self, extends_clause: TsExtendsClause) -> Self {
        self.extends_clause = Some(extends_clause);
        self
    }
    pub fn build(self) -> TsInterfaceDeclaration {
        TsInterfaceDeclaration::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_INTERFACE_DECLARATION,
            [
                Some(SyntaxElement::Token(self.interface_token)),
                Some(SyntaxElement::Node(self.id.into_syntax())),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.extends_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.l_curly_token)),
                Some(SyntaxElement::Node(self.members.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn ts_intersection_type(types: TsIntersectionTypeElementList) -> TsIntersectionTypeBuilder {
    TsIntersectionTypeBuilder {
        types,
        leading_separator_token: None,
    }
}
pub struct TsIntersectionTypeBuilder {
    types: TsIntersectionTypeElementList,
    leading_separator_token: Option<SyntaxToken>,
}
impl TsIntersectionTypeBuilder {
    pub fn with_leading_separator_token(mut self, leading_separator_token: SyntaxToken) -> Self {
        self.leading_separator_token = Some(leading_separator_token);
        self
    }
    pub fn build(self) -> TsIntersectionType {
        TsIntersectionType::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_INTERSECTION_TYPE,
            [
                self.leading_separator_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.types.into_syntax())),
            ],
        ))
    }
}
pub fn ts_mapped_type(
    l_curly_token: SyntaxToken,
    l_brack_token: SyntaxToken,
    property_name: TsTypeParameterName,
    in_token: SyntaxToken,
    keys_type: TsType,
    r_brack_token: SyntaxToken,
    r_curly_token: SyntaxToken,
) -> TsMappedTypeBuilder {
    TsMappedTypeBuilder {
        l_curly_token,
        l_brack_token,
        property_name,
        in_token,
        keys_type,
        r_brack_token,
        r_curly_token,
        readonly_modifier: None,
        as_clause: None,
        optional_modifier: None,
        mapped_type: None,
        semicolon_token: None,
    }
}
pub struct TsMappedTypeBuilder {
    l_curly_token: SyntaxToken,
    l_brack_token: SyntaxToken,
    property_name: TsTypeParameterName,
    in_token: SyntaxToken,
    keys_type: TsType,
    r_brack_token: SyntaxToken,
    r_curly_token: SyntaxToken,
    readonly_modifier: Option<TsMappedTypeReadonlyModifierClause>,
    as_clause: Option<TsMappedTypeAsClause>,
    optional_modifier: Option<TsMappedTypeOptionalModifierClause>,
    mapped_type: Option<TsTypeAnnotation>,
    semicolon_token: Option<SyntaxToken>,
}
impl TsMappedTypeBuilder {
    pub fn with_readonly_modifier(
        mut self,
        readonly_modifier: TsMappedTypeReadonlyModifierClause,
    ) -> Self {
        self.readonly_modifier = Some(readonly_modifier);
        self
    }
    pub fn with_as_clause(mut self, as_clause: TsMappedTypeAsClause) -> Self {
        self.as_clause = Some(as_clause);
        self
    }
    pub fn with_optional_modifier(
        mut self,
        optional_modifier: TsMappedTypeOptionalModifierClause,
    ) -> Self {
        self.optional_modifier = Some(optional_modifier);
        self
    }
    pub fn with_mapped_type(mut self, mapped_type: TsTypeAnnotation) -> Self {
        self.mapped_type = Some(mapped_type);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> TsMappedType {
        TsMappedType::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_MAPPED_TYPE,
            [
                Some(SyntaxElement::Token(self.l_curly_token)),
                self.readonly_modifier
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Node(self.property_name.into_syntax())),
                Some(SyntaxElement::Token(self.in_token)),
                Some(SyntaxElement::Node(self.keys_type.into_syntax())),
                self.as_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
                self.optional_modifier
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.mapped_type
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn ts_mapped_type_as_clause(as_token: SyntaxToken, ty: TsType) -> TsMappedTypeAsClause {
    TsMappedTypeAsClause::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_MAPPED_TYPE_AS_CLAUSE,
        [
            Some(SyntaxElement::Token(as_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn ts_mapped_type_optional_modifier_clause(
    question_mark_token: SyntaxToken,
) -> TsMappedTypeOptionalModifierClauseBuilder {
    TsMappedTypeOptionalModifierClauseBuilder {
        question_mark_token,
        operator_token_token: None,
    }
}
pub struct TsMappedTypeOptionalModifierClauseBuilder {
    question_mark_token: SyntaxToken,
    operator_token_token: Option<SyntaxToken>,
}
impl TsMappedTypeOptionalModifierClauseBuilder {
    pub fn with_operator_token_token(mut self, operator_token_token: SyntaxToken) -> Self {
        self.operator_token_token = Some(operator_token_token);
        self
    }
    pub fn build(self) -> TsMappedTypeOptionalModifierClause {
        TsMappedTypeOptionalModifierClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE,
            [
                self.operator_token_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.question_mark_token)),
            ],
        ))
    }
}
pub fn ts_mapped_type_readonly_modifier_clause(
    readonly_token: SyntaxToken,
) -> TsMappedTypeReadonlyModifierClauseBuilder {
    TsMappedTypeReadonlyModifierClauseBuilder {
        readonly_token,
        operator_token_token: None,
    }
}
pub struct TsMappedTypeReadonlyModifierClauseBuilder {
    readonly_token: SyntaxToken,
    operator_token_token: Option<SyntaxToken>,
}
impl TsMappedTypeReadonlyModifierClauseBuilder {
    pub fn with_operator_token_token(mut self, operator_token_token: SyntaxToken) -> Self {
        self.operator_token_token = Some(operator_token_token);
        self
    }
    pub fn build(self) -> TsMappedTypeReadonlyModifierClause {
        TsMappedTypeReadonlyModifierClause::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE,
            [
                self.operator_token_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.readonly_token)),
            ],
        ))
    }
}
pub fn ts_method_signature_class_member(
    modifiers: TsMethodSignatureModifierList,
    name: JsAnyClassMemberName,
    parameters: JsParameters,
) -> TsMethodSignatureClassMemberBuilder {
    TsMethodSignatureClassMemberBuilder {
        modifiers,
        name,
        parameters,
        async_token: None,
        question_mark_token: None,
        type_parameters: None,
        return_type_annotation: None,
        semicolon_token: None,
    }
}
pub struct TsMethodSignatureClassMemberBuilder {
    modifiers: TsMethodSignatureModifierList,
    name: JsAnyClassMemberName,
    parameters: JsParameters,
    async_token: Option<SyntaxToken>,
    question_mark_token: Option<SyntaxToken>,
    type_parameters: Option<TsTypeParameters>,
    return_type_annotation: Option<TsReturnTypeAnnotation>,
    semicolon_token: Option<SyntaxToken>,
}
impl TsMethodSignatureClassMemberBuilder {
    pub fn with_async_token(mut self, async_token: SyntaxToken) -> Self {
        self.async_token = Some(async_token);
        self
    }
    pub fn with_question_mark_token(mut self, question_mark_token: SyntaxToken) -> Self {
        self.question_mark_token = Some(question_mark_token);
        self
    }
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_return_type_annotation(
        mut self,
        return_type_annotation: TsReturnTypeAnnotation,
    ) -> Self {
        self.return_type_annotation = Some(return_type_annotation);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> TsMethodSignatureClassMember {
        TsMethodSignatureClassMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_METHOD_SIGNATURE_CLASS_MEMBER,
            [
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                self.async_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.question_mark_token
                    .map(|token| SyntaxElement::Token(token)),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                self.return_type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_method_signature_type_member(
    name: JsAnyObjectMemberName,
    parameters: JsParameters,
) -> TsMethodSignatureTypeMemberBuilder {
    TsMethodSignatureTypeMemberBuilder {
        name,
        parameters,
        optional_token: None,
        type_parameters: None,
        return_type_annotation: None,
        separator_token_token: None,
    }
}
pub struct TsMethodSignatureTypeMemberBuilder {
    name: JsAnyObjectMemberName,
    parameters: JsParameters,
    optional_token: Option<SyntaxToken>,
    type_parameters: Option<TsTypeParameters>,
    return_type_annotation: Option<TsReturnTypeAnnotation>,
    separator_token_token: Option<SyntaxToken>,
}
impl TsMethodSignatureTypeMemberBuilder {
    pub fn with_optional_token(mut self, optional_token: SyntaxToken) -> Self {
        self.optional_token = Some(optional_token);
        self
    }
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_return_type_annotation(
        mut self,
        return_type_annotation: TsReturnTypeAnnotation,
    ) -> Self {
        self.return_type_annotation = Some(return_type_annotation);
        self
    }
    pub fn with_separator_token_token(mut self, separator_token_token: SyntaxToken) -> Self {
        self.separator_token_token = Some(separator_token_token);
        self
    }
    pub fn build(self) -> TsMethodSignatureTypeMember {
        TsMethodSignatureTypeMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_METHOD_SIGNATURE_TYPE_MEMBER,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.optional_token.map(|token| SyntaxElement::Token(token)),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                self.return_type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.separator_token_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_module_block(
    l_curly_token: SyntaxToken,
    items: JsModuleItemList,
    r_curly_token: SyntaxToken,
) -> TsModuleBlock {
    TsModuleBlock::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_MODULE_BLOCK,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn ts_module_declaration(
    module_or_namespace_token: SyntaxToken,
    name: TsAnyModuleName,
    body: TsModuleBlock,
) -> TsModuleDeclaration {
    TsModuleDeclaration::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_MODULE_DECLARATION,
        [
            Some(SyntaxElement::Token(module_or_namespace_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn ts_name_with_type_arguments(name: TsAnyName) -> TsNameWithTypeArgumentsBuilder {
    TsNameWithTypeArgumentsBuilder {
        name,
        type_arguments: None,
    }
}
pub struct TsNameWithTypeArgumentsBuilder {
    name: TsAnyName,
    type_arguments: Option<TsTypeArguments>,
}
impl TsNameWithTypeArgumentsBuilder {
    pub fn with_type_arguments(mut self, type_arguments: TsTypeArguments) -> Self {
        self.type_arguments = Some(type_arguments);
        self
    }
    pub fn build(self) -> TsNameWithTypeArguments {
        TsNameWithTypeArguments::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_NAME_WITH_TYPE_ARGUMENTS,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.type_arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn ts_named_tuple_type_element(
    name: JsName,
    colon_token: SyntaxToken,
    ty: TsType,
) -> TsNamedTupleTypeElementBuilder {
    TsNamedTupleTypeElementBuilder {
        name,
        colon_token,
        ty,
        dotdotdot_token: None,
        question_mark_token: None,
    }
}
pub struct TsNamedTupleTypeElementBuilder {
    name: JsName,
    colon_token: SyntaxToken,
    ty: TsType,
    dotdotdot_token: Option<SyntaxToken>,
    question_mark_token: Option<SyntaxToken>,
}
impl TsNamedTupleTypeElementBuilder {
    pub fn with_dotdotdot_token(mut self, dotdotdot_token: SyntaxToken) -> Self {
        self.dotdotdot_token = Some(dotdotdot_token);
        self
    }
    pub fn with_question_mark_token(mut self, question_mark_token: SyntaxToken) -> Self {
        self.question_mark_token = Some(question_mark_token);
        self
    }
    pub fn build(self) -> TsNamedTupleTypeElement {
        TsNamedTupleTypeElement::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_NAMED_TUPLE_TYPE_ELEMENT,
            [
                self.dotdotdot_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.question_mark_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.colon_token)),
                Some(SyntaxElement::Node(self.ty.into_syntax())),
            ],
        ))
    }
}
pub fn ts_never_type(never_token: SyntaxToken) -> TsNeverType {
    TsNeverType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_NEVER_TYPE,
        [Some(SyntaxElement::Token(never_token))],
    ))
}
pub fn ts_non_null_assertion_assignment(
    assignment: JsAnyAssignment,
    excl_token: SyntaxToken,
) -> TsNonNullAssertionAssignment {
    TsNonNullAssertionAssignment::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT,
        [
            Some(SyntaxElement::Node(assignment.into_syntax())),
            Some(SyntaxElement::Token(excl_token)),
        ],
    ))
}
pub fn ts_non_null_assertion_expression(
    expression: JsAnyExpression,
    excl_token: SyntaxToken,
) -> TsNonNullAssertionExpression {
    TsNonNullAssertionExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION,
        [
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(excl_token)),
        ],
    ))
}
pub fn ts_non_primitive_type(object_token: SyntaxToken) -> TsNonPrimitiveType {
    TsNonPrimitiveType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_NON_PRIMITIVE_TYPE,
        [Some(SyntaxElement::Token(object_token))],
    ))
}
pub fn ts_null_literal_type(literal_token: SyntaxToken) -> TsNullLiteralType {
    TsNullLiteralType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_NULL_LITERAL_TYPE,
        [Some(SyntaxElement::Token(literal_token))],
    ))
}
pub fn ts_number_literal_type(literal_token: SyntaxToken) -> TsNumberLiteralTypeBuilder {
    TsNumberLiteralTypeBuilder {
        literal_token,
        minus_token: None,
    }
}
pub struct TsNumberLiteralTypeBuilder {
    literal_token: SyntaxToken,
    minus_token: Option<SyntaxToken>,
}
impl TsNumberLiteralTypeBuilder {
    pub fn with_minus_token(mut self, minus_token: SyntaxToken) -> Self {
        self.minus_token = Some(minus_token);
        self
    }
    pub fn build(self) -> TsNumberLiteralType {
        TsNumberLiteralType::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_NUMBER_LITERAL_TYPE,
            [
                self.minus_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.literal_token)),
            ],
        ))
    }
}
pub fn ts_number_type(number_token: SyntaxToken) -> TsNumberType {
    TsNumberType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_NUMBER_TYPE,
        [Some(SyntaxElement::Token(number_token))],
    ))
}
pub fn ts_object_type(
    l_curly_token: SyntaxToken,
    members: TsTypeMemberList,
    r_curly_token: SyntaxToken,
) -> TsObjectType {
    TsObjectType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_OBJECT_TYPE,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(members.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn ts_optional_property_annotation(
    question_mark_token: SyntaxToken,
) -> TsOptionalPropertyAnnotationBuilder {
    TsOptionalPropertyAnnotationBuilder {
        question_mark_token,
        type_annotation: None,
    }
}
pub struct TsOptionalPropertyAnnotationBuilder {
    question_mark_token: SyntaxToken,
    type_annotation: Option<TsTypeAnnotation>,
}
impl TsOptionalPropertyAnnotationBuilder {
    pub fn with_type_annotation(mut self, type_annotation: TsTypeAnnotation) -> Self {
        self.type_annotation = Some(type_annotation);
        self
    }
    pub fn build(self) -> TsOptionalPropertyAnnotation {
        TsOptionalPropertyAnnotation::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_OPTIONAL_PROPERTY_ANNOTATION,
            [
                Some(SyntaxElement::Token(self.question_mark_token)),
                self.type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn ts_optional_tuple_type_element(
    ty: TsType,
    question_mark_token: SyntaxToken,
) -> TsOptionalTupleTypeElement {
    TsOptionalTupleTypeElement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_OPTIONAL_TUPLE_TYPE_ELEMENT,
        [
            Some(SyntaxElement::Node(ty.into_syntax())),
            Some(SyntaxElement::Token(question_mark_token)),
        ],
    ))
}
pub fn ts_override_modifier(modifier_token: SyntaxToken) -> TsOverrideModifier {
    TsOverrideModifier::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_OVERRIDE_MODIFIER,
        [Some(SyntaxElement::Token(modifier_token))],
    ))
}
pub fn ts_parenthesized_type(
    l_paren_token: SyntaxToken,
    ty: TsType,
    r_paren_token: SyntaxToken,
) -> TsParenthesizedType {
    TsParenthesizedType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_PARENTHESIZED_TYPE,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn ts_predicate_return_type(
    parameter_name: TsAnyTypePredicateParameterName,
    is_token: SyntaxToken,
    ty: TsType,
) -> TsPredicateReturnType {
    TsPredicateReturnType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_PREDICATE_RETURN_TYPE,
        [
            Some(SyntaxElement::Node(parameter_name.into_syntax())),
            Some(SyntaxElement::Token(is_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn ts_property_parameter(
    modifiers: TsPropertyParameterModifierList,
    formal_parameter: JsAnyFormalParameter,
) -> TsPropertyParameter {
    TsPropertyParameter::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_PROPERTY_PARAMETER,
        [
            Some(SyntaxElement::Node(modifiers.into_syntax())),
            Some(SyntaxElement::Node(formal_parameter.into_syntax())),
        ],
    ))
}
pub fn ts_property_signature_class_member(
    modifiers: TsPropertySignatureModifierList,
    name: JsAnyClassMemberName,
) -> TsPropertySignatureClassMemberBuilder {
    TsPropertySignatureClassMemberBuilder {
        modifiers,
        name,
        property_annotation: None,
        semicolon_token: None,
    }
}
pub struct TsPropertySignatureClassMemberBuilder {
    modifiers: TsPropertySignatureModifierList,
    name: JsAnyClassMemberName,
    property_annotation: Option<TsAnyPropertySignatureAnnotation>,
    semicolon_token: Option<SyntaxToken>,
}
impl TsPropertySignatureClassMemberBuilder {
    pub fn with_property_annotation(
        mut self,
        property_annotation: TsAnyPropertySignatureAnnotation,
    ) -> Self {
        self.property_annotation = Some(property_annotation);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> TsPropertySignatureClassMember {
        TsPropertySignatureClassMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_PROPERTY_SIGNATURE_CLASS_MEMBER,
            [
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.property_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_property_signature_type_member(
    name: JsAnyObjectMemberName,
) -> TsPropertySignatureTypeMemberBuilder {
    TsPropertySignatureTypeMemberBuilder {
        name,
        readonly_token: None,
        optional_token: None,
        type_annotation: None,
        separator_token_token: None,
    }
}
pub struct TsPropertySignatureTypeMemberBuilder {
    name: JsAnyObjectMemberName,
    readonly_token: Option<SyntaxToken>,
    optional_token: Option<SyntaxToken>,
    type_annotation: Option<TsTypeAnnotation>,
    separator_token_token: Option<SyntaxToken>,
}
impl TsPropertySignatureTypeMemberBuilder {
    pub fn with_readonly_token(mut self, readonly_token: SyntaxToken) -> Self {
        self.readonly_token = Some(readonly_token);
        self
    }
    pub fn with_optional_token(mut self, optional_token: SyntaxToken) -> Self {
        self.optional_token = Some(optional_token);
        self
    }
    pub fn with_type_annotation(mut self, type_annotation: TsTypeAnnotation) -> Self {
        self.type_annotation = Some(type_annotation);
        self
    }
    pub fn with_separator_token_token(mut self, separator_token_token: SyntaxToken) -> Self {
        self.separator_token_token = Some(separator_token_token);
        self
    }
    pub fn build(self) -> TsPropertySignatureTypeMember {
        TsPropertySignatureTypeMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_PROPERTY_SIGNATURE_TYPE_MEMBER,
            [
                self.readonly_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.optional_token.map(|token| SyntaxElement::Token(token)),
                self.type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.separator_token_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_qualified_module_name(
    left: TsAnyModuleName,
    dot_token: SyntaxToken,
    right: JsName,
) -> TsQualifiedModuleName {
    TsQualifiedModuleName::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_QUALIFIED_MODULE_NAME,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn ts_qualified_name(
    left: TsAnyName,
    dot_token: SyntaxToken,
    right: JsName,
) -> TsQualifiedName {
    TsQualifiedName::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_QUALIFIED_NAME,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn ts_readonly_modifier(modifier_token: SyntaxToken) -> TsReadonlyModifier {
    TsReadonlyModifier::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_READONLY_MODIFIER,
        [Some(SyntaxElement::Token(modifier_token))],
    ))
}
pub fn ts_reference_type(name: TsAnyName) -> TsReferenceTypeBuilder {
    TsReferenceTypeBuilder {
        name,
        type_arguments: None,
    }
}
pub struct TsReferenceTypeBuilder {
    name: TsAnyName,
    type_arguments: Option<TsTypeArguments>,
}
impl TsReferenceTypeBuilder {
    pub fn with_type_arguments(mut self, type_arguments: TsTypeArguments) -> Self {
        self.type_arguments = Some(type_arguments);
        self
    }
    pub fn build(self) -> TsReferenceType {
        TsReferenceType::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_REFERENCE_TYPE,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.type_arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn ts_rest_tuple_type_element(
    dotdotdot_token: SyntaxToken,
    ty: TsType,
) -> TsRestTupleTypeElement {
    TsRestTupleTypeElement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_REST_TUPLE_TYPE_ELEMENT,
        [
            Some(SyntaxElement::Token(dotdotdot_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn ts_return_type_annotation(
    colon_token: SyntaxToken,
    ty: TsAnyReturnType,
) -> TsReturnTypeAnnotation {
    TsReturnTypeAnnotation::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_RETURN_TYPE_ANNOTATION,
        [
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn ts_setter_signature_class_member(
    modifiers: TsMethodSignatureModifierList,
    set_token: SyntaxToken,
    name: JsAnyClassMemberName,
    l_paren_token: SyntaxToken,
    parameter: JsAnyFormalParameter,
    r_paren_token: SyntaxToken,
) -> TsSetterSignatureClassMemberBuilder {
    TsSetterSignatureClassMemberBuilder {
        modifiers,
        set_token,
        name,
        l_paren_token,
        parameter,
        r_paren_token,
        semicolon_token: None,
    }
}
pub struct TsSetterSignatureClassMemberBuilder {
    modifiers: TsMethodSignatureModifierList,
    set_token: SyntaxToken,
    name: JsAnyClassMemberName,
    l_paren_token: SyntaxToken,
    parameter: JsAnyFormalParameter,
    r_paren_token: SyntaxToken,
    semicolon_token: Option<SyntaxToken>,
}
impl TsSetterSignatureClassMemberBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> TsSetterSignatureClassMember {
        TsSetterSignatureClassMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_SETTER_SIGNATURE_CLASS_MEMBER,
            [
                Some(SyntaxElement::Node(self.modifiers.into_syntax())),
                Some(SyntaxElement::Token(self.set_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.parameter.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_setter_signature_type_member(
    set_token: SyntaxToken,
    name: JsAnyObjectMemberName,
    l_paren_token: SyntaxToken,
    parameter: JsAnyFormalParameter,
    r_paren_token: SyntaxToken,
) -> TsSetterSignatureTypeMemberBuilder {
    TsSetterSignatureTypeMemberBuilder {
        set_token,
        name,
        l_paren_token,
        parameter,
        r_paren_token,
        separator_token_token: None,
    }
}
pub struct TsSetterSignatureTypeMemberBuilder {
    set_token: SyntaxToken,
    name: JsAnyObjectMemberName,
    l_paren_token: SyntaxToken,
    parameter: JsAnyFormalParameter,
    r_paren_token: SyntaxToken,
    separator_token_token: Option<SyntaxToken>,
}
impl TsSetterSignatureTypeMemberBuilder {
    pub fn with_separator_token_token(mut self, separator_token_token: SyntaxToken) -> Self {
        self.separator_token_token = Some(separator_token_token);
        self
    }
    pub fn build(self) -> TsSetterSignatureTypeMember {
        TsSetterSignatureTypeMember::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_SETTER_SIGNATURE_TYPE_MEMBER,
            [
                Some(SyntaxElement::Token(self.set_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.parameter.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.separator_token_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_string_literal_type(literal_token: SyntaxToken) -> TsStringLiteralType {
    TsStringLiteralType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_STRING_LITERAL_TYPE,
        [Some(SyntaxElement::Token(literal_token))],
    ))
}
pub fn ts_string_type(string_token: SyntaxToken) -> TsStringType {
    TsStringType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_STRING_TYPE,
        [Some(SyntaxElement::Token(string_token))],
    ))
}
pub fn ts_symbol_type(symbol_token: SyntaxToken) -> TsSymbolType {
    TsSymbolType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_SYMBOL_TYPE,
        [Some(SyntaxElement::Token(symbol_token))],
    ))
}
pub fn ts_template_chunk_element(template_chunk_token: SyntaxToken) -> TsTemplateChunkElement {
    TsTemplateChunkElement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TEMPLATE_CHUNK_ELEMENT,
        [Some(SyntaxElement::Token(template_chunk_token))],
    ))
}
pub fn ts_template_element(
    dollar_curly_token: SyntaxToken,
    ty: TsType,
    r_curly_token: SyntaxToken,
) -> TsTemplateElement {
    TsTemplateElement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TEMPLATE_ELEMENT,
        [
            Some(SyntaxElement::Token(dollar_curly_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn ts_template_literal_type(
    l_tick_token: SyntaxToken,
    elements: TsTemplateElementList,
    r_tick_token: SyntaxToken,
) -> TsTemplateLiteralType {
    TsTemplateLiteralType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TEMPLATE_LITERAL_TYPE,
        [
            Some(SyntaxElement::Token(l_tick_token)),
            Some(SyntaxElement::Node(elements.into_syntax())),
            Some(SyntaxElement::Token(r_tick_token)),
        ],
    ))
}
pub fn ts_this_parameter(this_token: SyntaxToken) -> TsThisParameterBuilder {
    TsThisParameterBuilder {
        this_token,
        type_annotation: None,
    }
}
pub struct TsThisParameterBuilder {
    this_token: SyntaxToken,
    type_annotation: Option<TsTypeAnnotation>,
}
impl TsThisParameterBuilder {
    pub fn with_type_annotation(mut self, type_annotation: TsTypeAnnotation) -> Self {
        self.type_annotation = Some(type_annotation);
        self
    }
    pub fn build(self) -> TsThisParameter {
        TsThisParameter::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_THIS_PARAMETER,
            [
                Some(SyntaxElement::Token(self.this_token)),
                self.type_annotation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn ts_this_type(this_token: SyntaxToken) -> TsThisType {
    TsThisType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_THIS_TYPE,
        [Some(SyntaxElement::Token(this_token))],
    ))
}
pub fn ts_tuple_type(
    l_brack_token: SyntaxToken,
    elements: TsTupleTypeElementList,
    r_brack_token: SyntaxToken,
) -> TsTupleType {
    TsTupleType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TUPLE_TYPE,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(elements.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn ts_type_alias_declaration(
    type_token: SyntaxToken,
    binding_identifier: TsIdentifierBinding,
    eq_token: SyntaxToken,
    ty: TsType,
) -> TsTypeAliasDeclarationBuilder {
    TsTypeAliasDeclarationBuilder {
        type_token,
        binding_identifier,
        eq_token,
        ty,
        type_parameters: None,
        semicolon_token: None,
    }
}
pub struct TsTypeAliasDeclarationBuilder {
    type_token: SyntaxToken,
    binding_identifier: TsIdentifierBinding,
    eq_token: SyntaxToken,
    ty: TsType,
    type_parameters: Option<TsTypeParameters>,
    semicolon_token: Option<SyntaxToken>,
}
impl TsTypeAliasDeclarationBuilder {
    pub fn with_type_parameters(mut self, type_parameters: TsTypeParameters) -> Self {
        self.type_parameters = Some(type_parameters);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> TsTypeAliasDeclaration {
        TsTypeAliasDeclaration::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION,
            [
                Some(SyntaxElement::Token(self.type_token)),
                Some(SyntaxElement::Node(self.binding_identifier.into_syntax())),
                self.type_parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.eq_token)),
                Some(SyntaxElement::Node(self.ty.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn ts_type_annotation(colon_token: SyntaxToken, ty: TsType) -> TsTypeAnnotation {
    TsTypeAnnotation::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TYPE_ANNOTATION,
        [
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn ts_type_arguments(
    l_angle_token: SyntaxToken,
    ts_type_argument_list: TsTypeArgumentList,
    r_angle_token: SyntaxToken,
) -> TsTypeArguments {
    TsTypeArguments::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TYPE_ARGUMENTS,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Node(ts_type_argument_list.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn ts_type_assertion_assignment(
    l_angle_token: SyntaxToken,
    ty: TsType,
    r_angle_token: SyntaxToken,
    assignment: JsAnyAssignment,
) -> TsTypeAssertionAssignment {
    TsTypeAssertionAssignment::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
            Some(SyntaxElement::Node(assignment.into_syntax())),
        ],
    ))
}
pub fn ts_type_assertion_expression(
    l_angle_token: SyntaxToken,
    ty: TsType,
    r_angle_token: SyntaxToken,
    expression: JsAnyExpression,
) -> TsTypeAssertionExpression {
    TsTypeAssertionExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
        ],
    ))
}
pub fn ts_type_constraint_clause(extends_token: SyntaxToken, ty: TsType) -> TsTypeConstraintClause {
    TsTypeConstraintClause::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TYPE_CONSTRAINT_CLAUSE,
        [
            Some(SyntaxElement::Token(extends_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn ts_type_operator_type(operator_token_token: SyntaxToken, ty: TsType) -> TsTypeOperatorType {
    TsTypeOperatorType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TYPE_OPERATOR_TYPE,
        [
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn ts_type_parameter(name: TsTypeParameterName) -> TsTypeParameterBuilder {
    TsTypeParameterBuilder {
        name,
        constraint: None,
        default: None,
    }
}
pub struct TsTypeParameterBuilder {
    name: TsTypeParameterName,
    constraint: Option<TsTypeConstraintClause>,
    default: Option<TsDefaultTypeClause>,
}
impl TsTypeParameterBuilder {
    pub fn with_constraint(mut self, constraint: TsTypeConstraintClause) -> Self {
        self.constraint = Some(constraint);
        self
    }
    pub fn with_default(mut self, default: TsDefaultTypeClause) -> Self {
        self.default = Some(default);
        self
    }
    pub fn build(self) -> TsTypeParameter {
        TsTypeParameter::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_TYPE_PARAMETER,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.constraint
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.default
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn ts_type_parameter_name(ident_token: SyntaxToken) -> TsTypeParameterName {
    TsTypeParameterName::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TYPE_PARAMETER_NAME,
        [Some(SyntaxElement::Token(ident_token))],
    ))
}
pub fn ts_type_parameters(
    l_angle_token: SyntaxToken,
    items: TsTypeParameterList,
    r_angle_token: SyntaxToken,
) -> TsTypeParameters {
    TsTypeParameters::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TYPE_PARAMETERS,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn ts_typeof_type(
    typeof_token: SyntaxToken,
    expression_name: TsAnyName,
) -> TsTypeofTypeBuilder {
    TsTypeofTypeBuilder {
        typeof_token,
        expression_name,
        type_arguments: None,
    }
}
pub struct TsTypeofTypeBuilder {
    typeof_token: SyntaxToken,
    expression_name: TsAnyName,
    type_arguments: Option<TsTypeArguments>,
}
impl TsTypeofTypeBuilder {
    pub fn with_type_arguments(mut self, type_arguments: TsTypeArguments) -> Self {
        self.type_arguments = Some(type_arguments);
        self
    }
    pub fn build(self) -> TsTypeofType {
        TsTypeofType::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_TYPEOF_TYPE,
            [
                Some(SyntaxElement::Token(self.typeof_token)),
                Some(SyntaxElement::Node(self.expression_name.into_syntax())),
                self.type_arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn ts_undefined_type(undefined_token: SyntaxToken) -> TsUndefinedType {
    TsUndefinedType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_UNDEFINED_TYPE,
        [Some(SyntaxElement::Token(undefined_token))],
    ))
}
pub fn ts_union_type(types: TsUnionTypeVariantList) -> TsUnionTypeBuilder {
    TsUnionTypeBuilder {
        types,
        leading_separator_token: None,
    }
}
pub struct TsUnionTypeBuilder {
    types: TsUnionTypeVariantList,
    leading_separator_token: Option<SyntaxToken>,
}
impl TsUnionTypeBuilder {
    pub fn with_leading_separator_token(mut self, leading_separator_token: SyntaxToken) -> Self {
        self.leading_separator_token = Some(leading_separator_token);
        self
    }
    pub fn build(self) -> TsUnionType {
        TsUnionType::unwrap_cast(SyntaxNode::new_detached(
            JsSyntaxKind::TS_UNION_TYPE,
            [
                self.leading_separator_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.types.into_syntax())),
            ],
        ))
    }
}
pub fn ts_unknown_type(unknown_token: SyntaxToken) -> TsUnknownType {
    TsUnknownType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_UNKNOWN_TYPE,
        [Some(SyntaxElement::Token(unknown_token))],
    ))
}
pub fn ts_void_type(void_token: SyntaxToken) -> TsVoidType {
    TsVoidType::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_VOID_TYPE,
        [Some(SyntaxElement::Token(void_token))],
    ))
}
pub fn js_array_assignment_pattern_element_list<I, S>(
    items: I,
    separators: S,
) -> JsArrayAssignmentPatternElementList
where
    I: IntoIterator<Item = JsAnyArrayAssignmentPatternElement>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsArrayAssignmentPatternElementList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn js_array_binding_pattern_element_list<I, S>(
    items: I,
    separators: S,
) -> JsArrayBindingPatternElementList
where
    I: IntoIterator<Item = JsAnyArrayBindingPatternElement>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsArrayBindingPatternElementList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn js_array_element_list<I, S>(items: I, separators: S) -> JsArrayElementList
where
    I: IntoIterator<Item = JsAnyArrayElement>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsArrayElementList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_ARRAY_ELEMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn js_call_argument_list<I, S>(items: I, separators: S) -> JsCallArgumentList
where
    I: IntoIterator<Item = JsAnyCallArgument>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsCallArgumentList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_CALL_ARGUMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn js_class_member_list<I>(items: I) -> JsClassMemberList
where
    I: IntoIterator<Item = JsAnyClassMember>,
    I::IntoIter: ExactSizeIterator,
{
    JsClassMemberList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_CLASS_MEMBER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn js_constructor_modifier_list<I>(items: I) -> JsConstructorModifierList
where
    I: IntoIterator<Item = TsAccessibilityModifier>,
    I::IntoIter: ExactSizeIterator,
{
    JsConstructorModifierList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_CONSTRUCTOR_MODIFIER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn js_constructor_parameter_list<I, S>(items: I, separators: S) -> JsConstructorParameterList
where
    I: IntoIterator<Item = JsAnyConstructorParameter>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsConstructorParameterList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_CONSTRUCTOR_PARAMETER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn js_directive_list<I>(items: I) -> JsDirectiveList
where
    I: IntoIterator<Item = JsDirective>,
    I::IntoIter: ExactSizeIterator,
{
    JsDirectiveList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_DIRECTIVE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn js_export_named_from_specifier_list<I, S>(
    items: I,
    separators: S,
) -> JsExportNamedFromSpecifierList
where
    I: IntoIterator<Item = JsExportNamedFromSpecifier>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsExportNamedFromSpecifierList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn js_export_named_specifier_list<I, S>(items: I, separators: S) -> JsExportNamedSpecifierList
where
    I: IntoIterator<Item = JsAnyExportNamedSpecifier>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsExportNamedSpecifierList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn js_import_assertion_entry_list<I, S>(items: I, separators: S) -> JsImportAssertionEntryList
where
    I: IntoIterator<Item = JsAnyImportAssertionEntry>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsImportAssertionEntryList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn js_method_modifier_list<I>(items: I) -> JsMethodModifierList
where
    I: IntoIterator<Item = JsAnyMethodModifier>,
    I::IntoIter: ExactSizeIterator,
{
    JsMethodModifierList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_METHOD_MODIFIER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn js_module_item_list<I>(items: I) -> JsModuleItemList
where
    I: IntoIterator<Item = JsAnyModuleItem>,
    I::IntoIter: ExactSizeIterator,
{
    JsModuleItemList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_MODULE_ITEM_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn js_named_import_specifier_list<I, S>(items: I, separators: S) -> JsNamedImportSpecifierList
where
    I: IntoIterator<Item = JsAnyNamedImportSpecifier>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsNamedImportSpecifierList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn js_object_assignment_pattern_property_list<I, S>(
    items: I,
    separators: S,
) -> JsObjectAssignmentPatternPropertyList
where
    I: IntoIterator<Item = JsAnyObjectAssignmentPatternMember>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsObjectAssignmentPatternPropertyList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn js_object_binding_pattern_property_list<I, S>(
    items: I,
    separators: S,
) -> JsObjectBindingPatternPropertyList
where
    I: IntoIterator<Item = JsAnyObjectBindingPatternMember>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsObjectBindingPatternPropertyList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn js_object_member_list<I, S>(items: I, separators: S) -> JsObjectMemberList
where
    I: IntoIterator<Item = JsAnyObjectMember>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsObjectMemberList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_OBJECT_MEMBER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn js_parameter_list<I, S>(items: I, separators: S) -> JsParameterList
where
    I: IntoIterator<Item = JsAnyParameter>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsParameterList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_PARAMETER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn js_property_modifier_list<I>(items: I) -> JsPropertyModifierList
where
    I: IntoIterator<Item = JsAnyPropertyModifier>,
    I::IntoIter: ExactSizeIterator,
{
    JsPropertyModifierList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_PROPERTY_MODIFIER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn js_statement_list<I>(items: I) -> JsStatementList
where
    I: IntoIterator<Item = JsAnyStatement>,
    I::IntoIter: ExactSizeIterator,
{
    JsStatementList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_STATEMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn js_switch_case_list<I>(items: I) -> JsSwitchCaseList
where
    I: IntoIterator<Item = JsAnySwitchClause>,
    I::IntoIter: ExactSizeIterator,
{
    JsSwitchCaseList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_SWITCH_CASE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn js_template_element_list<I>(items: I) -> JsTemplateElementList
where
    I: IntoIterator<Item = JsAnyTemplateElement>,
    I::IntoIter: ExactSizeIterator,
{
    JsTemplateElementList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_TEMPLATE_ELEMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn js_variable_declarator_list<I, S>(items: I, separators: S) -> JsVariableDeclaratorList
where
    I: IntoIterator<Item = JsVariableDeclarator>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsVariableDeclaratorList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_VARIABLE_DECLARATOR_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn jsx_attribute_list<I>(items: I) -> JsxAttributeList
where
    I: IntoIterator<Item = JsxAnyAttribute>,
    I::IntoIter: ExactSizeIterator,
{
    JsxAttributeList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_ATTRIBUTE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn jsx_child_list<I>(items: I) -> JsxChildList
where
    I: IntoIterator<Item = JsxAnyChild>,
    I::IntoIter: ExactSizeIterator,
{
    JsxChildList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JSX_CHILD_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn ts_enum_member_list<I, S>(items: I, separators: S) -> TsEnumMemberList
where
    I: IntoIterator<Item = TsEnumMember>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    TsEnumMemberList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_ENUM_MEMBER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn ts_index_signature_modifier_list<I>(items: I) -> TsIndexSignatureModifierList
where
    I: IntoIterator<Item = TsAnyIndexSignatureModifier>,
    I::IntoIter: ExactSizeIterator,
{
    TsIndexSignatureModifierList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_INDEX_SIGNATURE_MODIFIER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn ts_intersection_type_element_list<I, S>(
    items: I,
    separators: S,
) -> TsIntersectionTypeElementList
where
    I: IntoIterator<Item = TsType>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    TsIntersectionTypeElementList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_INTERSECTION_TYPE_ELEMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn ts_method_signature_modifier_list<I>(items: I) -> TsMethodSignatureModifierList
where
    I: IntoIterator<Item = TsAnyMethodSignatureModifier>,
    I::IntoIter: ExactSizeIterator,
{
    TsMethodSignatureModifierList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_METHOD_SIGNATURE_MODIFIER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn ts_property_parameter_modifier_list<I>(items: I) -> TsPropertyParameterModifierList
where
    I: IntoIterator<Item = TsAnyPropertyParameterModifier>,
    I::IntoIter: ExactSizeIterator,
{
    TsPropertyParameterModifierList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_PROPERTY_PARAMETER_MODIFIER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn ts_property_signature_modifier_list<I>(items: I) -> TsPropertySignatureModifierList
where
    I: IntoIterator<Item = TsAnyPropertySignatureModifier>,
    I::IntoIter: ExactSizeIterator,
{
    TsPropertySignatureModifierList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_PROPERTY_SIGNATURE_MODIFIER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn ts_template_element_list<I>(items: I) -> TsTemplateElementList
where
    I: IntoIterator<Item = TsAnyTemplateElement>,
    I::IntoIter: ExactSizeIterator,
{
    TsTemplateElementList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TEMPLATE_ELEMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn ts_tuple_type_element_list<I, S>(items: I, separators: S) -> TsTupleTypeElementList
where
    I: IntoIterator<Item = TsAnyTupleTypeElement>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    TsTupleTypeElementList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TUPLE_TYPE_ELEMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn ts_type_argument_list<I, S>(items: I, separators: S) -> TsTypeArgumentList
where
    I: IntoIterator<Item = TsType>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    TsTypeArgumentList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TYPE_ARGUMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn ts_type_list<I, S>(items: I, separators: S) -> TsTypeList
where
    I: IntoIterator<Item = TsNameWithTypeArguments>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    TsTypeList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TYPE_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn ts_type_member_list<I>(items: I) -> TsTypeMemberList
where
    I: IntoIterator<Item = TsAnyTypeMember>,
    I::IntoIter: ExactSizeIterator,
{
    TsTypeMemberList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TYPE_MEMBER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn ts_type_parameter_list<I, S>(items: I, separators: S) -> TsTypeParameterList
where
    I: IntoIterator<Item = TsTypeParameter>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    TsTypeParameterList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_TYPE_PARAMETER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn ts_union_type_variant_list<I, S>(items: I, separators: S) -> TsUnionTypeVariantList
where
    I: IntoIterator<Item = TsType>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    TsUnionTypeVariantList::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn js_unknown<I>(slots: I) -> JsUnknown
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    JsUnknown::unwrap_cast(SyntaxNode::new_detached(JsSyntaxKind::JS_UNKNOWN, slots))
}
pub fn js_unknown_assignment<I>(slots: I) -> JsUnknownAssignment
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    JsUnknownAssignment::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_UNKNOWN_ASSIGNMENT,
        slots,
    ))
}
pub fn js_unknown_binding<I>(slots: I) -> JsUnknownBinding
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    JsUnknownBinding::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_UNKNOWN_BINDING,
        slots,
    ))
}
pub fn js_unknown_expression<I>(slots: I) -> JsUnknownExpression
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    JsUnknownExpression::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_UNKNOWN_EXPRESSION,
        slots,
    ))
}
pub fn js_unknown_import_assertion_entry<I>(slots: I) -> JsUnknownImportAssertionEntry
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    JsUnknownImportAssertionEntry::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_UNKNOWN_IMPORT_ASSERTION_ENTRY,
        slots,
    ))
}
pub fn js_unknown_member<I>(slots: I) -> JsUnknownMember
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    JsUnknownMember::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_UNKNOWN_MEMBER,
        slots,
    ))
}
pub fn js_unknown_named_import_specifier<I>(slots: I) -> JsUnknownNamedImportSpecifier
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    JsUnknownNamedImportSpecifier::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_UNKNOWN_NAMED_IMPORT_SPECIFIER,
        slots,
    ))
}
pub fn js_unknown_parameter<I>(slots: I) -> JsUnknownParameter
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    JsUnknownParameter::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_UNKNOWN_PARAMETER,
        slots,
    ))
}
pub fn js_unknown_statement<I>(slots: I) -> JsUnknownStatement
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    JsUnknownStatement::unwrap_cast(SyntaxNode::new_detached(
        JsSyntaxKind::JS_UNKNOWN_STATEMENT,
        slots,
    ))
}
