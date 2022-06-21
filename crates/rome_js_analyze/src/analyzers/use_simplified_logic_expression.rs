use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsAnyStatement, JsBooleanLiteralExpression,
    JsForStatement, JsForStatementFields, JsLogicalExpression, JsUnaryExpression, JsUnaryOperator,
    T,
};
use rome_rowan::{AstNode, AstNodeExt};

use crate::JsRuleAction;

declare_rule! {
    /// Enforce the use of `while` loops instead of `for` loops when the
    /// initializer and update expressions are not needed
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (; x.running;) {
    ///     x.step();
    /// }
    /// ```
    pub(crate) UseSimplifiedLogicExpression = "useSimplifiedLogicExpression"
}

impl Rule for UseSimplifiedLogicExpression {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsLogicalExpression;
    type State = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        match node.operator().ok()? {
            rome_js_syntax::JsLogicalOperator::NullishCoalescing => {}
            rome_js_syntax::JsLogicalOperator::LogicalOr => {}
            rome_js_syntax::JsLogicalOperator::LogicalAnd => {}
        }
        todo!()
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "Use "<Emphasis>"while"</Emphasis>" loops instead of "<Emphasis>"for"</Emphasis>" loops."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let root = ctx.root();

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use a while loop" }.to_owned(),
            root,
        })
    }
}

/// https://en.wikipedia.org/wiki/De_Morgan%27s_laws
fn could_apply_de_morgan(node: &JsLogicalExpression) -> Option<bool> {
    let left = node.left().ok()?;
    let right = node.right().ok()?;
    match (left, right) {
        (JsAnyExpression::JsUnaryExpression(left), JsAnyExpression::JsUnaryExpression(right)) => {
            Some(
                matches!(left.operator().ok()?, JsUnaryOperator::LogicalNot)
                    && matches!(right.operator().ok()?, JsUnaryOperator::LogicalNot)
                    && !matches!(left.argument().ok()?, JsAnyExpression::JsUnaryExpression(_))
                    && !matches!(
                        right.argument().ok()?,
                        JsAnyExpression::JsUnaryExpression(_)
                    ),
            )
        }
        _ => Some(false),
    }
}

fn simplify_and_expression(
    literal: JsBooleanLiteralExpression,
    expression: JsAnyExpression,
) -> Option<JsAnyExpression> {
    keep_expression_if_literal(literal, expression, true)
}

fn simplify_or_expression(
    literal: JsBooleanLiteralExpression,
    expression: JsAnyExpression,
) -> Option<JsAnyExpression> {
    keep_expression_if_literal(literal, expression, false)
}

fn keep_expression_if_literal(
    literal: JsBooleanLiteralExpression,
    expression: JsAnyExpression,
    expected_value: bool,
) -> Option<JsAnyExpression> {
    let eval_value = match literal.value_token().ok()?.kind() {
        T![true] => true,
        T![false] => false,
        _ => return None,
    };
    if eval_value == expected_value {
        Some(expression)
    } else {
        Some(JsAnyExpression::JsAnyLiteralExpression(
            JsAnyLiteralExpression::JsBooleanLiteralExpression(literal),
        ))
    }
}

fn simplify_de_morgan(node: JsLogicalExpression) -> Option<JsUnaryExpression> {
    let left = node.left().ok()?;
    let right = node.right().ok()?;
    Some(make::js_unary_expression(make::token(T![!]), todo!()))
}
