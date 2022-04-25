use rome_console::markup;
use rome_diagnostics::Severity;
use rome_js_factory::make;
use rome_js_syntax::{JsAnyExpression, JsAnyLiteralExpression, JsAnyRoot, JsBinaryExpression, T};
use rome_js_syntax::{JsSyntaxKind::*, JsSyntaxToken};
use rome_rowan::{AstNodeExt, SyntaxResult};

use crate::registry::{Rule, RuleCodeFix, RuleDiagnostic};

pub(crate) enum NoDoubleEquals {}

impl Rule for NoDoubleEquals {
    const NAME: &'static str = "noDoubleEquals";
    const ACTION_CATEGORIES: &'static [crate::ActionCategory] = &[];

    type Query = JsBinaryExpression;
    type State = JsSyntaxToken;

    fn run(n: &Self::Query) -> Option<Self::State> {
        let op = n.operator_token().ok()?;

        if !matches!(op.kind(), EQ2 | NEQ) {
            return None;
        }

        // TODO: Implement SyntaxResult helpers to make this cleaner
        if is_null_literal(n.left()) || is_null_literal(n.right()) {
            return None;
        }

        Some(op)
    }

    fn diagnostic(_: &Self::Query, op: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic {
            severity: Severity::Error,
            message: markup! {
                "Do not use the "{op.text_trimmed()}" operator"
            }
            .to_owned(),
            range: op.text_trimmed_range(),
        })
    }

    fn code_fix(root: JsAnyRoot, _: &Self::Query, op: &Self::State) -> Option<RuleCodeFix> {
        let root = root.replace_token_retain_trivia(
            op.clone(),
            make::token(if op.kind() == EQ2 { T![===] } else { T![!==] }),
        )?;

        Some(RuleCodeFix { root })
    }
}

fn is_null_literal(res: SyntaxResult<JsAnyExpression>) -> bool {
    matches!(
        res,
        Ok(JsAnyExpression::JsAnyLiteralExpression(
            JsAnyLiteralExpression::JsNullLiteralExpression(_)
        ))
    )
}
