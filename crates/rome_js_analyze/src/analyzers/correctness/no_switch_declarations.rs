use std::iter;

use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    AnyJsDeclaration, AnyJsStatement, AnyJsSwitchClause, JsSyntaxNode, JsVariableStatement,
    TriviaPieceKind, T,
};
use rome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow lexical declarations in `switch` clauses.
    ///
    /// Lexical declarations in `switch` clauses are accessible in the entire `switch`.
    /// However, it only gets initialized when it is assigned, which will only happen if the `switch` clause where it is defined is reached.
    ///
    /// To ensure that the lexical declarations only apply to the current `switch` clause wrap your declarations in a block.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-case-declarations
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// switch (foo) {
    ///     case 0:
    ///         const x = 1;
    ///         break;
    ///     case 2:
    ///         x; // `x` can be used while it is not initialized
    ///         break;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// switch (foo) {
    ///     case 0:
    ///         function f() {}
    ///         break;
    ///     case 2:
    ///         f(); // `f` can be called here
    ///         break;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// switch (foo) {
    ///     case 0:
    ///         class A {}
    ///         break;
    ///     default:
    ///         new A(); // `A` can be instantiated here
    ///         break;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// switch (foo) {
    ///     case 0: {
    ///         const x = 1;
    ///         break;
    ///     }
    ///     case 1:
    ///         // `x` is not visible here
    ///         break;
    /// }
    /// ```
    ///
    pub(crate) NoSwitchDeclarations {
        version: "12.0.0",
        name: "noSwitchDeclarations",
        recommended: true,
    }
}

fn declaration_cast(node: JsSyntaxNode) -> Option<AnyJsDeclaration> {
    if JsVariableStatement::can_cast(node.kind()) {
        Some(AnyJsDeclaration::JsVariableDeclaration(
            JsVariableStatement::cast(node)?.declaration().ok()?,
        ))
    } else {
        AnyJsDeclaration::cast(node)
    }
}

impl Rule for NoSwitchDeclarations {
    type Query = Ast<AnyJsSwitchClause>;
    type State = AnyJsDeclaration;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let switch_clause = ctx.query();
        switch_clause
            .consequent()
            .syntax()
            .children()
            .filter_map(declaration_cast)
            .collect()
    }

    fn diagnostic(ctx: &RuleContext<Self>, decl: &Self::State) -> Option<RuleDiagnostic> {
        let switch_clause = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            decl.range(),
            markup! {
                "Other switch clauses can erroneously access this "<Emphasis>"declaration"</Emphasis>".\nWrap the declaration in a block to restrict its access to the switch clause."
            },
        ).detail(switch_clause.range(), markup! {
            "The declaration is defined in this "<Emphasis>"switch clause"</Emphasis>":"
        }))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let switch_clause = ctx.query();
        let clause_token = switch_clause.clause_token().ok()?;
        let colon_token = switch_clause.colon_token().ok()?;
        let consequent = switch_clause.consequent();
        let new_colon_token = colon_token.with_trailing_trivia(iter::empty());
        let new_consequent = make::js_statement_list(Some(AnyJsStatement::JsBlockStatement(
            make::js_block_statement(
                make::token(T!['{'])
                    .with_leading_trivia(Some((TriviaPieceKind::Whitespace, " ")))
                    .with_trailing_trivia_pieces(colon_token.trailing_trivia().pieces()),
                consequent.to_owned(),
                make::token(T!['}']).with_leading_trivia_pieces(clause_token.indentation_trivia()),
            ),
        )));
        let mut mutation = ctx.root().begin();
        mutation.replace_token_discard_trivia(colon_token, new_colon_token);
        mutation.replace_node_discard_trivia(consequent, new_consequent);
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Wrap the "<Emphasis>"declaration"</Emphasis>" in a block." }
                .to_owned(),
            mutation,
        })
    }
}
