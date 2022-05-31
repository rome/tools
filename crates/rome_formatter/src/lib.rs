//! Infrastructure for code formatting
//!
//! This module defines [FormatElement], an IR to format code documents and provides a mean to print
//! such a document to a string. Objects that know how to format themselves implement the [Format] trait.
//!
//! ## Formatting Traits
//!
//! * [Format]: Implemented by objects that can be formatted.
//! * [IntoFormatElement]: The arguments passed to the `formatted[formatter, arg1, arg2]` must implement the.
//!  [IntoFormatElement] trait. Its main difference to the [Format] trait is that it consumes self rather than borrowing it.
//!  This module provides [IntoFormatElement] implementations for every object implementing [Format] and [FormatElement].
//! * [FormatRule]: Rule that knows how to format an object of another type. Necessary in the situation where
//!  it's necessary to implement [Format] on an object from another crate. This module defines the
//!  [FormatRefWithRule] and [FormatOwnedWithRule] structs to pass an item with its corresponding rule.
//! * [FormatWithRule] implemented by objects that know how to format another type. Useful for implementing
//!  some reusable formatting logic inside of this module if the type itself doesn't implement [Format]
//!
//! ## Formatting Macros
//!
//! This trait defines two macros to construct the IR.
//! * [format_elements]: Allows concatenating multiple [FormatElement]s
//! * [formatted]: Concatenates a sequence of [FormatElement]s and/or objects implementing [Format].

extern crate core;

mod builders;
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

use crate::formatter::Formatter;
use crate::printer::{Printer, PrinterOptions};
pub use builders::ConcatBuilder;
pub use format_element::{
    block_indent, comment, concat_elements, empty_element, empty_line, fill_elements,
    group_elements, hard_line_break, if_group_breaks, if_group_fits_on_single_line, indent,
    join_elements, join_elements_hard_line, join_elements_soft_line, join_elements_with,
    line_suffix, normalize_newlines, soft_block_indent, soft_line_break, soft_line_break_or_space,
    soft_line_indent_or_space, space_token, token, FormatElement, Token, Verbatim,
    LINE_TERMINATORS,
};
pub use group_id::GroupId;
use rome_rowan::{
    Language, SyntaxElement, SyntaxError, SyntaxNode, SyntaxResult, TextRange, TextSize,
    TokenAtOffset,
};
use std::error::Error;
use std::fmt;
use std::marker::PhantomData;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum IndentStyle {
    /// Tab
    Tab,
    /// Space, with its quantity
    Space(u8),
}

impl IndentStyle {
    pub const DEFAULT_SPACES: u8 = 2;
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

impl fmt::Display for IndentStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndentStyle::Tab => write!(f, "Tab"),
            IndentStyle::Space(size) => write!(f, "Spaces, size: {}", size),
        }
    }
}

/// Validated value for the `line_width` formatter options
///
/// The allowed range of values is 1..=320
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

impl fmt::Display for ParseLineWidthError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{self:?}")
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
    fn line_with(&self) -> LineWidth;

    /// Derives the print options from the these format options
    fn as_print_options(&self) -> PrinterOptions;
}

/// Lightweight sourcemap marker between source and output tokens
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceMarker {
    /// Position of the marker in the original source
    pub source: TextSize,
    /// Position of the marker in the output code
    pub dest: TextSize,
}

#[derive(Debug, Clone)]
pub struct Formatted {
    root: FormatElement,
    options: PrinterOptions,
}

impl Formatted {
    pub fn new(root: FormatElement, options: PrinterOptions) -> Self {
        Self { root, options }
    }

    pub fn print(&self) -> Printed {
        Printer::new(self.options.clone()).print(&self.root)
    }

    pub fn print_with_indent(&self, indent: u16) -> Printed {
        Printer::new(self.options.clone()).print_with_indent(&self.root, indent)
    }

