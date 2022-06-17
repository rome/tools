use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, ActionCategory, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyStatement, JsBinaryExpression, JsForStatement, JsForStatementFields, JsInExpression,
    JsInstanceofExpression, T,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeExt};
pub(crate) enum NoUnsafeNegation {}

impl Rule for NoUnsafeNegation {
    const NAME: &'static str = "noUnsafeNegation";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsInOrInstanceOfExpression;
    type State = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        match node {
            JsInOrInstanceOfExpression::JsInstanceofExpression(expr) => {
                let left = expr.left().ok()?;
                if let Some(unary) = left.as_js_unary_expression() {
                    match unary.operator().ok()? {
                        rome_js_syntax::JsUnaryOperator::LogicalNot => Some(()),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            JsInOrInstanceOfExpression::JsInExpression(expr) => {
				let left = expr.property().ok()?;
                if let Some(rome_js_syntax::JsAnyExpression::JsUnaryExpression(unary)) = left.as_js_any_expression() {
                    match unary.operator().ok()? {
                        rome_js_syntax::JsUnaryOperator::LogicalNot => Some(()),
                        _ => None,
                    }
                } else {
                    None
                }
			},
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::warning(
			ctx.query().range(),
            markup! {
				"The negation operator is used unsafely on the left side of this binary expression."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        None
        // Some(JsRuleAction {
        //     category: ActionCategory::QuickFix,
        //     applicability: Applicability::MaybeIncorrect,
        //     message: markup! { "Use a while loop" }.to_owned(),
        //     root,
        // })
    }
}

declare_node_union! {
    /// Enum for [JsImport] and [JsExport]
    #[allow(dead_code)]
    pub(crate) JsInOrInstanceOfExpression  = JsInstanceofExpression  | JsInExpression
}
