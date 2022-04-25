use std::marker::PhantomData;

use rome_console::MarkupBuf;
use rome_diagnostics::Severity;
use rome_js_syntax::{JsAnyRoot, TextRange};

use crate::{registry::Rule, ActionCategory};

/// Event raised by the analyzer when a [Rule](crate::analysis_server::Rule)
/// emits a diagnostic, a code fix, or both
pub trait AnalyzerSignal {
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic>;
    fn code_fix(&self) -> Option<AnalyzerCodeFix>;
}

/// Diagnostic object returned by the analyzer, generated from a [RuleDiagnostic](crate::analysis_server::RuleDiagnostic)
/// with additional informations about the rule injected by the analyzer
#[derive(Debug, PartialEq, Eq)]
pub struct AnalyzerDiagnostic {
    pub rule_name: &'static str,
    pub severity: Severity,
    pub range: TextRange,
    pub message: MarkupBuf,
}

/// Code fix object returned by the analyzer, generated from a [RuleCodeFix](crate::analysis_server::RuleCodeFix)
/// with additional informations about the rule injected by the analyzer
#[derive(Debug, PartialEq, Eq)]
pub struct AnalyzerCodeFix {
    pub rule_name: &'static str,
    pub action_categories: &'static [ActionCategory],
    pub root: JsAnyRoot,
}

/// Analyzer-internal implementation of [AnalyzerSignal] for a specific [Rule](crate::analysis_server::Rule)
pub(crate) struct RuleSignal<'a, R: Rule> {
    root: &'a JsAnyRoot,
    node: R::Query,
    state: R::State,
    _rule: PhantomData<R>,
}

impl<'a, R: Rule + 'static> RuleSignal<'a, R> {
    pub(crate) fn new_boxed(
        root: &'a JsAnyRoot,
        node: R::Query,
        state: R::State,
    ) -> Box<dyn AnalyzerSignal + 'a> {
        Box::new(Self {
            root,
            node,
            state,
            _rule: PhantomData,
        })
    }
}

impl<'a, R: Rule> AnalyzerSignal for RuleSignal<'a, R> {
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic> {
        R::diagnostic(&self.node, &self.state).map(|diag| AnalyzerDiagnostic {
            rule_name: R::NAME,
            severity: diag.severity,
            range: diag.range,
            message: diag.message,
        })
    }

    fn code_fix(&self) -> Option<AnalyzerCodeFix> {
        R::code_fix(self.root.clone(), &self.node, &self.state).map(|code_fix| AnalyzerCodeFix {
            rule_name: R::NAME,
            action_categories: R::ACTION_CATEGORIES,
            root: code_fix.root,
        })
    }
}
