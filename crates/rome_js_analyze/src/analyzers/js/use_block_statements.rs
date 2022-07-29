use std::iter;

use rome_analyze::context::RuleContext;
use rome_analyze::{
    declare_rule, ActionCategory, Ast, Rule, RuleAction, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{JsAnyStatement, JsElseClauseFields, JsIfStatementFields, TriviaPieceKind, T};

use rome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;
use crate::{use_block_statements_diagnostic, use_block_statements_replace_body};

declare_rule! {
    /// Requires following curly brace conventions.
    /// JavaScript allows the omission of curly braces when a block contains only one statement. However, it is considered by many to be best practice to never omit curly braces around blocks, even when they are optional, because it can lead to bugs and reduces code clarity.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    ///  if (x) x;
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///  if (x) {
    ///    x;
    ///  } else y;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (x) {
    ///   x;
    /// } else if (y) y;
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///    for (;;);
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///    for (p in obj);
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///   for (x of xs);
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///   do;
    ///   while (x);
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///    while (x);
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///   with (x);
    /// ```
    pub(crate) UseBlockStatements {
        version: "0.7.0",
        name: "useBlockStatements",
        recommended: true,
    }
}

impl Rule for UseBlockStatements {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsAnyStatement>;
    type State = UseBlockStatementsOperationType;
    type Signals = Option<Self::State>;

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
        let mut mutation = ctx.root().begin();

        match nodes_need_to_replaced {
            UseBlockStatementsOperationType::Wrap(stmt) => mutation.replace_node(
                stmt.clone(),
                JsAnyStatement::JsBlockStatement(make::js_block_statement(
                    make::token(T!['{'])
                        .with_trailing_trivia(iter::once((TriviaPieceKind::Whitespace, " "))),
                    make::js_statement_list(iter::once(stmt.clone())),
                    make::token(T!['}'])
                        .with_leading_trivia(iter::once((TriviaPieceKind::Whitespace, " "))),
                )),
            ),
            UseBlockStatementsOperationType::ReplaceBody => match node {
                JsAnyStatement::JsDoWhileStatement(stmt) => {
                    use_block_statements_replace_body!(JsDoWhileStatement, mutation, node, stmt)
                }
                JsAnyStatement::JsForInStatement(stmt) => {
                    use_block_statements_replace_body!(JsForInStatement, mutation, node, stmt)
                }
                JsAnyStatement::JsForOfStatement(stmt) => {
                    use_block_statements_replace_body!(JsForOfStatement, mutation, node, stmt)
                }
                JsAnyStatement::JsForStatement(stmt) => {
                    use_block_statements_replace_body!(JsForStatement, mutation, node, stmt)
                }
                JsAnyStatement::JsWhileStatement(stmt) => {
                    use_block_statements_replace_body!(JsWhileStatement, mutation, node, stmt)
                }
                JsAnyStatement::JsWithStatement(stmt) => {
                    use_block_statements_replace_body!(JsWithStatement, mutation, node, stmt)
                }
                _ => return None,
            },
        };
        Some(RuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Wrap the statement with a `JsBlockStatement`" }.to_owned(),
            mutation,
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
    ($stmt_type:ident, $mutation:ident, $node:ident, $stmt:ident) => {{
        $mutation.replace_node(
            $node.clone(),
            JsAnyStatement::$stmt_type(
                $stmt.clone().with_body(JsAnyStatement::JsBlockStatement(
                    make::js_block_statement(
                        make::token(T!['{'])
                            .with_leading_trivia(iter::once((TriviaPieceKind::Whitespace, " "))),
                        make::js_statement_list([]),
                        make::token(T!['}']),
                    ),
                )),
            ),
        )
    }};
}
