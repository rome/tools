use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{JsAnyStatement, JsDebuggerStatement, JsModuleItemList, JsStatementList, T};
use rome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow the use of `debugger`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// debugger;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const test = { debugger: 1 };
    /// test.debugger;
    ///```
    pub(crate) NoDebugger {
        version: "0.7.0",
        name: "noDebugger",
        recommended: true
    }
}

impl Rule for NoDebugger {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsDebuggerStatement>;
    type State = ();
    type Signals = Option<Self::State>;

    fn run(_: &RuleContext<Self>) -> Option<Self::State> {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::warning(
            node.syntax().text_trimmed_range(),
            markup! {
                "This is an unexpected use of the "<Emphasis>"debugger"</Emphasis>" statement."
            }
            .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        let prev_parent = node.syntax().parent()?;

        let mut mutation = ctx.root().begin();
        if JsStatementList::can_cast(prev_parent.kind())
            || JsModuleItemList::can_cast(prev_parent.kind())
        {
            mutation.remove_node(node.clone());
        } else {
            mutation.replace_node(
                JsAnyStatement::JsDebuggerStatement(node.clone()),
                JsAnyStatement::JsEmptyStatement(make::js_empty_statement(make::token(T![;]))),
            );
        }

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove debugger statement" }.to_owned(),
            mutation,
        })
    }
}
