use crate::{semantic_services::Semantic, utils::rename::RenamableNode};
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::SemanticModel;
use rome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsLiteralExpression, AnyJsStatement,
    AnyJsTemplateElement, JsAssignmentOperator, JsConditionalExpression, JsDoWhileStatement,
    JsForStatement, JsFunctionDeclaration, JsFunctionExpression, JsIfStatement, JsLogicalOperator,
    JsStatementList, JsUnaryOperator, JsWhileStatement, JsYieldExpression,
};
use rome_rowan::{declare_node_union, AstNode, AstSeparatedList};

declare_rule! {
    /// Disallow constant expressions in conditions
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (false) {
    ///     doSomethingUnfinished();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (Boolean(1)) {
    ///     doSomethingAlways();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (undefined) {
    ///     doSomethingUnfinished();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (;-2;) {
    ///     doSomethingForever();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// while (typeof x) {
    ///     doSomethingForever();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var result = 0 ? a : b;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (x === 0) {
    ///     doSomething();
    /// }
    ///
    /// for (;;) {
    ///     doSomethingForever();
    /// }
    ///
    /// while (typeof x === "undefined") {
    ///     doSomething();
    /// }
    ///
    /// do {
    ///     doSomething();
    /// } while (x);
    ///
    /// var result = x !== 0 ? a : b;
    /// ```
    ///
    pub(crate) NoConstantCondition    {
        version: "next",
        name: "noConstantCondition",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) ConditionalStatement = JsConditionalExpression | JsWhileStatement | JsDoWhileStatement | JsIfStatement | JsForStatement
}

impl Rule for NoConstantCondition {
    type Query = Semantic<ConditionalStatement>;
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let conditional_stmt = ctx.query();
        let model = ctx.model();

        // We need to check the conditional statmenet is in a generator function.
        // If the statement has valid yield expression, we don't need to check the statement's `test`.
        if let Some(any_js_stmt) = conditional_stmt.body() {
            if conditional_stmt.is_in_generator_function().unwrap_or(false)
                && has_valid_yield_expression(&any_js_stmt).unwrap_or(false)
            {
                return None;
            }
        }

        let test = conditional_stmt.test()?.omit_parentheses();
        is_constant_condition(&test, true, model).map(|_| test)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.range(),
            markup! {
                "Unexpected constant condition."
            },
        ))
    }
}

impl ConditionalStatement {
    fn test(&self) -> Option<AnyJsExpression> {
        match self {
            Self::JsConditionalExpression(it) => it.test().ok(),
            Self::JsWhileStatement(it) => it.test().ok(),
            Self::JsDoWhileStatement(it) => it.test().ok(),
            Self::JsIfStatement(it) => it.test().ok(),
            Self::JsForStatement(it) => it.test(),
        }
    }
    fn body(&self) -> Option<AnyJsStatement> {
        match self {
            Self::JsWhileStatement(it) => it.body().ok(),
            Self::JsDoWhileStatement(it) => it.body().ok(),
            Self::JsForStatement(it) => it.body().ok(),
            _ => None,
        }
    }
    // Checks if the self statement is in a generator function
    fn is_in_generator_function(&self) -> Option<bool> {
        self.syntax().ancestors().find_map(|node| {
            if let Some(func_decl) = JsFunctionDeclaration::cast(node.clone()) {
                return Some(func_decl.as_fields().star_token.is_some());
            };
            if let Some(func_expr) = JsFunctionExpression::cast(node) {
                return Some(func_expr.as_fields().star_token.is_some());
            };
            None
        })
    }
}

impl From<AnyJsStatement> for ConditionalStatement {
    fn from(node: AnyJsStatement) -> Self {
        match node {
            AnyJsStatement::JsWhileStatement(it) => Self::JsWhileStatement(it),
            AnyJsStatement::JsDoWhileStatement(it) => Self::JsDoWhileStatement(it),
            AnyJsStatement::JsIfStatement(it) => Self::JsIfStatement(it),
            AnyJsStatement::JsForStatement(it) => Self::JsForStatement(it),
            _ => unreachable!(),
        }
    }
}

// Gets a yield expression from the given statement
fn get_yield_expression(stmt: &AnyJsStatement) -> Option<JsYieldExpression> {
    let stmt = stmt.as_js_expression_statement()?;
    let expr = stmt.as_fields().expression.ok()?;
    let expr = expr.as_js_yield_expression()?;
    Some(expr.clone())
}

fn get_stmt_list(stmt: &AnyJsStatement) -> Option<JsStatementList> {
    Some(stmt.as_js_block_statement()?.as_fields().statements)
}

