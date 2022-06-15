use rome_analyze::context::RuleContext;
use rome_analyze::{ActionCategory, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{JsAnyExpression, JsAnyLiteralExpression, JsBinaryExpression, T};
use rome_js_syntax::{JsSyntaxKind::*, JsSyntaxToken};
use rome_rowan::{AstNodeExt, SyntaxResult};

use crate::JsRuleAction;

pub(crate) enum NoDoubleEquals {}

impl Rule for NoDoubleEquals {
    const NAME: &'static str = "noDoubleEquals";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsBinaryExpression;
    type State = JsSyntaxToken;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query_result();

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

    fn diagnostic(_: &RuleContext<Self>, op: &Self::State) -> Option<RuleDiagnostic> {
        let text_trimmed = op.text_trimmed();
        let suggestion = if op.kind() == EQ2 { "===" } else { "!==" };

        Some(
            RuleDiagnostic::warning(op.text_trimmed_range(),markup! {
                "Use "<Emphasis>{suggestion}</Emphasis>" instead of "<Emphasis>{text_trimmed}</Emphasis>
            })
            .primary( markup! {
                <Emphasis>{text_trimmed}</Emphasis>" is only allowed when comparing against "<Emphasis>"null"</Emphasis>
            })
            .footer_note(markup! {
                "Using "<Emphasis>{suggestion}</Emphasis>" may be unsafe if you are relying on type coercion"
            })
            .summary(format!("Use {suggestion} instead of {text_trimmed}.\n{text_trimmed} is only allowed when comparing against `null`"))
        )
    }

    fn action(ctx: &RuleContext<Self>, op: &Self::State) -> Option<JsRuleAction> {
        let suggestion = if op.kind() == EQ2 { T![===] } else { T![!==] };
        let root = ctx
            .root()
            .clone()
            .replace_token(op.clone(), make::token(suggestion))?;

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            // SAFETY: `suggestion` can only be JsSyntaxKind::EQ3 or JsSyntaxKind::NEQ2,
            // the implementation of `to_string` for these two variants always returns Some
            message: markup! { "Use "<Emphasis>{suggestion.to_string().unwrap()}</Emphasis> }
                .to_owned(),
            root,
        })
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
