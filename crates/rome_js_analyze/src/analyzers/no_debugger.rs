use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyRoot, JsAnyStatement, JsDebuggerStatement, JsModuleItemList, JsStatementList, T,
};
use rome_rowan::{AstNode, AstNodeExt};

use crate::JsRuleAction;

declare_rule! {
    pub(crate) NoDebugger = "noDebugger"
}

impl Rule for NoDebugger {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsDebuggerStatement;
    type State = ();

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

        let root = if JsStatementList::can_cast(prev_parent.kind())
            || JsModuleItemList::can_cast(prev_parent.kind())
        {
            let index = prev_parent
                .children()
                .position(|slot| &slot == node.syntax())?;

            let next_parent = prev_parent
                .clone()
                .splice_slots(index..=index, std::iter::empty());

            // SAFETY: We know the kind of root is `JsAnyRoot` so cast `root.into_syntax()` will not panic
            JsAnyRoot::unwrap_cast(
                ctx.root()
                    .into_syntax()
                    .replace_child(prev_parent.into(), next_parent.into())?,
            )
        } else {
            ctx.root().replace_node(
                JsAnyStatement::JsDebuggerStatement(node.clone()),
                JsAnyStatement::JsEmptyStatement(make::js_empty_statement(make::token(T![;]))),
            )?
        };
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove debugger statement" }.to_owned(),
            root,
        })
    }
}
