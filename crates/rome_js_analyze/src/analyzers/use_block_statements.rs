use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Rule, RuleAction, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::{Applicability, Severity};
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsAnyRoot, JsAnyStatement, JsBinaryExpression,
    JsElseClauseFields, JsIfStatementFields, TextRange, T,
};
use rome_js_syntax::{JsSyntaxKind::*, JsSyntaxToken};
use rome_rowan::{AstNode, AstNodeExt, SyntaxResult, SyntaxToken};

use crate::JsRuleAction;

declare_rule! {
    /// Block statements are preferred in this position.
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
    pub(crate) UseBlockStatements = "useBlockStatements"
}

impl Rule for UseBlockStatements {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsAnyStatement;
    type State = UseBlockStatementsOperationType;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        match node {
            JsAnyStatement::JsIfStatement(stmt) => {
                let JsIfStatementFields {
                    if_token: _,
                    l_paren_token: _,
                    test: _,
                    r_paren_token: _,
                    consequent,
                    else_clause,
                } = stmt.as_fields();
                let consequent = consequent.ok()?;
                // if `IfStatement` has not consequent then it must has no else clause,
                // so this `?` operation here is safe
                if !matches!(&consequent, JsAnyStatement::JsBlockStatement(_)) {
                    return Some(UseBlockStatementsOperationType::Wrap(consequent.clone()));
                }
                if let Some(else_clause) = else_clause {
                    // SAFETY: because we know the variant of `else_clause` is `Some(_)`
                    let JsElseClauseFields {
                        else_token: _,
                        alternate,
                    } = else_clause.as_fields();
                    // make::js_block_statement(l_curly_token, statements, r_curly_token);
                    let alternate = alternate.ok()?;
                    if !matches!(
                        alternate,
                        JsAnyStatement::JsBlockStatement(_) | JsAnyStatement::JsIfStatement(_)
                    ) {
                        return Some(UseBlockStatementsOperationType::Wrap(alternate.clone()));
                    }
                }
                None
            }
            JsAnyStatement::JsDoWhileStatement(stmt) => {
                let body = stmt.body().ok()?;
                if matches!(body, JsAnyStatement::JsEmptyStatement(_)) {
                    return None;
                }
                None
            }
            JsAnyStatement::JsForInStatement(_)
            | JsAnyStatement::JsForOfStatement(_)
            | JsAnyStatement::JsForStatement(_)
            | JsAnyStatement::JsWhileStatement(_)
            | JsAnyStatement::JsWithStatement(_) => None,
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "Block statements are preferred in this position."
            },
        ))
    }

    fn action(
        ctx: &RuleContext<Self>,
        nodes_need_to_replaced: &Self::State,
    ) -> Option<JsRuleAction> {
        let root = ctx.root();
        let root = match nodes_need_to_replaced {
            UseBlockStatementsOperationType::Wrap(stmt) => {
                let root = root.replace_node(
                    stmt.clone(),
                    JsAnyStatement::JsBlockStatement(make::js_block_statement(
                        make::token(T!['{']),
                        make::js_statement_list(std::iter::once(stmt.clone())),
                        make::token(T!['}']),
                    )),
                )?;
                root
            }
            UseBlockStatementsOperationType::ReplaceBody => todo!(),
        };
        Some(RuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Wrap the statement with a `JsBlockStatement`" }.to_owned(),
            root,
        })
    }
}

pub enum UseBlockStatementsOperationType {
    Wrap(JsAnyStatement),
    ReplaceBody,
}
