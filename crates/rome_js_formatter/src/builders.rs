use crate::prelude::*;
use crate::{AsFormat, TextRange};
use rome_formatter::{format_args, write, GroupId, PreambleBuffer, VecBuffer};
use rome_js_syntax::{JsLanguage, JsSyntaxNode, JsSyntaxToken};
use rome_rowan::syntax::SyntaxTriviaPiecesIterator;
use rome_rowan::{AstNode, Language, SyntaxTriviaPiece};

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

        write!(f, [syntax_token_text_slice(self.token, trimmed_range)])
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
        let snapshot = Formatter::snapshot(f);

        match write_leading_trivia_pieces(
            self.token.leading_trivia().pieces(),
            self.trim_mode,
            false,
            f,
        ) {
            Ok(()) => Ok(()),
            Err(_) => {
                f.restore_snapshot(snapshot);

                write_leading_trivia_with_skipped_tokens(self.token, self.trim_mode, f)
            }
        }
    }
}

/// Writes the leading trivia pieces of a token.
///
/// ## Returns
///
/// Returns [Err] if the leading trivia contains any skipped trivia. Returns the formatted
/// leading trivia otherwise.
fn write_leading_trivia_pieces<I>(
    pieces: I,
    trim_mode: TriviaPrintMode,
    has_trailing_newline: bool,
    f: &mut Formatter<JsFormatContext>,
) -> Result<(), FormatError>
where
    I: Iterator<Item = SyntaxTriviaPiece<JsLanguage>> + DoubleEndedIterator + ExactSizeIterator,
{
    let mut buffer = VecBuffer::new(f.state_mut());

    let mut line_count = 0;

    // Get the index of the first comment in the trivia pieces list, and
    // checks whether this token has any leading newline the comment
    let mut has_leading_newline = false;
    let mut first_comment = 0;

    let mut pieces = pieces.enumerate().peekable();

    // Peek at the next trivia piece, stopping if it is a comment and
    // advancing the iterator if it's not
    while let Some((index, piece)) = pieces.peek() {
        if piece.is_comments() {
            // Save the index and break the loop
            // without consuming the comment piece
            first_comment = *index;
            break;
        }

        if piece.is_skipped() {
            return Err(FormatError::MissingRequiredChild);
        }

        if piece.is_newline() {
            has_leading_newline = true;
        }

        pieces.next();
    }

    // If any newline was found between the previous token and the first comment,
    // it will be prepended with a line break instead of a space
    let prepend_newline = has_trailing_newline || has_leading_newline;
    let mut trim_mode = trim_mode;

    // This consumes the previously created iterator from the last trivia piece
    // towards the first (that was not consumed by the previous loop)
    for (index, piece) in pieces.rev() {
        if let Some(comment_piece) = piece.as_comments() {
            let is_single_line = comment_piece.text().starts_with("//");

            let format_content = format_with(|f| {
                if prepend_newline && index == first_comment {
                    write!(f, [hard_line_break()])?;
                } else {
                    write!(f, [space_token()])?;
                };

                write!(f, [comment(&comment_piece)])?;

                if is_single_line {
                    match line_count {
                        0 | 1 => write!(f, [hard_line_break()])?,
                        _ => write!(f, [empty_line()])?,
                    }
                } else {
                    match line_count {
                        0 => write!(f, [space_token()])?,
                        1 => write!(f, [hard_line_break()])?,
                        _ => write!(f, [empty_line()])?,
                    }
                };

                Ok(())
            });

            write!(buffer, [comment(&format_content)])?;

            line_count = 0;
            trim_mode = TriviaPrintMode::Full;
        } else if piece.is_newline() && trim_mode == TriviaPrintMode::Full {
            line_count += 1;
        } else if piece.is_skipped() {
            return Err(FormatError::MissingRequiredChild);
        }
    }

    let elements = buffer.into_vec();

    for comment in elements.into_iter().rev() {
        f.write_element(comment)?;
    }

    Ok(())
}

