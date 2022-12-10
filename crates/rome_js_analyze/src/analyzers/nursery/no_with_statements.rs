use std::iter;

use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleAction, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    AnyJsStatement, JsWithStatement,
    TriviaPieceKind, T, JsSyntaxTrivia, JsLanguage,
};

use rome_rowan::{declare_node_union, AstNode, BatchMutationExt, SyntaxTriviaPiece};

use crate::JsRuleAction;
use crate::{use_with_statements_diagnostic, use_with_statements_replace_body};

declare_rule! {
    /// Disallow with statements.
    ///
    /// The with statement is potentially problematic because it adds members of an object to the current 
    /// scope, making it impossible to tell what a variable inside the block actually refers to.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// with (point) {
    ///   r = Math.sqrt(x * x + y * y); // is r a member of point?
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// const r = ({x, y}) => Math.sqrt(x * x + y * y);
    /// ```
    pub(crate) NoWithStatements {
        version: "12.0.0",
        name: "noWithStatements",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) AnyJsWithStatement = JsWithStatement
}

impl Rule for NoWithStatements {
    type Query = Ast<AnyJsWithStatement>;
    type State = UseWithStatementsOperationType;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        match node {
            AnyJsWithStatement::JsWithStatement(stmt) => {
                use_with_statements_diagnostic!(stmt)
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Unexpected use of "<Emphasis>"with"</Emphasis>" statement."
            },
        ).note(
            r#"The with statement is potentially problematic because it adds members of an object to the current\n
               scope, making it impossible to tell what a variable inside the block actually refers to."#
        ))
    }

    fn action(
        ctx: &RuleContext<Self>,
        nodes_need_to_replaced: &Self::State,
    ) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        match nodes_need_to_replaced {
            UseWithStatementsOperationType::Wrap(stmt) => {
                let mut l_curly_token = make::token(T!['{']);
                let r_curly_token = make::token(T!['}']);

                // Ensure the opening curly token is separated from the previous token by at least one space
                let has_previous_space = stmt
                    .syntax()
                    .first_token()
                    .and_then(|token| token.prev_token())
                    .map(|token| {
                        token
                            .trailing_trivia()
                            .pieces()
                            .rev()
                            .take_while(|piece| !piece.is_newline())
                            .any(|piece| piece.is_whitespace())
                    })
                    .unwrap_or(false);

                if !has_previous_space {
                    l_curly_token = l_curly_token
                        .with_leading_trivia(iter::once((TriviaPieceKind::Whitespace, " ")));
                }

                // Clone the leading trivia of the single statement as the
                // leading trivia of the closing curly token
                let mut leading_trivia = stmt
                    .syntax()
                    .first_leading_trivia()
                    .map(collect_to_first_newline)
                    .unwrap_or_else(Vec::new);

                // If the statement has no leading trivia, add a space after
                // the opening curly token
                if leading_trivia.is_empty() {
                    l_curly_token = l_curly_token
                        .with_trailing_trivia(iter::once((TriviaPieceKind::Whitespace, " ")));
                }

                // If the leading trivia for the statement contains any newline,
                // then the indentation is probably one level too deep for the
                // closing curly token, clone the leading trivia from the
                // parent node instead
                if leading_trivia.iter().any(|piece| piece.is_newline()) {
                    // Find the parent block statement node, skipping over
                    // else-clause nodes if this statement is part of an
                    // else-if chain
                    let node = node.clone();

                    leading_trivia = node
                        .syntax()
                        .first_leading_trivia()
                        .map(collect_to_first_newline)
                        .unwrap_or_else(Vec::new);
                }

                // Apply the cloned trivia to the closing curly token, or
                // fallback to a single space if it's still empty
                let r_curly_token = if !leading_trivia.is_empty() {
                    let leading_trivia = leading_trivia
                        .iter()
                        .rev()
                        .map(|piece| (piece.kind(), piece.text()));

                    r_curly_token.with_leading_trivia(leading_trivia)
                } else {
                    let has_trailing_single_line_comments = stmt
                        .syntax()
                        .last_trailing_trivia()
                        .map(|trivia| {
                            trivia
                                .pieces()
                                .any(|trivia| trivia.kind() == TriviaPieceKind::SingleLineComment)
                        })
                        .unwrap_or(false);
                    // if the node we have to enclose has some trailing comments, then we add a new line
                    // to the leading trivia of the right curly brace
                    if !has_trailing_single_line_comments {
                        r_curly_token
                            .with_leading_trivia(iter::once((TriviaPieceKind::Whitespace, " ")))
                    } else {
                        r_curly_token
                            .with_leading_trivia(iter::once((TriviaPieceKind::Newline, "\n")))
                    }
                };

                mutation.replace_node_discard_trivia(
                    stmt.clone(),
                    AnyJsStatement::JsBlockStatement(make::js_block_statement(
                        l_curly_token,
                        make::js_statement_list(iter::once(stmt.clone())),
                        r_curly_token,
                    )),
                );
            }
            UseWithStatementsOperationType::ReplaceBody => match node {
                AnyJsWithStatement::JsWithStatement(stmt) => {
                    use_with_statements_replace_body!(JsWithStatement, mutation, node, stmt)
                }
            },
        };
        Some(RuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Unwrap the the body of statement `with`" }.to_owned(),
            mutation,
        })
    }
}

/// Collect newline and comment trivia pieces in reverse order up to the first newline included
fn collect_to_first_newline(trivia: JsSyntaxTrivia) -> Vec<SyntaxTriviaPiece<JsLanguage>> {
    let mut has_newline = false;
    trivia
        .pieces()
        .rev()
        .filter(|piece| piece.is_newline() || piece.is_whitespace())
        .take_while(|piece| {
            let had_newline = has_newline;
            has_newline |= piece.is_newline();
            !had_newline
        })
        .collect()
}

pub enum UseWithStatementsOperationType {
    Wrap(AnyJsStatement),
    ReplaceBody,
}

#[macro_export]
macro_rules! use_with_statements_diagnostic {
    ($id:ident, $field:ident) => {{
        let body = $id.$field().ok()?;
        if matches!(body, AnyJsStatement::JsEmptyStatement(_)) {
            Some(UseWithStatementsOperationType::ReplaceBody)
        } else if !matches!(body, AnyJsStatement::JsBlockStatement(_)) {
            Some(UseWithStatementsOperationType::Wrap(body))
        } else {
            None
        }
    }};
    ($id:ident) => {
        use_with_statements_diagnostic!($id, body)
    };
}

#[macro_export]
macro_rules! use_with_statements_replace_body {
    ($stmt_type:ident, $builder_method:ident, $mutation:ident, $node:ident, $stmt:ident) => {
        $mutation.replace_node(
            $node.clone(),
            AnyJsWithStatement::$stmt_type(
                $stmt
                    .clone()
                    .$builder_method(AnyJsStatement::JsBlockStatement(make::js_block_statement(
                        make::token(T!['{'])
                            .with_leading_trivia(iter::once((TriviaPieceKind::Whitespace, " "))),
                        make::js_statement_list([]),
                        make::token(T!['}']),
                    ))),
            ),
        )
    };

    ($stmt_type:ident, $mutation:ident, $node:ident, $stmt:ident) => {
        use_with_statements_replace_body!($stmt_type, with_body, $mutation, $node, $stmt)
    };
}