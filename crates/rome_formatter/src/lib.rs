//! Rome's official formatter.
//!
//! The crate exposes some API and utilities to implement the formatting logic.
//!
//! The formatter relies on an [IR], which allows to format any kind of data structure.
//!
//! In order to implement the formatting logic, you need to implement the trait [FormatValue] for
//! the data structure you want to format.
//!
//! Let's say, for example that you have a small data structure that represents a key/value data:
//!
//! ```rust,no_test
//! struct KeyValue {
//!     key: &'static str,
//!     value: &'static str
//! }
//! ```
//!
//! Now, we do want to create this IR for the data structure:
//! ```rust
//! use rome_formatter::{format_elements, format_element, Formatter, ToFormatElement, FormatElement, FormatResult, FormatOptions, space_token, token };
//!
//! struct KeyValue {
//!     key: &'static str,
//!     value: &'static str
//! }
//!
//! impl ToFormatElement for KeyValue {
//!     fn to_format_element(&self, formatter: &Formatter)-> FormatResult<FormatElement>  {
//!         Ok(format_elements![
//!             token(self.key),
//!             space_token(),
//!             token("=>"),
//!             space_token(),
//!             token(self.value)
//!         ])
//!     }
//! }
//!
//! fn my_function() {
//!     let key_value = KeyValue { key: "lorem", value: "ipsum" };
//!     let element = key_value.to_format_element(&Formatter::default()).unwrap();
//!     let result = format_element(&element, FormatOptions::default());
//!     assert_eq!(result.code(), "lorem => ipsum");
//! }
//!
//! ```
//! [IR]: https://en.wikipedia.org/wiki/Intermediate_representation

mod cst;
mod format_element;
mod format_elements;
mod formatter;
mod intersperse;
mod printer;
mod ts;
pub use formatter::Formatter;
use rome_rowan::TextRange;
use rome_rowan::TextSize;
use rslint_parser::{parse, Syntax, SyntaxError, SyntaxNode};

pub use format_element::{
    block_indent, concat_elements, empty_element, empty_line, group_elements, hard_line_break,
    if_group_breaks, if_group_fits_on_single_line, indent, join_elements, join_elements_hard_line,
    line_suffix, soft_indent, soft_line_break, soft_line_break_or_space, space_token, token,
    FormatElement,
};
pub use printer::Printer;
pub use printer::PrinterOptions;
use rome_core::App;
use rome_path::RomePath;
use std::str::FromStr;
use thiserror::Error;

/// This trait should be implemented on each node/value that should have a formatted representation
pub trait ToFormatElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement>;
}

/// Public return type of the formatter
pub type FormatResult<F> = Result<F, FormatError>;

#[derive(Debug, PartialEq, Error)]
/// Series of errors encountered during formatting
pub enum FormatError {
    /// Node is missing and it should be required for a correct formatting
    #[error("missing required child")]
    MissingRequiredChild,

    /// In case our formatter doesn't know how to format a certain language
    #[error("language is not supported")]
    UnsupportedLanguage,

    /// When the ability to format the current file has been turned off on purpose
    #[error("formatting capability is disabled")]
    CapabilityDisabled,
}

impl From<SyntaxError> for FormatError {
    fn from(syntax_error: SyntaxError) -> Self {
        match syntax_error {
            SyntaxError::MissingRequiredChild(_node) => FormatError::MissingRequiredChild,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum IndentStyle {
    /// Tab
    Tab,
    /// Space, with its quantity
    Space(u8),
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
            "tab" => Ok(Self::Tab),
            "space" => Ok(Self::Space(2)),
            // TODO: replace this error with a diagnostic
            _ => Err("Value not supported for IndentStyle"),
        }
    }
}

#[derive(Debug)]
pub struct FormatOptions {
    /// The indent style
    pub indent_style: IndentStyle,

    /// What's the max width of a line. Defaults to 80
    pub line_width: u16,
}

impl FormatOptions {
    pub fn new(indent_style: IndentStyle) -> Self {
        Self {
            indent_style,
            ..Self::default()
        }
    }
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            indent_style: IndentStyle::default(),
            line_width: 80,
        }
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
}

impl Formatted {
    fn new(code: String, range: Option<TextRange>, sourcemap: Vec<SourceMarker>) -> Self {
        Self {
            code,
            range,
            sourcemap,
        }
    }

