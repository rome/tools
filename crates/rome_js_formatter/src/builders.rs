use crate::prelude::*;
use crate::AsFormat;
use rome_formatter::{format_args, write, Argument, Arguments, GroupId, PreambleBuffer, VecBuffer};
use rome_js_syntax::{JsLanguage, JsSyntaxNode, JsSyntaxToken};
use rome_rowan::{AstNode, Direction, Language, SyntaxTriviaPiece, SyntaxTriviaPieceComments};

/// Formats a token without its leading or trailing trivia
///
/// ## Warning
/// It's your responsibility to format leading or trailing comments and skipped trivia.
pub const fn format_trimmed_token(token: &JsSyntaxToken) -> FormatTrimmedToken {
    FormatTrimmedToken { token }
}
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct FormatTrimmedToken<'a> {
    token: &'a JsSyntaxToken,
}

impl Format<JsFormatContext> for FormatTrimmedToken<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let trimmed_range = self.token.text_trimmed_range();

        syntax_token_text_slice(self.token, trimmed_range).fmt(f)
    }
}

/// Formats the leading trivia (comments, skipped token trivia) of a token
pub const fn format_leading_trivia(
    token: &JsSyntaxToken,
    trim_mode: TriviaPrintMode,
) -> FormatLeadingTrivia {
    FormatLeadingTrivia { token, trim_mode }
}

/// Determines if the whitespace separating comment trivias
/// from their associated tokens should be printed or trimmed
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TriviaPrintMode {
    Full,
    Trim,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FormatLeadingTrivia<'a> {
    token: &'a JsSyntaxToken,
    trim_mode: TriviaPrintMode,
}

impl Format<JsFormatContext> for FormatLeadingTrivia<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let mut lines_before = 0;
        let mut comments = Vec::new();
        let mut pieces = self.token.leading_trivia().pieces();

        while let Some(piece) = pieces.next() {
            if let Some(comment) = piece.as_comments() {
                comments.push(Comment {
                    lines_before,
                    piece: comment,
                });
                lines_before = 0;
            } else if piece.is_newline() {
                lines_before += 1;
            } else if piece.is_skipped() {
                // Special handling for tokens that have skipped trivia:
                //
                // ```
                // class {
                //   // comment
                //   @test(/* inner */) // trailing
                //   /* token leading */
                //   method() {}
                // }
                // ```
                // If `@test(/*inner)` are skipped trivia that are part of the `method` tokens leading trivia, then the
                // following code splits the trivia into for parts:
                // 1. The first skipped trivia's leading comments: Comments that come before the first skipped trivia `@`: The `// comment`
                // 2. Skipped trivia: All trivia pieces between the first and last skipped trivia: `@test(/* inner *)`. Gets formatted as verbatim
                // 3. Trailing comments of the last skipped token: All comments that are on the same line as the last skipped trivia. The `// trailing` comment
                // 4. The token's leading trivia: All comments that are not on the same line as the last skipped token trivia: `/* token leading */`

                // Format the 1. part, the skipped trivia's leading comments
                FormatLeadingComments {
                    comments: &comments,
                    trim_mode: TriviaPrintMode::Full,
                    lines_before_token: lines_before,
                }
                .fmt(f)?;

                comments.clear();
                lines_before = 0;

                // Count the whitespace between the last skipped token trivia and the token
                let mut spaces = 0;
                // The range that covers from the first to the last skipped token trivia
                let mut skipped_trivia_range = piece.text_range();

                for piece in pieces {
                    if piece.is_whitespace() {
                        spaces += 1;
                        continue;
                    }

                    spaces = 0;

                    // If this is another skipped trivia, then extend the skipped range and
                    // clear all accumulated comments because they are formatted as verbatim as part of the
                    // skipped token trivia
                    if piece.is_skipped() {
                        skipped_trivia_range = skipped_trivia_range.cover(piece.text_range());
                        comments.clear();
                        lines_before = 0;
                    } else if let Some(comment) = piece.as_comments() {
                        comments.push(Comment {
                            piece: comment,
                            lines_before,
                        });
                        lines_before = 0;
                    } else if piece.is_newline() {
                        lines_before += 1;
                    }
                }

                // Format the skipped token trivia range
                syntax_token_text_slice(self.token, skipped_trivia_range).fmt(f)?;

                // Find the start position of the next token's leading comments.
                // The start is the first comment that is preceded by a line break.
                let first_token_leading_comment = comments
                    .iter()
                    .position(|comment| comment.lines_before > 0)
                    .unwrap_or(comments.len());

                // Everything before the start position are trailing comments of the last skipped token
                let (skipped_trailing_comments, token_leading_comments) =
                    comments.split_at(first_token_leading_comment);

                // Format the trailing comments of the last skipped token trivia
                FormatTrailingTriviaPieces {
                    pieces: skipped_trailing_comments
                        .iter()
                        .map(|comment| comment.piece.clone()),
                }
                .fmt(f)?;

                // Ensure that there's some whitespace between the last skipped token trivia and the
                // next token except if there was no whitespace present in the source.
                if lines_before > 0 {
                    write!(f, [hard_line_break()])?;
                } else if spaces > 0 {
                    write!(f, [space_token()])?;
                };

                // Write  leading comments of the next token
                return FormatLeadingComments {
                    comments: token_leading_comments,
                    lines_before_token: lines_before,
                    trim_mode: self.trim_mode,
                }
                .fmt(f);
            }
        }

        FormatLeadingComments {
            comments: &comments,
            trim_mode: self.trim_mode,
            lines_before_token: lines_before,
        }
        .fmt(f)
    }
}

