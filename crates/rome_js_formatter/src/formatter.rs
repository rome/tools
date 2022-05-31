use crate::prelude::*;
use rome_formatter::{format_args, write, Buffer, VecBuffer};
use std::cell::Cell;

use rome_formatter::{normalize_newlines, FormatResult, GroupId, LINE_TERMINATORS};
use rome_js_syntax::{JsLanguage, JsSyntaxNode, JsSyntaxToken};

use crate::{AsFormat, IntoFormat};
use rome_rowan::{
    AstNode, AstNodeList, AstNodeListIterator, AstSeparatedList, AstSeparatedListElementsIterator,
    Language, SyntaxResult, SyntaxTriviaPiece, TextRange,
};

use rome_rowan::syntax::SyntaxTriviaPiecesIterator;
use std::iter::FusedIterator;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrailingSeparator {
    Allowed,
    Disallowed,
    Mandatory,
}

impl TrailingSeparator {
    pub fn is_allowed(&self) -> bool {
        matches!(self, TrailingSeparator::Allowed)
    }
    pub fn is_mandatory(&self) -> bool {
        matches!(self, TrailingSeparator::Mandatory)
    }
}

impl Default for TrailingSeparator {
    fn default() -> Self {
        TrailingSeparator::Allowed
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct FormatSeparatedOptions {
    trailing_separator: TrailingSeparator,
    group_id: Option<GroupId>,
}

impl FormatSeparatedOptions {
    pub fn with_trailing_separator(mut self, separator: TrailingSeparator) -> Self {
        self.trailing_separator = separator;
        self
    }

    pub fn with_group_id(mut self, group_id: Option<GroupId>) -> Self {
        self.group_id = group_id;
        self
    }
}

/// Determines if the whitespace separating comment trivias
/// from their associated tokens should be printed or trimmed
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TriviaPrintMode {
    Full,
    Trim,
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
pub fn verbatim_node(node: &JsSyntaxNode) -> FormatVerbatimNode {
    FormatVerbatimNode {
        node,
        kind: VerbatimKind::Verbatim {
            length: node.text_range().len(),
        },
    }
}

#[derive(Debug, Clone)]
pub struct FormatVerbatimNode<'node> {
    node: &'node JsSyntaxNode,
    kind: VerbatimKind,
}
impl Format<JsFormatContext> for FormatVerbatimNode<'_> {
    fn format(&self, f: &mut JsFormatter) -> FormatResult<()> {
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

        f.write_element(FormatElement::Verbatim(verbatim));

        Ok(())
    }
}

/// Formats unknown nodes. The difference between this method  and `format_verbatim` is that this method
/// doesn't track nodes/tokens as [FormatElement::Verbatim]. They are just printed as they are.
pub fn unknown_node(node: &JsSyntaxNode) -> FormatUnknownNode {
    FormatUnknownNode { node }
}

#[derive(Debug, Clone)]
pub struct FormatUnknownNode<'node> {
    node: &'node JsSyntaxNode,
}

impl Format<JsFormatContext> for FormatUnknownNode<'_> {
    fn format(&self, f: &mut JsFormatter) -> FormatResult<()> {
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
pub fn suppressed_node(node: &JsSyntaxNode) -> FormatSuppressedNode {
    FormatSuppressedNode { node }
}

#[derive(Debug, Clone)]
pub struct FormatSuppressedNode<'node> {
    node: &'node JsSyntaxNode,
}

