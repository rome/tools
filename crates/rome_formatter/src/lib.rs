//! Infrastructure for code formatting
//!
//! This module defines [FormatElement], an IR to format code documents and provides a mean to print
//! such a document to a string. Objects that know how to format themselves implement the [Format] trait.
//!
//! ## Formatting Traits
//!
//! * [Format]: Implemented by objects that can be formatted.
//! * [FormatRule]: Rule that knows how to format an object of another type. Necessary in the situation where
//!  it's necessary to implement [Format] on an object from another crate. This module defines the
//!  [FormatRefWithRule] and [FormatOwnedWithRule] structs to pass an item with its corresponding rule.
//! * [FormatWithRule] implemented by objects that know how to format another type. Useful for implementing
//!  some reusable formatting logic inside of this module if the type itself doesn't implement [Format]
//!
//! ## Formatting Macros
//!
//! This crate defines two macros to construct the IR. These are inspired by Rust's `fmt` macros
//! * [`format!`]: Formats a formatable object
//! * [`format_args!`]: Concatenates a sequence of Format objects.
//! * [`write!`]: Writes a sequence of formatable objects into an output buffer.

#![deny(rustdoc::broken_intra_doc_links)]

mod arguments;
mod buffer;
mod builders;
mod comments;
pub mod format_element;
mod format_extensions;
pub mod formatter;
pub mod group_id;
pub mod intersperse;
pub mod macros;
pub mod prelude;
#[cfg(debug_assertions)]
pub mod printed_tokens;
pub mod printer;
pub mod token;

use crate::formatter::Formatter;
use crate::group_id::UniqueGroupIdBuilder;
use crate::prelude::syntax_token_cow_slice;
use std::any::TypeId;

#[cfg(debug_assertions)]
use crate::printed_tokens::PrintedTokens;
use crate::printer::{Printer, PrinterOptions};
pub use arguments::{Argument, Arguments};
pub use buffer::{
    Buffer, BufferExtensions, BufferSnapshot, HasLabelBuffer, Inspect, PreambleBuffer, VecBuffer,
    WillBreakBuffer,
};
pub use builders::{
    block_indent, comment, empty_line, get_lines_before, group, hard_line_break, if_group_breaks,
    if_group_fits_on_line, indent, labelled, line_suffix, soft_block_indent, soft_line_break,
    soft_line_break_or_space, soft_line_indent_or_space, space, text, BestFitting,
};
pub use comments::{CommentKind, CommentStyle, Comments, SourceComment};
pub use format_element::{normalize_newlines, FormatElement, Text, Verbatim, LINE_TERMINATORS};
pub use group_id::GroupId;
use indexmap::IndexSet;
use rome_rowan::{
    Language, RawSyntaxKind, SyntaxElement, SyntaxError, SyntaxKind, SyntaxNode, SyntaxResult,
    SyntaxToken, SyntaxTriviaPieceComments, TextRange, TextSize, TokenAtOffset,
};
use std::error::Error;
use std::num::ParseIntError;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub enum IndentStyle {
    /// Tab
    Tab,
    /// Space, with its quantity
    Space(u8),
}

impl IndentStyle {
    pub const DEFAULT_SPACES: u8 = 2;

    /// Returns `true` if this is an [IndentStyle::Tab].
    pub const fn is_tab(&self) -> bool {
        matches!(self, IndentStyle::Tab)
    }

    /// Returns `true` if this is an [IndentStyle::Space].
    pub const fn is_space(&self) -> bool {
        matches!(self, IndentStyle::Space(_))
    }
}

impl Default for IndentStyle {
    fn default() -> Self {
        Self::Tab
    }
}

impl FromStr for IndentStyle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tab" | "Tabs" => Ok(Self::Tab),
            "space" | "Spaces" => Ok(Self::Space(IndentStyle::DEFAULT_SPACES)),
            // TODO: replace this error with a diagnostic
            _ => Err("Value not supported for IndentStyle"),
        }
    }
}

impl std::fmt::Display for IndentStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndentStyle::Tab => std::write!(f, "Tab"),
            IndentStyle::Space(size) => std::write!(f, "Spaces, size: {}", size),
        }
    }
}

/// Validated value for the `line_width` formatter options
///
/// The allowed range of values is 1..=320
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct LineWidth(u16);

impl LineWidth {
    /// Maximum allowed value for a valid [LineWidth]
    pub const MAX: u16 = 320;

    /// Return the numeric value for this [LineWidth]
    pub fn value(&self) -> u16 {
        self.0
    }
}

impl Default for LineWidth {
    fn default() -> Self {
        Self(80)
    }
}

/// Error type returned when parsing a [LineWidth] from a string fails
#[derive(Debug)]
pub enum ParseLineWidthError {
    /// The string could not be parsed as a valid [u16]
    ParseError(ParseIntError),
    /// The [u16] value of the string is not a valid [LineWidth]
    TryFromIntError(LineWidthFromIntError),
}

impl std::fmt::Display for ParseLineWidthError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(fmt, "{self:?}")
    }
}

impl FromStr for LineWidth {
    type Err = ParseLineWidthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = u16::from_str(s).map_err(ParseLineWidthError::ParseError)?;
        let value = Self::try_from(value).map_err(ParseLineWidthError::TryFromIntError)?;
        Ok(value)
    }
}