/// Writes the leading trivia of a token that has leading skipped trivia.
///
/// It splits the leading trivia piece into four parts, so that it behaves as if it is a regular token:
/// 1. All pieces that come before the first skipped trivia token.
/// 2. All the skipped trivia pieces, formatted as is.
/// 3. Any trivia after the last skipped token trivia up to, but not including, the first line break.
/// 4. The leading trivia of the token.
///
/// ## Returns
/// The format element for the tokens leading trivia.
///
/// ## Panics
///
/// If called on a token that does not have skipped trivia
fn write_leading_trivia_with_skipped_tokens(
    token: &JsSyntaxToken,
    trim_mode: TriviaPrintMode,
    f: &mut Formatter<JsFormatContext>,
) -> FormatResult<()> {
    let mut skipped_trivia_range: Option<TextRange> = None;
    // The leading trivia for the first skipped token trivia OR the leading trivia for the token
    let mut trailing_trivia = vec![];
    // The trailing trivia for the last skipped token trivia
    let mut leading_trivia = vec![];
    //  The formatted elements
    let mut after_newline = true;

    for piece in token.leading_trivia().pieces() {
        if piece.is_skipped() {
            if let Some(previous_range) = skipped_trivia_range {
                // Another skipped token trivia: `.. first_skipped....piece`. Everything between the skipped token trivia should
                // be formatted as is.
                skipped_trivia_range = Some(previous_range.cover(piece.text_range()));
                // Clear the collected leading/trailing trivia. They are part of the skipped
                // token trivia range.
                leading_trivia.clear();
                trailing_trivia.clear();
            } else {
                // This is the first skipped token trivia.
                // Format the  collected leading trivia as the leading trivia of this "skipped token trivia"
                skipped_trivia_range = Some(piece.text_range());

                write_leading_trivia_pieces(leading_trivia.drain(..), trim_mode, false, f)
                    .expect("All skipped trivia pieces should have been filtered out");
            }

            after_newline = false;
            continue;
        }

        // Everything coming after a new line (including the new line) is considered a leading trivia and not trailing trivia.
        if piece.is_newline() {
            after_newline = true;
        }

        if after_newline {
            leading_trivia.push(piece);
        } else {
            trailing_trivia.push(piece);
        }
    }

    let skipped_trivia_range = skipped_trivia_range.expect(
        "Only call this method for leading trivia containing at least one skipped token trivia.",
    );

    // Format the skipped token trivia range
    write!(f, [syntax_token_text_slice(token, skipped_trivia_range)])?;

    // `print_trailing_trivia_pieces` and `format_leading_trivia_pieces` remove any whitespace except
    // if there's a comment but removing all whitespace may have a different semantic meaning.
    // Insert a:
    // * space if the skipped token has no trailing trivia (`skipped\n`, also works for `skipped//comment` because the comment must either be followed by a line break or the token is the EOF).
    // * new line if the token has any leading trivia. This can only be the case if there was any new line between the skipped trivia and the token
    // * empty: There's literally nothing between skipped and token, so don't insert anything
    if !trailing_trivia.is_empty() {
        write!(f, [space_token()])?;
    } else if !leading_trivia.is_empty() {
        write!(f, [hard_line_break()])?;
    };

    // Format the trailing pieces of the skipped token trivia
    write!(
        f,
        [FormatTrailingTriviaPieces {
            pieces: trailing_trivia.into_iter()
        }]
    )?;

    write_leading_trivia_pieces(leading_trivia.into_iter(), trim_mode, after_newline, f)
        .expect("All skipped trivia pieces should have been filtered out");

    Ok(())
}

