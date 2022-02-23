use crate::{Language, Parser, SourceType};
use bitflags::bitflags;
use indexmap::IndexMap;
use std::collections::HashSet;
use std::ops::{Deref, DerefMut, Range};

type LabelSet = IndexMap<String, LabelledItem>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum LabelledItem {
    Iteration(Range<usize>),
    Other(Range<usize>),
}

impl LabelledItem {
    pub(crate) fn range(&self) -> &Range<usize> {
        match self {
            LabelledItem::Iteration(range) | LabelledItem::Other(range) => range,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum ExportDefaultItemKind {
    Unknown,
    Expression,
    FunctionOverload,
    FunctionDeclaration,
    Interface,
    // Any other declaration
    Declaration,
}

impl ExportDefaultItemKind {
    pub(crate) fn is_overload(&self) -> bool {
        matches!(self, ExportDefaultItemKind::FunctionOverload)
    }

    pub(crate) fn is_function_declaration(&self) -> bool {
        matches!(self, ExportDefaultItemKind::FunctionDeclaration)
    }

    pub(crate) fn is_interface(&self) -> bool {
        matches!(self, ExportDefaultItemKind::Interface)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct ExportDefaultItem {
    pub kind: ExportDefaultItemKind,
    pub range: Range<usize>,
}

/// State kept by the parser while parsing.
/// It is required for things such as strict mode or async functions
#[derive(Debug)]
pub(crate) struct ParserState {
    parsing_context: ParsingContextFlags,
    /// A list of labels for labelled statements used to report undefined label errors
    /// for break and continue, as well as duplicate labels.
    /// Often called label set in the spec.
    label_set: LabelSet,
    /// Whether we are in strict mode code
    strict: Option<StrictMode>,
    /// The exported default item, used for checking duplicate defaults
    pub default_item: Option<ExportDefaultItem>,
    /// If set, the parser reports bindings with identical names. The option stores the name of the
    /// node that disallows duplicate bindings, for example `let`, `const` or `import`.
    pub duplicate_binding_parent: Option<&'static str>,
    pub name_map: IndexMap<String, Range<usize>>,

    /// Indicates that the parser is speculatively parsing a syntax. Speculative parsing means that the
    /// parser tries to parse a syntax as one kind and determines at the end if the assumption was right
    /// by testing if the parser is at a specific token (or has no errors). For this approach to work,
    /// the parser isn't allowed to skip any tokens while doing error recovery because it may then successfully
    /// skip over all invalid tokens, so that it appears as if it was able to parse the syntax correctly.
    ///
    /// Speculative parsing is useful if a syntax is ambiguous and no amount of lookahead (except parsing the whole syntax)
    /// is sufficient to determine what syntax it is. For example, the syntax `(a, b) ...`
    /// in JavaScript is either a parenthesized expression or an arrow expression if `...` is a `=>`.
    /// The challenge is, that it isn't possible to tell which of the two kinds it is until the parser
    /// processed all of `(a, b)`.
    pub(crate) speculative_parsing: bool,

    /// Stores the token positions of all syntax that looks like an arrow expressions but aren't one.
    /// Optimization to reduce the back-tracking required when parsing parenthesized and arrow function expressions.
    pub(crate) not_parenthesized_arrow: HashSet<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StrictMode {
    Module,
    Explicit(Range<usize>),
    Class(Range<usize>),
}

impl ParserState {
    pub fn new(source_type: &SourceType) -> Self {
        let mut state = ParserState {
            parsing_context: ParsingContextFlags::TOP_LEVEL,
            label_set: IndexMap::new(),
            strict: if source_type.module_kind().is_module() {
                Some(StrictMode::Module)
            } else {
                None
            },
            default_item: None,
            name_map: IndexMap::new(),
            duplicate_binding_parent: None,
            not_parenthesized_arrow: Default::default(),
            speculative_parsing: false,
        };

        if source_type.module_kind().is_module() {
            state.parsing_context |= ParsingContextFlags::IN_ASYNC
        }

        if matches!(
            source_type.language(),
            Language::TS {
                definition_file: true
            }
        ) {
            state.parsing_context |= ParsingContextFlags::AMBIENT_CONTEXT
        }

        state
    }

    pub fn in_function(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::IN_FUNCTION)
    }

    pub fn in_generator(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::IN_GENERATOR)
    }

    pub fn in_async(&self) -> bool {
        self.parsing_context.contains(ParsingContextFlags::IN_ASYNC)
    }

    pub fn in_ambient_context(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::AMBIENT_CONTEXT)
    }

    pub fn in_constructor(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::IN_CONSTRUCTOR)
    }

    pub fn is_top_level(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::TOP_LEVEL)
    }

    pub fn continue_allowed(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::CONTINUE_ALLOWED)
    }
    pub fn break_allowed(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::BREAK_ALLOWED)
    }

    pub fn strict(&self) -> Option<&StrictMode> {
        self.strict.as_ref()
    }

    pub fn get_labelled_item(&self, label: &str) -> Option<&LabelledItem> {
        self.label_set.get(label)
    }

    pub(super) fn checkpoint(&self) -> ParserStateCheckpoint {
        ParserStateCheckpoint::snapshot(self)
    }

    pub(super) fn restore(&mut self, checkpoint: ParserStateCheckpoint) {
        checkpoint.rewind(self);
    }
}

