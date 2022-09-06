use crate::prelude::*;
use crate::AsFormat;
use rome_formatter::token::{FormatInserted, FormatInsertedCloseParen, FormatInsertedOpenParen};
use rome_formatter::{
    format_args, write, Argument, Arguments, CstFormatContext, FormatContext, GroupId,
    PreambleBuffer, VecBuffer,
};
use rome_js_syntax::{JsLanguage, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken};
use rome_rowan::{AstNode, Direction, Language, SyntaxElement, SyntaxTriviaPiece, TextRange};

/// Formats a node using its [`AsFormat`] implementation but falls back to printing the node as
/// it is in the source document if the formatting returns an [`FormatError`].
pub const fn format_or_verbatim<'a, Node>(node: &'a Node) -> FormatNodeOrVerbatim<'a, Node>
where
    Node: AstNode<Language = JsLanguage> + AsFormat<'a>,
{
    FormatNodeOrVerbatim { node }
}

/// Formats a node or falls back to verbatim printing if formating this node fails.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FormatNodeOrVerbatim<'a, Node> {
    node: &'a Node,
}

impl<'a, Node> Format<JsFormatContext> for FormatNodeOrVerbatim<'a, Node>
where
    Node: AstNode<Language = JsLanguage> + AsFormat<'a>,
{
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let snapshot = Formatter::state_snapshot(f);

        match self.node.format().fmt(f) {
            Ok(result) => Ok(result),

            Err(_) => {
                f.restore_state_snapshot(snapshot);

                // Lists that yield errors are formatted as they were unknown nodes.
                // Doing so, the formatter formats the nodes/tokens as is.
                format_unknown_node(self.node.syntax()).fmt(f)
            }
        }
    }
}

pub fn format_inserted(kind: JsSyntaxKind) -> FormatInserted<JsLanguage> {
    FormatInserted::new(
        kind,
        kind.to_string().expect("Expected a punctuation token"),
    )
}

pub fn format_inserted_open_paren(
    before_token: Option<&JsSyntaxToken>,
    kind: JsSyntaxKind,
) -> FormatInsertedOpenParen<JsLanguage> {
    FormatInsertedOpenParen::new(
        before_token,
        kind,
        kind.to_string()
            .expect("Expected a punctuation token as the open paren token."),
    )
}

pub fn format_inserted_close_paren(
    after_token: Option<&JsSyntaxToken>,
    kind: JsSyntaxKind,
    f: &mut JsFormatter,
) -> FormatInsertedCloseParen<JsLanguage> {
    FormatInsertedCloseParen::after_token(
        after_token,
        kind,
        kind.to_string()
            .expect("Expected a punctuation token as the close paren token."),
        f,
    )
}

/// Adds parentheses around some content
/// Ensures that the leading trivia of the `first_content_token` is moved
/// before the opening parentheses and the trailing trivia of the `last_content_token`
/// is moved after the closing parentheses.
///
/// # Examples
/// Adding parentheses around the string literal
///
/// ```javascript
/// /* leading */ "test" /* trailing */;
/// ```
///
/// becomes
///
/// ```javascript
/// /* leading */ ("test") /* trailing */;
/// ```
pub fn format_parenthesize<'a, Content>(
    first_content_token: Option<&'a JsSyntaxToken>,
    content: &'a Content,
    last_content_token: Option<&'a JsSyntaxToken>,
) -> FormatParenthesize<'a>
where
    Content: Format<JsFormatContext>,
{
    FormatParenthesize {
        first_content_token,
        content: Argument::new(content),
        last_content_token,
        grouped: false,
    }
}

/// Adds parentheses around an expression
#[derive(Clone)]
pub struct FormatParenthesize<'a> {
    grouped: bool,
    first_content_token: Option<&'a JsSyntaxToken>,
    content: Argument<'a, JsFormatContext>,
    last_content_token: Option<&'a JsSyntaxToken>,
}

impl FormatParenthesize<'_> {
    /// Groups the open parenthesis, the content, and the closing parenthesis inside of a group
    /// and indents the content with a soft block indent.
    pub fn grouped_with_soft_block_indent(mut self) -> Self {
        self.grouped = true;
        self
    }
}

impl Format<JsFormatContext> for FormatParenthesize<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let format_open_paren =
            format_inserted_open_paren(self.first_content_token, JsSyntaxKind::L_PAREN);
        let format_close_paren =
            format_inserted_close_paren(self.last_content_token, JsSyntaxKind::R_PAREN, f);

        if self.grouped {
            write!(
                f,
                [group(&format_args![
                    format_open_paren,
                    soft_block_indent(&Arguments::from(&self.content)),
                    format_close_paren
                ])]
            )
        } else {
            write!(
                f,
                [
                    format_open_paren,
                    Arguments::from(&self.content),
                    format_close_paren
                ]
            )
        }
    }
}