/// Formats the trailing trivia (comments) of a token
pub fn format_trailing_trivia(
    token: &JsSyntaxToken,
) -> FormatTrailingTriviaPieces<SyntaxTriviaPiecesIterator<JsLanguage>> {
    FormatTrailingTriviaPieces {
        pieces: token.trailing_trivia().pieces(),
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FormatTrailingTriviaPieces<I> {
    pieces: I,
}

impl<I> Format<JsFormatContext> for FormatTrailingTriviaPieces<I>
where
    I: Iterator<Item = SyntaxTriviaPiece<JsLanguage>> + Clone,
{
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let pieces = self.pieces.clone();

        for piece in pieces {
            if let Some(comment_piece) = piece.as_comments() {
                let is_single_line = comment_piece.text().trim_start().starts_with("//");

                let content = format_with(|f| {
                    if !is_single_line {
                        write!(f, [space_token(), comment_piece, space_token()])
                    } else {
                        write![
                            f,
                            [
                                line_suffix(&format_args![space_token(), comment_piece]),
                                expand_parent()
                            ]
                        ]
                    }
                });

                write!(f, [comment(&content)])?;
            }
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

        match write![f, [self.node.format()]] {
            Ok(result) => Ok(result),

            Err(_) => {
                f.restore_snapshot(snapshot);

                // Lists that yield errors are formatted as they were unknown nodes.
                // Doing so, the formatter formats the nodes/tokens as is.
                write!(f, [format_unknown_node(self.node.syntax())])
            }
        }
    }
}

/// Print out a `token` from the original source with a different `content`.
///
/// This will print the trivia that belong to `token` to `content`;
/// `token` is then marked as consumed by the formatter.
pub const fn format_replaced<'a, 'content>(
    token: &'a JsSyntaxToken,
    content: &'content dyn Format<JsFormatContext>,
) -> FormatReplaced<'a, 'content> {
    FormatReplaced { token, content }
}

#[derive(Copy, Clone)]
pub struct FormatReplaced<'a, 'content> {
    token: &'a JsSyntaxToken,
    content: &'content dyn Format<JsFormatContext>,
}

impl Format<JsFormatContext> for FormatReplaced<'_, '_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        f.state_mut().track_token(self.token);

        write!(
            f,
            [
                format_leading_trivia(self.token, TriviaPrintMode::Full),
                &self.content,
                format_trailing_trivia(self.token)
            ]
        )
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
        for token in self.node.descendants_tokens() {
            f.state_mut().track_token(&token);
        }

        fn skip_whitespace<L: Language>(piece: &SyntaxTriviaPiece<L>) -> bool {
            piece.is_newline() || piece.is_whitespace()
        }

        fn write_trivia_token<L: Language>(
            f: &mut JsFormatter,
            piece: SyntaxTriviaPiece<L>,
        ) -> FormatResult<()> {
            write!(
                f,
                [syntax_token_cow_slice(
                    normalize_newlines(piece.text(), LINE_TERMINATORS),
                    &piece.token(),
                    piece.text_range().start(),
                )]
            )
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

                write!(
                    f,
                    [dynamic_token(
                        &normalize_newlines(
                            &self.node.text_trimmed().to_string(),
                            LINE_TERMINATORS
                        ),
                        self.node.text_trimmed_range().start()
                    )]
                )?;

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
        write!(
            f,
            [FormatVerbatimNode {
                node: self.node,
                kind: VerbatimKind::Unknown
            }]
        )
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
pub const fn format_delimited<'a, 'content>(
    open_token: &'a JsSyntaxToken,
    content: &'content dyn Format<JsFormatContext>,
    close_token: &'a JsSyntaxToken,
) -> FormatDelimited<'a, 'content> {
    FormatDelimited {
        open_token,
        content,
        close_token,
        mode: DelimitedMode::SoftBlockIndent(None),
    }
}

#[derive(Copy, Clone)]
pub struct FormatDelimited<'a, 'content> {
    open_token: &'a JsSyntaxToken,
    content: &'content dyn Format<JsFormatContext>,
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

        write!(
            f,
            [format_leading_trivia(open_token, TriviaPrintMode::Full)]
        )?;

        let open_token_trailing_trivia = format_with(|f| {
            // Not really interested in the pre-amble, but want to know if it was written
            let mut buffer = VecBuffer::new(f.state_mut());

            write!(buffer, [format_trailing_trivia(open_token)])?;

            let trivia = buffer.into_element();

            if !trivia.is_empty() {
                f.write_element(trivia)?;
                write!(f, [soft_line_break_or_space()])?;
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
            write!(f, [format_trimmed_token(open_token)])?;

            match mode {
                DelimitedMode::BlockIndent => {
                    write!(
                        f,
                        [block_indent(&format_args![
                            open_token_trailing_trivia,
                            content, close_token_leading_trivia
                        ])]
                    )?;
                }
                DelimitedMode::SoftBlockIndent(_) => write!(
                    f,
                    [soft_block_indent(&format_args![
                        open_token_trailing_trivia,
                        content, close_token_leading_trivia
                    ])]
                )?,
                DelimitedMode::SoftBlockSpaces(_) => {
                    let mut buffer = VecBuffer::new(f.state_mut());
                    write!(
                        buffer,
                        [
                            open_token_trailing_trivia,
                            content,
                            close_token_leading_trivia
                        ]
                    )?;
                    let content = buffer.into_element();

                    if !content.is_empty() {
                        write!(
                            f,
                            [
                                indent(&format_once(|f| {
                                    write!(f, [soft_line_break_or_space()])?;
                                    f.write_element(content)
                                }),),
                                soft_line_break_or_space()
                            ]
                        )?;
                    }
                }
            };

            write!(f, [format_trimmed_token(close_token)])
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
