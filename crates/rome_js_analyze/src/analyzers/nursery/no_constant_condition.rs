use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;

use rome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsConditionalExpression, JsDoWhileStatement,
    JsForStatement, JsIfStatement, JsLogicalOperator, JsSyntaxKind, JsSyntaxToken,
    JsWhileStatement,
};
use rome_rowan::{declare_node_union, AstNode, SyntaxToken};

declare_rule! {
    /// Disallow constant expressions in conditions
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <input type="submit" accessKey="s" value="Submit" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a href="https://webaim.org/" accessKey="w">WebAIM.org</a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <button accessKey="n">Next</button>
    /// ```
    ///
    /// ## Resources
    ///
    /// - [WebAIM: Keyboard Accessibility - Accesskey](https://webaim.org/techniques/keyboard/accesskey#spec)
    /// - [MDN `accesskey` documentation](https://developer.mozilla.org/docs/Web/HTML/Global_attributes/accesskey)
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

        is_constant_condition(&test).map(|_| test)
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

fn is_constant_condition(test: &AnyJsExpression) -> Option<()> {
    use AnyJsExpression::*;
    match test.clone().omit_parentheses() {
        AnyJsLiteralExpression(_)
        | JsTemplateExpression(_)
        | JsObjectExpression(_)
        | JsAssignmentExpression(_) => Some(()),
        JsUnaryExpression(node) => is_constant_condition(&node.argument().ok()?),
        JsBinaryExpression(node) => is_constant_condition(&node.left().ok()?)
            .and_then(|_| is_constant_condition(&node.right().ok()?)),
        JsLogicalExpression(node) => {
            let left = node.left().ok()?;
            let right = node.left().ok()?;
            let op = node.operator().ok()?;
            let is_left_constant = is_constant_condition(&left).is_some();
            let is_right_constant = is_constant_condition(&right).is_some();

            let is_left_short_circuit = if is_left_constant {
                is_logical_identity(left, op)
            } else {
                false
            };
            let is_right_short_circuit = if is_right_constant {
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
        _ => {
            dbg!(test);
            todo!("is_constant_condition not cover this case");
        }
    }
}

fn is_logical_identity(node: AnyJsExpression, operator: JsLogicalOperator) -> bool {
    use AnyJsExpression::*;

    match node {
        AnyJsLiteralExpression(expr) => {
            let boolean_value = get_boolean_value(expr).unwrap_or(false);
            operator == JsLogicalOperator::LogicalOr && boolean_value
                || (operator == JsLogicalOperator::LogicalAnd && !boolean_value)
        }
        _ => todo!(),
    }
}

fn get_boolean_value(node: AnyJsLiteralExpression) -> Option<bool> {
    match node {
        AnyJsLiteralExpression::JsBigIntLiteralExpression(node) => {
            node.value_token().ok().map(|x| x.text_trimmed() != "0n")
        }
        AnyJsLiteralExpression::JsBooleanLiteralExpression(node) => {
            node.value_token().ok().map(|x| x.text_trimmed() == "true")
        }
        AnyJsLiteralExpression::JsNullLiteralExpression(_) => Some(false),
        AnyJsLiteralExpression::JsNumberLiteralExpression(node) => node
            .value_token()
            .ok()
            .map(|value| value.text_trimmed() != "0"),
        AnyJsLiteralExpression::JsRegexLiteralExpression(_) => Some(true),
        AnyJsLiteralExpression::JsStringLiteralExpression(node) => {
            dbg!(node.value_token().unwrap().text_trimmed());
            node.value_token().ok().map(|value| {
                let text_trimmed = value.text_trimmed();
                text_trimmed != "''" && text_trimmed != "\"\""
            })
        }
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
