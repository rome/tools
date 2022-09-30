use crate::comments::{FormatJsLeadingComment, JsCommentStyle, JsComments};
use rome_formatter::printer::PrinterOptions;
use rome_formatter::{
    CstFormatContext, FormatContext, FormatElement, FormatOptions, IndentStyle, LineWidth,
    TransformSourceMap,
};
use rome_js_syntax::{JsAnyFunctionBody, JsLanguage, SourceType};
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct JsFormatContext {
    options: JsFormatOptions,

    /// The comments of the nodes and tokens in the program.
    comments: Rc<JsComments>,

    /// Stores the formatted content of one function body.
    ///
    /// Used during formatting of call arguments where function expressions and arrow function expressions
    /// are formatted a second time if they are the first or last call argument.
    ///
    /// Caching the body in the call arguments formatting is important. It minimises the cases
    /// where the algorithm is quadratic, in case the function or arrow expression contains another
    /// call expression with a function or call expression as first or last argument.
    ///
    /// It's sufficient to only store a single cached body to cover the vast majority of cases
    /// (there's no exception in any of our tests nor benchmark tests). The only case not covered is when
    /// a parameter has an initializer that contains a call expression:
    ///
    /// ```javascript
    ///  test((
    ///    problematic = test(() => body)
    ///  ) => {});
    ///  ```
    ///
    /// This should be rare enough for us not to care about it.
    cached_function_body: Option<(JsAnyFunctionBody, FormatElement)>,

    source_map: Option<TransformSourceMap>,
}

impl JsFormatContext {
    pub fn new(options: JsFormatOptions, comments: JsComments) -> Self {
        Self {
            options,
            comments: Rc::new(comments),
            cached_function_body: None,
            source_map: None,
        }
    }

    /// Returns the formatted content for the passed function body if it is cached or `None` if the currently
    /// cached content belongs to another function body or the cache is empty.
    ///
    /// See [JsFormatContext::cached_function_body] for more in depth documentation.
    pub(crate) fn get_cached_function_body(
        &self,
        body: &JsAnyFunctionBody,
    ) -> Option<FormatElement> {
        self.cached_function_body
            .as_ref()
            .and_then(|(expected_body, formatted)| {
                if expected_body == body {
                    Some(formatted.clone())
                } else {
                    None
                }
            })
    }

    /// Sets the currently cached formatted function body.
    ///
    /// See [JsFormatContext::cached_function_body] for more in depth documentation.
    pub(crate) fn set_cached_function_body(
        &mut self,
        body: &JsAnyFunctionBody,
        formatted: FormatElement,
    ) {
        self.cached_function_body = Some((body.clone(), formatted))
    }

    pub fn with_source_map(mut self, source_map: Option<TransformSourceMap>) -> Self {
        self.source_map = source_map;
        self
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct TabWidth(u8);

impl From<u8> for TabWidth {
    fn from(value: u8) -> Self {
        TabWidth(value)
    }
}

impl From<TabWidth> for u8 {
    fn from(width: TabWidth) -> Self {
        width.0
    }
}

impl FormatContext for JsFormatContext {
    type Options = JsFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        self.source_map.as_ref()
    }
}

impl CstFormatContext for JsFormatContext {
    type Language = JsLanguage;
    type Style = JsCommentStyle;
    type CommentRule = FormatJsLeadingComment;

    fn comments(&self) -> &JsComments {
        &self.comments
    }
}

#[derive(Debug, Clone)]
pub struct JsFormatOptions {
    /// The indent style.
    indent_style: IndentStyle,

    /// What's the max width of a line. Defaults to 80.
    line_width: LineWidth,

    /// The style for quotes. Defaults to double.
    quote_style: QuoteStyle,

    /// When properties in objects are quoted. Defaults to as-needed.
    quote_properties: QuoteProperties,

    /// Information related to the current file
    source_type: SourceType,
}

impl JsFormatOptions {
    pub fn new(source_type: SourceType) -> Self {
        Self {
            source_type,
            indent_style: IndentStyle::default(),
            line_width: LineWidth::default(),
            quote_style: QuoteStyle::default(),
            quote_properties: QuoteProperties::default(),
        }
    }

    pub fn with_indent_style(mut self, indent_style: IndentStyle) -> Self {
        self.indent_style = indent_style;
        self
    }

    pub fn with_line_width(mut self, line_width: LineWidth) -> Self {
        self.line_width = line_width;
        self
    }

    pub fn with_quote_style(mut self, quote_style: QuoteStyle) -> Self {
        self.quote_style = quote_style;
        self
    }

    pub fn with_quote_properties(mut self, quote_properties: QuoteProperties) -> Self {
        self.quote_properties = quote_properties;
        self
    }

    pub fn quote_style(&self) -> QuoteStyle {
        self.quote_style
    }

    pub fn quote_properties(&self) -> QuoteProperties {
        self.quote_properties
    }

    pub fn source_type(&self) -> SourceType {
        self.source_type
    }

    pub fn tab_width(&self) -> TabWidth {
        match self.indent_style {
            IndentStyle::Tab => 2.into(),
            IndentStyle::Space(quantities) => quantities.into(),
        }
    }
}

impl FormatOptions for JsFormatOptions {
    fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    fn line_width(&self) -> LineWidth {
        self.line_width
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions::default()
            .with_indent(self.indent_style)
            .with_print_width(self.line_width.into())
    }
}

impl fmt::Display for JsFormatOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Line width: {}", self.line_width.value())?;
        writeln!(f, "Quote style: {}", self.quote_style)?;
        writeln!(f, "Quote properties: {}", self.quote_properties)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
#[derive(Default)]
pub enum QuoteStyle {
    #[default]
    Double,
    Single,
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

impl QuoteStyle {
    pub fn as_char(&self) -> char {
        match self {
            QuoteStyle::Double => '"',
            QuoteStyle::Single => '\'',
        }
    }

    pub fn as_string(&self) -> &str {
        match self {
            QuoteStyle::Double => "\"",
            QuoteStyle::Single => "'",
        }
    }

    /// Returns the quote, prepended with a backslash (escaped)
    pub fn as_escaped(&self) -> &str {
        match self {
            QuoteStyle::Double => "\\\"",
            QuoteStyle::Single => "\\'",
        }
    }

    pub fn as_bytes(&self) -> u8 {
        self.as_char() as u8
    }

    /// Returns the quote in HTML entity
    pub fn as_html_entity(&self) -> &str {
        match self {
            QuoteStyle::Double => "&quot;",
            QuoteStyle::Single => "&apos;",
        }
    }

    /// Given the current quote, it returns the other one
    pub fn other(&self) -> Self {
        match self {
            QuoteStyle::Double => QuoteStyle::Single,
            QuoteStyle::Single => QuoteStyle::Double,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub enum QuoteProperties {
    AsNeeded,
    Preserve,
}

impl Default for QuoteProperties {
    fn default() -> Self {
        Self::AsNeeded
    }
}

impl FromStr for QuoteProperties {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "as-needed" | "AsNeeded" => Ok(Self::AsNeeded),
            "preserve" | "Preserve" => Ok(Self::Preserve),
            // TODO: replace this error with a diagnostic
            _ => Err("Value not supported for QuoteProperties"),
        }
    }
}

impl fmt::Display for QuoteProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteProperties::AsNeeded => write!(f, "As needed"),
            QuoteProperties::Preserve => write!(f, "Preserve"),
        }
    }
}