    /// Range of the input source file covered by this formatted code,
    /// or None if the entire file is covered in this instance
    pub fn range(&self) -> Option<TextRange> {
        self.range
    }

    pub fn sourcemap(&self) -> &[SourceMarker] {
        &self.sourcemap
    }

    pub fn code(&self) -> &String {
        &self.code
    }

    pub fn into_code(self) -> String {
        self.code
    }
}

/// Formats a JavaScript (and its super languages) file based on its features.
///
/// It returns a [Formatted] result, which the user can use to override a file.
pub fn format(options: FormatOptions, syntax: &SyntaxNode) -> FormatResult<Formatted> {
    Formatter::new(options).format_root(syntax)
}

/// Formats a range withing a JavaScript file
///
/// It returns a [Formatted] result with a range corresponding to the
/// range of the input that was effectively overwritten by the formatter
pub fn format_range(
    options: FormatOptions,
    root: &SyntaxNode,
    range: TextRange,
) -> FormatResult<Formatted> {
    let formatted = format(options, root)?;

    // This finds the closests marker to the beginning of the source
    // starting before or at said starting point, and the closest
    // marker to the end of the source range starting after or at
    // said ending point respectively
    let mut range_start = None;
    let mut range_end = None;

    for marker in &formatted.sourcemap {
        if let Some(start_dist) = marker.source.checked_sub(range.start()) {
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

        if let Some(end_dist) = range.end().checked_sub(marker.source) {
            range_end = match range_end {
                Some((prev_marker, prev_dist)) => {
                    if end_dist < prev_dist {
                        Some((marker, end_dist))
                    } else {
                        Some((prev_marker, prev_dist))
                    }
                }
                None => Some((marker, end_dist)),
            }
        }
    }

    // If no start or end were found this means either the input was empty,
    // or the edge of the formatting range was near the edge of the input
    // and no marker was emitted before the start (or after the end) of the
    // formatting range: in this case the start/end marker default to the
    // start/end of the input
    let (start_source, start_dest) = match range_start {
        Some((start_marker, _)) => (start_marker.source, start_marker.dest),
        None => (TextSize::from(0), TextSize::from(0)),
    };
    let (end_source, end_dest) = match range_end {
        Some((end_marker, _)) => (end_marker.source, end_marker.dest),
        None => {
            let end = root.text().len();
            (end, end)
        }
    };

    let input_range = TextRange::new(start_source, end_source);
    let output_range = TextRange::new(start_dest, end_dest);
    let code = &formatted.code[output_range];

    Ok(Formatted::new(
        code.into(),
        Some(input_range),
        formatted.sourcemap,
    ))
}

pub fn format_element(element: &FormatElement, options: FormatOptions) -> Formatted {
    let printer = Printer::new(options);
    printer.print(element)
}

pub fn format_file_and_save(rome_path: &mut RomePath, options: FormatOptions, app: &App) {
    let result = if app.can_format(rome_path) {
        let buffer = rome_path.get_buffer_from_file();
        let syntax = Syntax::default().module();
        let root = parse(buffer.as_str(), 0, syntax).syntax();
        Some(format(options, &root))
    } else {
        None
    };
    if let Some(Ok(result)) = result {
        rome_path
            .save(result.code())
            .expect("Could not write the formatted code on file");
    }
}

#[cfg(test)]
mod tests {
    use super::{format_range, FormatOptions};

    use rome_rowan::{TextRange, TextSize};
    use rslint_parser::parse_script;

    #[test]
    fn test_range_formatting() {
        let input = "
        
        func(     /* comment */
        );
        
        let array =
            [ 1
        , 2];
        
        const no_format    =    () => {};

        ";

        let range_start = TextSize::try_from(input.find("let").unwrap()).unwrap();
        let range_end = TextSize::try_from(input.find("const").unwrap()).unwrap();

        let tree = parse_script(input, 0);
        let result = format_range(
            FormatOptions::default(),
            &tree.syntax(),
            TextRange::new(range_start, range_end),
        );

        let result = result.expect("range formatting failed");
        assert_eq!(result.range(), Some(TextRange::new(range_start, range_end)));
        assert_eq!(result.code(), "let array = [1, 2];\n\n");
    }
}