/// Error type returned when converting a u16 to a [LineWidth] fails
#[derive(Clone, Copy, Debug)]
pub struct LineWidthFromIntError(pub u16);

impl TryFrom<u16> for LineWidth {
    type Error = LineWidthFromIntError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 0 && value <= Self::MAX {
            Ok(Self(value))
        } else {
            Err(LineWidthFromIntError(value))
        }
    }
}

impl std::fmt::Display for LineWidthFromIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "The line width exceeds the maximum value ({})",
            LineWidth::MAX
        )
    }
}

impl From<LineWidth> for u16 {
    fn from(value: LineWidth) -> Self {
        value.0
    }
}

/// Context configuring how an object gets formatted.
///
/// Defines the common formatting options. Implementations can define additional options that
/// are specific to formatting a specific object.
pub trait FormatContext {
    /// The indent style.
    fn indent_style(&self) -> IndentStyle;

    /// What's the max width of a line. Defaults to 80.
    fn line_width(&self) -> LineWidth;

    /// Derives the print options from the these format options
    fn as_print_options(&self) -> PrinterOptions;
}

/// The [CstFormatContext] is an extension of the CST unaware [FormatContext] and must be implemented
/// by every language.
///
/// The context customizes the comments formatting and stores the comments of the CST.
pub trait CstFormatContext: FormatContext {
    type Language: Language;
    type Style: CommentStyle<Self::Language>;

    /// Customizes how comments are formatted
    fn comment_style(&self) -> Self::Style;

    /// Returns a ref counted [Comments].
    ///
    /// The use of a [Rc] is necessary to achieve that [Comments] has a lifetime that is independent of the [crate::Formatter].
    /// Having independent lifetimes is necessary to support the use case where a (formattable object)[Format]
    /// iterates over all comments and writes them into the [crate::Formatter] (mutably borrowing the [crate::Formatter] and in turn this context).
    ///
    /// ```block
    /// for leading in f.context().comments().leading_comments(node) {
    ///     ^
    ///     |- Borrows comments
    ///   write!(f, [comment(leading.piece.text())])?;
    ///          ^
    ///          |- Mutably borrows the formatter, state, context (and comments, if they aren't wrapped by a Rc)
    /// }
    /// ```
    fn comments(&self) -> Rc<Comments<Self::Language>>;

