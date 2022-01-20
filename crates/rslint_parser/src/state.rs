use crate::{FileKind, Parser, Syntax};
use bitflags::bitflags;
use indexmap::IndexMap;
use std::fmt::Formatter;
use std::ops::{Deref, DerefMut, Range};

type LabelSet = IndexMap<String, LabelledItem>;
type HoistedNames = IndexMap<String, Range<usize>>;
type LexicalNames = IndexMap<String, Range<usize>>;

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
    pub default_item: Option<Range<usize>>,
    pub(crate) no_recovery: bool,
    /// If set, the parser reports bindings with identical names. The option stores the name of the
    /// node that disallows duplicate bindings, for example `let`, `const` or `import`.
    binding_context: BindingContext,
    /// Tracks variables defined as lexical (let, const)
    lexical_names: LexicalNames,
    /// Tracks variables defined as hoisted (var)
    hoisted_names: HoistedNames,
}

/// Used to track the different kind of bindings.
/// Depending of the kind of binding, the variables will tracked in a different environment
#[derive(Debug, Clone, PartialEq)]
pub enum BindingContext {
    /// The default context of a JavaScript file
    Hoisted,
    /// For generic blocks  { }
    Block,
    /// A block of a function, which applies different rules compared to [BindingContext::Block]
    FunctionBlock,
    /// Track statements that involve a loop such as `for` loops
    IterationStatement,
    /// Function arguments
    Parameters,
}

impl Default for BindingContext {
    fn default() -> Self {
        Self::Hoisted
    }
}

/// Identifies the types of bindings encountered
#[derive(Debug, Clone, PartialEq)]
pub enum NameType {
    /// Bindings for functions
    Function,
    Hoisted,
    Lexical(LexicalType),
}