/// "Formats" a node according to its original formatting in the source text. Being able to format
/// a node "as is" is useful if a node contains syntax errors. Formatting a node with syntax errors
/// has the risk that Rome misinterprets the structure of the code and formatting it could
/// "mess up" the developers, yet incomplete, work or accidentally introduce new syntax errors.
///
/// You may be inclined to call `node.text` directly. However, using `text` doesn't track the nodes
/// nor its children source mapping information, resulting in incorrect source maps for this subtree.
///
/// These nodes and tokens get tracked as [FormatElement::Verbatim], useful to understand
/// if these nodes still need to have their own implementation.
pub fn format_verbatim_node(node: &JsSyntaxNode) -> FormatVerbatimNode {
    FormatVerbatimNode {
        node,
        kind: VerbatimKind::Verbatim {
            length: node.text_range().len(),
        },
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FormatVerbatimNode<'node> {
    node: &'node JsSyntaxNode,
    kind: VerbatimKind,
}
impl Format<JsFormatContext> for FormatVerbatimNode<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        for element in self.node.descendants_with_tokens(Direction::Next) {
            match element {
                SyntaxElement::Token(token) => f.state_mut().track_token(&token),
                SyntaxElement::Node(node) => {
                    f.context().comments().mark_suppression_checked(&node);
                }
            }
        }

        fn skip_whitespace<L: Language>(piece: &SyntaxTriviaPiece<L>) -> bool {
            piece.is_newline() || piece.is_whitespace()
        }

        fn write_trivia_token<L: Language>(
            f: &mut JsFormatter,
            piece: SyntaxTriviaPiece<L>,
        ) -> FormatResult<()> {
            syntax_token_cow_slice(
                normalize_newlines(piece.text(), LINE_TERMINATORS),
                &piece.token(),
                piece.text_range().start(),
            )
            .fmt(f)
        }

        let trimmed_source_range = f.context().source_map().map_or_else(
            || self.node.text_trimmed_range(),
            |source_map| source_map.trimmed_source_range(self.node),
        );

        let mut buffer = VecBuffer::new(f.state_mut());

        write!(
            buffer,
            [format_with(|f: &mut JsFormatter| {
                fn source_range(f: &JsFormatter, range: TextRange) -> TextRange {
                    f.context()
                        .source_map()
                        .map_or_else(|| range, |source_map| source_map.source_range(range))
                }

                for leading_trivia in self
                    .node
                    .first_leading_trivia()
                    .into_iter()
                    .flat_map(|trivia| trivia.pieces())
                    .skip_while(skip_whitespace)
                {
                    let trivia_source_range = source_range(f, leading_trivia.text_range());

                    if trivia_source_range.start() >= trimmed_source_range.start() {
                        break;
                    }

                    write_trivia_token(f, leading_trivia)?;
                }

                let original_source = f
                    .context()
                    .source_map()
                    .map_or_else(
                        || self.node.text_trimmed(),
                        |source_map| source_map.text().slice(trimmed_source_range),
                    )
                    .to_string();

                dynamic_text(
                    &normalize_newlines(&original_source, LINE_TERMINATORS),
                    self.node.text_trimmed_range().start(),
                )
                .fmt(f)?;

                let mut trailing_trivia = self
                    .node
                    .last_trailing_trivia()
                    .into_iter()
                    .flat_map(|trivia| trivia.pieces());

                let mut trailing_back = trailing_trivia.by_ref().rev().peekable();

                while let Some(trailing) = trailing_back.peek() {
                    let is_whitespace = skip_whitespace(trailing);

                    let trailing_source_range = source_range(f, trailing.text_range());
                    let is_in_trimmed_range =
                        trailing_source_range.start() < trimmed_source_range.end();

                    if is_whitespace || is_in_trimmed_range {
                        trailing_back.next();
                    } else {
                        break;
                    }
                }

                for trailing_trivia in trailing_trivia {
                    write_trivia_token(f, trailing_trivia)?;
                }

                Ok(())
            })]
        )?;

        let content = buffer.into_vec();

        let verbatim = Verbatim {
            content: content.into_boxed_slice(),
            kind: self.kind,
        };

        f.write_element(FormatElement::Verbatim(verbatim))
    }
}