    /// Consumes `self` and returns a new context with the provided extracted (`comments`)[Comments].
    fn with_comments(self, comments: Rc<Comments<Self::Language>>) -> Self;
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct SimpleFormatContext {
    pub indent_style: IndentStyle,
    pub line_width: LineWidth,
}

impl FormatContext for SimpleFormatContext {
    fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    fn line_width(&self) -> LineWidth {
        self.line_width
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions::default()
            .with_indent(self.indent_style)
            .with_print_width(self.line_width)
    }
}

/// Lightweight sourcemap marker between source and output tokens
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct SourceMarker {
    /// Position of the marker in the original source
    #[cfg_attr(feature = "serde", schemars(with = "u32"))]
    pub source: TextSize,
    /// Position of the marker in the output code
    #[cfg_attr(feature = "serde", schemars(with = "u32"))]
    pub dest: TextSize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Formatted<Context> {
    root: FormatElement,
    context: Context,
}

impl<Context> Formatted<Context> {
    pub fn new(root: FormatElement, context: Context) -> Self {
        Self { root, context }
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn into_format_element(self) -> FormatElement {
        self.root
    }
}

impl<Context> Formatted<Context>
where
    Context: FormatContext,
{
    pub fn print(&self) -> Printed {
        Printer::new(self.context.as_print_options()).print(&self.root)
    }

    pub fn print_with_indent(&self, indent: u16) -> Printed {
        Printer::new(self.context.as_print_options()).print_with_indent(&self.root, indent)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct Printed {
    code: String,
    #[cfg_attr(
        feature = "serde",
        schemars(with = "Option<rome_rowan::TextRangeSchema>")
    )]
    range: Option<TextRange>,
    sourcemap: Vec<SourceMarker>,
    #[cfg_attr(feature = "serde", schemars(with = "Vec<rome_rowan::TextRangeSchema>"))]
    verbatim_ranges: Vec<TextRange>,
}

impl Printed {
    pub fn new(
        code: String,
        range: Option<TextRange>,
        sourcemap: Vec<SourceMarker>,
        verbatim_source: Vec<TextRange>,
    ) -> Self {
        Self {
            code,
            range,
            sourcemap,
            verbatim_ranges: verbatim_source,
        }
    }

    /// Construct an empty formatter result
    pub fn new_empty() -> Self {
        Self {
            code: String::new(),
            range: None,
            sourcemap: Vec::new(),
            verbatim_ranges: Vec::new(),
        }
    }

    /// Range of the input source file covered by this formatted code,
    /// or None if the entire file is covered in this instance
    pub fn range(&self) -> Option<TextRange> {
        self.range
    }

    /// Returns a list of [SourceMarker] mapping byte positions
    /// in the output string to the input source code
    pub fn sourcemap(&self) -> &[SourceMarker] {
        &self.sourcemap
    }

    /// Returns a list of [SourceMarker] mapping byte positions
    /// in the output string to the input source code, consuming the result
    pub fn into_sourcemap(self) -> Vec<SourceMarker> {
        self.sourcemap
    }

    /// Access the resulting code, borrowing the result
    pub fn as_code(&self) -> &str {
        &self.code
    }

    /// Access the resulting code, consuming the result
    pub fn into_code(self) -> String {
        self.code
    }

    /// The text in the formatted code that has been formatted as verbatim.
    pub fn verbatim(&self) -> impl Iterator<Item = (TextRange, &str)> {
        self.verbatim_ranges
            .iter()
            .map(|range| (*range, &self.code[*range]))
    }

    /// Ranges of the formatted code that have been formatted as verbatim.
    pub fn verbatim_ranges(&self) -> &[TextRange] {
        &self.verbatim_ranges
    }
}

/// Public return type of the formatter
pub type FormatResult<F> = Result<F, FormatError>;

#[derive(Debug, PartialEq,Eq, Copy, Clone)]
/// Series of errors encountered during formatting
pub enum FormatError {
    /// In case a node can't be formatted because it either misses a require child element or
    /// a child is present that should not (e.g. a trailing comma after a rest element).
    SyntaxError,
    /// In case range formatting failed because the provided range was larger
    /// than the formatted syntax tree
    RangeError { input: TextRange, tree: TextRange },
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError::SyntaxError => fmt.write_str("syntax error"),
            FormatError::RangeError { input, tree } => std::write!(
                fmt,
                "formatting range {input:?} is larger than syntax tree {tree:?}"
            ),
        }
    }
}

impl Error for FormatError {}

impl From<SyntaxError> for FormatError {
    fn from(error: SyntaxError) -> Self {
        FormatError::from(&error)
    }
}

impl From<&SyntaxError> for FormatError {
    fn from(syntax_error: &SyntaxError) -> Self {
        match syntax_error {
            SyntaxError::MissingRequiredChild => FormatError::SyntaxError,
        }
    }
}

/// Formatting trait for types that can create a formatted representation. The `rome_formatter` equivalent
/// to [std::fmt::Display].
///
/// ## Example
/// Implementing `Format` for a custom struct
///
/// ```
/// use rome_formatter::{format, write, IndentStyle, LineWidth};
/// use rome_formatter::prelude::*;
/// use rome_rowan::TextSize;
///
/// struct Paragraph(String);
///
/// impl Format<SimpleFormatContext> for Paragraph {
///     fn fmt(&self, f: &mut Formatter<SimpleFormatContext>) -> FormatResult<()> {
///         write!(f, [
///             hard_line_break(),
///             dynamic_text(&self.0, TextSize::from(0)),
///             hard_line_break(),
///         ])
///     }
/// }
///
/// let paragraph = Paragraph(String::from("test"));
/// let formatted = format!(SimpleFormatContext::default(), [paragraph]).unwrap();
///
/// assert_eq!("test\n", formatted.print().as_code())
/// ```
pub trait Format<Context> {
    /// Formats the object using the given formatter.
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()>;
}

impl<T, Context> Format<Context> for &T
where
    T: ?Sized + Format<Context>,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        Format::fmt(&**self, f)
    }
}

impl<T, Context> Format<Context> for &mut T
where
    T: ?Sized + Format<Context>,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        Format::fmt(&**self, f)
    }
}

impl<T, Context> Format<Context> for Option<T>
where
    T: Format<Context>,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        match self {
            Some(value) => value.fmt(f),
            None => Ok(()),
        }
    }
}

impl<T, Context> Format<Context> for SyntaxResult<T>
where
    T: Format<Context>,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        match self {
            Ok(value) => value.fmt(f),
            Err(err) => Err(err.into()),
        }
    }
}

impl<Context> Format<Context> for () {
    #[inline]
    fn fmt(&self, _: &mut Formatter<Context>) -> FormatResult<()> {
        // Intentionally left empty
        Ok(())
    }
}

/// Rule that knows how to format an object of type `T`.
///
/// Implementing [Format] on the object itself is preferred over implementing [FormatRule] but
/// this isn't possible inside of a dependent crate for external type.
///
/// For example, the `rome_js_formatter` crate isn't able to implement [Format] on `JsIfStatement`
/// because both the [Format] trait and `JsIfStatement` are external types (Rust's orphan rule).
///
/// That's why the `rome_js_formatter` crate must define a new-type that implements the formatting
/// of `JsIfStatement`.
pub trait FormatRule<T> {
    type Context;

    fn fmt(&self, item: &T, f: &mut Formatter<Self::Context>) -> FormatResult<()>;
}

/// Rule that supports customizing how it formats an object of type `T`.
pub trait FormatRuleWithOptions<T>: FormatRule<T> {
    type Options;

    /// Returns a new rule that uses the given options to format an object.
    fn with_options(self, options: Self::Options) -> Self;
}

