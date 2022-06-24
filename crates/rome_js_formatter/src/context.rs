#[cfg(debug_assertions)]
use indexmap::IndexSet;
use rome_formatter::printer::PrinterOptions;
use rome_formatter::{
    CommentContext, CommentKind, CommentStyle, FormatContext, IndentStyle, LineWidth,
};
use rome_js_syntax::suppression::{has_suppressions_category, SuppressionCategory};
use rome_js_syntax::{JsLanguage, JsSyntaxKind, JsSyntaxNode, SourceType};
use rome_rowan::SyntaxTriviaPieceComments;
use std::fmt;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct JsFormatContext {
    /// The indent style.
    indent_style: IndentStyle,

    /// What's the max width of a line. Defaults to 80.
    line_width: LineWidth,

    /// The style for quotes. Defaults to double.
    quote_style: QuoteStyle,

    /// Information relative to the current file
    source_type: SourceType,

    #[cfg(debug_assertions)]
    checked_node_suppressions: IndexSet<JsSyntaxNode>,
}

impl JsFormatContext {
    pub fn new(source_type: SourceType) -> Self {
        Self {
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

    pub fn with_quote_style(mut self, quote_style: QuoteStyle) -> Self {
        self.quote_style = quote_style;
        self
    }

    pub fn with_source_type(mut self, source_type: SourceType) -> Self {
        self.source_type = source_type;
        self
    }

    pub fn line_width(&self) -> LineWidth {
        self.line_width
    }

    pub fn quote_style(&self) -> QuoteStyle {
        self.quote_style
    }

    pub fn source_type(&self) -> SourceType {
        self.source_type
    }

    /// Returns `true` if the passed node has a suppression comment and
    /// tracks in debug builds that the suppression comments of this node have been checked.
    #[inline]
    pub(crate) fn is_suppressed(&mut self, node: &JsSyntaxNode) -> bool {
        self.checked_suppressed(node);
        has_suppressions_category(SuppressionCategory::Format, node)
    }

    /// Tracks that the formatting manually checked if the passed in node has a suppression comment
    /// in debug builds.
    pub(crate) fn checked_suppressed(&mut self, #[allow(unused_variables)] node: &JsSyntaxNode) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                self.checked_node_suppressions.insert(node.clone());
            }
        }
    }

    /// Asserts that the suppression comments of all nodes have been handled in debug builds.
    ///
    /// # Panics
    /// If the suppression comments of a node have not been checked.
    #[inline]
    pub(crate) fn assert_checked_all_suppressions(
        &self,
        #[allow(unused_variables)] root: &JsSyntaxNode,
    ) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                for node in root.descendants() {
                    if !self.checked_node_suppressions.contains(&node) {
                        // No need to explicitly check nodes where the node is the first element of its parent
                        // (in which case the suppression would suppress the parent node), or when
                        // the node is a list for which Rome doesn't support suppression comments
                        if node.prev_sibling_or_token().is_none() || node.kind().is_list() {
                            continue;
                        }

                        panic!(r#"The node {node:#?} has been formatted without checking if it has suppression comments.
Ensure that the formatter calls into the node's formatting rule by using `node.format()` or
manually test if the node has a suppression comment using `f.context_mut().is_suppressed(node.syntax())` if using the node's format rule isn't an option"#)
                    }
                }
            }
        }
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

impl JsFormatContext {
    pub fn tab_width(&self) -> TabWidth {
        match self.indent_style {
            IndentStyle::Tab => 2.into(),
            IndentStyle::Space(quantities) => quantities.into(),
        }
    }
}

impl FormatContext for JsFormatContext {
    type Snapshot = JsFormatContextSnapshot;

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

    fn snapshot(&self) -> Self::Snapshot {
        JsFormatContextSnapshot {
            #[cfg(debug_assertions)]
            node_suppressions_len: self.checked_node_suppressions.len(),
        }
    }

    fn restore_snapshot(&mut self, #[allow(unused)] snapshot: Self::Snapshot) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                self.checked_node_suppressions.truncate(snapshot.node_suppressions_len);
            }
        }
    }
}

pub struct JsFormatContextSnapshot {
    #[cfg(debug_assertions)]
    node_suppressions_len: usize,
}

impl fmt::Display for JsFormatContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Line width: {}", self.line_width.value())?;
        writeln!(f, "Quote style: {}", self.quote_style)
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
