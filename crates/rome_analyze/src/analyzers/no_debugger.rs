use rome_console::markup;
use rome_diagnostics::Severity;
use rome_js_syntax::JsDebuggerStatement;
use rome_rowan::AstNode;

use crate::registry::{Rule, RuleDiagnostic};
use crate::RuleCategory;

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
                "This is an unexpected use of the "<Emphasis>"debugger"</Emphasis>" operator."
            }
            .to_owned(),
        })
    }
}
