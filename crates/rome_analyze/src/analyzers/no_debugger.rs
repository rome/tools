use rome_console::markup;
use rome_diagnostics::{Applicability, Severity};
use rome_js_syntax::{
    JsAnyRoot, JsAnyStatement, JsDebuggerStatement, JsModuleItemList, JsStatementList, T,
};
use rome_rowan::{AstNode, AstNodeExt};

use crate::registry::{Rule, RuleAction, RuleDiagnostic};
use crate::{ActionCategory, RuleCategory};
use rome_js_factory::make;

pub(crate) enum NoDebugger {}

impl Rule for NoDebugger {
    const NAME: &'static str = "noDebugger";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsDebuggerStatement;
    type State = ();

    fn run(_: &Self::Query) -> Option<Self::State> {
        Some(())
    }

    fn diagnostic(node: &Self::Query, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic {
            severity: Severity::Warning,
            range: node.syntax().text_trimmed_range(),
            message: markup! {
                "This is an unexpected use of the "<Emphasis>"debugger"</Emphasis>" statement."
            }
            .to_owned(),
        })
    }

    fn action(
        root: rome_js_syntax::JsAnyRoot,
        node: &Self::Query,
        _state: &Self::State,
    ) -> Option<crate::registry::JsRuleAction> {
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
                root.into_syntax()
                    .replace_child(prev_parent.into(), next_parent.into())?,
            )
        } else {
            root.replace_node(
                JsAnyStatement::JsDebuggerStatement(node.clone()),
                JsAnyStatement::JsEmptyStatement(make::js_empty_statement(make::token(T![;]))),
            )?
        };
        Some(RuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove debugger statement" }.to_owned(),
            root,
        })
    }
}
