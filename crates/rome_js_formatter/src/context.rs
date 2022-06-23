use rome_formatter::printer::PrinterOptions;
use rome_formatter::{
    CommentContext, CommentKind, CommentStyle, FormatContext, IndentStyle, LineWidth,
};
use rome_js_syntax::{JsLanguage, JsSyntaxKind, SourceType};
use rome_rowan::SyntaxTriviaPieceComments;
use std::fmt;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Default)]
pub struct JsFormatContext {
    /// The indent style.
    pub indent_style: IndentStyle,

    /// What's the max width of a line. Defaults to 80.
    pub line_width: LineWidth,

    /// Options of current instance of the formatter
    pub options: JsFormatOptions,

    /// Information relative to the current file
    pub source_type: SourceType,
}

impl JsFormatContext {
    pub fn new(options: JsFormatOptions, source_type: SourceType) -> Self {
        Self {
            options,
            source_type,
            ..JsFormatContext::default()
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

    pub fn quote_style(&self) -> QuoteStyle {
        self.options.quote_style
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct JsFormatOptions {
    // The style for quotes. Defaults to double.
    pub quote_style: QuoteStyle,
}

impl JsFormatContext {
    pub fn tab_width(&self) -> u8 {
        match self.indent_style {
            IndentStyle::Tab => 2,
            IndentStyle::Space(quantities) => quantities,
        }
    }
}

impl FormatContext for JsFormatContext {
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

impl fmt::Display for JsFormatContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Line width: {}", self.line_width.value())?;
        write!(f, "{}", self.options)?;
        Ok(())
    }
}

impl CommentContext<JsLanguage> for JsFormatContext {
    type Style = JsCommentStyle;

    fn comment_style(&self) -> Self::Style {
        JsCommentStyle
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct JsCommentStyle;

impl CommentStyle<JsLanguage> for JsCommentStyle {
    fn get_comment_kind(&self, comment: &SyntaxTriviaPieceComments<JsLanguage>) -> CommentKind {
        if comment.text().starts_with("/*") {
            if comment.has_newline() {
                CommentKind::Block
            } else {
                CommentKind::InlineBlock
            }
        } else {
            CommentKind::Line
        }
    }

    fn is_group_start_token(&self, kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JsSyntaxKind::L_PAREN | JsSyntaxKind::L_BRACK | JsSyntaxKind::L_CURLY
        )
    }

    fn is_group_end_token(&self, kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JsSyntaxKind::R_BRACK
                | JsSyntaxKind::R_CURLY
                | JsSyntaxKind::R_PAREN
                | JsSyntaxKind::COMMA
                | JsSyntaxKind::SEMICOLON
                | JsSyntaxKind::DOT
                | JsSyntaxKind::EOF
        )
    }
}

impl fmt::Display for JsFormatOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Quote style: {}", self.quote_style)?;
        Ok(())
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
