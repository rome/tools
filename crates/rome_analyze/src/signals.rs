use anyhow::Result;
use rslint_parser::{SyntaxElement, TextRange};

use crate::{ActionCategory, Indel, SyntaxEdit};

pub type AnalyzerResult = Result<Analysis>;

#[derive(Debug)]
pub enum Signal {
	Diagnostic(AnalyzeDiagnostic),
	Action(Action),
}

impl Signal {
	pub fn diagnostic(target: impl Into<SyntaxElement>, message: impl Into<String>) -> Self {
		let range = target.into().text_trimmed_range();
		let diag = AnalyzeDiagnostic {
			range,
			message: message.into(),
			actions: Vec::new(),
		};
		diag.into()
	}

	pub fn diagnostic_with_replacement(
		target: impl Into<SyntaxElement>,
		message: impl Into<String>,
		action_title: impl Into<String>,
		replacement: impl Into<SyntaxElement>,
		category: ActionCategory,
	) -> Self {
		let target: SyntaxElement = target.into();
		let range = target.text_trimmed_range();
		let edit = SyntaxEdit::Replace {
			target,
			replacement: replacement.into(),
			trimmed: true,
		};
		let action = Action {
			title: action_title.into(),
			range,
			edits: vec![edit],
			category,
		};
		let diag = AnalyzeDiagnostic {
			range,
			message: message.into(),
			actions: vec![action],
		};
		diag.into()
	}
}

impl Signal {
	pub fn is_diagnostic(&self) -> bool {
		matches!(self, Signal::Diagnostic(_))
	}

	pub fn is_action(&self) -> bool {
		matches!(self, Signal::Action(_))
	}

	pub fn range(&self) -> TextRange {
		match self {
			Signal::Diagnostic(it) => it.range,
			Signal::Action(it) => it.range,
		}
	}
}

#[derive(Debug)]
pub struct DiagnosticWithActions {
	pub diagnostic: AnalyzeDiagnostic,
	pub actions: Vec<Action>,
}

impl From<AnalyzeDiagnostic> for Signal {
	fn from(d: AnalyzeDiagnostic) -> Self {
		Self::Diagnostic(d)
	}
}

impl From<AnalyzeDiagnostic> for Analysis {
	fn from(d: AnalyzeDiagnostic) -> Self {
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

impl FromIterator<Signal> for Result<Analysis> {
	fn from_iter<T: IntoIterator<Item = Signal>>(iter: T) -> Self {
		let analysis = Analysis {
			signals: Vec::from_iter(iter),
		};
		Ok(analysis)
	}
}

#[derive(Debug, Clone)]
pub struct AnalyzeDiagnostic {
	pub range: TextRange,
	pub message: String,
	pub actions: Vec<Action>,
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

#[derive(Default, Debug)]
pub struct Analysis {
	pub signals: Vec<Signal>,
}

impl Analysis {
	pub fn into_actions(self) -> impl Iterator<Item = Action> {
		self.signals
			.into_iter()
			// TODO: There must be a better way to do this.
			.map(|s| match s {
				Signal::Action(a) => vec![a].into_iter(),
				Signal::Diagnostic(d) => d.actions.into_iter(),
			})
			.flatten()
	}
}

impl From<Vec<Signal>> for Analysis {
	fn from(signals: Vec<Signal>) -> Self {
		Self { signals }
	}
}