struct Comment {
    lines_before: u32,
    piece: SyntaxTriviaPieceComments<JsLanguage>,
}

impl Comment {
    pub fn kind(&self) -> CommentKind {
        if self.piece.text().starts_with("/*") {
            if self.piece.has_newline() {
                CommentKind::Block
            } else {
                CommentKind::InlineBlock
            }
        } else {
            CommentKind::Line
        }
    }
}

enum CommentKind {
    InlineBlock,
    Block,
    Line,
}

struct FormatLeadingComments<'a> {
    comments: &'a [Comment],
    trim_mode: TriviaPrintMode,
    lines_before_token: u32,
}

impl Format<JsFormatContext> for FormatLeadingComments<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        for (index, comment) in self.comments.iter().enumerate() {
            let is_line_comment = matches!(comment.kind(), CommentKind::Line);
            let lines_after = self
                .comments
                .get(index + 1)
                .map(|comment| comment.lines_before)
                .unwrap_or_else(|| match self.trim_mode {
                    TriviaPrintMode::Full => self.lines_before_token,
                    TriviaPrintMode::Trim => 0,
                });

            let format_content = format_with(|f| {
                if comment.lines_before > 0 && index == 0 {
                    write!(f, [hard_line_break()])?;
                } else {
                    write!(f, [space_token()])?;
                };

                write!(f, [&comment.piece])?;

                if is_line_comment {
                    match lines_after {
                        0 | 1 => write!(f, [hard_line_break()])?,
                        _ => write!(f, [empty_line()])?,
                    }
                } else {
                    match lines_after {
                        0 => write!(f, [space_token()])?,
                        1 => write!(f, [hard_line_break()])?,
                        _ => write!(f, [empty_line()])?,
                    }
                };

                Ok(())
            });

            write!(f, [rome_formatter::comment(&format_content)])?;
        }

        Ok(())
    }
}

/// Formats the trailing trivia (comments) of a token
pub fn format_trailing_trivia(token: &JsSyntaxToken) -> impl Format<JsFormatContext> {
    FormatTrailingTriviaPieces {
        pieces: token
            .trailing_trivia()
            .pieces()
            .filter_map(|piece| piece.as_comments()),
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FormatTrailingTriviaPieces<I> {
    pieces: I,
}

impl<I> Format<JsFormatContext> for FormatTrailingTriviaPieces<I>
where
    I: Iterator<Item = SyntaxTriviaPieceComments<JsLanguage>> + Clone,
{
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let pieces = self.pieces.clone();

        for piece in pieces {
            let is_single_line = piece.text().trim_start().starts_with("//");

            let content = format_with(|f| {
                if !is_single_line {
                    write!(f, [space_token(), piece, space_token()])
                } else {
                    write![
                        f,
                        [
                            line_suffix(&format_args![space_token(), piece]),
                            expand_parent()
                        ]
                    ]
                }
            });

            comment(&content).fmt(f)?;
        }

        Ok(())
    }
}

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
        let snapshot = Formatter::snapshot(f);

        match self.node.format().fmt(f) {
            Ok(result) => Ok(result),

            Err(_) => {
                f.restore_snapshot(snapshot);

                // Lists that yield errors are formatted as they were unknown nodes.
                // Doing so, the formatter formats the nodes/tokens as is.
                format_unknown_node(self.node.syntax()).fmt(f)
            }
        }
    }
}