impl Format<JsFormatContext> for FormatSuppressedNode<'_> {
    fn format(&self, f: &mut JsFormatter) -> FormatResult<()> {
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

pub struct FormatTrailingTriviaPieces<I> {
    pieces: I,
}

impl<I> Format<JsFormatContext> for FormatTrailingTriviaPieces<I>
where
    I: Iterator<Item = SyntaxTriviaPiece<JsLanguage>> + Clone,
{
    fn format(&self, f: &mut JsFormatter) -> FormatResult<()> {
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

pub(crate) fn format_trailing_trivia(
    token: &JsSyntaxToken,
) -> FormatTrailingTriviaPieces<SyntaxTriviaPiecesIterator<JsLanguage>> {
    FormatTrailingTriviaPieces {
        pieces: token.trailing_trivia().pieces(),
    }
}

pub fn format_leading_trivia(
    token: &JsSyntaxToken,
    trim_mode: TriviaPrintMode,
) -> FormatLeadingTrivia {
    FormatLeadingTrivia { token, trim_mode }
}

pub struct FormatLeadingTrivia<'a> {
    token: &'a JsSyntaxToken,
    trim_mode: TriviaPrintMode,
}

impl Format<JsFormatContext> for FormatLeadingTrivia<'_> {
    fn format(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let trivia_written = write!(
            f,
            [FormatLeadingTriviaPieces {
                pieces: Cell::new(Some(self.token.leading_trivia().pieces())),
                trim_mode: self.trim_mode,
                has_trailing_newline: false
            }]
        );

        if trivia_written.is_err() {
            write!(
                f,
                [FormatLeadingTriviaWithSkippedTokens {
                    token: self.token,
                    trim_mode: self.trim_mode
                }]
            )?;
        }

        Ok(())
    }
}

/// Formats the leading trivia of a token that has leading skipped trivia.
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
struct FormatLeadingTriviaWithSkippedTokens<'a> {
    token: &'a JsSyntaxToken,
    trim_mode: TriviaPrintMode,
}

impl Format<JsFormatContext> for FormatLeadingTriviaWithSkippedTokens<'_> {
    fn format(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let mut skipped_trivia_range: Option<TextRange> = None;
        // The leading trivia for the first skipped token trivia OR the leading trivia for the token
        let mut trailing_trivia = vec![];
        // The trailing trivia for the last skipped token trivia
        let mut leading_trivia = vec![];
        //  The formatted elements
        let mut after_newline = true;

        for piece in self.token.leading_trivia().pieces() {
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
                    write!(
                        f,
                        [FormatLeadingTriviaPieces {
                            pieces: Cell::new(Some(leading_trivia.drain(..))),
                            trim_mode: self.trim_mode,
                            has_trailing_newline: false
                        }]
                    )
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
        write!(
            f,
            [syntax_token_text_slice(self.token, skipped_trivia_range)]
        )?;

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

        write!(
            f,
            [FormatLeadingTriviaPieces {
                pieces: Cell::new(Some(leading_trivia.into_iter())),
                trim_mode: self.trim_mode,
                has_trailing_newline: after_newline
            }]
        )
    }
}

struct FormatLeadingTriviaPieces<I> {
    pieces: Cell<Option<I>>,
    trim_mode: TriviaPrintMode,
    has_trailing_newline: bool,
}

/// Formats the leading trivia pieces of a token.
///
/// ## Returns
///
/// Returns [Err] if the leading trivia contains any skipped trivia. Returns the formatted
/// leading trivia otherwise.
impl<I> Format<JsFormatContext> for FormatLeadingTriviaPieces<I>
where
    I: Iterator<Item = SyntaxTriviaPiece<JsLanguage>> + DoubleEndedIterator + ExactSizeIterator,
{
    fn format(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let pieces = self
            .pieces
            .take()
            .expect("Leading trivia pieces can only be formatted once");

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
        let prepend_newline = self.has_trailing_newline || has_leading_newline;
        let mut trim_mode = self.trim_mode;

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

                write!(f, [comment(&format_content)])?;

                line_count = 0;
                trim_mode = TriviaPrintMode::Full;
            } else if piece.is_newline() && trim_mode == TriviaPrintMode::Full {
                line_count += 1;
            } else if piece.is_skipped() {
                return Err(FormatError::MissingRequiredChild);
            }
        }

        Ok(())
    }
}

pub(crate) type JsFormatter<'buf> = Formatter<'buf, JsFormatContext>;

/// JS specific formatter extensions
pub(crate) trait JsFormatterExt<'buf> {
    fn as_formatter(&self) -> &Formatter<'buf, JsFormatContext>;

    #[must_use]
    fn delimited<'a>(
        &self,
        open_token: &'a JsSyntaxToken,
        content: &'a dyn Format<JsFormatContext>,
        close_token: &'a JsSyntaxToken,
    ) -> FormatDelimited<'a> {
        FormatDelimited::new(open_token, content, close_token)
    }

    /// Print out a `token` from the original source with a different `content`.
    ///
    /// This will print the trivias that belong to `token` to `content`;
    /// `token` is then marked as consumed by the formatter.
    fn format_replaced<'a>(
        &self,
        current_token: &'a JsSyntaxToken,
        content_to_replace_with: &'a dyn Format<JsFormatContext>,
    ) -> FormatReplaced<'a> {
        FormatReplaced {
            token: current_token,
            content: content_to_replace_with,
        }
    }
}

