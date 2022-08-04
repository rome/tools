use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyStatement, JsConditionalExpression, JsIfStatement, JsUnaryExpression,
    JsUnaryOperator,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeExt, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow negation in the condition of an `if` statement if it has an `else` clause
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (!true) {consequent;} else {alternate;}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// !true ? consequent : alternate
    ///```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (!true) {consequent;}
    ///```
    ///
    /// ```js
    /// true ? consequent : alternate
    ///```
    pub(crate) NoNegationElse {
        version: "0.7.0",
        name: "noNegationElse",
        recommended: true,
    }
}

impl Rule for NoNegationElse {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsAnyCondition>;
    type State = JsUnaryExpression;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query();

        match n {
            JsAnyCondition::JsConditionalExpression(expr) => {
                if is_negation(&expr.test().ok()?).unwrap_or(false) {
                    Some(expr.test().ok()?.as_js_unary_expression().unwrap().clone())
                } else {
                    None
                }
            }
            JsAnyCondition::JsIfStatement(stmt) => {
                if is_negation(&stmt.test().ok()?).unwrap_or(false)
                    && matches!(
                        stmt.else_clause()?.alternate().ok()?,
                        JsAnyStatement::JsBlockStatement(_)
                    )
                {
                    Some(stmt.test().ok()?.as_js_unary_expression().unwrap().clone())
                } else {
                    None
                }
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            node.range(),
            markup! {
                "Invert blocks when performing a negation test."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        match node {
            JsAnyCondition::JsConditionalExpression(expr) => {
                let mut next_expr = expr
                    .clone()
                    .replace_node(expr.test().ok()?, state.argument().ok()?)?;
                next_expr = next_expr
                    .clone()
                    .replace_node(next_expr.alternate().ok()?, expr.consequent().ok()?)?;
                next_expr = next_expr
                    .clone()
                    .replace_node(next_expr.consequent().ok()?, expr.alternate().ok()?)?;
                mutation.replace_node(
                    node.clone(),
                    JsAnyCondition::JsConditionalExpression(next_expr),
                );
            }
            JsAnyCondition::JsIfStatement(stmt) => {
                let next_stmt = stmt
                    .clone()
                    .replace_node(stmt.test().ok()?, state.argument().ok()?)?;
                let next_stmt = next_stmt.clone().replace_node(
                    next_stmt.else_clause()?,
                    make::js_else_clause(
                        stmt.else_clause()?.else_token().ok()?,
                        stmt.consequent().ok()?,
                    ),
                )?;
                let next_stmt = next_stmt.clone().replace_node(
                    next_stmt.consequent().ok()?,
                    stmt.else_clause()?.alternate().ok()?,
                )?;
                mutation.replace_node(node.clone(), JsAnyCondition::JsIfStatement(next_stmt));
            }
        }

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Exchange alternate and consequent of the node" }.to_owned(),
            mutation,
        })
    }
}

fn is_negation(node: &JsAnyExpression) -> Option<bool> {
    match node {
        JsAnyExpression::JsUnaryExpression(expr) => {
            Some(expr.operator().ok()? == JsUnaryOperator::LogicalNot)
        }
        _ => Some(false),
    }
}

declare_node_union! {
    pub JsAnyCondition = JsConditionalExpression | JsIfStatement
}
