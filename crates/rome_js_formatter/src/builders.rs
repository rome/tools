use crate::prelude::*;
use crate::{AsFormat, JsCommentStyle};
use rome_formatter::token::{
    format_token_trailing_trivia, FormatInserted, FormatLeadingTrivia, FormatOnlyIfBreaks,
    FormatRemoved, FormatReplaced, InsertedToken, TriviaPrintMode,
};
use rome_formatter::{format_args, write, Argument, Arguments, GroupId, PreambleBuffer, VecBuffer};
use rome_js_syntax::{JsLanguage, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken};
use rome_rowan::{AstNode, Direction, Language, SyntaxTriviaPiece};

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

pub fn format_inserted(kind: JsSyntaxKind) -> FormatInserted<JsCommentStyle> {
    FormatInserted::new(
        InsertedToken::new(
            kind,
            kind.to_string().expect("Expected a punctuation token"),
        ),
        JsCommentStyle,
    )
}

/// Adds parentheses around some content
pub fn format_parenthesize<Content>(
    open_paren: JsSyntaxKind,
    content: &Content,
    close_paren: JsSyntaxKind,
) -> FormatParenthesize<JsFormatContext, JsCommentStyle>
where
    Content: Format<JsFormatContext>,
{
    let open_paren = InsertedToken::new(
        open_paren,
        open_paren
            .to_string()
            .expect("Expected a punctuation token as the open paren token."),
    );
    let close_paren = InsertedToken::new(
        close_paren,
        close_paren
            .to_string()
            .expect("Expected a punctuation token as the close paren token."),
    );

    FormatParenthesize::new(open_paren, content, close_paren, JsCommentStyle)
}

/// Formats the leading and trailing trivia of a removed token.
///
/// Formats all leading and trailing comments up to the first line break or skipped token trivia as a trailing
/// comment of the previous token. The remaining trivia is then printed as leading trivia of the next token.
pub const fn format_removed(token: &JsSyntaxToken) -> FormatRemoved<JsCommentStyle> {
    FormatRemoved::new(token, JsCommentStyle)
}

/// Print out a `token` from the original source with a different `content`.
///
/// This will print the trivia that belong to `token` to `content`;
/// `token` is then marked as consumed by the formatter.
pub fn format_replaced<'a, 'content>(
    token: &'a JsSyntaxToken,
    content: &'content impl Format<JsFormatContext>,
) -> FormatReplaced<'a, 'content, JsCommentStyle, JsFormatContext> {
    FormatReplaced::new(token, content, JsCommentStyle)
}

/// Formats the given token only if the group does break and otherwise retains the tokens trivia.
pub fn format_only_if_breaks<'a, 'content, Content>(
    token: &'a JsSyntaxToken,
    content: &'content Content,
) -> FormatOnlyIfBreaks<'a, 'content, JsCommentStyle, JsFormatContext>
where
    Content: Format<JsFormatContext>,
{
    FormatOnlyIfBreaks::new(token, content, JsCommentStyle)
}

/// Formats the leading trivia (comments, skipped token trivia) of a token
pub fn format_leading_trivia(token: &JsSyntaxToken) -> FormatLeadingTrivia<JsCommentStyle> {
    FormatLeadingTrivia::new(token, JsCommentStyle)
}

/// Formats the trailing trivia (comments) of a token
pub fn format_trailing_trivia(token: &JsSyntaxToken) -> impl Format<JsFormatContext> {
    format_token_trailing_trivia(token, JsCommentStyle)
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

        format_leading_trivia(open_token).fmt(f)?;

        let open_token_trailing_trivia = format_with(|f| {
            // Not really interested in the pre-amble, but want to know if it was written
            let mut buffer = VecBuffer::new(f.state_mut());

            write!(buffer, [format_trailing_trivia(open_token)])?;

            let trivia = buffer.into_vec();

            if !trivia.is_empty() {
                f.write_elements(trivia)?;
                soft_line_break().fmt(f)?;
            }

            Ok(())
        });

        let close_token_leading_trivia = format_with(|f| {
            let mut buffer = PreambleBuffer::new(f, soft_line_break());

            write!(
                buffer,
                [format_leading_trivia(close_token).with_trim_mode(TriviaPrintMode::Trim)]
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