/// Stores a checkpoint of the [ParserState].
/// Allows rewinding the state to its previous state.
///
/// It's important that creating and rewinding a snapshot is cheap. Consider the performance implications
/// before adding new unscoped state.
#[derive(Debug)]
pub(super) struct ParserStateCheckpoint {
    /// Additional data that we only want to store in debug mode
    #[cfg(debug_assertions)]
    debug_checkpoint: DebugParserStateCheckpoint,
}

impl ParserStateCheckpoint {
    /// Creates a snapshot of the passed in state.
    #[cfg(debug_assertions)]
    fn snapshot(state: &ParserState) -> Self {
        Self {
            debug_checkpoint: DebugParserStateCheckpoint::snapshot(state),
        }
    }

    #[cfg(not(debug_assertions))]
    fn snapshot(_: &ParserState) -> Self {
        Self {}
    }

    /// Restores the `state values` to the time when this snapshot was created.
    #[cfg(debug_assertions)]
    fn rewind(self, state: &mut ParserState) {
        self.debug_checkpoint.rewind(state);
    }

    #[cfg(not(debug_assertions))]
    fn rewind(self, _: &ParserState) {}
}

/// Most of the [ParserState] is scoped state. It should, therefore, not be necessary to rewind
/// that state because that's already taken care of by `with_state` and `with_scoped_state`.
/// But, you can never no and better be safe than sorry. That's why we use some heuristics
/// to verify that non of the scoped state did change and assert for it when rewinding.
#[derive(Debug, Clone)]
#[cfg(debug_assertions)]
pub(super) struct DebugParserStateCheckpoint {
    parsing_context: ParsingContextFlags,
    label_set_len: usize,
    strict: Option<StrictMode>,
    default_item: Option<ExportDefaultItem>,
    duplicate_binding_parent: Option<&'static str>,
    name_map_len: usize,
}

#[cfg(debug_assertions)]
impl DebugParserStateCheckpoint {
    fn snapshot(state: &ParserState) -> Self {
        Self {
            parsing_context: state.parsing_context,
            label_set_len: state.label_set.len(),
            strict: state.strict.clone(),
            default_item: state.default_item.clone(),
            duplicate_binding_parent: state.duplicate_binding_parent,
            name_map_len: state.name_map.len(),
        }
    }

    fn rewind(self, state: &mut ParserState) {
        assert_eq!(state.parsing_context, self.parsing_context);
        assert_eq!(state.label_set.len(), self.label_set_len);
        assert_eq!(state.strict, self.strict);
        assert_eq!(state.default_item, self.default_item);
        assert_eq!(
            state.duplicate_binding_parent,
            self.duplicate_binding_parent
        );
        assert_eq!(state.name_map.len(), self.name_map_len);
    }
}

