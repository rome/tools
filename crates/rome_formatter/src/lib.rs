mod builders;
pub mod format_element;
mod format_extensions;
pub mod formatter;
pub mod intersperse;
pub mod macros;
pub mod prelude;
#[cfg(debug_assertions)]
pub mod printed_tokens;
pub mod printer;

use crate::formatter::Formatter;
use crate::printer::Printer;
pub use builders::ConcatBuilder;
pub use format_element::{
    block_indent, comment, concat_elements, empty_element, empty_line, fill_elements,
    group_elements, hard_group_elements, hard_line_break, if_group_breaks,
    if_group_fits_on_single_line, indent, join_elements, join_elements_hard_line,
    join_elements_soft_line, join_elements_with, line_suffix, normalize_newlines,
    soft_block_indent, soft_line_break, soft_line_break_or_space, soft_line_indent_or_space,
    space_token, token, FormatElement, Token, Verbatim, LINE_TERMINATORS,
};
use rome_rowan::{SyntaxError, SyntaxResult, TextRange, TextSize};
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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum QuoteStyle {
    Double,
    Single,
}

impl Default for QuoteStyle {
    fn default() -> Self {
        Self::Double
    }
}

impl FromStr for QuoteStyle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "double" | "Double" => Ok(Self::Double),
            "single" | "Single" => Ok(Self::Single),
            // TODO: replace this error with a diagnostic
            _ => Err("Value not supported for QuoteStyle"),
        }
    }
}

impl fmt::Display for QuoteStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteStyle::Double => write!(f, "Double Quotes"),
            QuoteStyle::Single => write!(f, "Single Quotes"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FormatOptions {
    /// The indent style.
    pub indent_style: IndentStyle,

    /// What's the max width of a line. Defaults to 80.
    pub line_width: LineWidth,

    // The style for quotes. Defaults to double.
    pub quote_style: QuoteStyle,
}

impl FormatOptions {
    pub fn new(indent_style: IndentStyle) -> Self {
        Self {
            indent_style,
            ..Self::default()
        }
    }

    /// Given the current ident style, it returns its width
    pub fn tab_width(&self) -> u8 {
        match self.indent_style {
            IndentStyle::Tab => 2,
            IndentStyle::Space(quantity) => quantity,
        }
    }
}

impl fmt::Display for FormatOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Line width: {}", self.line_width.value())?;
        writeln!(f, "Quote style: {}", self.quote_style)?;
        Ok(())
    }
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
    options: FormatOptions,
}

impl Formatted {
    pub fn new(root: FormatElement, options: FormatOptions) -> Self {
        Self { root, options }
    }

    pub fn print(&self) -> Printed {
        Printer::new(self.options).print(&self.root)
    }

    pub fn print_with_indent(&self, indent: u16) -> Printed {
        Printer::new(self.options).print_with_indent(&self.root, indent)
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
/// use rome_formatter::{format, FormatOptions};
/// use rome_formatter::prelude::*;
/// use rome_rowan::TextSize;
///
/// struct Paragraph(String);
///
/// impl Format for Paragraph {fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
///         formatted![
///             formatter,
///             hard_line_break(),
///             FormatElement::from(Token::new_dynamic(self.0.clone(), TextSize::from(0))),
///             hard_line_break(),
///         ]
///     }
/// }
///
/// let paragraph = Paragraph(String::from("test"));
/// let printed = format(FormatOptions::default(), &paragraph).unwrap().print();
///
/// assert_eq!("test\n", printed.as_code())
/// ```
pub trait Format {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement>;
}

impl<T> Format for &T
where
    T: ?Sized + Format,
{
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Format::format(&**self, formatter)
    }
}

impl<T> Format for &mut T
where
    T: ?Sized + Format,
{
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Format::format(&**self, formatter)
    }
}

impl<T> Format for Option<T>
where
    T: Format,
{
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
pub trait IntoFormatElement {
    fn into_format_element(self, formatter: &Formatter) -> FormatResult<FormatElement>;
}

impl IntoFormatElement for FormatElement {
    #[inline]
    fn into_format_element(self, _: &Formatter) -> FormatResult<FormatElement> {
        Ok(self)
    }
}

impl<T> IntoFormatElement for T
where
    T: Format,
{
    #[inline]
    fn into_format_element(self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
    fn format(item: &T, formatter: &Formatter) -> FormatResult<FormatElement>;
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
/// use rome_formatter::{FormatOptions, FormatWithRule};
/// use rome_rowan::{Language, SyntaxNode};
/// fn format_node<L: Language, F: FormatWithRule<Item=SyntaxNode<L>>>(node: F) -> FormatResult<FormatElement> {
///     let formatter = Formatter::new(FormatOptions::default());
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
    #[inline]
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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

    pub fn into_item(self) -> T {
        self.item
    }
}

impl<T, R> Format for FormatOwnedWithRule<T, R>
where
    R: FormatRule<T>,
{
    #[inline]
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
pub fn format(options: FormatOptions, root: &dyn Format) -> FormatResult<Formatted> {
    tracing::trace_span!("format").in_scope(move || {
        let formatter = Formatter::new(options);
        let element = root.format(&formatter)?;
        Ok(Formatted::new(element, options))
    })
}
