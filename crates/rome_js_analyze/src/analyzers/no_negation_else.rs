use rome_analyze::{context::RuleContext, ActionCategory, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyRoot, JsConditionalExpression, JsIfStatement, JsLanguage, JsSyntaxKind,
    JsUnaryExpression, JsUnaryOperator,
};
use rome_rowan::{AstNode, AstNodeExt};

use crate::JsRuleAction;

pub(crate) enum NoNegationElse {}

impl Rule for NoNegationElse {
    const NAME: &'static str = "noNegationElse";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsAnyCondition;
    type State = JsUnaryExpression;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query_result();

        match n {
            JsAnyCondition::JsConditionalExpression(expr) => {
                if is_negation(&expr.test().ok()?).unwrap_or(false) {
                    Some(expr.test().ok()?.as_js_unary_expression().unwrap().clone())
                } else {
                    None
                }
            }
            JsAnyCondition::JsIfStatement(stmt) => {
                if is_negation(&stmt.test().ok()?).unwrap_or(false) && stmt.else_clause().is_some()
                {
                    Some(stmt.test().ok()?.as_js_unary_expression().unwrap().clone())
                } else {
                    None
                }
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query_result();
        
        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "Invert blocks when performing a negation test."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query_result();

        let root = match node {
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
                ctx.root().clone().replace_node(
                    node.clone(),
                    JsAnyCondition::JsConditionalExpression(next_expr),
                )
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
                ctx.root().clone().replace_node(node.clone(), JsAnyCondition::JsIfStatement(next_stmt))
            }
        }?;
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Exchange alternate and consequent of the node" }.to_owned(),
            root,
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

#[derive(Debug, Clone)]
pub enum JsAnyCondition {
    JsConditionalExpression(JsConditionalExpression),
    JsIfStatement(JsIfStatement),
}

impl AstNode for JsAnyCondition {
    type Language = JsLanguage;

    fn can_cast(kind: <Self::Language as rome_rowan::Language>::Kind) -> bool {
        matches!(
            kind,
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION | JsSyntaxKind::JS_IF_STATEMENT
        )
    }

    fn cast(syntax: rome_rowan::SyntaxNode<Self::Language>) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                JsConditionalExpression::cast(syntax).map(JsAnyCondition::JsConditionalExpression)
            }
            JsSyntaxKind::JS_IF_STATEMENT => {
                JsIfStatement::cast(syntax).map(JsAnyCondition::JsIfStatement)
            }
            _ => None,
        }
    }

    fn syntax(&self) -> &rome_rowan::SyntaxNode<Self::Language> {
        match self {
            JsAnyCondition::JsConditionalExpression(expr) => expr.syntax(),
            JsAnyCondition::JsIfStatement(stmt) => stmt.syntax(),
        }
    }

    fn into_syntax(self) -> rome_rowan::SyntaxNode<Self::Language> {
        match self {
            JsAnyCondition::JsConditionalExpression(expr) => expr.into_syntax(),
            JsAnyCondition::JsIfStatement(stmt) => stmt.into_syntax(),
        }
    }
}