impl<'t> Parser<'t> {
    /// Applies the passed in change to the parser's state and reverts the
    /// changes when the returned [ParserStateGuard] goes out of scope.
    pub(crate) fn with_scoped_state<'p, C: ChangeParserState>(
        &'p mut self,
        change: C,
    ) -> ParserStateGuard<'p, 't, C> {
        let snapshot = change.apply(&mut self.state);
        ParserStateGuard::new(self, snapshot)
    }

    /// Applies the passed in change to the parser state before applying the passed `func` and
    /// restores the state to before the change before returning the result.
    #[inline]
    pub(crate) fn with_state<C, F, R>(&mut self, change: C, func: F) -> R
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
pub(crate) struct ParserStateGuard<'parser, 't, C>
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
pub(crate) trait ChangeParserState {
    type Snapshot: Default;

    /// Applies the change to the passed in state and returns snapshot that allows restoring the previous state.
    fn apply(self, state: &mut ParserState) -> Self::Snapshot;

    /// Restores the state to its previous value
    fn restore(state: &mut ParserState, value: Self::Snapshot);
}

#[derive(Default, Debug)]
pub struct EnableStrictModeSnapshot(Option<StrictMode>);

/// Enables strict mode
pub(crate) struct EnableStrictMode(pub StrictMode);

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

bitflags! {
    /// Flags describing the context of a function.
    pub(crate) struct SignatureFlags: u8 {
        /// Is the function in an async context
        const ASYNC 		= 1 << 0;
        /// Is the function in a generator context
        const GENERATOR 	= 1 << 1;
        /// Is the function a constructor (or constructor context)
        const CONSTRUCTOR 	= 1 << 2;
    }
}

impl From<SignatureFlags> for ParsingContextFlags {
    fn from(flags: SignatureFlags) -> Self {
        let mut parsing_context = ParsingContextFlags::empty();

        if flags.contains(SignatureFlags::ASYNC) {
            parsing_context |= ParsingContextFlags::IN_ASYNC;
        }

        if flags.contains(SignatureFlags::GENERATOR) {
            parsing_context |= ParsingContextFlags::IN_GENERATOR;
        }

        if flags.contains(SignatureFlags::CONSTRUCTOR) {
            parsing_context |= ParsingContextFlags::IN_CONSTRUCTOR;
        }

        parsing_context
    }
}

