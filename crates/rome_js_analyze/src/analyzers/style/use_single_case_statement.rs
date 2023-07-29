use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{AnyJsStatement, AnyJsSwitchClause, TriviaPieceKind, T};
use rome_rowan::{AstNode, AstNodeList, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Enforces switch clauses have a single statement, emits a quick fix wrapping
    /// the statements in a block.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// switch (foo) {
    ///     case true:
    ///     case false:
    ///         let foo = '';
    ///         foo;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// switch (foo) {
    ///     case true:
    ///     case false: {
    ///         let foo = '';
    ///         foo;
    ///     }
    /// }
    /// ```
    pub(crate) UseSingleCaseStatement {
        version: "0.7.0",
        name: "useSingleCaseStatement",
        recommended: false,
    }
}

impl Rule for UseSingleCaseStatement {
    type Query = Ast<AnyJsSwitchClause>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let switch_clause = ctx.query();
        if switch_clause.consequent().len() > 1 {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let switch_clause = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            switch_clause.consequent().range(),
            markup! {
                "A "<Emphasis>"switch clause"</Emphasis>" should only have a single statement."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let switch_clause = ctx.query();
        let clause_token = switch_clause.clause_token().ok()?;
        let colon_token = switch_clause.colon_token().ok()?;
        let consequent = switch_clause.consequent();
        let new_colon_token = colon_token.with_trailing_trivia([]);
        let new_consequent = make::js_statement_list(Some(AnyJsStatement::JsBlockStatement(
            make::js_block_statement(
                make::token(T!['{'])
                    .with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
                    .with_trailing_trivia_pieces(colon_token.trailing_trivia().pieces()),
                consequent.clone(),
                make::token(T!['}'])
                    .with_leading_trivia_pieces(clause_token.indentation_trivia_pieces()),
            ),
        )));
        let mut mutation = ctx.root().begin();
        mutation.replace_token_discard_trivia(colon_token, new_colon_token);
        mutation.replace_node_discard_trivia(consequent, new_consequent);
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Wrap the statements in a block." }.to_owned(),
            mutation,
        })
    }
}
