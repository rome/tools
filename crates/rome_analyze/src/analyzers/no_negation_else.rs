use std::iter;

use crate::registry::{Rule, RuleAction, RuleDiagnostic};
use crate::{ActionCategory, RuleCategory};
use rome_console::markup;
use rome_diagnostics::{Applicability, Severity};
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsAnyRoot, JsAnyStatement, JsBinaryExpression,
    JsBooleanLiteralExpression, JsConditionalExpression, JsConditionalExpressionFields,
    JsIfStatement, JsUnaryExpression, T,
};
use rome_js_syntax::{JsSyntaxKind::*, JsSyntaxToken};
use rome_rowan::{AstNode, AstNodeExt, SyntaxResult};

pub(crate) enum NoNegationElse {}

impl Rule for NoNegationElse {
    const NAME: &'static str = "noNegationElse";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsAnyStatement;
    type State = JsUnaryExpression;

    fn run(n: &Self::Query) -> Option<Self::State> {
        match n {
            JsAnyStatement::JsExpressionStatement(stmt) => match stmt.expression() {
                Ok(JsAnyExpression::JsConditionalExpression(expr)) => {
                    if is_negation(&expr.test().ok()?).unwrap_or(false) {
                        Some(expr.test().ok()?.as_js_unary_expression().unwrap().clone())
                    } else {
                        None
                    }
                }
                _ => None,
            },
            JsAnyStatement::JsIfStatement(stmt) => {
                if is_negation(&stmt.test().ok()?).unwrap_or(false) && stmt.else_clause().is_some()
                {
                    Some(stmt.test().ok()?.as_js_unary_expression().unwrap().clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn diagnostic(node: &Self::Query, state: &Self::State) -> Option<RuleDiagnostic> {
        println!("This is node_range: {:?}", node.range());
        Some(RuleDiagnostic {
            severity: Severity::Error,
            message: markup! {
                "Invert blocks when performing a negation test."
            }
            .to_owned(),
            range: node.range(),
        })
    }

    fn action(root: JsAnyRoot, node: &Self::Query, state: &Self::State) -> Option<RuleAction> {
        let root = match node {
            JsAnyStatement::JsExpressionStatement(stmt) => match stmt.expression() {
                Ok(JsAnyExpression::JsConditionalExpression(expr)) => {
                    // let JsConditionalExpressionFields {
                    //     test,
                    //     question_mark_token,
                    //     consequent,
                    //     colon_token,
                    //     alternate,
                    // } = expr.as_fields();
                    let mut next_expr = expr
                        .clone()
                        .replace_node(expr.test().ok()?, state.argument().ok()?)?;
                    next_expr = next_expr
                        .clone()
                        .replace_node(next_expr.alternate().ok()?, expr.consequent().ok()?)?;
                    next_expr = next_expr
                        .clone()
                        .replace_node(next_expr.consequent().ok()?, expr.alternate().ok()?)?;
                    root.replace_node(
                        node.clone(),
                        JsAnyStatement::JsExpressionStatement(
                            make::js_expression_statement(
                                JsAnyExpression::JsConditionalExpression(next_expr),
                            )
                            .build()
                            .with_semicolon_token(stmt.semicolon_token()),
                        ),
                    )
                }
                _ => None,
            },
            JsAnyStatement::JsIfStatement(stmt) => {
                // replace test
                let next_stmt = stmt
                    .clone()
                    .replace_node(stmt.test().ok()?, state.argument().ok()?)?;
                let  next_stmt = next_stmt.clone().replace_node(
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
                let root =
                    root.replace_node(node.clone(), JsAnyStatement::JsIfStatement(next_stmt));
                root
            }
            _ => unreachable!(),
        }
        .unwrap();
        Some(RuleAction {
            category: ActionCategory::Refactor,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Replace with strict equality" }.to_owned(),
            root,
        })
    }
}

#[derive(Debug)]
pub enum IfStatementOrConditionalExpression {
    JsConditionalExpression(JsConditionalExpression),
    JsIfStatement(JsIfStatement),
}

fn is_negation(node: &JsAnyExpression) -> Option<bool> {
    match node {
        JsAnyExpression::JsUnaryExpression(expr) => {
            Some(expr.operator_token().ok()?.text_trimmed() == "!")
        }
        _ => Some(false),
    }
}
