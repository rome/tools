use rome_diagnostics::{Diagnostic, Span};
use rome_js_syntax::TextRange;

use crate::{ActionCategory, Indel, SyntaxEdit};

#[derive(Debug)]
pub enum Signal {
    Diagnostic(AnalyzeDiagnostic),
    Action(Action),
}

impl Signal {
    pub fn is_diagnostic(&self) -> bool {
        matches!(self, Signal::Diagnostic(_))
    }

    pub fn is_action(&self) -> bool {
        matches!(self, Signal::Action(_))
    }

    /// For an [Action], returns the valid range.
    /// For a [Diagnostic], returns the text range of the primary label.
    pub fn range(&self) -> Option<TextRange> {
        match self {
            Signal::Diagnostic(it) => it.range(),
            Signal::Action(it) => Some(it.range),
        }
    }
}

impl From<Diagnostic> for Signal {
    fn from(d: Diagnostic) -> Self {
        Self::Diagnostic(d.into())
    }
}

impl From<Diagnostic> for AnalyzeDiagnostic {
    fn from(d: Diagnostic) -> Self {
        AnalyzeDiagnostic::new(d)
    }
}

impl From<Diagnostic> for Analysis {
    fn from(d: Diagnostic) -> Self {
        Analysis {
            signals: vec![d.into()],
        }
    }
}

impl From<Action> for Signal {
    fn from(a: Action) -> Self {
        Self::Action(a)
    }
}

impl From<Action> for Analysis {
    fn from(a: Action) -> Self {
        Analysis {
            signals: vec![a.into()],
        }
    }
}

impl FromIterator<Signal> for Option<Analysis> {
    fn from_iter<T: IntoIterator<Item = Signal>>(iter: T) -> Self {
        let analysis = Analysis {
            signals: Vec::from_iter(iter),
        };
        Some(analysis)
    }
}

impl FromIterator<Signal> for Analysis {
    fn from_iter<T: IntoIterator<Item = Signal>>(iter: T) -> Self {
        Analysis {
            signals: Vec::from_iter(iter),
        }
    }
}

impl FromIterator<AnalyzeDiagnostic> for Analysis {
    fn from_iter<T: IntoIterator<Item = AnalyzeDiagnostic>>(iter: T) -> Self {
        Analysis {
            signals: iter.into_iter().map(Signal::Diagnostic).collect(),
        }
    }
}

#[derive(Debug, Clone)]
/// Combines an rome_diagnostics Diagnostic with [SyntaxEdit] actions.
///
/// The suggestions on a [rome_diagnostics::Diagnostic] are only suitable for text edits.
/// Perhaps that diagnostic type can be modified so that this type is
/// unnecessary, but we may not want the core diagnostics format to directly
/// reference syntax nodes.
pub struct AnalyzeDiagnostic {
    pub diagnostic: Diagnostic,
    pub actions: Vec<Action>,
}

impl AnalyzeDiagnostic {
    pub fn new(diagnostic: Diagnostic) -> Self {
        Self {
            diagnostic,
            actions: vec![],
        }
    }

    pub fn with_actions(diagnostic: Diagnostic, actions: Vec<Action>) -> Self {
        Self {
            diagnostic,
            actions,
        }
    }

    /// Get the [TextRange] corresponding to the primary label of this diagnostic.
    pub fn range(&self) -> Option<TextRange> {
        self.diagnostic.primary_text_range()
    }
}

#[derive(Debug, Clone)]
pub struct Action {
    pub title: String,
    pub range: TextRange,
    pub edits: Vec<SyntaxEdit>,
    pub category: ActionCategory,
}

pub struct TextAction {
    pub title: String,
    pub target: TextRange,
    pub edits: Vec<Indel>,
    pub category: ActionCategory,
}

impl From<Action> for TextAction {
    fn from(a: Action) -> Self {
        let edits = a.edits.into_iter().map(Indel::from).collect();
        TextAction {
            title: a.title,
            target: a.range,
            edits,
            category: a.category,
        }
    }
}

// TODO: Errors produced by analyzers should be collected on Analysis
#[derive(Default, Debug)]
pub struct Analysis {
    pub signals: Vec<Signal>,
}

impl Analysis {
    pub fn into_actions(self) -> impl Iterator<Item = Action> {
        self.signals
            // TODO: There must be a better way to do this.
            .into_iter()
            .flat_map(|s| match s {
                Signal::Action(a) => vec![a].into_iter(),
                Signal::Diagnostic(d) => d.actions.into_iter(),
            })
    }

    pub fn into_diagnostics(self) -> impl Iterator<Item = AnalyzeDiagnostic> {
        self.signals.into_iter().filter_map(|s| match s {
            Signal::Diagnostic(d) => Some(d),
            _ => None,
        })
    }
}

impl From<Vec<Signal>> for Analysis {
    fn from(signals: Vec<Signal>) -> Self {
        Self { signals }
    }
}

/// An extension trait for [rome_diagnostics::Diagnostic]
/// In the future, the Diagnostic format might be modified directly.
pub trait DiagnosticExt {
    fn into_signal(self) -> Signal;

    fn primary_text_range(&self) -> Option<TextRange>;
}

impl DiagnosticExt for Diagnostic {
    /// Convenience method to wrap a [Diagnostic] in [Signal::Diagnostic]
    fn into_signal(self) -> Signal {
        Signal::Diagnostic(self.into())
    }

    /// Get the [TextRange] of the diagnostic's primary label
    fn primary_text_range(&self) -> Option<TextRange> {
        self.primary.as_ref().map(|p| p.span.range.as_text_range())
    }
}