impl From<&NameType> for NameType {
    fn from(n: &NameType) -> Self {
        match n {
            NameType::Function => NameType::Function,
            NameType::Hoisted => NameType::Hoisted,
            NameType::Lexical(l) => NameType::Lexical(l.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum HoistedType {
    Var,
    Parameter,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexicalType {
    Import,
    Let,
    Const,
}

impl From<&LexicalType> for LexicalType {
    fn from(l: &LexicalType) -> Self {
        match l {
            LexicalType::Let => LexicalType::Let,
            LexicalType::Const => LexicalType::Const,
            LexicalType::Import => LexicalType::Import,
        }
    }
}

impl std::fmt::Display for LexicalType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexicalType::Let => write!(f, "let"),
            LexicalType::Const => write!(f, "const"),
            LexicalType::Import => write!(f, "import"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StrictMode {
    Module,
    Explicit(Range<usize>),
    Class(Range<usize>),
}

impl Default for ParserState {
    fn default() -> Self {
        ParserState::new(Syntax::default())
    }
}

impl ParserState {
    pub fn new(syntax: Syntax) -> Self {
        let mut state = ParserState {
            parsing_context: ParsingContextFlags::ALLOW_OBJECT_EXPRESSION
                | ParsingContextFlags::INCLUDE_IN
                | ParsingContextFlags::TOP_LEVEL,
            label_set: IndexMap::new(),
            strict: if syntax.file_kind == FileKind::Module {
                Some(StrictMode::Module)
            } else {
                None
            },
            default_item: None,
            binding_context: BindingContext::default(),
            no_recovery: false,
            lexical_names: IndexMap::new(),
            hoisted_names: IndexMap::new(),
        };

        if syntax.top_level_await {
            state.parsing_context |= ParsingContextFlags::IN_ASYNC
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

    pub fn include_in(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::INCLUDE_IN)
    }

    pub fn in_condition_expression(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::IN_CONDITION_EXPRESSION)
    }

    pub fn potential_arrow_start(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::POTENTIAL_ARROW_START)
    }

    pub fn allow_object_expression(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::ALLOW_OBJECT_EXPRESSION)
    }

    pub fn strict(&self) -> Option<&StrictMode> {
        self.strict.as_ref()
    }

    pub fn binding_context(&self) -> &BindingContext {
        &self.binding_context
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

    pub fn in_function_block(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::IN_FUNCTION_BLOCK)
    }

    /// Checks if a binding has been already registered
    pub fn clashes_with_defined_name(
        &self,
        identifier_name: &str,
        name_type: &Option<NameType>,
    ) -> Option<&Range<usize>> {
        match self.binding_context() {
            // hoisted variables can be redeclared without problems
            // which means that we only need to check the variables the lexical environment
            BindingContext::Block => {
                if let Some(name_type) = name_type {
                    match name_type {
                        // Here we want to cover this particular case:
                        // Example:
                        NameType::Hoisted => self
                            .strict()
                            .and_then(|_| self.lexical_names.get(identifier_name)),

                        NameType::Function => {
                            // At the top level of a function or script, inner function declarations are treated like var declarations.
                            //
                            // https://tc39.es/ecma262/multipage/syntax-directed-operations.html#sec-static-semantics-toplevelvardeclarednames
                            if self.in_function_block() || self.strict().is_none() {
                                self.lexical_names.get(identifier_name)
                            } else {
                                self.lexical_names
                                    .get(identifier_name)
                                    .or_else(|| self.hoisted_names.get(identifier_name))
                            }
                        }

                        NameType::Lexical(_) => self.lexical_names.get(identifier_name),
                    }
                } else {
                    None
                }
            }
            BindingContext::Parameters => self
                .strict()
                .and_then(|_| self.hoisted_names.get(identifier_name)),
            BindingContext::Hoisted => match name_type {
                Some(name_type) => match name_type {
                    NameType::Hoisted | NameType::Function => self
                        .strict()
                        .and_then(|_| self.lexical_names.get(identifier_name)),
                    NameType::Lexical(_) => self
                        .hoisted_names
                        .get(identifier_name)
                        .or_else(|| self.lexical_names.get(identifier_name)),
                },
                _ => None,
            },
            BindingContext::FunctionBlock => {
                if let Some(binding_type) = name_type {
                    match binding_type {
                        // At the top level of a function, or script, function declarations are treated
                        // like var declarations rather than like lexical declarations.
                        //
                        // https://tc39.es/ecma262/multipage/syntax-directed-operations.html#sec-static-semantics-toplevellexicallydeclarednames
                        NameType::Hoisted | NameType::Function => {
                            self.lexical_names.get(identifier_name)
                        }

                        NameType::Lexical(_) => self
                            .lexical_names
                            .get(identifier_name)
                            .or_else(|| self.hoisted_names.get(identifier_name)),
                    }
                } else {
                    None
                }
            }

            _ => match name_type {
                None => self
                    .strict()
                    .and_then(|_| self.lexical_names.get(identifier_name)),
                Some(binding_type) => match binding_type {
                    NameType::Hoisted | NameType::Function => self
                        .strict()
                        .and_then(|_| self.lexical_names.get(identifier_name)),
                    NameType::Lexical(_) => self
                        .hoisted_names
                        .get(identifier_name)
                        .or_else(|| self.lexical_names.get(identifier_name)),
                },
            },
        }
    }

    /// It registers the name of a binding based in its name type (lexical or hoisted) and its current binding context
    pub fn register_name(
        &mut self,
        identifier_name: String,
        range: Range<usize>,
        name_type: Option<NameType>,
    ) {
        match self.binding_context() {
            BindingContext::Hoisted => {
                match name_type {
                    Some(name_type) => match name_type {
                        NameType::Hoisted => {
                            self.hoisted_names.insert(identifier_name, range);
                        }
                        NameType::Lexical(_) => {
                            self.lexical_names.insert(identifier_name, range);
                        }

                        NameType::Function => {
                            self.hoisted_names.insert(identifier_name, range);
                        }
                    },
                    // cases where variables don't have a binding variable "var", they are
                    // assigned to the hoisted environment
                    None => {
                        self.hoisted_names.insert(identifier_name, range);
                    }
                }
            }
            BindingContext::Block => {
                match name_type {
                    Some(binding_type) => match binding_type {
                        NameType::Hoisted => {
                            self.hoisted_names.insert(identifier_name, range);
                        }
                        NameType::Lexical(_) => {
                            self.lexical_names.insert(identifier_name, range);
                        }

                        NameType::Function => {
                            // At the top level of a function or script, inner function declarations are treated like var declarations.
                            //
                            // https://tc39.es/ecma262/multipage/syntax-directed-operations.html#sec-static-semantics-toplevelvardeclarednames
                            if self.in_function_block() || self.strict().is_none() {
                                self.hoisted_names.insert(identifier_name, range);
                            } else {
                                self.lexical_names.insert(identifier_name, range);
                            }
                        }
                    },
                    // cases where variables don't have a binding variable "var", they are
                    // assigned to the lexical environment
                    None => {
                        self.lexical_names.insert(identifier_name, range);
                    }
                }
            }
            BindingContext::FunctionBlock => {
                match name_type {
                    Some(binding_type) => match binding_type {
                        NameType::Hoisted => {
                            self.hoisted_names.insert(identifier_name, range);
                        }
                        NameType::Lexical(_) => {
                            self.lexical_names.insert(identifier_name, range);
                        }

                        // At the top level of a function, or script, function declarations are treated
                        // like var declarations rather than like lexical declarations.
                        //
                        // https://tc39.es/ecma262/multipage/syntax-directed-operations.html#sec-static-semantics-toplevellexicallydeclarednames
                        NameType::Function => {
                            self.hoisted_names.insert(identifier_name, range);
                        }
                    },
                    // cases where variables don't have a binding variable "var", they are
                    // assigned to the lexical environment
                    None => {
                        self.lexical_names.insert(identifier_name, range);
                    }
                }
            }
            BindingContext::IterationStatement | BindingContext::Parameters => {
                self.hoisted_names.insert(identifier_name, range);
            }
        }
    }
}

/// Stores a checkpoint of the [ParserState].
/// Allows rewinding the state to its previous state.
///
/// It's important that creating and rewinding a snapshot is cheap. Consider the performance implications
/// before adding new unscoped state.
#[derive(Debug, Clone)]
pub(super) struct ParserStateCheckpoint {
    /// Additional data that we only want to store in debug mode
    #[cfg(debug_assertions)]
    debug_checkpoint: DebugParserStateCheckpoint,
    lexical_names_len: usize,
    hoisted_names_len: usize,
}

impl ParserStateCheckpoint {
    /// Creates a snapshot of the passed in state.
    #[cfg(debug_assertions)]
    fn snapshot(state: &ParserState) -> Self {
        Self {
            lexical_names_len: state.lexical_names.len(),
            hoisted_names_len: state.hoisted_names.len(),
            debug_checkpoint: DebugParserStateCheckpoint::snapshot(state),
        }
    }

    #[cfg(not(debug_assertions))]
    fn snapshot(state: &ParserState) -> Self {
        Self {
            lexical_names_len: state.lexical_names.len(),
            hoisted_names_len: state.hoisted_names.len(),
        }
    }

    /// Restores the `state values` to the time when this snapshot was created.
    #[cfg(debug_assertions)]
    fn rewind(self, state: &mut ParserState) {
        self.rewind_impl(state);
        self.debug_checkpoint.rewind(state);
    }

    #[cfg(not(debug_assertions))]
    fn rewind(self, state: &mut ParserState) {
        self.rewind_impl(state);
    }

    fn rewind_impl(&self, state: &mut ParserState) {
        state.lexical_names.truncate(self.lexical_names_len);
        state.hoisted_names.truncate(self.hoisted_names_len);
    }
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
    default_item: Option<Range<usize>>,
}

#[cfg(debug_assertions)]
impl DebugParserStateCheckpoint {
    fn snapshot(state: &ParserState) -> Self {
        Self {
            parsing_context: state.parsing_context,
            label_set_len: state.label_set.len(),
            strict: state.strict.clone(),
            default_item: state.default_item.clone(),
        }
    }

    fn rewind(self, state: &mut ParserState) {
        assert_eq!(state.parsing_context, self.parsing_context);
        assert_eq!(state.label_set.len(), self.label_set_len);
        assert_eq!(state.strict, self.strict);
        assert_eq!(state.default_item, self.default_item);
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

/// Macro for creating a [ChangeParserState] that changes the value of a single [ParserState] field.
/// * `$name`: The name of the [ChangeParserState] implementation
/// * `$field`: The [ParserState] field's name that the implementation *changes*
/// * `$type`: The [ParserState] field's type
/// * `snapshot`: The name of the snapshot struct
macro_rules! gen_change_parser_state {
    ($name:ident, $flag:expr) => {
        /// Changes the [ParserState] `$field` field
        #[derive(Debug)]
        pub(crate) struct $name(pub(crate) bool);

        impl ChangeParserState for $name {
            type Snapshot = ParsingContextFlagsSnapshot;

            #[inline]
            fn apply(self, state: &mut ParserState) -> Self::Snapshot {
                let new_flags = if self.0 {
                    state.parsing_context | $flag
                } else {
                    state.parsing_context - $flag
                };
                ParsingContextFlagsSnapshot(std::mem::replace(
                    &mut state.parsing_context,
                    new_flags,
                ))
            }

            #[inline]
            fn restore(state: &mut ParserState, value: Self::Snapshot) {
                state.parsing_context = value.0
            }
        }
    };
}

gen_change_parser_state!(IncludeIn, ParsingContextFlags::INCLUDE_IN);
gen_change_parser_state!(
    InConditionExpression,
    ParsingContextFlags::IN_CONDITION_EXPRESSION
);
gen_change_parser_state!(
    AllowObjectExpression,
    ParsingContextFlags::ALLOW_OBJECT_EXPRESSION
);
gen_change_parser_state!(
    PotentialArrowStart,
    ParsingContextFlags::POTENTIAL_ARROW_START
);

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
    struct ParsingContextFlags: u16 {
        /// Whether the parser is in a generator function like `function* a() {}`
        /// Matches the `Yield` parameter in the ECMA spec
        const IN_GENERATOR = 1 << 0;
        /// Whether the parser is inside a function
        const IN_FUNCTION = 1 << 2;
        /// Whatever the parser is inside a constructor
        const IN_CONSTRUCTOR = 1 << 3;

        /// Is async allowed in this context. Either because it's an async function or top level await is supported.
        /// Equivalent to the `Async` generator in the ECMA spec
        const IN_ASYNC = 1 << 4;

        /// Whether the parser is parsing a top-level statement (not inside a class, function, parameter) or not
        const TOP_LEVEL = 1 << 5;

        /// Whether `in` should be counted in a binary expression
        /// this is for `for...in` statements to prevent ambiguity.
        const INCLUDE_IN = 1 << 6;
        /// Whether the parser is in a conditional expr (ternary expr)
        const IN_CONDITION_EXPRESSION = 1 << 7;

        /// Whether the parser is in an iteration or switch statement and
        /// `break` is allowed.
        const BREAK_ALLOWED = 1 << 8;

        /// Whether the parser is in an iteration statement and `continue` is allowed.
        const CONTINUE_ALLOWED = 1 << 9;

        /// If false, object expressions are not allowed to be parsed
        /// inside an expression.
        ///
        /// Also applies for object patterns
        const ALLOW_OBJECT_EXPRESSION = 1 << 10;

        /// Whether we potentially are in a place to parse an arrow expression
        const POTENTIAL_ARROW_START = 1 << 11;

        /// Whether the parser is inside a function body
        const IN_FUNCTION_BLOCK = 1 << 12;

        const LOOP = Self::BREAK_ALLOWED.bits | Self::CONTINUE_ALLOWED.bits;

        /// Bitmask of all the flags that must be reset (shouldn't be inherited) when the parser enters a function
        const FUNCTION_RESET_MASK = Self::BREAK_ALLOWED.bits | Self::CONTINUE_ALLOWED.bits | Self::IN_CONSTRUCTOR.bits | Self::IN_ASYNC.bits | Self::IN_GENERATOR.bits | Self::TOP_LEVEL.bits;

        /// Bitmask of all the flags that must be reset (shouldn't be inherited) when entering parameters.
        const PARAMETER_RESET_MASK = Self::IN_CONSTRUCTOR.bits | Self::IN_FUNCTION.bits | Self::TOP_LEVEL.bits | Self::IN_GENERATOR.bits | Self::IN_ASYNC.bits;
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct ParsingContextFlagsSnapshot(ParsingContextFlags);

/// Enters the parsing of function/method parameters
pub(crate) struct EnterParameters {
    /// Whether async and yield are reserved keywords
    pub(crate) signature_flags: SignatureFlags,

    /// Whether an object expression is valid
    pub(crate) allow_object_expressions: bool,
}

impl ChangeParserState for EnterParameters {
    type Snapshot = ParsingContextFlagsSnapshot;

    fn apply(self, state: &mut ParserState) -> Self::Snapshot {
        let mut flags = (state.parsing_context - ParsingContextFlags::PARAMETER_RESET_MASK)
            | ParsingContextFlags::from(self.signature_flags);

        if self.allow_object_expressions {
            flags |= ParsingContextFlags::ALLOW_OBJECT_EXPRESSION
        } else {
            flags -= ParsingContextFlags::ALLOW_OBJECT_EXPRESSION;
        }

        ParsingContextFlagsSnapshot(std::mem::replace(&mut state.parsing_context, flags))
    }

    fn restore(state: &mut ParserState, value: Self::Snapshot) {
        state.parsing_context = value.0;
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

impl ChangeParserState for EnterBreakable {
    type Snapshot = ParsingContextFlagsSnapshot;

    fn apply(self, state: &mut ParserState) -> Self::Snapshot {
        let mut flags = state.parsing_context | ParsingContextFlags::BREAK_ALLOWED;

        if self.0 == BreakableKind::Iteration {
            flags |= ParsingContextFlags::CONTINUE_ALLOWED;
        }

        ParsingContextFlagsSnapshot(std::mem::replace(&mut state.parsing_context, flags))
    }

    fn restore(state: &mut ParserState, value: Self::Snapshot) {
        state.parsing_context = value.0;
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

impl ChangeParserState for EnterClassPropertyInitializer {
    type Snapshot = ParsingContextFlagsSnapshot;

    fn apply(self, state: &mut ParserState) -> Self::Snapshot {
        let flags = state.parsing_context
            - ParsingContextFlags::TOP_LEVEL
            - ParsingContextFlags::IN_ASYNC
            - ParsingContextFlags::IN_GENERATOR;
        ParsingContextFlagsSnapshot(std::mem::replace(&mut state.parsing_context, flags))
    }

    fn restore(state: &mut ParserState, value: Self::Snapshot) {
        state.parsing_context = value.0;
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct EnterClassStaticInitializationBlockSnapshot {
    label_set: LabelSet,
    flags: ParsingContextFlags,
    binding_context: BindingContext,
    lexical_names: LexicalNames,
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
            binding_context: BindingContext::Block,
            lexical_names: std::mem::take(&mut state.lexical_names),
        }
    }

    fn restore(state: &mut ParserState, value: Self::Snapshot) {
        state.parsing_context = value.flags;
        state.label_set = value.label_set;
        state.binding_context = value.binding_context;
        state.lexical_names = value.lexical_names;
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

#[derive(Default, Debug, Clone)]
pub struct EnterHoistedScopeSnapshot {
    scoped_variables: LexicalNames,
    binding_context: BindingContext,
    hoisted_names: HoistedNames,
}

/// Use this to track when a parser enters a scope where hoisted variables are bound
/// to this new [BindingContext]
pub(crate) struct EnterHoistedScope(pub(crate) BindingContext);

impl ChangeParserState for EnterHoistedScope {
    type Snapshot = EnterHoistedScopeSnapshot;

    fn apply(self, state: &mut ParserState) -> Self::Snapshot {
        EnterHoistedScopeSnapshot {
            scoped_variables: std::mem::take(&mut state.lexical_names),
            binding_context: std::mem::replace(&mut state.binding_context, self.0),
            hoisted_names: std::mem::take(&mut state.hoisted_names),
        }
    }

    fn restore(state: &mut ParserState, value: Self::Snapshot) {
        state.lexical_names = value.scoped_variables;
        state.binding_context = value.binding_context;
        state.hoisted_names = value.hoisted_names;
    }
}

#[derive(Default, Debug, Clone)]
pub struct EnterLexicalScopeSnapshot {
    lexical_names: LexicalNames,
    binding_context: BindingContext,
    flags: ParsingContextFlags,
}

/// Use this action to track when the parser enters inside a new lexical scope
pub(crate) struct EnterLexicalScope(pub(crate) BindingContext);

impl ChangeParserState for EnterLexicalScope {
    type Snapshot = EnterLexicalScopeSnapshot;

    fn apply(self, state: &mut ParserState) -> Self::Snapshot {
        let flags = if self.0 == BindingContext::FunctionBlock {
            state.parsing_context | ParsingContextFlags::IN_FUNCTION_BLOCK
        } else {
            state.parsing_context
        };
        EnterLexicalScopeSnapshot {
            flags: std::mem::replace(&mut state.parsing_context, flags),
            lexical_names: std::mem::take(&mut state.lexical_names),
            binding_context: std::mem::replace(&mut state.binding_context, self.0),
        }
    }

    fn restore(state: &mut ParserState, value: Self::Snapshot) {
        state.lexical_names = value.lexical_names;
        state.binding_context = value.binding_context;
        state.parsing_context = value.flags;
    }
}
