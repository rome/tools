use crate::{FileKind, Parser, Syntax};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut, Range};

/// State kept by the parser while parsing.
/// It is required for things such as strict mode or async functions
#[derive(Debug, PartialEq)]
pub struct ParserState {
	/// If false, object expressions are not allowed to be parsed
	/// inside an expression.
	///
	/// Also applies for object patterns
	allow_object_expr: bool,
	/// Whether `in` should be counted in a binary expression
	/// this is for `for...in` statements to prevent ambiguity.
	include_in: bool,
	/// Whether the parser is in an iteration statement and `continue` is allowed.
	continue_allowed: bool,
	/// Whether the parser is in an iteration or switch statement and
	/// `break` is allowed.
	break_allowed: bool,
	/// A list of labels for labelled statements used to report undefined label errors
	/// for break and continue, as well as duplicate labels
	pub labels: HashMap<String, Range<usize>>,
	/// Whether the parser is in a generator function like `function* a() {}`
	in_generator: bool,
	/// Whether the parser is inside of a function
	in_function: bool,
	/// Whatever the parser is inside of a constructor
	in_constructor: bool,
	/// Whether we potentially are in a place to parse an arrow expression
	potential_arrow_start: bool,
	/// Whether we are in an async function
	in_async: bool,
	/// Whether we are in strict mode code
	strict: Option<StrictMode>,
	/// The exported default item, used for checking duplicate defaults
	pub default_item: Option<Range<usize>>,
	/// If set, the parser reports bindings with identical names. The option stores the name of the
	/// node that disallows duplicate bindings, for example `let`, `const` or `import`.
	pub duplicate_binding_parent: Option<&'static str>,
	pub name_map: HashMap<String, Range<usize>>,
	/// Whether the parser is in a conditional expr (ternary expr)
	in_cond_expr: bool,
	pub(crate) no_recovery: bool,
	in_binding_list_for_signature: bool,
}

#[derive(Debug, PartialEq, Eq)]
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
			in_constructor: false,
			potential_arrow_start: false,
			in_async: false,
			strict: None,
			default_item: None,
			name_map: HashMap::with_capacity(3),
			duplicate_binding_parent: None,
			in_cond_expr: false,
			no_recovery: false,
			in_binding_list_for_signature: false,
		}
	}
}

impl ParserState {
	pub fn new(syntax: Syntax) -> Self {
		// TODO(RDambrosio016): Does TypeScript imply Module/Strict?
		let strict = if syntax.file_kind == FileKind::Module {
			Some(StrictMode::Module)
		} else {
			None
		};

		Self {
			strict,
			..ParserState::default()
		}
	}

	pub fn in_function(&self) -> bool {
		self.in_function
	}

	pub fn in_generator(&self) -> bool {
		self.in_generator
	}

	pub fn in_async(&self) -> bool {
		self.in_async
	}

	pub fn in_constructor(&self) -> bool {
		self.in_constructor
	}

	pub fn continue_allowed(&self) -> bool {
		self.continue_allowed
	}
	pub fn break_allowed(&self) -> bool {
		self.break_allowed
	}

	pub fn include_in(&self) -> bool {
		self.include_in
	}

	pub fn in_condition_expression(&self) -> bool {
		self.in_cond_expr
	}

	pub fn potential_arrow_start(&self) -> bool {
		self.potential_arrow_start
	}

	pub fn allow_object_expression(&self) -> bool {
		self.allow_object_expr
	}

	pub fn strict(&self) -> Option<&StrictMode> {
		self.strict.as_ref()
	}

	pub fn in_binding_list_for_signature(&self) -> bool {
		self.in_binding_list_for_signature
	}
}

impl<'t> Parser<'t> {
	/// Applies the passed in change to the parser's state and reverts the
	/// changes when the returned [ParserStateGuard] goes out of scope.
	pub fn with_scoped_state<'p, C: ChangeParserState>(
		&'p mut self,
		change: C,
	) -> ParserStateGuard<'p, 't, C> {
		let snapshot = change.apply(&mut self.state);
		ParserStateGuard::new(self, snapshot)
	}

	/// Applies the passed in change to the parser state before applying the passed `func` and
	/// restores the state to before the change before returning the result.
	#[inline]
	pub fn with_state<C, F, R>(&mut self, change: C, func: F) -> R
	where
		C: ChangeParserState,
		F: FnOnce(&mut Parser) -> R,
	{
		let snapshot = change.apply(&mut self.state);
		let result = func(self);
		C::restore(&mut self.state, snapshot);
		result
	}
}

/// Reverts state changes to their previous value when it goes out of scope.
/// Can be used like a regular parser.
pub struct ParserStateGuard<'parser, 't, C>
where
	C: ChangeParserState,
{
	snapshot: C::Snapshot,
	inner: &'parser mut Parser<'t>,
}

impl<'parser, 't, C: ChangeParserState> ParserStateGuard<'parser, 't, C> {
	fn new(parser: &'parser mut Parser<'t>, snapshot: C::Snapshot) -> Self {
		Self {
			snapshot,
			inner: parser,
		}
	}
}

impl<'parser, 't, C: ChangeParserState> Drop for ParserStateGuard<'parser, 't, C> {
	fn drop(&mut self) {
		let snapshot = std::mem::take(&mut self.snapshot);

		C::restore(&mut self.inner.state, snapshot);
	}
}

impl<'parser, 't, C: ChangeParserState> Deref for ParserStateGuard<'parser, 't, C> {
	type Target = Parser<'t>;

	fn deref(&self) -> &Self::Target {
		self.inner
	}
}

impl<'parser, 't, C: ChangeParserState> DerefMut for ParserStateGuard<'parser, 't, C> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.inner
	}
}

