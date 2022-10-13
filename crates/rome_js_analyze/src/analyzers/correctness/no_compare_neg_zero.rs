use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsBinaryExpression, JsSyntaxKind, JsUnaryOperator,
};
use rome_rowan::{AstNode, BatchMutationExt, SyntaxToken};

use crate::JsRuleAction;

pub struct NoCompareNegZeroState {
    operator_kind: &'static str,
    left_need_replaced: bool,
    right_need_replaced: bool,
}

declare_rule! {
    /// Disallow comparing against `-0`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// (1 >= -0)
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// (1 >= 0)
    ///```
    pub(crate) NoCompareNegZero {
        version: "0.7.0",
        name: "noCompareNegZero",
        recommended: true,
    }
}

impl Rule for NoCompareNegZero {
    type Query = Ast<JsBinaryExpression>;
    type State = NoCompareNegZeroState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        if !node.is_comparison_operator() {
            return None;
        }

        let op = node.operator_token().ok()?;
        let left = node.left().ok()?;
        let right = node.right().ok()?;
        let is_left_neg_zero = is_neg_zero(&left).unwrap_or(false);
        let is_right_neg_zero = is_neg_zero(&right).unwrap_or(false);
        if is_left_neg_zero || is_right_neg_zero {
            // SAFETY: Because we know those T![>] | T![>=] | T![<] | T![<=] | T![==] | T![===] | T![!=] | T![!==] SyntaxKind will
            // always success in to_string, you could look at our test case `noCompareNegZero.js`
            let operator_kind = op.kind().to_string().unwrap();

            Some(NoCompareNegZeroState {
                operator_kind,
                left_need_replaced: is_left_neg_zero,
                right_need_replaced: is_right_neg_zero,
            })
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Do not use the "{state.operator_kind}" operator to compare against -0."
            },
        ))
    }
    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        if state.left_need_replaced {
            mutation.replace_node(
                node.left().ok()?,
                JsAnyExpression::JsAnyLiteralExpression(
                    JsAnyLiteralExpression::JsNumberLiteralExpression(
                        make::js_number_literal_expression(SyntaxToken::new_detached(
                            JsSyntaxKind::JS_NUMBER_LITERAL,
                            "0",
                            [],
                            [],
                        )),
                    ),
                ),
            );
        }

        if state.right_need_replaced {
            mutation.replace_node(
                node.right().ok()?,
                JsAnyExpression::JsAnyLiteralExpression(
                    JsAnyLiteralExpression::JsNumberLiteralExpression(
                        make::js_number_literal_expression(SyntaxToken::new_detached(
                            JsSyntaxKind::JS_NUMBER_LITERAL,
                            "0",
                            [],
                            [],
                        )),
                    ),
                ),
            );
        }

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Replace -0 with 0" }.to_owned(),
            mutation,
        })
    }
}

fn is_neg_zero(node: &JsAnyExpression) -> Option<bool> {
    match node {
        JsAnyExpression::JsUnaryExpression(expr) => {
            if !matches!(expr.operator().ok()?, JsUnaryOperator::Minus) {
                return Some(false);
            }
            let argument = expr.argument().ok()?;

            if let JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsNumberLiteralExpression(expr),
            ) = argument
            {
                Some(expr.value_token().ok()?.text_trimmed() == "0")
            } else {
                Some(false)
            }
        }
        _ => Some(false),
    }
}
