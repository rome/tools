use rslint_errors::Diagnostic;

use crate::syntax::expr::EXPR_RECOVERY_SET;
use crate::{CompletedMarker, Parser, SyntaxKind, TokenSet};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut, Range};

/// State kept by the parser while parsing.
/// It is required for things such as strict mode or async functions
#[derive(Debug, Clone, PartialEq)]
pub struct ParserState {
	/// If false, object expressions are not allowed to be parsed
	/// inside an expression.
	///
	/// Also applies for object patterns
	pub allow_object_expr: bool,
	/// Whether `in` should be counted in a binary expression
	/// this is for `for...in` statements to prevent ambiguity.
	pub include_in: bool,
	/// Whether the parser is in an iteration statement and `continue` is allowed.
	pub continue_allowed: bool,
	/// Whether the parser is in an iteration or switch statement and
	/// `break` is allowed.
	pub break_allowed: bool,
	/// A list of labels for labelled statements used to report undefined label errors
	/// for break and continue, as well as duplicate labels
	pub labels: HashMap<String, Range<usize>>,
	/// Whether the parser is in a generator function like `function* a() {}`
	pub in_generator: bool,
	/// Whether the parser is inside of a function
	pub in_function: bool,
	/// Whether we potentially are in a place to parse an arrow expression
	pub potential_arrow_start: bool,
	/// Whether we are in an async function
	pub in_async: bool,
	/// Whether we are in strict mode code
	pub strict: Option<StrictMode>,
	/// Whether the code we are parsing is a module
	pub is_module: bool,
	/// The exported default item, used for checking duplicate defaults
	pub default_item: Option<Range<usize>>,
	/// The recovery set primary_expr will use
	pub expr_recovery_set: TokenSet,
	pub should_record_names: bool,
	pub name_map: HashMap<String, Range<usize>>,
	/// Whether the parser is in a conditional expr (ternary expr)
	pub in_cond_expr: bool,
	pub in_case_cond: bool,
	pub(crate) no_recovery: bool,
	pub in_declare: bool,
	pub in_binding_list_for_signature: bool,
	pub decorators_were_valid: bool,
	pub in_default: bool,
	pub for_head_error: Option<Diagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrictMode {
	Module,
	Explicit(Range<usize>),
	Class(Range<usize>),
}

impl Default for ParserState {
	fn default() -> Self {
		Self {
			allow_object_expr: true,
			include_in: true,
			continue_allowed: false,
			break_allowed: false,
			labels: HashMap::new(),
			in_generator: false,
			in_function: false,
			potential_arrow_start: false,
			in_async: false,
			strict: None,
			is_module: false,
			default_item: None,
			expr_recovery_set: EXPR_RECOVERY_SET,
			name_map: HashMap::with_capacity(3),
			should_record_names: false,
			in_cond_expr: false,
			in_case_cond: false,
			no_recovery: false,
			in_declare: false,
			in_binding_list_for_signature: false,
			decorators_were_valid: false,
			in_default: false,
			for_head_error: None,
		}
	}
}

impl ParserState {
	/// Check for duplicate defaults and update state
	pub fn check_default(
		&mut self,
		p: &mut Parser,
		mut marker: CompletedMarker,
	) -> CompletedMarker {
		// A default export is already present
		if let Some(range) = self.default_item.as_ref().filter(|_| self.is_module) {
			let err = p
				.err_builder("Illegal duplicate default export declarations")
				.secondary(
					range.to_owned(),
					"the module's default export is first defined here",
				)
				.primary(marker.range(p), "multiple default exports are erroneous");

			p.error(err);
			marker.change_kind(p, SyntaxKind::ERROR);
		} else if self.is_module {
			self.default_item = Some(marker.range(p).into());
		}
		marker
	}

	/// Turn on strict mode and issue a warning for redundant strict mode declarations
	pub fn strict(&mut self, p: &mut Parser, range: Range<usize>) {
		if let Some(strict) = self.strict.to_owned() {
			let mut err = p.warning_builder("Redundant strict mode declaration");

			match strict {
				StrictMode::Explicit(prev_range) => {
					err = err.secondary(prev_range, "strict mode is previous declared here");
				}
				StrictMode::Module => {
					err = err.footer_note("modules are always strict mode");
				}
				StrictMode::Class(prev_range) => {
					err = err.secondary(prev_range, "class bodies are always strict mode");
				}
			}

			err = err.primary(range, "this declaration is redundant");
			p.error(err);
		} else {
			self.strict = Some(StrictMode::Explicit(range));
		}
	}
}

impl<'t> Parser<'t> {
	pub fn with_state<'a>(&'a mut self, state: ParserState) -> StateGuard<'a, 't> {
		let original_state = self.state.clone();
		self.state = state;
		StateGuard {
			original_state,
			inner: self,
		}
	}
}

pub struct StateGuard<'p, 't> {
	inner: &'p mut Parser<'t>,
	original_state: ParserState,
}

impl<'p, 't> Deref for StateGuard<'p, 't> {
	type Target = Parser<'t>;

	fn deref(&self) -> &Parser<'t> {
		self.inner
	}
}

impl<'p, 't> DerefMut for StateGuard<'p, 't> {
	fn deref_mut(&mut self) -> &mut Parser<'t> {
		&mut self.inner
	}
}

impl<'p, 't> Drop for StateGuard<'p, 't> {
	fn drop(&mut self) {
		std::mem::swap(&mut self.inner.state, &mut self.original_state);
	}
}