bitflags! {
    /// Flags representing the parsing state.
    /// The reasons to use flags instead of individual boolean fields on `ParserState` are:
    /// * It's possible to use bit masks to define what state should be inherited. For example,
    ///   functions inherit whether they're defined inside a parameter but override the `in_async` flag
    /// * It's easier to snapshot the previous state. Individual boolean fields would require that a change
    ///   snapshots each individual boolean field to allow restoring the previous state. With bitflags, all that
    ///   is needed is to copy away the flags field and restore it after.
    #[derive(Default)]
    pub(crate) struct ParsingContextFlags: u8 {
        /// Whether the parser is in a generator function like `function* a() {}`
        /// Matches the `Yield` parameter in the ECMA spec
        const IN_GENERATOR = 1 << 0;
        /// Whether the parser is inside a function
        const IN_FUNCTION = 1 << 1;
        /// Whatever the parser is inside a constructor
        const IN_CONSTRUCTOR = 1 << 2;

        /// Is async allowed in this context. Either because it's an async function or top level await is supported.
        /// Equivalent to the `Async` generator in the ECMA spec
        const IN_ASYNC = 1 << 3;

        /// Whether the parser is parsing a top-level statement (not inside a class, function, parameter) or not
        const TOP_LEVEL = 1 << 4;

        /// Whether the parser is in an iteration or switch statement and
        /// `break` is allowed.
        const BREAK_ALLOWED = 1 << 5;

        /// Whether the parser is in an iteration statement and `continue` is allowed.
        const CONTINUE_ALLOWED = 1 << 6;

        /// Whatever the parser is in a TypeScript ambient context
        const AMBIENT_CONTEXT = 1 << 7;

        const LOOP = Self::BREAK_ALLOWED.bits | Self::CONTINUE_ALLOWED.bits;

        /// Bitmask of all the flags that must be reset (shouldn't be inherited) when the parser enters a function
        const FUNCTION_RESET_MASK = Self::BREAK_ALLOWED.bits | Self::CONTINUE_ALLOWED.bits | Self::IN_CONSTRUCTOR.bits | Self::IN_ASYNC.bits | Self::IN_GENERATOR.bits | Self::TOP_LEVEL.bits;

        /// Bitmask of all the flags that must be reset (shouldn't be inherited) when entering parameters.
        const PARAMETER_RESET_MASK = Self::IN_CONSTRUCTOR.bits | Self::IN_FUNCTION.bits | Self::TOP_LEVEL.bits | Self::IN_GENERATOR.bits | Self::IN_ASYNC.bits;
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct ParsingContextFlagsSnapshot(ParsingContextFlags);

pub(crate) trait ChangeParserStateFlags {
    fn compute_new_flags(&self, existing: ParsingContextFlags) -> ParsingContextFlags;
}

impl<T: ChangeParserStateFlags> ChangeParserState for T {
    type Snapshot = ParsingContextFlagsSnapshot;

    fn apply(self, state: &mut ParserState) -> Self::Snapshot {
        let new_flags = self.compute_new_flags(state.parsing_context);
        ParsingContextFlagsSnapshot(std::mem::replace(&mut state.parsing_context, new_flags))
    }

    fn restore(state: &mut ParserState, value: Self::Snapshot) {
        state.parsing_context = value.0
    }
}

/// Enters the parsing of function/method parameters
pub(crate) struct EnterParameters(
    /// Whether async and yield are reserved keywords
    pub(crate) SignatureFlags,
);

impl ChangeParserStateFlags for EnterParameters {
    fn compute_new_flags(&self, existing: ParsingContextFlags) -> ParsingContextFlags {
        (existing - ParsingContextFlags::PARAMETER_RESET_MASK) | ParsingContextFlags::from(self.0)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum BreakableKind {
    // Iteration statement like Do, While, For
    Iteration,

    // Switch statement
    Switch,
}

pub(crate) struct EnterBreakable(pub(crate) BreakableKind);

impl ChangeParserStateFlags for EnterBreakable {
    fn compute_new_flags(&self, existing: ParsingContextFlags) -> ParsingContextFlags {
        let mut flags = existing | ParsingContextFlags::BREAK_ALLOWED;

        if self.0 == BreakableKind::Iteration {
            flags |= ParsingContextFlags::CONTINUE_ALLOWED;
        }

        flags
    }
}

#[derive(Debug, Clone, Default)]
pub struct EnterFunctionSnapshot {
    parsing_context: ParsingContextFlags,
    label_set: LabelSet,
}

/// Enters the parsing of a function/method. Resets the relevant parser state and sets the state
/// according to the passed [SignatureFlags]
pub(crate) struct EnterFunction(pub(crate) SignatureFlags);

impl ChangeParserState for EnterFunction {
    type Snapshot = EnterFunctionSnapshot;

    #[inline]
    fn apply(self, state: &mut ParserState) -> Self::Snapshot {
        let new_flags = (state.parsing_context - ParsingContextFlags::FUNCTION_RESET_MASK)
            | ParsingContextFlags::IN_FUNCTION
            | ParsingContextFlags::from(self.0);

        EnterFunctionSnapshot {
            parsing_context: std::mem::replace(&mut state.parsing_context, new_flags),
            label_set: std::mem::take(&mut state.label_set),
        }
    }

    #[inline]
    fn restore(state: &mut ParserState, value: Self::Snapshot) {
        state.parsing_context = value.parsing_context;
        state.label_set = value.label_set;
    }
}

pub(crate) struct EnterClassPropertyInitializer;

impl ChangeParserStateFlags for EnterClassPropertyInitializer {
    fn compute_new_flags(&self, existing: ParsingContextFlags) -> ParsingContextFlags {
        existing
            - ParsingContextFlags::TOP_LEVEL
            - ParsingContextFlags::IN_ASYNC
            - ParsingContextFlags::IN_GENERATOR
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct EnterClassStaticInitializationBlockSnapshot {
    label_set: LabelSet,
    flags: ParsingContextFlags,
}

pub(crate) struct EnterClassStaticInitializationBlock;

impl ChangeParserState for EnterClassStaticInitializationBlock {
    type Snapshot = EnterClassStaticInitializationBlockSnapshot;

    fn apply(self, state: &mut ParserState) -> Self::Snapshot {
        let flags = (state.parsing_context
            - ParsingContextFlags::FUNCTION_RESET_MASK
            - ParsingContextFlags::IN_FUNCTION)
            | ParsingContextFlags::IN_ASYNC; // allow async for better error recovery
        EnterClassStaticInitializationBlockSnapshot {
            flags: std::mem::replace(&mut state.parsing_context, flags),
            label_set: std::mem::take(&mut state.label_set),
        }
    }

    fn restore(state: &mut ParserState, value: Self::Snapshot) {
        state.parsing_context = value.flags;
        state.label_set = value.label_set;
    }
}

#[derive(Debug, Default)]
pub(crate) struct WithLabelSnapshot {
    #[cfg(debug_assertions)]
    label_set_len: usize,
}

/// Adds the labelled item with the given label to the `label_set`.
/// Removes the label when the change is undone.
pub(crate) struct WithLabel(pub String, pub LabelledItem);

impl ChangeParserState for WithLabel {
    type Snapshot = WithLabelSnapshot;

    fn apply(self, state: &mut ParserState) -> Self::Snapshot {
        #[cfg(debug_assertions)]
        let previous_len = state.label_set.len();
        state.label_set.insert(self.0, self.1);
        WithLabelSnapshot {
            // Capturing the len is sufficient because:
            // * The labels are stored in an index map that uses insertion-order
            // * Labels are scoped and new labels are always appended to the end of the list
            #[cfg(debug_assertions)]
            label_set_len: previous_len,
        }
    }

    #[cfg(not(debug_assertions))]
    fn restore(state: &mut ParserState, _: Self::Snapshot) {
        state.label_set.pop();
    }

    #[cfg(debug_assertions)]
    fn restore(state: &mut ParserState, value: Self::Snapshot) {
        assert_eq!(state.label_set.len(), value.label_set_len + 1);
        state.label_set.pop();
    }
}

/// Sets the state changes needed when parsing a TS type declaration (async and await are not reserved identifiers)
pub(crate) struct EnterType;

impl ChangeParserStateFlags for EnterType {
    fn compute_new_flags(&self, existing: ParsingContextFlags) -> ParsingContextFlags {
        existing - ParsingContextFlags::IN_ASYNC - ParsingContextFlags::IN_GENERATOR
    }
}

#[derive(Default)]
pub(crate) struct EnterAmbientContextSnapshot {
    flags: ParsingContextFlags,
    default_item: Option<ExportDefaultItem>,
    strict_mode: Option<StrictMode>,
}

pub(crate) struct EnterAmbientContext;

impl ChangeParserState for EnterAmbientContext {
    type Snapshot = EnterAmbientContextSnapshot;

    fn apply(self, state: &mut ParserState) -> Self::Snapshot {
        let new_flags = state.parsing_context | ParsingContextFlags::AMBIENT_CONTEXT;
        EnterAmbientContextSnapshot {
            flags: std::mem::replace(&mut state.parsing_context, new_flags),
            default_item: state.default_item.take(),
            strict_mode: state.strict.take(),
        }
    }

    fn restore(state: &mut ParserState, value: Self::Snapshot) {
        state.parsing_context = value.flags;
        state.default_item = value.default_item;
        state.strict = value.strict_mode;
    }
}