/// Formats unknown nodes. The difference between this method  and `format_verbatim` is that this method
/// doesn't track nodes/tokens as [FormatElement::Verbatim]. They are just printed as they are.
pub fn format_unknown_node(node: &JsSyntaxNode) -> FormatUnknownNode {
    FormatUnknownNode { node }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FormatUnknownNode<'node> {
    node: &'node JsSyntaxNode,
}

impl Format<JsFormatContext> for FormatUnknownNode<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        FormatVerbatimNode {
            node: self.node,
            kind: VerbatimKind::Unknown,
        }
        .fmt(f)
    }
}

/// Format a node having formatter suppression comment applied to it
pub fn format_suppressed_node(node: &JsSyntaxNode) -> FormatSuppressedNode {
    FormatSuppressedNode { node }
}

#[derive(Debug, Clone)]
pub struct FormatSuppressedNode<'node> {
    node: &'node JsSyntaxNode,
}

impl Format<JsFormatContext> for FormatSuppressedNode<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        // Insert a force a line break to ensure the suppression comment is on its own line
        // and correctly registers as a leading trivia on the opening token of this node
        write!(
            f,
            [
                hard_line_break(),
                FormatVerbatimNode {
                    node: self.node,
                    kind: VerbatimKind::Suppressed
                }
            ]
        )
    }
}

/// Formats a group delimited by an opening and closing token,
/// such as a function body delimited by '{' and '}' tokens
///
/// Calling this method is required to correctly handle the comments attached
/// to the opening and closing tokens and insert them inside the group block
pub fn format_delimited<'a, 'content>(
    open_token: &'a JsSyntaxToken,
    content: &'content impl Format<JsFormatContext>,
    close_token: &'a JsSyntaxToken,
) -> FormatDelimited<'a, 'content> {
    FormatDelimited {
        open_token,
        content: Argument::new(content),
        close_token,
        mode: DelimitedMode::SoftBlockIndent(None),
        grouped: true,
    }
}

#[derive(Copy, Clone)]
pub struct FormatDelimited<'a, 'content> {
    open_token: &'a JsSyntaxToken,
    content: Argument<'content, JsFormatContext>,
    close_token: &'a JsSyntaxToken,
    mode: DelimitedMode,
    grouped: bool,
}

impl FormatDelimited<'_, '_> {
    fn with_mode(mut self, mode: DelimitedMode) -> Self {
        self.mode = mode;
        self
    }

    /// Formats a group delimited by an opening and closing token, placing the
    /// content in a [block_indent] group
    pub fn block_indent(self) -> Self {
        self.with_mode(DelimitedMode::BlockIndent)
    }

    /// Formats a group delimited by an opening and closing token, placing the
    /// content in a [soft_block_indent] group
    pub fn soft_block_indent(self) -> Self {
        self.with_mode(DelimitedMode::SoftBlockIndent(None))
    }

    /// Formats a group delimited by an opening and closing token, placing the
    /// content in an [indent] group with [soft_line_break_or_space] tokens at the
    /// start and end
    pub fn soft_block_spaces(self) -> Self {
        self.with_mode(DelimitedMode::SoftBlockSpaces(None))
    }

    pub fn soft_block_indent_with_group_id(self, group_id: Option<GroupId>) -> Self {
        self.with_mode(DelimitedMode::SoftBlockIndent(group_id))
    }

    /// Prevents the formatter from grouping the content even in soft block or soft block spaces mode.
    pub fn ungrouped(mut self) -> Self {
        self.grouped = false;
        self
    }
}

