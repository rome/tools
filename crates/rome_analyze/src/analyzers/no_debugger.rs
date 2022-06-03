use rome_console::markup;
use rome_diagnostics::{Applicability, Severity};
use rome_js_syntax::{JsAnyStatement, JsDebuggerStatement, T};
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
    ) -> Option<crate::registry::RuleAction> {
        let root = root.replace_node(
            JsAnyStatement::JsDebuggerStatement(node.clone()),
            JsAnyStatement::JsEmptyStatement(make::js_empty_statement(make::token(T![;]))),
        )?;
        Some(RuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove debugger statement" }.to_owned(),
            root,
        })
    }
}
