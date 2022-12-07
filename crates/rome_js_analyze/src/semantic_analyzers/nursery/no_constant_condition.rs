use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;

use rome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsLiteralExpression, AnyJsTemplateElement,
    JsAssignmentOperator, JsConditionalExpression, JsDoWhileStatement, JsForStatement,
    JsIfStatement, JsLogicalOperator, JsUnaryOperator, JsWhileStatement,
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
        version: "12.0.0",
        name: "noConstantCondition",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) ConditionalStatement = JsConditionalExpression | JsWhileStatement | JsDoWhileStatement | JsIfStatement | JsForStatement
}

impl Rule for NoConstantCondition {
    type Query = Ast<ConditionalStatement>;
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let test = ctx.query().test()?.omit_parentheses();

        is_constant_condition(&test, true).map(|_| test)
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
}

fn is_constant_condition(test: &AnyJsExpression, in_boolean_position: bool) -> Option<()> {
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
                return is_constant_condition(&node.argument().ok()?, true);
            }
            is_constant_condition(&node.argument().ok()?, false)
        }
        JsBinaryExpression(node) => is_constant_condition(&node.left().ok()?, false)
            .and_then(|_| is_constant_condition(&node.right().ok()?, false)),
        JsLogicalExpression(node) => {
            let left = node.left().ok()?;
            let right = node.right().ok()?;
            let op = node.operator().ok()?;
            let is_left_constant = is_constant_condition(&left, in_boolean_position).is_some();
            let is_right_constant = is_constant_condition(&right, in_boolean_position).is_some();
            let is_left_short_circuit = if is_left_constant {
                is_logical_identity(left, op)
            } else {
                false
            };
            let is_right_short_circuit = if in_boolean_position && is_right_constant {
                is_logical_identity(right, op)
            } else {
                false
            };

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
            is_constant_condition(&node.right().ok()?, in_boolean_position)
        }
        JsIdentifierExpression(node) => {
            let is_named_undefined = node.is_undefined().ok()?;
            // TODO check if node is reference to global
            is_named_undefined.then_some(())
        }
        JsArrayExpression(node) => {
            if !in_boolean_position {
                node.elements()
                    .into_iter()
                    .all(|x| {
                        if let Some(element) = x.ok() {
                            match element {
                                AnyJsArrayElement::JsArrayHole(_) => true,
                                AnyJsArrayElement::JsSpread(node) => {
                                    if let Some(argument) = node.argument().ok() {
                                        is_constant_condition(&argument, in_boolean_position)
                                            .is_some()
                                    } else {
                                        false
                                    }
                                }
                                _ => element
                                    .as_any_js_expression()
                                    .and_then(|node| is_constant_condition(node, false))
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
                let args = node.arguments().ok()?.args();
                if args.is_empty() {
                    return Some(());
                }
                // TODO check Boolean is referrenced to global.
                return is_constant_condition(args.first()?.ok()?.as_any_js_expression()?, true);
            }

            None
        }
        JsAssignmentExpression(node) => {
            use JsAssignmentOperator::*;

            let operator = node.operator().ok()?;
            if operator == Assign {
                return is_constant_condition(&node.right().ok()?, in_boolean_position);
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
                        if let Some(quasi) = element.template_chunk_token().ok() {
                            quasi.text_trimmed().len() > 0
                        } else {
                            false
                        }
                    }
                    AnyJsTemplateElement::JsTemplateElement(_) => false,
                });
            if has_truthy_quasi && in_boolean_position {
                return Some(());
            }

            let are_expressions_constant = elements.into_iter().all(|element| match element {
                AnyJsTemplateElement::JsTemplateChunkElement(_) => !is_tag,
                AnyJsTemplateElement::JsTemplateElement(element) => {
                    if let Some(expr) = element.expression().ok() {
                        is_constant_condition(&expr, false).is_some()
                    } else {
                        false
                    }
                }
            });

            (are_expressions_constant).then_some(())
        }
        _ => None,
    }
}

fn is_logical_identity(node: AnyJsExpression, operator: JsLogicalOperator) -> bool {
    use AnyJsExpression::*;
    use JsLogicalOperator::*;
    match node.omit_parentheses() {
        AnyJsLiteralExpression(node) => {
            let boolean_value = get_boolean_value(node).unwrap_or(false);
            operator == LogicalOr && boolean_value || (operator == LogicalAnd && !boolean_value)
        }
        JsUnaryExpression(node) => {
            if operator != LogicalAnd {
                return false;
            }

            if let Some(node_operator) = node.operator().ok() {
                node_operator == JsUnaryOperator::Void
            } else {
                false
            }
        }
        JsLogicalExpression(node) => {
            if let Some(node_operator) = node.operator().ok() {
                // handles `a && false || b`
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

                let is_right_logical_identify = node
                    .right()
                    .ok()
                    .map_or(false, |right| is_logical_identity(right, operator));

                is_right_logical_identify
            } else {
                false
            }
        }
        JsAssignmentExpression(node) => {
            if let Some(node_operator) = node.operator().ok() {
                if let Some(right) = node.right().ok() {
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

fn get_boolean_value(node: AnyJsLiteralExpression) -> Option<bool> {
    use AnyJsLiteralExpression::*;

    match node {
        JsBigIntLiteralExpression(node) => {
            node.value_token().ok().map(|x| x.text_trimmed() != "0n")
        }
        JsBooleanLiteralExpression(node) => {
            node.value_token().ok().map(|x| x.text_trimmed() == "true")
        }
        JsNullLiteralExpression(_) => Some(false),
        JsNumberLiteralExpression(node) => node
            .value_token()
            .ok()
            .map(|value| value.text_trimmed() != "0"),
        JsRegexLiteralExpression(_) => Some(true),
        JsStringLiteralExpression(node) => node.value_token().ok().map(|value| {
            let text_trimmed = value.text_trimmed();
            text_trimmed != "''" && text_trimmed != "\"\""
        }),
    }
}

#[cfg(test)]
mod tests {
    use rome_diagnostics::FileId;
    use rome_js_syntax::{AnyJsLiteralExpression, SourceType};
    use rome_rowan::SyntaxNodeCast;

    use super::get_boolean_value;

    fn assert_boolean_value(code: &str, value: bool) {
        let source = rome_js_parser::parse(code, FileId::zero(), SourceType::tsx());

        if source.has_errors() {
            panic!("syntax error")
        }

        let literal_expression = source
            .syntax()
            .descendants()
            .find_map(|x| x.clone().cast::<AnyJsLiteralExpression>());

        assert_eq!(get_boolean_value(literal_expression.unwrap()), Some(value));
    }
    #[test]
    fn test_get_boolean_value() {
        assert_boolean_value("false", false);
        assert_boolean_value("0", false);
        assert_boolean_value("-0", false);
        assert_boolean_value("0n", false);
        assert_boolean_value("let a =\"\"", false);
        assert_boolean_value("let a = ''", false);
        assert_boolean_value("null", false);

        assert_boolean_value("true", true);
        assert_boolean_value("let a = \"0\"", true);
        assert_boolean_value("let a = \"false\"", true);
        assert_boolean_value("-42", true);
        assert_boolean_value("12n", true);
        assert_boolean_value("3.14", true);
        assert_boolean_value("-3.14", true);
    }
}