impl<'buf> JsFormatterExt<'buf> for Formatter<'buf, JsFormatContext> {
    fn as_formatter(&self) -> &Formatter<'buf, JsFormatContext> {
        self
    }
}

/// Formats a group delimited by an opening and closing token,
/// such as a function body delimited by '{' and '}' tokens
///
/// Calling this method is required to correctly handle the comments attached
/// to the opening and closing tokens and insert them inside the group block
pub struct FormatDelimited<'a> {
    open_token: &'a JsSyntaxToken,
    content: &'a dyn Format<JsFormatContext>,
    close_token: &'a JsSyntaxToken,
    mode: DelimitedMode,
}

impl<'a> FormatDelimited<'a> {
    fn new(
        open_token: &'a JsSyntaxToken,
        content: &'a dyn Format<JsFormatContext>,
        close_token: &'a JsSyntaxToken,
    ) -> Self {
        Self {
            open_token,
            content,
            close_token,
            mode: DelimitedMode::SoftBlockIndent(None),
        }
    }

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

    pub fn soft_block_spaces_with_group_id(self, group_id: Option<GroupId>) -> Self {
        self.with_mode(DelimitedMode::SoftBlockSpaces(group_id))
    }
}

impl Format<JsFormatContext> for FormatDelimited<'_> {
    fn format(&self, f: &mut JsFormatter) -> FormatResult<()> {
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
            let mut buffer = VecBuffer::new(f.state_mut());

            write!(buffer, [format_trailing_trivia(open_token)])?;

            let trivia = buffer.into_element();
            if !trivia.is_empty() {
                write!(f, [soft_line_break_or_space()])?;
                f.write_element(trivia);
            }

            Ok(())
        });

        let close_token_leading_trivia = format_with(|f| {
            let mut buffer = VecBuffer::new(f.state_mut());

            write!(
                buffer,
                [format_leading_trivia(close_token, TriviaPrintMode::Trim)]
            )?;
            let trivia = buffer.into_element();

            if !trivia.is_empty() {
                f.write_element(trivia);
                write!(f, [soft_line_break_or_space()])?;
            }

            Ok(())
        });

        let delimited = format_with(|f| {
            write!(f, [FormatTrimmedToken::new(open_token)])?;

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
                DelimitedMode::SoftBlockSpaces(_) => write![
                    f,
                    [
                        indent(&format_args![
                            soft_line_break_or_space(),
                            open_token_trailing_trivia, content, close_token_leading_trivia,
                        ]),
                        soft_line_break_or_space(),
                    ]
                ]?,
            };

            write!(f, [FormatTrimmedToken::new(close_token)])
        });

        let _grouped = match mode {
            // Group is useless, the block indent would expand it right anyway
            DelimitedMode::BlockIndent => write!(f, [delimited])?,
            DelimitedMode::SoftBlockIndent(group_id) | DelimitedMode::SoftBlockSpaces(group_id) => {
                match group_id {
                    None => write!(f, [group_elements(&delimited)])?,
                    Some(group_id) => write!(
                        f,
                        [group_elements_with_options(
                            &delimited,
                            GroupElementsOptions {
                                group_id: Some(*group_id),
                            },
                        )]
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

/// Print out a `token` from the original source with a different `content`.
///
/// This will print the trivias that belong to `token` to `content`;
/// `token` is then marked as consumed by the formatter.
pub struct FormatReplaced<'a> {
    token: &'a JsSyntaxToken,
    content: &'a dyn Format<JsFormatContext>,
}

impl Format<JsFormatContext> for FormatReplaced<'_> {
    fn format(&self, f: &mut JsFormatter) -> FormatResult<()> {
        f.state_mut().track_token(self.token);

        write!(
            f,
            [
                format_leading_trivia(self.token, TriviaPrintMode::Full),
                self.content,
                format_trailing_trivia(self.token)
            ]
        )
    }
}

// Idea, implement as an iterator extension iter.formatted_separated()
// Each call must return an owned element

pub struct FormatSeparatedItem<F, S> {
    node: F,
    separator: S,
    trailing_separator_token: SyntaxResult<Option<JsSyntaxToken>>,
    last: bool,
    options: FormatSeparatedOptions,
}

impl<F, S> Format<JsFormatContext> for FormatSeparatedItem<F, S>
where
    F: Format<JsFormatContext>,
    S: Format<JsFormatContext>,
{
    fn format(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let trailing_separator_factory = format_with(|f| {
            if let Some(group_id) = self.options.group_id {
                write!(f, [if_group_with_id_breaks(&self.separator, group_id)])
            } else {
                write!(f, [if_group_breaks(&self.separator)])
            }
        });

        write!(f, [group_elements(&self.node)])?;

        // Reuse the existing trailing separator or create it if it wasn't in the
        // input source. Only print the last trailing token if the outer group breaks
        if let Some(separator) = self.trailing_separator_token.as_ref()? {
            if self.last {
                if self.options.trailing_separator.is_allowed() {
                    // Use format_replaced instead of wrapping the result of format_token
                    // in order to remove only the token itself when the group doesn't break
                    // but still print its associated trivias unconditionally
                    write!(
                        f,
                        [f.format_replaced(separator, &trailing_separator_factory)]
                    )?;
                } else if self.options.trailing_separator.is_mandatory() {
                    write!(f, [separator.format()])?;
                }
            } else {
                write!(f, [separator.format()])?;
            }
        } else if self.last {
            if self.options.trailing_separator.is_allowed() {
                write!(f, [trailing_separator_factory])?;
            } else if self.options.trailing_separator.is_mandatory() {
                write!(f, [&self.separator])?;
            }
        } else {
            write!(f, [&self.separator])?;
        };

        Ok(())
    }
}

pub struct FormatSeparatedIter<I, Content, Separator>
where
    I: Iterator<Item = (Content, SyntaxResult<Option<JsSyntaxToken>>)>,
{
    inner: std::iter::Peekable<I>,
    separator: Separator,
    options: FormatSeparatedOptions,
}

impl<I, Content, Separator> FormatSeparatedIter<I, Content, Separator>
where
    I: Iterator<Item = (Content, SyntaxResult<Option<JsSyntaxToken>>)>,
    Content: Format<JsFormatContext>,
    Separator: Format<JsFormatContext> + Copy,
{
    pub fn new(inner: I, separator: Separator) -> Self {
        Self::with_options(inner, separator, FormatSeparatedOptions::default())
    }

    pub fn with_options(inner: I, separator: Separator, options: FormatSeparatedOptions) -> Self {
        Self {
            inner: inner.peekable(),
            separator,
            options,
        }
    }
}

impl<I, Content, Separator> Iterator for FormatSeparatedIter<I, Content, Separator>
where
    I: Iterator<Item = (Content, SyntaxResult<Option<JsSyntaxToken>>)>,
    Content: Format<JsFormatContext>,
    Separator: Format<JsFormatContext> + Copy,
{
    type Item = FormatSeparatedItem<Content, Separator>;

    fn next(&mut self) -> Option<Self::Item> {
        let (content, separator) = self.inner.next()?;

        Some(FormatSeparatedItem {
            node: content,
            separator: self.separator,
            trailing_separator_token: separator,
            last: self.inner.peek().is_none(),
            options: self.options,
        })
    }
}

impl<I, Content, Separator> FusedIterator for FormatSeparatedIter<I, Content, Separator>
where
    I: Iterator<Item = (Content, SyntaxResult<Option<JsSyntaxToken>>)>,
    Content: Format<JsFormatContext>,
    Separator: Format<JsFormatContext> + Copy,
{
}

impl<I, Content, Separator> std::iter::ExactSizeIterator
    for FormatSeparatedIter<I, Content, Separator>
where
    I: Iterator<Item = (Content, SyntaxResult<Option<JsSyntaxToken>>)>,
    Content: Format<JsFormatContext>,
    Separator: Format<JsFormatContext> + Copy,
{
}

pub struct FormatSeparatedListItemIter<N> {
    inner: AstSeparatedListElementsIterator<JsLanguage, N>,
}

impl<N> Iterator for FormatSeparatedListItemIter<N>
where
    N: AstNode<Language = JsLanguage> + IntoFormat<JsFormatContext> + Clone,
{
    type Item = (SyntaxResult<N::Format>, SyntaxResult<Option<JsSyntaxToken>>);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;

        let separator = match next.trailing_separator() {
            Ok(sep) => Ok(sep.cloned()),
            Err(err) => Err(err),
        };

        Some((next.node().cloned().into_format(), separator))
    }
}

pub trait FormatSeparatedExtension: AstSeparatedList<Language = JsLanguage>
where
    Self::Node: IntoFormat<JsFormatContext> + Clone,
{
    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    ///
    fn format_separated<Separator>(
        &self,
        separator: Separator,
    ) -> FormatSeparatedIter<
        FormatSeparatedListItemIter<Self::Node>,
        SyntaxResult<<Self::Node as IntoFormat<JsFormatContext>>::Format>,
        Separator,
    >
    where
        Separator: Format<JsFormatContext> + Copy;

    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    fn format_separated_with_options<Separator>(
        &self,
        separator_factory: Separator,
        options: FormatSeparatedOptions,
    ) -> FormatSeparatedIter<
        FormatSeparatedListItemIter<Self::Node>,
        SyntaxResult<<Self::Node as IntoFormat<JsFormatContext>>::Format>,
        Separator,
    >
    where
        Separator: Format<JsFormatContext> + Copy;
}

impl<T> FormatSeparatedExtension for T
where
    T: AstSeparatedList<Language = JsLanguage>,
    T::Node: IntoFormat<JsFormatContext> + Clone,
{
    fn format_separated<Separator>(
        &self,
        separator: Separator,
    ) -> FormatSeparatedIter<
        FormatSeparatedListItemIter<T::Node>,
        SyntaxResult<<T::Node as IntoFormat<JsFormatContext>>::Format>,
        Separator,
    >
    where
        Separator: Format<JsFormatContext> + Copy,
    {
        let inner = FormatSeparatedListItemIter {
            inner: self.elements(),
        };

        FormatSeparatedIter::new(inner, separator)
    }

    fn format_separated_with_options<Separator>(
        &self,
        separator: Separator,
        options: FormatSeparatedOptions,
    ) -> FormatSeparatedIter<
        FormatSeparatedListItemIter<T::Node>,
        SyntaxResult<<T::Node as IntoFormat<JsFormatContext>>::Format>,
        Separator,
    >
    where
        Separator: Format<JsFormatContext> + Copy,
    {
        let inner = FormatSeparatedListItemIter {
            inner: self.elements(),
        };

        FormatSeparatedIter::with_options(inner, separator, options)
    }
}

/// Formats a node or falls back to verbatim printing if formating this node fails.
pub struct FormatNodeOrVerbatim<'a, Node> {
    pub node: &'a Node,
}

