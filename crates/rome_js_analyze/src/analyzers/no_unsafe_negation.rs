use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, ActionCategory, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyStatement, JsBinaryExpression, JsForStatement, JsForStatementFields,
    JsInExpression, JsInstanceofExpression, T,
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
                if let Some(rome_js_syntax::JsAnyExpression::JsUnaryExpression(unary)) =
                    left.as_js_any_expression()
                {
                    match unary.operator().ok()? {
                        rome_js_syntax::JsUnaryOperator::LogicalNot => Some(()),
                        _ => None,
                    }
                } else {
                    None
                }
            }
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
        let node = ctx.query();
        let mut root = ctx.root();
        // The action could be splitted to three steps
        // 1. Remove `!` operator of unary expression
        // 2. Wrap the expression with `()`, convert the expression to a `JsParenthesizedExpression`
        // 3. Replace the `JsParenthesizedExpression` to `JsUnaryExpression` by adding a `JsUnaryOperator::LogicalNot`
        match node {
            JsInOrInstanceOfExpression::JsInstanceofExpression(expr) => {
                // let next_expr = expr.replace_node(JsAnyExpression::Unary, );
                let left = expr.left().ok()?;
                // SAFETY: We check it in run stage, if `expr.left()` is not a `JsUnaryExpression`,
                // the `run` function will return `None`, which will not reach here.
                let unary_expression = left.as_js_unary_expression().unwrap();
                let argument = unary_expression.argument().ok()?;
                let next_expr = expr.clone().replace_node_discard_trivia(left.clone(), argument)?;
                let next_parenthesis_expression = make::js_parenthesized_expression(
                    make::token(T!['(']),
                    rome_js_syntax::JsAnyExpression::JsInstanceofExpression(next_expr.clone()),
                    make::token(T![')']),
                );
                let next_unary_expression = make::js_unary_expression(
                    unary_expression.operator_token().ok()?,
                    JsAnyExpression::JsParenthesizedExpression(next_parenthesis_expression),
                );
                // root = root.replace_node(expr.clone(), next_expr.clone())?;
                // println!("{}", next_parenthesis_expression);
                root = root
                    .replace_node(
                        JsAnyExpression::JsInstanceofExpression(expr.clone()),
                        JsAnyExpression::JsUnaryExpression(next_unary_expression),
                    )
                    .unwrap();
            }
            JsInOrInstanceOfExpression::JsInExpression(expr) => return None,
        }
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Replace with string literal" }.to_owned(),
            root,
        })
    }
}

declare_node_union! {
    /// Enum for [JsImport] and [JsExport]
    #[allow(dead_code)]
    pub(crate) JsInOrInstanceOfExpression  = JsInstanceofExpression  | JsInExpression
}