    pub fn into_format_element(self) -> FormatElement {
        self.root
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Printed {
    code: String,
    range: Option<TextRange>,
    sourcemap: Vec<SourceMarker>,
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

#[derive(Debug, PartialEq, Copy, Clone)]
/// Series of errors encountered during formatting
pub enum FormatError {
    /// Node is missing and it should be required for a correct formatting
    MissingRequiredChild,

    /// In case our formatter doesn't know how to format a certain language
    UnsupportedLanguage,

    /// When the ability to format the current file has been turned off on purpose
    CapabilityDisabled,
}

impl fmt::Display for FormatError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatError::MissingRequiredChild => fmt.write_str("missing required child"),
            FormatError::UnsupportedLanguage => fmt.write_str("language is not supported"),
            FormatError::CapabilityDisabled => fmt.write_str("formatting capability is disabled"),
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
            SyntaxError::MissingRequiredChild => FormatError::MissingRequiredChild,
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
/// use rome_formatter::{format, FormatContext, IndentStyle, LineWidth};
/// use rome_formatter::prelude::*;
/// use rome_rowan::TextSize;
///
/// struct Paragraph(String);
///
/// impl Format for Paragraph {
///     type Context = Context;
///
///     fn format(&self, formatter: &Formatter<Self::Context>) -> FormatResult<FormatElement> {
///         formatted![
///             formatter,
///             [
///                 hard_line_break(),
///                 FormatElement::from(Token::new_dynamic(self.0.clone(), TextSize::from(0))),
///                 hard_line_break(),
///             ]
///         ]
///     }
/// }
///
/// struct Context;
///
/// impl FormatContext for Context {
///     fn indent_style(&self) -> IndentStyle {
///         IndentStyle::Tab
///     }
///
///     fn line_with(&self) -> LineWidth {
///         LineWidth::default()
///     }
///
///     fn as_print_options(&self) -> PrinterOptions {
///         PrinterOptions::default()
///     }
/// }
///
/// let paragraph = Paragraph(String::from("test"));
/// let printed = format(Context, &paragraph).unwrap().print();
///
/// assert_eq!("test\n", printed.as_code())
/// ```
pub trait Format {
    /// Type of the formatter options.
    type Context;

    /// Formats the object
    fn format(&self, formatter: &Formatter<Self::Context>) -> FormatResult<FormatElement>;
}

impl<T> Format for &T
where
    T: ?Sized + Format,
{
    type Context = T::Context;

    fn format(&self, formatter: &Formatter<Self::Context>) -> FormatResult<FormatElement> {
        Format::format(&**self, formatter)
    }
}

impl<T> Format for &mut T
where
    T: ?Sized + Format,
{
    type Context = T::Context;

    fn format(&self, formatter: &Formatter<Self::Context>) -> FormatResult<FormatElement> {
        Format::format(&**self, formatter)
    }
}

impl<T> Format for Option<T>
where
    T: Format,
{
    type Context = T::Context;

    fn format(&self, formatter: &Formatter<Self::Context>) -> FormatResult<FormatElement> {
        match self {
            Some(value) => value.format(formatter),
            None => Ok(empty_element()),
        }
    }
}

impl<T> Format for SyntaxResult<T>
where
    T: Format,
{
    type Context = T::Context;

    fn format(&self, formatter: &Formatter<Self::Context>) -> FormatResult<FormatElement> {
        match self {
            Ok(value) => value.format(formatter),
            Err(err) => Err(err.into()),
        }
    }
}

/// Implemented by traits that can be converted to a `FormatElement`.
///
/// This is similar to [Format] but with the difference that it consumes `self`, allowing it to also
/// be implemented on [FormatElement].format_elements.rs
pub trait IntoFormatElement<O> {
    fn into_format_element(self, formatter: &Formatter<O>) -> FormatResult<FormatElement>;
}

impl<O> IntoFormatElement<O> for FormatElement {
    #[inline]
    fn into_format_element(self, _: &Formatter<O>) -> FormatResult<FormatElement> {
        Ok(self)
    }
}

impl<O> IntoFormatElement<O> for FormatResult<FormatElement> {
    #[inline]
    fn into_format_element(self, _: &Formatter<O>) -> FormatResult<FormatElement> {
        self
    }
}

impl<T, O> IntoFormatElement<O> for T
where
    T: Format<Context = O>,
{
    #[inline]
    fn into_format_element(self, formatter: &Formatter<O>) -> FormatResult<FormatElement> {
        self.format(formatter)
    }
}

/// Rule that knows how to format an object of type [T].
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

    fn format(item: &T, formatter: &Formatter<Self::Context>) -> FormatResult<FormatElement>;
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
/// itself but the language agnostic crate implements `AsFormat` and `IntoFormat` for it and the returned [Format]
/// implement [FormatWithRule].
///
/// ```
/// use rome_formatter::prelude::*;
/// use rome_formatter::{FormatContext, FormatWithRule};
/// use rome_rowan::{Language, SyntaxNode};
/// fn format_node<L: Language, F: FormatWithRule<Item=SyntaxNode<L>, Context=()>>(node: F) -> FormatResult<FormatElement> {
///     let formatter = Formatter::default();
///
///     let formatted = node.format(&formatter);
///     let _syntax = node.item();
///
///     // Do something with syntax
///     formatted
/// }
/// ```
pub trait FormatWithRule: Format {
    type Item;

    /// Returns the associated item
    fn item(&self) -> &Self::Item;
}

/// Formats the referenced `item` with the specified rule.
pub struct FormatRefWithRule<'a, T, R>
where
    R: FormatRule<T>,
{
    item: &'a T,
    rule: PhantomData<R>,
}

impl<'a, T, R> FormatRefWithRule<'a, T, R>
where
    R: FormatRule<T>,
{
    pub fn new(item: &'a T) -> Self {
        Self {
            item,
            rule: PhantomData,
        }
    }
}

impl<T, R> FormatWithRule for FormatRefWithRule<'_, T, R>
where
    R: FormatRule<T>,
{
    type Item = T;

    fn item(&self) -> &Self::Item {
        self.item
    }
}

impl<T, R> Format for FormatRefWithRule<'_, T, R>
where
    R: FormatRule<T>,
{
    type Context = R::Context;

    #[inline]
    fn format(&self, formatter: &Formatter<R::Context>) -> FormatResult<FormatElement> {
        R::format(self.item, formatter)
    }
}

/// Formats the `item` with the specified rule.
pub struct FormatOwnedWithRule<T, R>
where
    R: FormatRule<T>,
{
    item: T,
    rule: PhantomData<R>,
}

impl<T, R> FormatOwnedWithRule<T, R>
where
    R: FormatRule<T>,
{
    pub fn new(item: T) -> Self {
        Self {
            item,
            rule: PhantomData,
        }
    }

    pub fn with_item(mut self, item: T) -> Self {
        self.item = item;
        self
    }

    pub fn into_item(self) -> T {
        self.item
    }
}

impl<T, R> Format for FormatOwnedWithRule<T, R>
where
    R: FormatRule<T>,
{
    type Context = R::Context;

    #[inline]
    fn format(&self, formatter: &Formatter<R::Context>) -> FormatResult<FormatElement> {
        R::format(&self.item, formatter)
    }
}

impl<T, R> FormatWithRule for FormatOwnedWithRule<T, R>
where
    R: FormatRule<T>,
{
    type Item = T;

    fn item(&self) -> &Self::Item {
        &self.item
    }
}

/// Formats any value that implements [Format].
///
/// Please note that [format_node] is preferred to format a [JsSyntaxNode]
pub fn format<C: FormatContext>(
    options: C,
    root: &dyn Format<Context = C>,
) -> FormatResult<Formatted> {
    tracing::trace_span!("format").in_scope(move || {
        let printer_options = options.as_print_options();
        let formatter = Formatter::new(options);
        let element = root.format(&formatter)?;
        Ok(Formatted::new(element, printer_options))
    })
}

/// Formats a syntax node file based on its features.
///
/// It returns a [Formatted] result, which the user can use to override a file.
pub fn format_node<
    C: FormatContext,
    L: Language,
    N: FormatWithRule<Item = SyntaxNode<L>, Context = C>,
>(
    options: C,
    root: &N,
) -> FormatResult<Formatted> {
    tracing::trace_span!("format_node").in_scope(move || {
        let printer_options = options.as_print_options();
        let formatter = Formatter::new(options);
        let element = formatted![&formatter, [root]]?;

        formatter.assert_formatted_all_tokens(root.item());

        Ok(Formatted::new(element, printer_options))
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
pub fn format_range<Context, L, R, P>(
    options: Context,
    root: &SyntaxNode<L>,
    mut range: TextRange,
    mut predicate: P,
) -> FormatResult<Printed>
where
    Context: FormatContext,
    L: Language,
    R: FormatRule<SyntaxNode<L>, Context = Context>,
    P: FnMut(&SyntaxNode<L>) -> bool,
{
    if range.is_empty() {
        return Ok(Printed::new(
            String::new(),
            Some(range),
            Vec::new(),
            Vec::new(),
        ));
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
    let formatted = format_sub_tree(options, &FormatRefWithRule::<_, R>::new(common_root))?;

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
    C: FormatContext,
    L: Language,
    N: FormatWithRule<Item = SyntaxNode<L>, Context = C>,
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
