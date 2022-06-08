use crate::prelude::*;
use crate::trivia::{write_leading_trivia, FormatTrailingTrivia, LeadingTriviaOptions};
use crate::{AsFormat, TriviaPrintMode};
use rome_formatter::{
    format_args, write, Argument, Arguments, CommentKind, GroupId, PreambleBuffer, VecBuffer,
};
use rome_js_syntax::{JsLanguage, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken};
use rome_rowan::{AstNode, Direction, Language, RawSyntaxKind, SyntaxKind, SyntaxTriviaPiece};

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
        f.state_mut().set_last_token(self.token.kind().to_raw());

        let trimmed_range = self.token.text_trimmed_range();

        syntax_token_text_slice(self.token, trimmed_range).fmt(f)
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

/// Formats the leading and trailing trivia of a removed token.
///
/// Formats all leading and trailing comments up to the first line break or skipped token trivia as a trailing
/// comment of the previous token. The remaining trivia is then printed as leading trivia of the next token.
pub const fn format_removed(token: &JsSyntaxToken) -> FormatRemoved {
    FormatRemoved {
        token,
        track: true,
        last_token: None,
        last_trailing_comment: None,
    }
}

/// Formats the trivia of a token that is present in the source text but should be omitted in the
/// formatted output
pub struct FormatRemoved<'a> {
    token: &'a JsSyntaxToken,
    track: bool,
    last_token: Option<RawSyntaxKind>,
    last_trailing_comment: Option<CommentKind>,
}

impl Format<JsFormatContext> for FormatRemoved<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        dbg!("format removed");
        let last_token = if self.track {
            let last = f.state().last_token();
            f.state_mut().track_token(self.token);
            last
        } else {
            self.last_token
        };

        let last_token = last_token.map(JsSyntaxKind::from_raw);

        let mut pieces = self
            .token
            .leading_trivia()
            .pieces()
            .chain(self.token.trailing_trivia().pieces())
            .peekable();

        let mut comments = vec![];
        // Collect all comments up to the first new line or skipped token trivia
        // These comments become the trailing comments of the previous token.
        while let Some(piece) = pieces.peek() {
            if let Some(comment) = piece.as_comments() {
                comments.push(comment);
            } else if piece.is_newline() || piece.is_skipped() {
                break;
            }

            pieces.next();
        }

        dbg!(&comments);

        if !comments.is_empty() {
            dbg!("non empty");
            let trailing_comments = comments
                .iter()
                .map(|piece| crate::trivia::Comment::trailing(piece.clone()));

            FormatTrailingTrivia::new(trailing_comments, last_token).fmt(f)?;
        };

        let trailing_comment_kind = f
            .state_mut()
            .take_last_trailing_comment_kind()
            .or(self.last_trailing_comment);

        // TODO: If the last comment written by leading trivia is an inline comment, force the next token
        // to print a trailing comment.
        let next_token_leading_comments = write_leading_trivia(
            pieces,
            self.token,
            LeadingTriviaOptions {
                ..Default::default()
            },
            f,
        )?;

        // Set kind to last of trailing comments & leading comments

        dbg!(trailing_comment_kind, &next_token_leading_comments);

        // Track the kind of the last comment so that the leading trivia formatting of the next token
        // can insert a leading whitespace if necessary
        f.state_mut().set_last_trailing_comment(
            next_token_leading_comments
                .last()
                .map(|c| c.kind())
                .or(trailing_comment_kind),
        );

        Ok(())
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

/// Formats the given token only if the group does break and otherwise retains the tokens trivia.
pub fn format_only_if_breaks<'a, 'content, Content>(
    token: &'a JsSyntaxToken,
    content: &'content Content,
) -> FormatOnlyIfBreaks<'a, 'content>
where
    Content: Format<JsFormatContext>,
{
    FormatOnlyIfBreaks {
        token,
        content: Argument::new(content),
        group_id: None,
    }
}

pub struct FormatOnlyIfBreaks<'a, 'content> {
    token: &'a JsSyntaxToken,
    content: Argument<'content, JsFormatContext>,
    group_id: Option<GroupId>,
}

impl FormatOnlyIfBreaks<'_, '_> {
    pub fn with_group_id(mut self, group_id: Option<GroupId>) -> Self {
        self.group_id = group_id;
        self
    }
}

impl Format<JsFormatContext> for FormatOnlyIfBreaks<'_, '_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        // Store the last token and last trailing comment before formatting the content which will override the
        // state
        // Is it safe to set `last_trailing_comment` only in the format removed because format removed may set it to true
        // but it's false for the "break" case. Ignorable, because it's after a new line break in that case?
        let last_token = f.state().last_token();
        let last_trailing_comment = f.state_mut().last_trailing_comment_kind();
        dbg!(last_token);
        write!(
            f,
            [
                if_group_breaks(&Arguments::from(&self.content)).with_group_id(self.group_id),
                // Print the trivia otherwise
                if_group_fits_on_line(&FormatRemoved {
                    token: self.token,
                    last_token,
                    last_trailing_comment,
                    track: false,
                })
                .with_group_id(self.group_id)
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

            let trivia = buffer.into_element();

            if !trivia.is_empty() {
                f.write_element(trivia)?;
                soft_line_break().fmt(f)?;
            }

            Ok(())
        });

        let close_token_leading_trivia = format_with(|f| {
            let mut buffer = PreambleBuffer::new(f, soft_line_break());

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
                    let mut buffer = VecBuffer::new(f.state_mut());
                    write!(
                        buffer,
                        [
                            open_token_trailing_trivia,
                            format_content,
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
                                })),
                                soft_line_break_or_space()
                            ]
                        )?;
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