fn has_valid_yield_expression(stmt: &AnyJsStatement) -> Option<bool> {
    let mut stmt_list = get_stmt_list(stmt)?.into_iter();

    loop {
        match stmt_list.next() {
            Some(first_stmt) => {
                if get_yield_expression(&first_stmt).is_some()
                    || stmt_list.any(|stmt| get_yield_expression(&stmt).is_some())
                {
                    return Some(true);
                } else {
                    match first_stmt {
                        AnyJsStatement::JsWhileStatement(stmt) => {
                            stmt_list = get_stmt_list(&stmt.body().ok()?)?.into_iter();
                        }
                        AnyJsStatement::JsDoWhileStatement(stmt) => {
                            stmt_list = get_stmt_list(&stmt.body().ok()?)?.into_iter();
                        }
                        AnyJsStatement::JsForStatement(stmt) => {
                            stmt_list = get_stmt_list(&stmt.body().ok()?)?.into_iter();
                        }
                        _ => return None,
                    }
                }
            }
            None => return None,
        }
    }
}

fn is_constant_condition(
    test: &AnyJsExpression,
    in_boolean_position: bool,
    model: &SemanticModel,
) -> Option<()> {
    use AnyJsExpression::*;

    match test.clone().omit_parentheses() {
        AnyJsLiteralExpression(_)
        | JsObjectExpression(_)
        | JsFunctionExpression(_)
        | JsArrowFunctionExpression(_)
        | JsClassExpression(_) => Some(()),
        JsUnaryExpression(node) => {
            use JsUnaryOperator::*;

            let op = node.operator().ok()?;
            if op == Void || op == Typeof && in_boolean_position {
                return Some(());
            }
            if op == LogicalNot {
                return is_constant_condition(&node.argument().ok()?, true, model);
            }
            is_constant_condition(&node.argument().ok()?, false, model)
        }
        JsBinaryExpression(node) => is_constant_condition(&node.left().ok()?, false, model)
            .and_then(|_| is_constant_condition(&node.right().ok()?, false, model)),
        JsLogicalExpression(node) => {
            let left = node.left().ok()?;
            let right = node.right().ok()?;
            let op = node.operator().ok()?;
            let is_left_constant =
                is_constant_condition(&left, in_boolean_position, model).is_some();
            let is_right_constant =
                is_constant_condition(&right, in_boolean_position, model).is_some();

            let is_left_short_circuit = is_left_constant && is_logical_identity(left, op);
            let is_right_short_circuit =
                in_boolean_position && is_right_constant && is_logical_identity(right, op);

            if (is_left_constant && is_right_constant)
                || is_left_short_circuit
                || is_right_short_circuit
            {
                Some(())
            } else {
                None
            }
        }
        JsSequenceExpression(node) => {
            is_constant_condition(&node.right().ok()?, in_boolean_position, model)
        }
        JsIdentifierExpression(node) => {
            if node.name().ok()?.binding(model).is_some() {
                // This is any_js_stmt edge case. Mordern browser doesn't allow to redeclare `undefined` but ESLint handle this so we do
                return None;
            }
            let is_named_undefined = node.name().ok()?.is_undefined();
            is_named_undefined.then_some(())
        }
        JsArrayExpression(node) => {
            if !in_boolean_position {
                node.elements()
                    .into_iter()
                    .all(|js_statement| {
                        if let Ok(element) = js_statement {
                            match element {
                                AnyJsArrayElement::JsArrayHole(_) => true,
                                AnyJsArrayElement::JsSpread(node) => {
                                    if let Ok(argument) = node.argument() {
                                        is_constant_condition(&argument, in_boolean_position, model)
                                            .is_some()
                                    } else {
                                        false
                                    }
                                }
                                _ => element
                                    .as_any_js_expression()
                                    .and_then(|node| is_constant_condition(node, false, model))
                                    .is_some(),
                            }
                        } else {
                            false
                        }
                    })
                    .then_some(())
            } else {
                Some(())
            }
        }
        JsNewExpression(_) => in_boolean_position.then_some(()),
        JsCallExpression(node) => {
            if node.has_callee("Boolean") {
                let callee = node.callee().ok()?;
                let ident = callee.as_js_identifier_expression()?.name().ok()?;
                let binding = ident.binding(model);
                if binding.is_some() {
                    return None;
                }

                let args = node.arguments().ok()?.args();
                if args.is_empty() {
                    return Some(());
                }
                return is_constant_condition(
                    args.first()?.ok()?.as_any_js_expression()?,
                    true,
                    model,
                );
            }

            None
        }
        JsAssignmentExpression(node) => {
            use JsAssignmentOperator::*;

            let operator = node.operator().ok()?;
            if operator == Assign {
                return is_constant_condition(&node.right().ok()?, in_boolean_position, model);
            }

            if matches!(operator, LogicalOrAssign | LogicalAndAssign) && in_boolean_position {
                let new_op = match operator {
                    LogicalAndAssign => JsLogicalOperator::LogicalAnd,
                    LogicalOrAssign => JsLogicalOperator::LogicalOr,
                    _ => unreachable!(),
                };

                return is_logical_identity(node.right().ok()?, new_op).then_some(());
            }
            None
        }
        JsTemplateExpression(node) => {
            let is_tag = node.tag().is_some();
            let elements = node.elements();
            let has_truthy_quasi = !is_tag
                && elements.clone().into_iter().any(|element| match element {
                    AnyJsTemplateElement::JsTemplateChunkElement(element) => {
                        if let Ok(quasi) = element.template_chunk_token() {
                            !quasi.text_trimmed().is_empty()
                        } else {
                            false
                        }
                    }
                    AnyJsTemplateElement::JsTemplateElement(_) => false,
                });
            if has_truthy_quasi && in_boolean_position {
                return Some(());
            }

            elements
                .into_iter()
                .all(|element| match element {
                    AnyJsTemplateElement::JsTemplateChunkElement(_) => !is_tag,
                    AnyJsTemplateElement::JsTemplateElement(element) => {
                        if let Ok(expr) = element.expression() {
                            is_constant_condition(&expr, false, model).is_some()
                        } else {
                            false
                        }
                    }
                })
                .then_some(())
        }
        _ => None,
    }
}

