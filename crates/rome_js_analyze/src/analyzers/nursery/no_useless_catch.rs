use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsCatchClause, TextRange};
use rome_rowan::{AstNode, AstNodeList};

declare_rule! {
    /// Disallow unnecessary catch clauses
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// try {
    ///     doSomething();
    /// } catch(e) {
    ///     throw e;
    /// }
    /// ```
    /// ```js,expect_diagnostic
    /// try {
    ///     doSomething();
    /// } catch(e) {
    ///     throw e;
    /// } finally {
    ///     doCleanUp();
    /// }
    /// ```
    /// ## Valid
    ///
    /// ```js
    /// try {
    ///     doSomething();
    /// } catch(e) {
    ///     doSomethingWhenCatch();
    ///     throw e;
    /// }
    /// ```
    ///
    /// ```js
    /// try {
    ///     doSomething();
    /// } catch(e) {
    ///     handleError(e);
    /// }
    /// ```
    ///
    pub(crate) NoUselessCatch {
        version: "next",
        name: "noUselessCatch",
        recommended: true,
    }
}

impl Rule for NoUselessCatch {
    type Query = Ast<JsCatchClause>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let binding = ctx.query();

        let catch_body = binding.body().ok()?;
        let body_statements = catch_body.statements();

        if body_statements.len() != 1 {
            return None;
        }

        let catch_declaration = binding.declaration()?;
        let catch_binding_err = catch_declaration
            .binding()
            .ok()?
            .as_any_js_binding()?
            .as_js_identifier_binding()?
            .name_token()
            .ok()?;
        let catch_err_name = catch_binding_err.text();

        // The statements must have the first one,
        // because the body_statements.lent().eq(1).
        let first = body_statements.first().unwrap();
        let js_throw_statement = first.as_js_throw_statement()?;
        let throw_ident = js_throw_statement
            .argument()
            .ok()?
            .as_js_identifier_expression()?
            .text();

        if throw_ident.eq(catch_err_name) {
            return Some(js_throw_statement.syntax().text_trimmed_range());
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, text_range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                text_range,
                markup!("The catch clause that only rethrows the original error is redundant."),
            )
            .note(markup!(
                "It is recommended that these unnecessary catch clauses be removed."
            )),
        )
    }
}