/// Trait for an object that formats an object with a specified rule.
///
/// Gives access to the underlying item.
///
/// Useful in situation where a type itself doesn't implement [Format] (e.g. because of Rust's orphan rule)
/// but you want to implement some common formatting logic.
///
/// ## Examples
///
/// This can be useful if you want to format a `SyntaxNode` inside rome_formatter.. `SyntaxNode` doesn't implement [Format]
/// itself but the language specific crate implements `AsFormat` and `IntoFormat` for it and the returned [Format]
/// implement [FormatWithRule].
///
/// ```ignore
/// use rome_formatter::prelude::*;
/// use rome_formatter::{format, Formatted, FormatWithRule};
/// use rome_rowan::{Language, SyntaxNode};
/// fn format_node<L: Language, F: FormatWithRule<SimpleFormatContext, Item=SyntaxNode<L>>>(node: F) -> FormatResult<Formatted<SimpleFormatContext>> {
///     let formatted = format!(SimpleFormatContext::default(), [node]);
///     let syntax = node.item();
///     // Do something with syntax
///     formatted;
/// }
/// ```
pub trait FormatWithRule<Context>: Format<Context> {
    type Item;

    /// Returns the associated item
    fn item(&self) -> &Self::Item;
}

/// Formats the referenced `item` with the specified rule.
#[derive(Debug, Copy, Clone)]
pub struct FormatRefWithRule<'a, T, R>
where
    R: FormatRule<T>,
{
    item: &'a T,
    rule: R,
}

impl<'a, T, R> FormatRefWithRule<'a, T, R>
where
    R: FormatRule<T>,
{
    pub fn new(item: &'a T, rule: R) -> Self {
        Self { item, rule }
    }
}

impl<T, R, O> FormatRefWithRule<'_, T, R>
where
    R: FormatRuleWithOptions<T, Options = O>,
{
    pub fn with_options(mut self, options: O) -> Self {
        self.rule = self.rule.with_options(options);
        self
    }
}

impl<T, R> FormatWithRule<R::Context> for FormatRefWithRule<'_, T, R>
where
    R: FormatRule<T>,
{
    type Item = T;

    fn item(&self) -> &Self::Item {
        self.item
    }
}

impl<T, R> Format<R::Context> for FormatRefWithRule<'_, T, R>
where
    R: FormatRule<T>,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<R::Context>) -> FormatResult<()> {
        self.rule.fmt(self.item, f)
    }
}

/// Formats the `item` with the specified rule.
#[derive(Debug, Clone)]
pub struct FormatOwnedWithRule<T, R>
where
    R: FormatRule<T>,
{
    item: T,
    rule: R,
}

impl<T, R> FormatOwnedWithRule<T, R>
where
    R: FormatRule<T>,
{
    pub fn new(item: T, rule: R) -> Self {
        Self { item, rule }
    }

    pub fn with_item(mut self, item: T) -> Self {
        self.item = item;
        self
    }

    pub fn into_item(self) -> T {
        self.item
    }
}

impl<T, R> Format<R::Context> for FormatOwnedWithRule<T, R>
where
    R: FormatRule<T>,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<R::Context>) -> FormatResult<()> {
        self.rule.fmt(&self.item, f)
    }
}

impl<T, R, O> FormatOwnedWithRule<T, R>
where
    R: FormatRuleWithOptions<T, Options = O>,
{
    pub fn with_options(mut self, options: O) -> Self {
        self.rule = self.rule.with_options(options);
        self
    }
}

impl<T, R> FormatWithRule<R::Context> for FormatOwnedWithRule<T, R>
where
    R: FormatRule<T>,
{
    type Item = T;

    fn item(&self) -> &Self::Item {
        &self.item
    }
}

/// The `write` function takes a target buffer and an `Arguments` struct that can be precompiled with the `format_args!` macro.
///
/// The arguments will be formatted in-order into the output buffer provided.
///
/// # Examples
///
/// ```
/// use rome_formatter::prelude::*;
/// use rome_formatter::{VecBuffer, format_args, FormatState, write, Formatted};
///
/// let mut state = FormatState::new(SimpleFormatContext::default());
/// let mut buffer = VecBuffer::new(&mut state);
///
/// write!(&mut buffer, [format_args!(text("Hello World"))]).unwrap();
///
/// let formatted = Formatted::new(buffer.into_element(), SimpleFormatContext::default());
///
/// assert_eq!("Hello World", formatted.print().as_code())
/// ```
///
/// Please note that using [`write!`] might be preferable. Example:
///
/// ```
/// use rome_formatter::prelude::*;
/// use rome_formatter::{VecBuffer, format_args, FormatState, write, Formatted};
///
/// let mut state = FormatState::new(SimpleFormatContext::default());
/// let mut buffer = VecBuffer::new(&mut state);
///
/// write!(&mut buffer, [text("Hello World")]).unwrap();
///
/// let formatted = Formatted::new(buffer.into_element(), SimpleFormatContext::default());
///
/// assert_eq!("Hello World", formatted.print().as_code())
/// ```
///
#[inline(always)]
pub fn write<Context>(
    output: &mut dyn Buffer<Context = Context>,
    args: Arguments<Context>,
) -> FormatResult<()> {
    let mut f = Formatter::new(output);

    f.write_fmt(args)
}