fn is_logical_identity(node: AnyJsExpression, operator: JsLogicalOperator) -> bool {
    use AnyJsExpression::*;
    use JsLogicalOperator::*;
    match node.omit_parentheses() {
        AnyJsLiteralExpression(node) => {
            let boolean_value = get_boolean_value(node);
            operator == LogicalOr && boolean_value || (operator == LogicalAnd && !boolean_value)
        }
        JsUnaryExpression(node) => {
            if operator != LogicalAnd {
                return false;
            }

            if let Ok(node_operator) = node.operator() {
                node_operator == JsUnaryOperator::Void
            } else {
                false
            }
        }
        JsLogicalExpression(node) => {
            if let Ok(node_operator) = node.operator() {
                // handles `any_js_stmt && false || b`
                // `false` is an identity element of `&&` but not `||`
                // so the logical identity of the whole expression can not be defined.
                if operator != node_operator {
                    return false;
                }

                let is_left_logical_identify = node
                    .left()
                    .ok()
                    .map_or(false, |left| is_logical_identity(left, operator));
                if is_left_logical_identify {
                    return true;
                }

                node.right()
                    .ok()
                    .map_or(false, |right| is_logical_identity(right, operator))
            } else {
                false
            }
        }
        JsAssignmentExpression(node) => {
            if let Ok(node_operator) = node.operator() {
                if let Ok(right) = node.right() {
                    let is_valid_logical_assignment = match node_operator {
                        JsAssignmentOperator::LogicalAndAssign
                            if operator == JsLogicalOperator::LogicalAnd =>
                        {
                            true
                        }
                        JsAssignmentOperator::LogicalOrAssign
                            if operator == JsLogicalOperator::LogicalOr =>
                        {
                            true
                        }
                        _ => false,
                    };

                    is_valid_logical_assignment && is_logical_identity(right, operator)
                } else {
                    false
                }
            } else {
                false
            }
        }
        _ => false,
    }
}

fn get_boolean_value(node: AnyJsLiteralExpression) -> bool {
    use AnyJsLiteralExpression::*;

    match node {
        JsBigintLiteralExpression(node) => {
            if let Ok(value_token) = node.value_token() {
                value_token.text_trimmed() != "0n"
            } else {
                false
            }
        }
        JsBooleanLiteralExpression(node) => {
            if let Ok(value_token) = node.value_token() {
                value_token.text_trimmed() != "false"
            } else {
                false
            }
        }
        JsNullLiteralExpression(_) => false,
        JsNumberLiteralExpression(node) => {
            if let Ok(value_token) = node.value_token() {
                value_token.text_trimmed() != "0"
            } else {
                false
            }
        }
        JsRegexLiteralExpression(_) => true,
        JsStringLiteralExpression(node) => {
            if let Ok(value_token) = node.value_token() {
                let text_trimmed = value_token.text_trimmed();
                text_trimmed != "''" && text_trimmed != "\"\""
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rome_js_syntax::{AnyJsLiteralExpression, SourceType};
    use rome_rowan::SyntaxNodeCast;

    use super::get_boolean_value;

    fn assert_boolean_value(code: &str, value: bool) {
        let source = rome_js_parser::parse(code, SourceType::tsx());

        if source.has_errors() {
            panic!("syntax error")
        }

        let literal_expression = source
            .syntax()
            .descendants()
            .find_map(|js_statement| js_statement.cast::<AnyJsLiteralExpression>());

        assert_eq!(
            get_boolean_value(literal_expression.expect("Not found AnyLiteralExpression.")),
            value
        );
    }
    #[test]
    fn test_get_boolean_value() {
        assert_boolean_value("false", false);
        assert_boolean_value("0", false);
        assert_boolean_value("-0", false);
        assert_boolean_value("0n", false);
        assert_boolean_value("let any_js_stmt =\"\"", false);
        assert_boolean_value("let any_js_stmt = ''", false);
        assert_boolean_value("null", false);

        assert_boolean_value("true", true);
        assert_boolean_value("let any_js_stmt = \"0\"", true);
        assert_boolean_value("let any_js_stmt = \"false\"", true);
        assert_boolean_value("-42", true);
        assert_boolean_value("12n", true);
        assert_boolean_value("3.14", true);
        assert_boolean_value("-3.14", true);
    }
}