impl<'a, Node> Format<JsFormatContext> for FormatNodeOrVerbatim<'a, Node>
where
    Node: AstNode<Language = JsLanguage> + AsFormat<'a>,
{
    fn format(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let snapshot = f.snapshot();

        match write![f, [self.node.format()]] {
            Ok(result) => {
                f.release_snapshot(snapshot);
                Ok(result)
            }
            Err(_) => {
                f.restore_snapshot(snapshot);

                // Lists that yield errors are formatted as they were unknown nodes.
                // Doing so, the formatter formats the nodes/tokens as is.
                write!(f, [unknown_node(self.node.syntax())])
            }
        }
    }
}

pub trait FormatNodeExtension {
    /// Formats a node or formats it as verbatim if formatting it fails.
    fn format_or_verbatim(&self) -> FormatNodeOrVerbatim<Self>
    where
        for<'a> Self: AstNode<Language = JsLanguage> + AsFormat<'a>,
        Self: Sized,
    {
        FormatNodeOrVerbatim { node: self }
    }
}

impl<T> FormatNodeExtension for T where for<'a> T: AsFormat<'a> + AstNode<Language = JsLanguage> {}

/// Formats a token without its leading or trailing trivia
///
/// ## Warning
/// It's your responsibility to format leading or trailing comments and skipped trivia.
pub struct FormatTrimmedToken<'a> {
    token: &'a JsSyntaxToken,
}

impl<'a> FormatTrimmedToken<'a> {
    pub fn new(token: &'a JsSyntaxToken) -> Self {
        Self { token }
    }
}

impl Format<JsFormatContext> for FormatTrimmedToken<'_> {
    fn format(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let trimmed_range = self.token.text_trimmed_range();

        write!(f, [syntax_token_text_slice(self.token, trimmed_range)])
    }
}