/// Implements a specific modification to the parser state that can later be reverted.
pub trait ChangeParserState {
	type Snapshot: Default;

	/// Applies the change to the passed in state and returns snapshot that allows restoring the previous state.
	fn apply(self, state: &mut ParserState) -> Self::Snapshot;

	/// Restores the state to its previous value
	fn restore(state: &mut ParserState, value: Self::Snapshot);

	/// Allows composing two changes.
	/// The returned change first applies this modifier and then `other`.
	fn and<O>(self, other: O) -> ComposedParserStateChange<Self, O>
	where
		Self: Sized,
		O: ChangeParserState,
	{
		ComposedParserStateChange::new(self, other)
	}
}

#[derive(Debug, Default)]
pub struct ComposedSnapshot<A, B>
where
	A: Default,
	B: Default,
{
	a: A,
	b: B,
}

#[derive(Debug)]
pub struct ComposedParserStateChange<A, B> {
	a: A,
	b: B,
}

impl<A, B> ComposedParserStateChange<A, B>
where
	A: ChangeParserState,
	B: ChangeParserState,
{
	pub fn new(a: A, b: B) -> Self {
		Self { a, b }
	}
}

impl<A, B> ChangeParserState for ComposedParserStateChange<A, B>
where
	A: ChangeParserState,
	B: ChangeParserState,
{
	type Snapshot = ComposedSnapshot<A::Snapshot, B::Snapshot>;

	fn apply(self, state: &mut ParserState) -> Self::Snapshot {
		ComposedSnapshot {
			a: self.a.apply(state),
			b: self.b.apply(state),
		}
	}

	fn restore(state: &mut ParserState, value: Self::Snapshot) {
		B::restore(state, value.b);
		A::restore(state, value.a);
	}
}

/// Macro for creating a [ChangeParserState] that changes the value of a single [ParserState] field.
/// * `$name`: The name of the [ChangeParserState] implementation
/// * `$field`: The [ParserState] field's name that the implementation *changes*
/// * `$type`: The [ParserState] field's type
/// * `snapshot`: The name of the snapshot struct
macro_rules! gen_change_parser_state {
	($name:ident { $field: ident: $type:ty } => $snapshot: ident) => {
		#[derive(Debug, Clone, Default)]
		pub(crate) struct $snapshot(pub(crate) $type);

		/// Changes the [ParserState] `$field` field
		#[derive(Debug)]
		pub(crate) struct $name(pub(crate) $type);

		impl ChangeParserState for $name {
			type Snapshot = $snapshot;

			#[inline]
			fn apply(self, state: &mut ParserState) -> Self::Snapshot {
				$snapshot(std::mem::replace(&mut state.$field, self.0))
			}

			#[inline]
			fn restore(state: &mut ParserState, value: Self::Snapshot) {
				state.$field = value.0
			}
		}
	};
}

gen_change_parser_state!(InGenerator { in_generator: bool } => InGeneratorSnapshot);
gen_change_parser_state!(InFunction { in_function: bool } => InFunctionSnapshot);
gen_change_parser_state!(InAsync { in_async: bool } => InAsyncSnapshot);
gen_change_parser_state!(InConstructor { in_constructor: bool } => InConstructorSnapshot);
gen_change_parser_state!(BreakAllowed {break_allowed: bool} =>BreakAllowedSnapshot);
gen_change_parser_state!(ContinueAllowed {continue_allowed: bool} => ContinueAllowedSnapshot);
gen_change_parser_state!(IncludeIn { include_in: bool } => IncludeInSnapshot);
gen_change_parser_state!(InConditionExpression { in_cond_expr: bool } => InConditionExpressionSnapshot);
gen_change_parser_state!(AllowObjectExpression { allow_object_expr: bool } => AllowObjectExpressionSnapshot);
gen_change_parser_state!(InBindingListForSignature { in_binding_list_for_signature: bool } => InBindingListForSignatureSnapshot);

#[derive(Debug, Clone, Default)]
pub struct SeenLabelsSnapshot(HashMap<String, Range<usize>>);

/// Resets the [ParserState] `labels` field to an empty map
pub struct NewLabelsScope;

impl ChangeParserState for NewLabelsScope {
	type Snapshot = SeenLabelsSnapshot;

	#[inline]
	fn apply(self, state: &mut ParserState) -> Self::Snapshot {
		SeenLabelsSnapshot(std::mem::take(&mut state.labels))
	}

	#[inline]
	fn restore(state: &mut ParserState, value: Self::Snapshot) {
		state.labels = value.0
	}
}

#[derive(Debug, Clone, Default)]
pub struct PotentialArrowStartSnapshot(bool);

/// Changes the [ParserState] `potential_arrow_expr` field
pub struct PotentialArrowStart(pub bool);

impl ChangeParserState for PotentialArrowStart {
	type Snapshot = PotentialArrowStartSnapshot;

	#[inline]
	fn apply(self, state: &mut ParserState) -> Self::Snapshot {
		PotentialArrowStartSnapshot(std::mem::replace(&mut state.potential_arrow_start, self.0))
	}

	#[inline]
	fn restore(state: &mut ParserState, value: Self::Snapshot) {
		state.potential_arrow_start = value.0;
	}
}

#[derive(Default, Debug)]
pub struct EnableStrictModeSnapshot(Option<StrictMode>);

/// Enables strict mode
pub struct EnableStrictMode(pub StrictMode);

impl ChangeParserState for EnableStrictMode {
	type Snapshot = EnableStrictModeSnapshot;

	#[inline]
	fn apply(self, state: &mut ParserState) -> Self::Snapshot {
		EnableStrictModeSnapshot(std::mem::replace(&mut state.strict, Some(self.0)))
	}

	#[inline]
	fn restore(state: &mut ParserState, value: Self::Snapshot) {
		state.strict = value.0
	}
}
