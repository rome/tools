use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Rule, RuleAction, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{JsAnyStatement, JsElseClauseFields, JsIfStatementFields, T};

use rome_rowan::{AstNode, AstNodeExt};

use crate::JsRuleAction;
use crate::{use_block_statements_diagnostic, use_block_statements_replace_body};

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
                    return Some(UseBlockStatementsOperationType::Wrap(consequent));
                }
                if let Some(else_clause) = else_clause {
                    // SAFETY: because we know the variant of `else_clause` is `Some(_)`
                    let JsElseClauseFields {
                        else_token: _,
                        alternate,
                    } = else_clause.as_fields();
                    let alternate = alternate.ok()?;
                    if !matches!(
                        alternate,
                        JsAnyStatement::JsBlockStatement(_) | JsAnyStatement::JsIfStatement(_)
                    ) {
                        return Some(UseBlockStatementsOperationType::Wrap(alternate));
                    }
                }
                None
            }
            JsAnyStatement::JsDoWhileStatement(stmt) => {
                use_block_statements_diagnostic!(stmt)
            }
            JsAnyStatement::JsForInStatement(stmt) => {
                use_block_statements_diagnostic!(stmt)
            }
            JsAnyStatement::JsForOfStatement(stmt) => {
                use_block_statements_diagnostic!(stmt)
            }
            JsAnyStatement::JsForStatement(stmt) => {
                use_block_statements_diagnostic!(stmt)
            }
            JsAnyStatement::JsWhileStatement(stmt) => {
                use_block_statements_diagnostic!(stmt)
            }
            JsAnyStatement::JsWithStatement(stmt) => {
                use_block_statements_diagnostic!(stmt)
            }
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
        let node = ctx.query();
        let root = ctx.root();
        let root = match nodes_need_to_replaced {
            UseBlockStatementsOperationType::Wrap(stmt) => root.replace_node(
                stmt.clone(),
                JsAnyStatement::JsBlockStatement(make::js_block_statement(
                    make::token(T!['{']),
                    make::js_statement_list(std::iter::once(stmt.clone())),
                    make::token(T!['}']),
                )),
            )?,
            UseBlockStatementsOperationType::ReplaceBody => match node {
                JsAnyStatement::JsDoWhileStatement(stmt) => {
                    use_block_statements_replace_body!(JsDoWhileStatement, root, node, stmt)
                }
                JsAnyStatement::JsForInStatement(stmt) => {
                    use_block_statements_replace_body!(JsForInStatement, root, node, stmt)
                }
                JsAnyStatement::JsForOfStatement(stmt) => {
                    use_block_statements_replace_body!(JsForOfStatement, root, node, stmt)
                }
                JsAnyStatement::JsForStatement(stmt) => {
                    use_block_statements_replace_body!(JsForStatement, root, node, stmt)
                }
                JsAnyStatement::JsWhileStatement(stmt) => {
                    use_block_statements_replace_body!(JsWhileStatement, root, node, stmt)
                }
                JsAnyStatement::JsWithStatement(stmt) => {
                    use_block_statements_replace_body!(JsWithStatement, root, node, stmt)
                }
                _ => return None,
            },
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

#[macro_export]
macro_rules! use_block_statements_diagnostic {
    ($id:ident) => {{
        let body = $id.body().ok()?;
        if matches!(body, JsAnyStatement::JsEmptyStatement(_)) {
            return Some(UseBlockStatementsOperationType::ReplaceBody);
        }
        if !matches!(body, JsAnyStatement::JsBlockStatement(_)) {
            return Some(UseBlockStatementsOperationType::Wrap(body.clone()));
        }
        None
    }};
}

#[macro_export]
macro_rules! use_block_statements_replace_body {
    ($stmt_type:ident, $root:ident, $node:ident, $stmt:ident) => {{
        $root.replace_node(
            $node.clone(),
            JsAnyStatement::$stmt_type($stmt.clone().with_body(JsAnyStatement::JsBlockStatement(
                make::js_block_statement(
                    make::token(T!['{']),
                    make::js_statement_list([]),
                    make::token(T!['}']),
                ),
            ))),
        )?
    }};
}