impl Format<JsFormatContext> for FormatDelimited<'_, '_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let FormatDelimited {
            open_token,
            close_token,
            content,
            mode,
            grouped,
        } = self;

        let open_delimiter = format_open_delimiter(open_token);
        let close_delimiter = format_close_delimiter(close_token);

        open_delimiter.format_leading_trivia().fmt(f)?;

        let open_token_trailing_trivia = open_delimiter.format_trailing_trivia();

        let close_token_leading_trivia = close_delimiter.format_leading_trivia();

        let delimited = format_with(|f| {
            open_delimiter.format_token().fmt(f)?;

            let format_content = format_with(|f| f.write_fmt(Arguments::from(content)));

            match mode {
                DelimitedMode::BlockIndent => block_indent(&format_args![
                    open_token_trailing_trivia,
                    format_content, close_token_leading_trivia
                ])
                .fmt(f)?,
                DelimitedMode::SoftBlockIndent(_) => soft_block_indent(&format_args![
                    open_token_trailing_trivia,
                    format_content, close_token_leading_trivia
                ])
                .fmt(f)?,
                DelimitedMode::SoftBlockSpaces(_) => {
                    let mut is_empty = true;

                    let format_content = format_once(|f| {
                        let mut recording = f.start_recording();

                        write!(
                            recording,
                            [
                                open_token_trailing_trivia,
                                format_content,
                                close_token_leading_trivia
                            ]
                        )?;

                        is_empty = recording.stop().is_empty();

                        Ok(())
                    });

                    soft_line_indent_or_space(&format_content).fmt(f)?;

                    if !is_empty {
                        soft_line_break_or_space().fmt(f)?;
                    }
                }
            };

            close_delimiter.format_token().fmt(f)
        });

        match mode {
            _ if !grouped => write!(f, [delimited])?,
            // Group is useless, the block indent would expand it right anyway
            DelimitedMode::SoftBlockIndent(group_id) | DelimitedMode::SoftBlockSpaces(group_id) => {
                match group_id {
                    None => write!(f, [group(&delimited)])?,
                    Some(group_id) => {
                        write!(f, [group(&delimited).with_group_id(Some(*group_id))])?
                    }
                }
            }
            DelimitedMode::BlockIndent => write!(f, [delimited])?,
        };

        write!(f, [format_trailing_trivia(close_token)])
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum DelimitedMode {
    BlockIndent,
    SoftBlockIndent(Option<GroupId>),
    SoftBlockSpaces(Option<GroupId>),
}

/// Use this function to create an open delimiter, where you can extract the formatting of
/// trivias and token, separately.
///
/// This function assumes that you will use the token to replicate [format_delimited], which means
/// that it will add possible line breaks
pub(crate) fn format_open_delimiter(open_token: &JsSyntaxToken) -> OpenDelimiter {
    OpenDelimiter::new(open_token)
}

/// Use this function to create an close delimiter, where you can extract the formatting of
/// trivias and token, separately.
///
/// This function assumes that you will use the token to replicate [format_delimited], which means
/// that it will add possible line breaks
pub(crate) fn format_close_delimiter(close_token: &JsSyntaxToken) -> CloseDelimiter {
    CloseDelimiter::new(close_token)
}

pub(crate) struct OpenDelimiter<'t> {
    open_token: &'t JsSyntaxToken,
}

impl<'t> OpenDelimiter<'t> {
    pub(crate) fn new(open_token: &'t JsSyntaxToken) -> Self {
        Self { open_token }
    }

    /// It extracts the formatted leading trivia of the token, without writing it in the buffer
    pub(crate) fn format_leading_trivia(&self) -> impl Format<JsFormatContext> + 't {
        format_leading_trivia(self.open_token)
    }

    /// It extracts the formatted trailing trivia of the token, without writing it in the buffer
    pub(crate) fn format_trailing_trivia(&self) -> impl Format<JsFormatContext> + 't {
        format_with(|f| {
            let mut recording = f.start_recording();
            write!(recording, [format_trailing_trivia(self.open_token)])?;
            let recorded = recording.stop();

            if !recorded.is_empty() {
                soft_line_break().fmt(f)?;
            }

            Ok(())
        })
    }

    /// It extracts the formatted token, without writing it in the buffer
    pub(crate) fn format_token(&self) -> impl Format<JsFormatContext> + 't {
        format_with(|f| {
            f.state_mut().track_token(self.open_token);
            write!(f, [format_trimmed_token(self.open_token)])
        })
    }
}

pub(crate) struct CloseDelimiter<'t> {
    close_token: &'t JsSyntaxToken,
}

impl<'t> CloseDelimiter<'t> {
    pub(crate) fn new(close_token: &'t JsSyntaxToken) -> Self {
        Self { close_token }
    }

    /// It extracts the formatted leading trivia of the token, without writing it in the buffer
    pub(crate) fn format_trailing_trivia(&self) -> impl Format<JsFormatContext> + 't {
        format_trailing_trivia(self.close_token)
    }

    /// It extracts the formatted trailing trivia of the token, without writing it in the buffer
    pub(crate) fn format_leading_trivia(&self) -> impl Format<JsFormatContext> + 't {
        format_with(|f| {
            let mut buffer = PreambleBuffer::new(f, soft_line_break());

            write!(buffer, [format_leading_trivia(self.close_token)])
        })
    }

    /// It extracts the formatted token, without writing it in the buffer
    pub(crate) fn format_token(&self) -> impl Format<JsFormatContext> + 't {
        format_with(|f| {
            f.state_mut().track_token(self.close_token);
            write!(f, [format_trimmed_token(self.close_token)])
        })
    }
}