/// The `format` function takes an [`Arguments`] struct and returns the resulting formatting IR.
///
/// The [`Arguments`] instance can be created with the [`format_args!`].
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rome_formatter::prelude::*;
/// use rome_formatter::{format, format_args};
///
/// let formatted = format!(SimpleFormatContext::default(), [&format_args!(text("test"))]).unwrap();
/// assert_eq!("test", formatted.print().as_code());
/// ```
///
/// Please note that using [`format!`] might be preferable. Example:
///
/// ```
/// use rome_formatter::prelude::*;
/// use rome_formatter::{format};
///
/// let formatted = format!(SimpleFormatContext::default(), [text("test")]).unwrap();
/// assert_eq!("test", formatted.print().as_code());
/// ```
pub fn format<Context>(
    context: Context,
    arguments: Arguments<Context>,
) -> FormatResult<Formatted<Context>>
where
    Context: FormatContext,
{
    let mut state = FormatState::new(context);
    let mut buffer = VecBuffer::with_capacity(arguments.items().len(), &mut state);

    buffer.write_fmt(arguments)?;

    Ok(Formatted {
        root: buffer.into_element(),
        context: state.into_context(),
    })
}

/// Formats a syntax node file based on its features.
///
/// It returns a [Formatted] result, which the user can use to override a file.
pub fn format_node<
    Context: CstFormatContext,
    N: FormatWithRule<Context, Item = SyntaxNode<Context::Language>>,
>(
    context: Context,
    root: &N,
) -> FormatResult<Formatted<Context>> {
    tracing::trace_span!("format_node").in_scope(move || {
        let suppressions = Comments::from_node(root.item(), &context);
        let context = context.with_comments(Rc::new(suppressions));

        let mut state = FormatState::new(context);
        let mut buffer = VecBuffer::new(&mut state);

        write!(&mut buffer, [root])?;

        let document = buffer.into_element();

        state.assert_formatted_all_tokens(root.item());
        state
            .context()
            .comments()
            .assert_checked_all_suppressions(root.item());

        Ok(Formatted::new(document, state.into_context()))
    })
}

/// Returns the [TextRange] for this [SyntaxElement] with the leading and
/// trailing whitespace trimmed (but keeping comments or skipped trivias)
fn text_non_whitespace_range<E, L>(elem: &E) -> TextRange
where
    E: Into<SyntaxElement<L>> + Clone,
    L: Language,
{
    let elem: SyntaxElement<L> = elem.clone().into();

    let start = elem
        .leading_trivia()
        .into_iter()
        .flat_map(|trivia| trivia.pieces())
        .find_map(|piece| {
            if piece.is_whitespace() || piece.is_newline() {
                None
            } else {
                Some(piece.text_range().start())
            }
        })
        .unwrap_or_else(|| elem.text_trimmed_range().start());

    let end = elem
        .trailing_trivia()
        .into_iter()
        .flat_map(|trivia| trivia.pieces().rev())
        .find_map(|piece| {
            if piece.is_whitespace() || piece.is_newline() {
                None
            } else {
                Some(piece.text_range().end())
            }
        })
        .unwrap_or_else(|| elem.text_trimmed_range().end());

    TextRange::new(start, end)
}

