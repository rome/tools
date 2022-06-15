use rome_analyze::{context::RuleContext, ActionCategory, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsBinaryExpression, JsSyntaxKind, JsUnaryOperator,
};
use rome_rowan::{AstNode, AstNodeExt, SyntaxToken};

use crate::JsRuleAction;

pub struct NoCompareNegZeroState {
    operator_kind: &'static str,
    left_need_replaced: bool,
    right_need_replaced: bool,
}
pub(crate) enum NoCompareNegZero {}

impl Rule for NoCompareNegZero {
    const NAME: &'static str = "noCompareNegZero";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsBinaryExpression;
    type State = NoCompareNegZeroState;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query_result();

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
        let node = ctx.query_result();

        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "Do not use the "{state.operator_kind}" operator to compare against -0."
            },
        ))
    }
    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query_result();
        let root = ctx.root();
        let root = if state.left_need_replaced && state.right_need_replaced {
            let binary = node.clone().replace_node(
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
            )?;
            // extract binary.right() as an extra variable because `binary.replace_node` will move ownership.
            let binary_right = binary.right().ok()?;
            let binary = binary.replace_node(
                binary_right,
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
            )?;
            root.clone().replace_node(node.clone(), binary)?
        } else if state.left_need_replaced {
            root.clone().replace_node(
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
            )?
        } else if state.right_need_replaced {
            root.clone().replace_node(
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
            )?
        } else {
            root.clone()
        };

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Replace -0 with 0" }.to_owned(),
            root,
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
