pub mod format_element;
pub mod format_elements;
pub mod intersperse;
pub mod printer;
use crate::printer::Printer;
pub use format_element::{
    block_indent, comment, concat_elements, empty_element, empty_line, fill_elements,
    group_elements, hard_group_elements, hard_line_break, if_group_breaks,
    if_group_fits_on_single_line, indent, join_elements, join_elements_hard_line,
    join_elements_soft_line, join_elements_with, line_suffix, normalize_newlines,
    soft_block_indent, soft_line_break, soft_line_break_or_space, soft_line_indent_or_space,
    space_token, token, FormatElement, Token, Verbatim, LINE_TERMINATORS,
};
use rome_rowan::{TextRange, TextSize};
use std::fmt::{self, Display, Formatter};
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

impl Display for IndentStyle {
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

impl Display for ParseLineWidthError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
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

impl Display for QuoteStyle {
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
}

impl Display for FormatOptions {
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Formatted {
    code: String,
    range: Option<TextRange>,
    sourcemap: Vec<SourceMarker>,
    verbatim_source: Vec<(String, TextRange)>,
}

impl Formatted {
    pub fn new(
        code: String,
        range: Option<TextRange>,
        sourcemap: Vec<SourceMarker>,
        verbatim_source: Vec<(String, TextRange)>,
    ) -> Self {
        Self {
            code,
            range,
            sourcemap,
            verbatim_source,
        }
    }

    /// Construct an empty formatter result
    pub fn new_empty() -> Self {
        Self {
            code: String::new(),
            range: None,
            sourcemap: Vec::new(),
            verbatim_source: Vec::new(),
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

    pub fn verbatim(&self) -> &[(String, TextRange)] {
        &self.verbatim_source
    }

    pub fn into_verbatim(self) -> Vec<(String, TextRange)> {
        self.verbatim_source
    }
}

pub fn format_element(element: &FormatElement, options: FormatOptions) -> Formatted {
    let printer = Printer::new(options);
    printer.print(element)
}