/// Formats a range within a file, supported by Rome
///
/// Because this function is language-agnostic, it must be provided with a
/// `predicate` function that's used to select appropriate "root nodes" for the
/// range formatting process: for instance in JavaScript the predicate returns
/// true for statement and declaration nodes, to ensure the entire statement
/// gets formatted instead of the smallest sub-expression that fits the range
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [FormatContext], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Formatted] result with a range corresponding to the
/// range of the input that was effectively overwritten by the formatter
pub fn format_range<Context, R, P>(
    context: Context,
    root: &SyntaxNode<Context::Language>,
    mut range: TextRange,
    mut predicate: P,
) -> FormatResult<Printed>
where
    Context: CstFormatContext,
    R: FormatRule<SyntaxNode<Context::Language>, Context = Context> + Default,
    P: FnMut(&SyntaxNode<Context::Language>) -> bool,
{
    if range.is_empty() {
        return Ok(Printed::new(
            String::new(),
            Some(range),
            Vec::new(),
            Vec::new(),
        ));
    }

    let root_range = root.text_range();
    if range.start() < root_range.start() || range.end() > root_range.end() {
        return Err(FormatError::RangeError {
            input: range,
            tree: root_range,
        });
    }

    // Find the tokens corresponding to the start and end of the range
    let start_token = root.token_at_offset(range.start());
    let end_token = root.token_at_offset(range.end());

    // If these tokens were not found this means either:
    // 1. The input [SyntaxNode] was empty
    // 2. The input node was not the root [SyntaxNode] of the file
    // In the first case we can return an empty result immediately,
    // otherwise default to the first and last tokens in the root node
    let mut start_token = match start_token {
        // If the start of the range lies between two tokens,
        // start at the rightmost one
        TokenAtOffset::Between(_, token) => token,
        TokenAtOffset::Single(token) => token,
        TokenAtOffset::None => match root.first_token() {
            Some(token) => token,
            // root node is empty
            None => return Ok(Printed::new_empty()),
        },
    };
    let mut end_token = match end_token {
        // If the end of the range lies between two tokens,
        // end at the leftmost one
        TokenAtOffset::Between(token, _) => token,
        TokenAtOffset::Single(token) => token,
        TokenAtOffset::None => match root.last_token() {
            Some(token) => token,
            // root node is empty
            None => return Ok(Printed::new_empty()),
        },
    };

    // Trim leading and trailing whitespace off from the formatting range
    let mut trimmed_start = range.start();

    let start_token_range = text_non_whitespace_range(&start_token);
    let start_token_trimmed_start = start_token_range.start();
    let start_token_trimmed_end = start_token_range.end();

    if start_token_trimmed_start >= range.start() && start_token_trimmed_start <= range.end() {
        // If the range starts before the trimmed start of the token, move the
        // start towards that position
        trimmed_start = start_token_trimmed_start;
    } else if start_token_trimmed_end <= range.start() {
        // If the range starts after the trimmed end of the token, move the
        // start to the trimmed start of the next token if it exists
        if let Some(next_token) = start_token.next_token() {
            let next_token_start = text_non_whitespace_range(&next_token).start();
            if next_token_start <= range.end() {
                trimmed_start = next_token_start;
                start_token = next_token;
            }
        }
    }

    let end_token_range = text_non_whitespace_range(&end_token);
    let end_token_trimmed_start = end_token_range.start();

    // If the range ends before the trimmed start of the token, move the
    // end to the trimmed end of the previous token if it exists
    if end_token_trimmed_start >= range.end() {
        if let Some(next_token) = end_token.prev_token() {
            let next_token_end = text_non_whitespace_range(&next_token).end();
            if next_token_end >= trimmed_start {
                end_token = next_token;
            }
        }
    }

    // Find suitable formatting-root nodes (matching the predicate provided by
    // the language implementation) in the ancestors of the start and end tokens
    let start_node = start_token
        .ancestors()
        .find(&mut predicate)
        .unwrap_or_else(|| root.clone());
    let end_node = end_token
        .ancestors()
        .find(predicate)
        .unwrap_or_else(|| root.clone());

    let common_root = if start_node == end_node {
        range = text_non_whitespace_range(&start_node);
        Some(start_node)
    } else {
        // Find the two highest sibling nodes that satisfy the formatting range
        // from the ancestors of the start and end nodes (this is roughly the
        // same algorithm as the findSiblingAncestors function in Prettier, see
        // https://github.com/prettier/prettier/blob/cae195187f524dd74e60849e0a4392654423415b/src/main/range-util.js#L36)
        let start_node_start = start_node.text_range().start();
        let end_node_end = end_node.text_range().end();

        let result_end_node = end_node
            .ancestors()
            .take_while(|end_parent| end_parent.text_range().start() >= start_node_start)
            .last()
            .unwrap_or(end_node);

        let result_start_node = start_node
            .ancestors()
            .take_while(|start_parent| start_parent.text_range().end() <= end_node_end)
            .last()
            .unwrap_or(start_node);

        range = text_non_whitespace_range(&result_start_node)
            .cover(text_non_whitespace_range(&result_end_node));

        // Find the lowest common ancestor node for the previously selected
        // sibling nodes by building the path to the root node from both
        // nodes and iterating along the two paths at once to find the first
        // divergence (the ancestors have to be collected into vectors first
        // since the ancestor iterator isn't double ended)
        #[allow(clippy::needless_collect)]
        let start_to_root: Vec<_> = result_start_node.ancestors().collect();
        #[allow(clippy::needless_collect)]
        let end_to_root: Vec<_> = result_end_node.ancestors().collect();

        start_to_root
            .into_iter()
            .rev()
            .zip(end_to_root.into_iter().rev())
            .map_while(|(lhs, rhs)| if lhs == rhs { Some(lhs) } else { None })
            .last()
    };

    // Logically this should always return at least the root node,
    // fallback to said node just in case
    let common_root = common_root.as_ref().unwrap_or(root);

    // Perform the actual formatting of the root node with
    // an appropriate indentation level
    let formatted = format_sub_tree(
        context,
        &FormatRefWithRule::<_, R>::new(common_root, R::default()),
    )?;

    // This finds the closest marker to the beginning of the source
    // starting before or at said starting point, and the closest
    // marker to the end of the source range starting after or at
    // said ending point respectively
    let mut range_start = None;
    let mut range_end = None;

    let sourcemap = Vec::from(formatted.sourcemap());
    for marker in &sourcemap {
        // marker.source <= range.start()
        if let Some(start_dist) = range.start().checked_sub(marker.source) {
            range_start = match range_start {
                Some((prev_marker, prev_dist)) => {
                    if start_dist < prev_dist {
                        Some((marker, start_dist))
                    } else {
                        Some((prev_marker, prev_dist))
                    }
                }
                None => Some((marker, start_dist)),
            }
        }

        // marker.source >= range.end()
        if let Some(end_dist) = marker.source.checked_sub(range.end()) {
            range_end = match range_end {
                Some((prev_marker, prev_dist)) => {
                    if end_dist <= prev_dist {
                        Some((marker, end_dist))
                    } else {
                        Some((prev_marker, prev_dist))
                    }
                }
                None => Some((marker, end_dist)),
            }
        }
    }

    // If no start or end were found, this means that the edge of the formatting
    // range was near the edge of the input, and no marker were emitted before
    // the start (or after the end) of the formatting range: in this case
    // the start/end marker default to the start/end of the input
    let (start_source, start_dest) = match range_start {
        Some((start_marker, _)) => (start_marker.source, start_marker.dest),
        None => (common_root.text_range().start(), TextSize::from(0)),
    };
    let (end_source, end_dest) = match range_end {
        Some((end_marker, _)) => (end_marker.source, end_marker.dest),
        None => (
            common_root.text_range().end(),
            TextSize::try_from(formatted.as_code().len()).expect("code length out of bounds"),
        ),
    };

    let input_range = TextRange::new(start_source, end_source);
    let output_range = TextRange::new(start_dest, end_dest);
    let sourcemap = Vec::from(formatted.sourcemap());
    let verbatim_ranges = Vec::from(formatted.verbatim_ranges());
    let code = &formatted.into_code()[output_range];
    Ok(Printed::new(
        code.into(),
        Some(input_range),
        sourcemap,
        verbatim_ranges,
    ))
}

