use crate::{FileKind, Parser, Syntax};
use bitflags::bitflags;
use std::collections::HashMap;
use std::fmt::Formatter;

use std::ops::{Deref, DerefMut, Range};

type LabelSet = HashMap<String, LabelledItem>;
type HoistedNames = HashMap<String, Range<usize>>;
type LexicalNames = HashMap<String, Range<usize>>;

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
pub struct ParserState {
    parsing_context: ParsingContextFlags,
    /// A list of labels for labelled statements used to report undefined label errors
    /// for break and continue, as well as duplicate labels.
    /// Often called label set in the spec.
    pub(crate) label_set: LabelSet,
    /// Whether we are in strict mode code
    strict: Option<StrictMode>,
    /// The exported default item, used for checking duplicate defaults
    pub default_item: Option<Range<usize>>,
    pub(crate) no_recovery: bool,
    /// Tracks the binding variable inside a declaration list: "var a, b, c;"
    binding_variable: Option<NameType>,
    /// If set, the parser reports bindings with identical names. The option stores the name of the
    /// node that disallows duplicate bindings, for example `let`, `const` or `import`.
    binding_context: Option<BindingContext>,
    /// Tracks variables defined as lexical (let, const)
    lexical_names: LexicalNames,
    /// Tracks variables defined as hoisted (var)
    hoisted_names: HoistedNames,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BindingContext {
    // the default context of the script/module
    Hoisted,
    // functions `function f()`
    Function,
    // import
    Module,
    // { }
    Block,
    // functions arguments, for statement
    LoopStatements,
}

impl Default for BindingContext {
    fn default() -> Self {
        Self::Hoisted
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NameType {
    // module bindings are hoisted but can't be redeclared like hoisted variables
    Module,
    Hoisted,
    Lexical(LexicalType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexicalType {
    Let,
    Const,
}

impl std::fmt::Display for LexicalType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexicalType::Let => write!(f, "let"),
            LexicalType::Const => write!(f, "const"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StrictMode {
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
            label_set: HashMap::new(),
            strict: if syntax.file_kind == FileKind::Module {
                Some(StrictMode::Module)
            } else {
                None
            },
            default_item: None,
            binding_variable: None,
            binding_context: Some(BindingContext::default()),
            no_recovery: false,
            lexical_names: HashMap::default(),
            hoisted_names: HashMap::default(),
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

    pub fn binding_variable(&self) -> Option<&NameType> {
        self.binding_variable.as_ref()
    }

    pub fn binding_context(&self) -> Option<&BindingContext> {
        self.binding_context.as_ref()
    }

    pub fn in_binding_list_for_signature(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::IN_BINDING_LIST_FOR_SIGNATURE)
    }

    /// Checks if a binding has been already registered
    pub fn clashes_with_defined_binding(&self, identifier_name: &str) -> Option<&Range<usize>> {
        if let Some(binding_context) = self.binding_context() {
            match binding_context {
                // hoisted variables can be redeclared without problems
                // which means that we only need to check the variables the lexical environment
                BindingContext::Block => self.lexical_names.get(identifier_name),
                BindingContext::Hoisted => match self.binding_variable() {
                    Some(name_type) => match name_type {
                        NameType::Hoisted | NameType::Module => {
                            self.lexical_names.get(identifier_name)
                        }
                        NameType::Lexical(_) => self
                            .hoisted_names
                            .get(identifier_name)
                            .or_else(|| self.lexical_names.get(identifier_name)),
                    },
                    _ => None,
                },
                _ => match self.binding_variable() {
                    None => self.hoisted_names.get(identifier_name),
                    Some(binding_variable) => match binding_variable {
                        NameType::Hoisted | NameType::Module => {
                            self.lexical_names.get(identifier_name)
                        }
                        NameType::Lexical(_) => self
                            .hoisted_names
                            .get(identifier_name)
                            .or_else(|| self.lexical_names.get(identifier_name)),
                    },
                },
            }
        } else {
            None
        }
    }

    /// It registers the name of a binding based in its name type (lexical or hoisted) and its current binding context
    pub fn register_name(&mut self, identifier_name: String, range: Range<usize>) {
        if let Some(binding_context) = self.binding_context() {
            match binding_context {
                BindingContext::Hoisted => self.register_lexical_type(identifier_name, range),
                BindingContext::Module => {
                    self.hoisted_names.insert(identifier_name, range);
                }
                BindingContext::Block => {
                    match self.binding_variable() {
                        Some(binding_variable) => {
                            match binding_variable {
                                NameType::Hoisted => {
                                    // inside a block scope, hoisted variables needs to be tracked
                                    // inside a lexical environment too;
                                    // For example `ar a; { var b; let b; }` should throw an error for  redeclaration of `b`
                                    // but `var a; var b; { let b; }` should not
                                    self.hoisted_names
                                        .insert(identifier_name.clone(), range.clone());
                                    self.lexical_names.insert(identifier_name, range);
                                }
                                NameType::Lexical(_) => {
                                    self.lexical_names.insert(identifier_name, range);
                                }
                                NameType::Module => {
                                    self.lexical_names.insert(identifier_name, range);
                                }
                            }
                        }
                        None => {
                            self.lexical_names.insert(identifier_name, range);
                        }
                    }
                }
                BindingContext::Function => {
                    self.lexical_names.insert(identifier_name, range);
                }
                BindingContext::LoopStatements => {
                    self.hoisted_names.insert(identifier_name, range);
                }
            }
        }
    }

    /// register a function based on its binding variables:
    /// `var a, b;` vs `let d, c;`
    fn register_lexical_type(&mut self, identifier_name: String, range: Range<usize>) {
        match self.binding_variable() {
            Some(name_type) => match name_type {
                NameType::Hoisted => {
                    self.hoisted_names.insert(identifier_name, range);
                }
                NameType::Lexical(_) | NameType::Module => {
                    self.lexical_names.insert(identifier_name, range);
                }
            },
            None => {
                self.hoisted_names.insert(identifier_name, range);
            }
        }
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
    InBindingListForSignature,
    ParsingContextFlags::IN_BINDING_LIST_FOR_SIGNATURE
);
gen_change_parser_state!(
    PotentialArrowStart,
    ParsingContextFlags::POTENTIAL_ARROW_START
);

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

bitflags! {
    /// Flags describing the context of a function.
    pub(crate) struct SignatureFlags: u8 {
        /// Is the function in an async context
        const ASYNC 		= 0b00001;
        /// Is the function in a generator context
        const GENERATOR 	= 0b00010;
        /// Is the function a constructor (or constructor context)
        const CONSTRUCTOR 	= 0b00100;
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
        const IN_GENERATOR = 0b0000000000000001;
        /// Whether the parser is inside a function
        const IN_FUNCTION = 0b0000000000000010;
        /// Whatever the parser is inside a constructor
        const IN_CONSTRUCTOR = 0b0000000000000100;

        /// Is async allowed in this context. Either because it's an async function or top level await is supported.
        /// Equivalent to the `Async` generator in the ECMA spec
        const IN_ASYNC = 0b0000000000001000;

        /// Whether the parser is parsing a top-level statement (not inside a class, function, parameter) or not
        const TOP_LEVEL = 0b0000000000010000;

        /// Whether `in` should be counted in a binary expression
        /// this is for `for...in` statements to prevent ambiguity.
        const INCLUDE_IN = 0b0000000000100000;
        const IN_BINDING_LIST_FOR_SIGNATURE = 0b0000000001000000;
        /// Whether the parser is in a conditional expr (ternary expr)
        const IN_CONDITION_EXPRESSION = 0b0000000010000000;

        /// Whether the parser is in an iteration or switch statement and
        /// `break` is allowed.
        const BREAK_ALLOWED = 0b0000000100000000;

        /// Whether the parser is in an iteration statement and `continue` is allowed.
        const CONTINUE_ALLOWED = 0b0000001000000000;

        /// If false, object expressions are not allowed to be parsed
        /// inside an expression.
        ///
        /// Also applies for object patterns
        const ALLOW_OBJECT_EXPRESSION = 0b0000010000000000;

        /// Whether we potentially are in a place to parse an arrow expression
        const POTENTIAL_ARROW_START = 0b0000100000000000;

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

#[derive(Default, Debug, Clone)]
pub struct EnterHoistedScopeSnapshot {
    scoped_variables: LexicalNames,
    binding_context: Option<BindingContext>,
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
            binding_context: std::mem::replace(&mut state.binding_context, Some(self.0)),
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
    binding_context: Option<BindingContext>,
}

/// Use this action to track when the parser enters inside a new lexical scope
pub(crate) struct EnterLexicalScope(pub(crate) BindingContext);

impl ChangeParserState for EnterLexicalScope {
    type Snapshot = EnterLexicalScopeSnapshot;

    fn apply(self, state: &mut ParserState) -> Self::Snapshot {
        dbg!("entered lexical scope");
        dbg!(&self.0);
        EnterLexicalScopeSnapshot {
            lexical_names: std::mem::take(&mut state.lexical_names),
            binding_context: std::mem::replace(&mut state.binding_context, Some(self.0)),
        }
    }

    fn restore(state: &mut ParserState, value: Self::Snapshot) {
        dbg!("exit lexical scope");
        state.lexical_names = value.lexical_names;
        state.binding_context = value.binding_context;
    }
}

#[derive(Default, Debug, Clone)]
pub struct EnterVariableDeclarationSnapshot {
    binding_variable: Option<NameType>,
}

/// Use this action to track the type of variable declaration
pub(crate) struct EnterVariableDeclaration(pub(crate) NameType);

impl ChangeParserState for EnterVariableDeclaration {
    type Snapshot = EnterVariableDeclarationSnapshot;

    fn apply(self, state: &mut ParserState) -> Self::Snapshot {
        EnterVariableDeclarationSnapshot {
            binding_variable: std::mem::replace(&mut state.binding_variable, Some(self.0)),
        }
    }

    fn restore(state: &mut ParserState, value: Self::Snapshot) {
        state.binding_variable = value.binding_variable;
    }
}
