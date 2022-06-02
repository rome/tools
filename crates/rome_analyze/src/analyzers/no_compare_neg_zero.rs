use rome_console::markup;
use rome_diagnostics::Severity;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsBinaryExpression, JsUnaryExpressionFields, T,
};
use rome_rowan::AstNode;

use crate::registry::{Rule, RuleDiagnostic};
use crate::RuleCategory;

pub(crate) enum NoCompareNegZero {}

impl Rule for NoCompareNegZero {
    const NAME: &'static str = "noCompareNegZero";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsBinaryExpression;
    type State = String;

    fn run(node: &Self::Query) -> Option<Self::State> {
        let op = node.operator_token().ok()?;
        if !matches!(
            op.kind(),
            T![>] | T![>=] | T![<] | T![<=] | T![==] | T![===] | T![!=] | T![!==]
        ) {
            return None;
        }

        let left = node.left().ok()?;
        let right = node.right().ok()?;
        if is_neg_zero(&left).unwrap_or(false) || is_neg_zero(&right).unwrap_or(false) {
            Some(
                // SAFETY: Because we know those T![>] | T![>=] | T![<] | T![<=] | T![==] | T![===] | T![!=] | T![!==] SyntaxKind will
                // always success in to_string, you could look at our test case `noCompareNegZero.js`
                op.kind()
                    .to_string()
                    .map(|kind_string| kind_string.to_string())
                    .unwrap(),
            )
        } else {
            None
        }
    }

    fn diagnostic(node: &Self::Query, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic {
            severity: Severity::Warning,
            range: node.syntax().text_trimmed_range(),
            message: markup! {
                "Do not use the "{state}" operator to compare against -0."
            }
            .to_owned(),
        })
    }
}

fn is_neg_zero(node: &JsAnyExpression) -> Option<bool> {
    match node {
        JsAnyExpression::JsUnaryExpression(expr) => {
            let JsUnaryExpressionFields {
                operator_token,
                argument,
            } = expr.as_fields();
            let operator_token = operator_token.ok()?;
            if !matches!(operator_token.kind(), T![-]) {
                return Some(false);
            }
            let argument = argument.ok()?;

            if let JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsNumberLiteralExpression(expr),
            ) = argument
            {
                Some(&expr.text() == "0")
            } else {
                Some(false)
            }
        }
        _ => Some(false),
    }
}