/// Formats a single node within a file, supported by Rome.
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [FormatContext], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Formatted] result
pub fn format_sub_tree<
    C: CstFormatContext,
    N: FormatWithRule<C, Item = SyntaxNode<C::Language>>,
>(
    context: C,
    root: &N,
) -> FormatResult<Printed> {
    let syntax = root.item();
    // Determine the initial indentation level for the printer by inspecting the trivia pieces
    // of each token from the first token of the common root towards the start of the file
    let mut tokens = std::iter::successors(syntax.first_token(), |token| token.prev_token());

    // From the iterator of tokens, build an iterator of trivia pieces (once again the iterator is
    // reversed, starting from the last trailing trivia towards the first leading trivia).
    // The first token is handled specially as we only wan to consider its leading trivia pieces
    let first_token = tokens.next();
    let first_token_trivias = first_token
        .into_iter()
        .flat_map(|token| token.leading_trivia().pieces().rev());

    let next_tokens_trivias = tokens.flat_map(|token| {
        token
            .trailing_trivia()
            .pieces()
            .rev()
            .chain(token.leading_trivia().pieces().rev())
    });

    let trivias = first_token_trivias
        .chain(next_tokens_trivias)
        .filter(|piece| {
            // We're only interested in newline and whitespace trivias, skip over comments
            let is_newline = piece.is_newline();
            let is_whitespace = piece.is_whitespace();
            is_newline || is_whitespace
        });

    // Finally run the iterator until a newline trivia is found, and get the last whitespace trivia before it
    let last_whitespace = trivias.map_while(|piece| piece.as_whitespace()).last();
    let initial_indent = match last_whitespace {
        Some(trivia) => {
            // This logic is based on the formatting options passed in
            // the be user (or the editor) as we do not have any kind
            // of indentation type detection yet. Unfortunately this
            // may not actually match the current content of the file
            let length = trivia.text().len() as u16;
            match context.indent_style() {
                IndentStyle::Tab => length,
                IndentStyle::Space(width) => length / u16::from(width),
            }
        }
        // No whitespace was found between the start of the range
        // and the start of the file
        None => 0,
    };

    let formatted = format_node(context, root)?;
    let printed = formatted.print_with_indent(initial_indent);
    let sourcemap = Vec::from(printed.sourcemap());
    let verbatim_ranges = Vec::from(printed.verbatim_ranges());
    Ok(Printed::new(
        printed.into_code(),
        Some(syntax.text_range()),
        sourcemap,
        verbatim_ranges,
    ))
}

impl<L: Language, Context> Format<Context> for SyntaxTriviaPieceComments<L> {
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        let range = self.text_range();

        write!(
            f,
            [syntax_token_cow_slice(
                normalize_newlines(self.text().trim(), LINE_TERMINATORS),
                &self.as_piece().token(),
                range.start()
            )]
        )
    }
}

/// This structure stores the state that is relevant for the formatting of the whole document.
///
/// This structure is different from [crate::Formatter] in that the formatting infrastructure
/// creates a new [crate::Formatter] for every [crate::write!] call, whereas this structure stays alive
/// for the whole process of formatting a root with [crate::format!].
#[derive(Default)]
pub struct FormatState<Context> {
    context: Context,
    group_id_builder: UniqueGroupIdBuilder,

    /// `true` if the last formatted output is an inline comment that may need a space between the next token or comment.
    last_content_inline_comment: bool,

    /// The kind of the last formatted token
    last_token_kind: Option<LastTokenKind>,

    /// Tracks comments that have been formatted manually and shouldn't be emitted again
    /// when formatting the token the comments belong to.
    ///
    /// The map stores the absolute position of the manually formatted comments.
    /// Storing the position is sufficient because comments are guaranteed to not be empty
    /// (all start with a specific comment sequence) and thus, no two comments can have the same
    /// absolute position.
    manually_formatted_comments: IndexSet<TextSize>,

    // This is using a RefCell as it only exists in debug mode,
    // the Formatter is still completely immutable in release builds
    #[cfg(debug_assertions)]
    pub printed_tokens: PrintedTokens,
}

impl<Context> std::fmt::Debug for FormatState<Context>
where
    Context: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FormatState")
            .field("context", &self.context)
            .field(
                "has_trailing_inline_comment",
                &self.last_content_inline_comment,
            )
            .field("last_token_kind", &self.last_token_kind)
            .finish()
    }
}

impl<Context> FormatState<Context> {
    /// Creates a new state with the given language specific context
    pub fn new(context: Context) -> Self {
        Self {
            context,
            group_id_builder: Default::default(),
            last_content_inline_comment: false,
            last_token_kind: None,
            manually_formatted_comments: IndexSet::default(),
            #[cfg(debug_assertions)]
            printed_tokens: Default::default(),
        }
    }