/// Print out a `token` from the original source with a different `content`.
///
/// This will print the trivia that belong to `token` to `content`;
/// `token` is then marked as consumed by the formatter.
pub fn format_replaced<'a, 'content>(
    token: &'a JsSyntaxToken,
    content: &'content impl Format<JsFormatContext>,
) -> FormatReplaced<'a, 'content> {
    FormatReplaced {
        token,
        content: Argument::new(content),
    }
}

#[derive(Copy, Clone)]
pub struct FormatReplaced<'a, 'content> {
    token: &'a JsSyntaxToken,
    content: Argument<'content, JsFormatContext>,
}

impl Format<JsFormatContext> for FormatReplaced<'_, '_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        f.state_mut().track_token(self.token);

        format_leading_trivia(self.token, TriviaPrintMode::Full).fmt(f)?;
        f.write_fmt(Arguments::from(&self.content))?;
        format_trailing_trivia(self.token).fmt(f)
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
        for token in self.node.descendants_tokens(Direction::Next) {
            f.state_mut().track_token(&token);
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

        let mut buffer = VecBuffer::new(f.state_mut());

        write!(
            buffer,
            [format_with(|f| {
                for leading_trivia in self
                    .node
                    .first_leading_trivia()
                    .into_iter()
                    .flat_map(|trivia| trivia.pieces())
                    .skip_while(skip_whitespace)
                {
                    write_trivia_token(f, leading_trivia)?;
                }

                dynamic_token(
                    &normalize_newlines(&self.node.text_trimmed().to_string(), LINE_TERMINATORS),
                    self.node.text_trimmed_range().start(),
                )
                .fmt(f)?;

                // Clippy false positive: SkipWhile does not implement DoubleEndedIterator
                #[allow(clippy::needless_collect)]
                let trailing_trivia: Vec<_> = self
                    .node
                    .last_trailing_trivia()
                    .into_iter()
                    .flat_map(|trivia| trivia.pieces().rev())
                    .skip_while(skip_whitespace)
                    .collect();

                for trailing_trivia in trailing_trivia.into_iter().rev() {
                    write_trivia_token(f, trailing_trivia)?;
                }

                Ok(())
            })]
        )?;

        let content = buffer.into_element();

        let verbatim = Verbatim {
            element: Box::new(content),
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
    }
}

#[derive(Copy, Clone)]
pub struct FormatDelimited<'a, 'content> {
    open_token: &'a JsSyntaxToken,
    content: Argument<'content, JsFormatContext>,
    close_token: &'a JsSyntaxToken,
    mode: DelimitedMode,
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
}

impl Format<JsFormatContext> for FormatDelimited<'_, '_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let FormatDelimited {
            open_token,
            close_token,
            content,
            mode,
        } = self;

        f.state_mut().track_token(open_token);
        f.state_mut().track_token(close_token);

        format_leading_trivia(open_token, TriviaPrintMode::Full).fmt(f)?;

        let open_token_trailing_trivia = format_with(|f| {
            // Not really interested in the pre-amble, but want to know if it was written
            let mut buffer = VecBuffer::new(f.state_mut());

            write!(buffer, [format_trailing_trivia(open_token)])?;

            let trivia = buffer.into_vec();

            if !trivia.is_empty() {
                f.write_elements(trivia)?;
                soft_line_break_or_space().fmt(f)?;
            }

            Ok(())
        });

        let close_token_leading_trivia = format_with(|f| {
            let mut buffer = PreambleBuffer::new(f, soft_line_break_or_space());

            write!(
                buffer,
                [format_leading_trivia(close_token, TriviaPrintMode::Trim)]
            )
        });

        let delimited = format_with(|f| {
            format_trimmed_token(open_token).fmt(f)?;

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
                        let mut buffer = f.inspect(|element| {
                            if !element.is_empty() {
                                is_empty = false
                            }
                        });

                        write!(
                            buffer,
                            [
                                open_token_trailing_trivia,
                                format_content,
                                close_token_leading_trivia
                            ]
                        )
                    });

                    soft_line_indent_or_space(&format_content).fmt(f)?;

                    if !is_empty {
                        soft_line_break_or_space().fmt(f)?;
                    }
                }
            };

            format_trimmed_token(close_token).fmt(f)
        });

        let _grouped = match mode {
            // Group is useless, the block indent would expand it right anyway
            DelimitedMode::BlockIndent => write!(f, [delimited])?,
            DelimitedMode::SoftBlockIndent(group_id) | DelimitedMode::SoftBlockSpaces(group_id) => {
                match group_id {
                    None => write!(f, [group_elements(&delimited)])?,
                    Some(group_id) => write!(
                        f,
                        [group_elements(&delimited).with_group_id(Some(*group_id))]
                    )?,
                }
            }
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
