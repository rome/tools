use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{JsAnyStatement, JsForStatement, JsForStatementFields, T};
use rome_rowan::AstNodeExt;

use crate::JsRuleAction;

declare_rule! {
    /// Enforce the use of `while` loops instead of `for` loops when the
    /// initializer and update expressions are not needed
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (; x.running;) {
    ///     x.step();
    /// }
    /// ```
    pub(crate) UseWhile = "useWhile"
}

impl Rule for UseWhile {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsForStatement>;
    type State = ();
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query();

        let JsForStatementFields {
            for_token: _,
            l_paren_token,
            initializer,
            first_semi_token: _,
            test,
            second_semi_token: _,
            update,
            r_paren_token,
            body,
        } = n.as_fields();

        if l_paren_token.is_err()
            || initializer.is_some()
            || test.is_none()
            || update.is_some()
            || r_paren_token.is_err()
            || body.is_err()
        {
            None
        } else {
            Some(())
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        // SAFETY: These tokens have been checked for errors in `run` already
        let for_range = node.for_token().unwrap().text_trimmed_range();
        let r_paren_range = node.r_paren_token().unwrap().text_trimmed_range();

        Some(RuleDiagnostic::warning(
            for_range.cover(r_paren_range),
            markup! {
                "Use "<Emphasis>"while"</Emphasis>" loops instead of "<Emphasis>"for"</Emphasis>" loops."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        let JsForStatementFields {
            for_token: _,
            l_paren_token,
            initializer: _,
            first_semi_token: _,
            test,
            second_semi_token: _,
            update: _,
            r_paren_token,
            body,
        } = node.as_fields();

        let root = ctx.root().replace_node(
            JsAnyStatement::from(node.clone()),
            JsAnyStatement::from(make::js_while_statement(
                make::token_decorated_with_space(T![while]),
                l_paren_token.ok()?,
                test?,
                r_paren_token.ok()?,
                body.ok()?,
            )),
        )?;

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use a while loop" }.to_owned(),
            root,
        })
    }
}