    pub fn into_context(self) -> Context {
        self.context
    }

    /// Returns `true` if the last written content is an inline comment with no trailing whitespace.
    ///
    /// The formatting of the next content may need to insert a whitespace to separate the
    /// inline comment from the next content.
    pub fn is_last_content_inline_comment(&self) -> bool {
        self.last_content_inline_comment
    }

    /// Sets whether the last written content is an inline comment that has no trailing whitespace.
    pub fn set_last_content_inline_comment(&mut self, has_comment: bool) {
        self.last_content_inline_comment = has_comment;
    }

    /// Returns the kind of the last formatted token.
    pub fn last_token_kind(&self) -> Option<LastTokenKind> {
        self.last_token_kind
    }

    /// Sets the kind of the last formatted token and sets `last_content_inline_comment` to `false`.
    pub fn set_last_token_kind<Kind: SyntaxKind + 'static>(&mut self, kind: Kind) {
        self.set_last_token_kind_raw(Some(LastTokenKind {
            kind_type: TypeId::of::<Kind>(),
            kind: kind.to_raw(),
        }));
    }

    pub fn set_last_token_kind_raw(&mut self, kind: Option<LastTokenKind>) {
        self.last_token_kind = kind;
    }

    /// Mark the passed comment as formatted. This is necessary if a comment from a token is formatted
    /// to avoid that the comment gets emitted again when formatting that token.
    ///
    /// # Examples
    /// This can be useful when you want to move comments from one token to another.
    /// For example, when parenthesising an expression:
    ///
    /// ```javascript
    /// console.log("test");
    /// /* leading */ "string" /* trailing */;
    /// ```
    ///
    /// It is then desired that the leading and trailing comments are outside of the parentheses.
    ///
    /// ```javascript
    /// /* leading */ ("string") /* trailing */;
    /// ```
    ///
    /// This can be accomplished by manually formatting the leading/trailing trivia of the string literal expression
    /// before/after the close parentheses and then mark the comments as handled.
    pub fn mark_comment_as_formatted<L: Language>(
        &mut self,
        comment: &SyntaxTriviaPieceComments<L>,
    ) {
        self.manually_formatted_comments
            .insert(comment.text_range().start());
    }

    /// Returns `true` if this comment has already been formatted manually
    /// and shouldn't be formatted again when formatting the token to which the comment belongs.
    pub fn is_comment_formatted<L: Language>(
        &self,
        comment: &SyntaxTriviaPieceComments<L>,
    ) -> bool {
        self.manually_formatted_comments
            .contains(&comment.text_range().start())
    }

    /// Returns the context specifying how to format the current CST
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Returns a mutable reference to the context
    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    /// Creates a new group id that is unique to this document. The passed debug name is used in the
    /// [std::fmt::Debug] of the document if this is a debug build.
    /// The name is unused for production builds and has no meaning on the equality of two group ids.
    pub fn group_id(&self, debug_name: &'static str) -> GroupId {
        self.group_id_builder.group_id(debug_name)
    }

    /// Tracks the given token as formatted
    #[inline]
    pub fn track_token<L: Language>(&mut self, #[allow(unused_variables)] token: &SyntaxToken<L>) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                self.printed_tokens.track_token(token);
            }
        }
    }

    /// Asserts in debug builds that all tokens have been printed.
    #[inline]
    pub fn assert_formatted_all_tokens<L: Language>(
        &self,
        #[allow(unused_variables)] root: &SyntaxNode<L>,
    ) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                self.printed_tokens.assert_all_tracked(root);
            }
        }
    }
}

impl<Context> FormatState<Context>
where
    Context: FormatContext,
{
    pub fn snapshot(&self) -> FormatStateSnapshot {
        FormatStateSnapshot {
            last_content_inline_comment: self.last_content_inline_comment,
            last_token_kind: self.last_token_kind,
            manual_handled_comments_len: self.manually_formatted_comments.len(),
            #[cfg(debug_assertions)]
            printed_tokens: self.printed_tokens.clone(),
        }
    }

    pub fn restore_snapshot(&mut self, snapshot: FormatStateSnapshot) {
        let FormatStateSnapshot {
            last_content_inline_comment,
            last_token_kind,
            manual_handled_comments_len,
            #[cfg(debug_assertions)]
            printed_tokens,
        } = snapshot;

        self.last_content_inline_comment = last_content_inline_comment;
        self.last_token_kind = last_token_kind;
        self.manually_formatted_comments
            .truncate(manual_handled_comments_len);
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                self.printed_tokens = printed_tokens;
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct LastTokenKind {
    kind_type: TypeId,
    kind: RawSyntaxKind,
}

impl LastTokenKind {
    pub fn as_language<L: Language + 'static>(&self) -> Option<L::Kind> {
        if self.kind_type == TypeId::of::<L::Kind>() {
            Some(L::Kind::from_raw(self.kind))
        } else {
            None
        }
    }
}

pub struct FormatStateSnapshot {
    last_content_inline_comment: bool,
    last_token_kind: Option<LastTokenKind>,
    manual_handled_comments_len: usize,
    #[cfg(debug_assertions)]
    printed_tokens: PrintedTokens,
}
