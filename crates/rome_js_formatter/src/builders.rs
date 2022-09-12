use crate::prelude::*;
use crate::AsFormat;
use rome_formatter::token::{FormatLeadingComments, FormatTrailingComments};
use rome_formatter::{
    write, Argument, Arguments, CstFormatContext, FormatContext, GroupId, VecBuffer,
};
use rome_js_syntax::{JsLanguage, JsSyntaxNode, JsSyntaxToken};
use rome_rowan::{AstNode, Direction, SyntaxElement, TextRange};

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

                // Lists that yield errors are formatted as they were suppressed nodes.
                // Doing so, the formatter formats the nodes/tokens as is.
                format_suppressed_node(self.node.syntax()).fmt(f)
            }
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

        // The trimmed range of a node is its range without any of its leading or trailing trivia.
        // Except for nodes that used to be parenthesized, the range than covers the source from the
        // `(` to the `)` (the trimmed range of the parenthesized expression, not the inner expression)
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

                // Format all leading comments that are outside of the node's source range.
                let comments = f.context().comments().clone();
                let leading_comments = comments.leading_comments(self.node);

                let outside_trimmed_range = leading_comments.partition_point(|comment| {
                    comment.piece().text_range().end() <= trimmed_source_range.start()
                });

                write!(
                    f,
                    [FormatLeadingComments::Comments(
                        &leading_comments[..outside_trimmed_range]
                    )]
                )?;

                // Find the first skipped token trivia, if any, and include it in the verbatim range because
                // the comments only format **up to** but not including skipped token trivia.
                let start_source = self
                    .node
                    .first_leading_trivia()
                    .into_iter()
                    .flat_map(|trivia| trivia.pieces())
                    .filter(|trivia| trivia.is_skipped())
                    .map(|trivia| source_range(f, trivia.text_range()).start())
                    .take_while(|start| *start < trimmed_source_range.start())
                    .next()
                    .unwrap_or(trimmed_source_range.start());

                let original_source = f.context().source_map().map_or_else(
                    || self.node.text_trimmed().to_string(),
                    |source_map| {
                        source_map.text()[trimmed_source_range.cover_offset(start_source)]
                            .to_string()
                    },
                );

                dynamic_text(
                    &normalize_newlines(&original_source, LINE_TERMINATORS),
                    self.node.text_trimmed_range().start(),
                )
                .fmt(f)?;

                // Format all trailing comments that are outside of the trimmed range.
                let comments = f.context().comments().clone();
                let trailing_comments = comments.trailing_comments(self.node);

                let outside_trimmed_range_start = trailing_comments.partition_point(|comment| {
                    source_range(f, comment.piece().text_range()).end()
                        <= trimmed_source_range.end()
                });

                write!(
                    f,
                    [FormatTrailingComments::Comments(
                        &trailing_comments[outside_trimmed_range_start..]
                    )]
                )
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
        write!(
            f,
            [FormatVerbatimNode {
                node: self.node,
                kind: VerbatimKind::Suppressed,
            }]
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

// FIXME delete `BlockIndent`, maybe delete soft_block_indent` and spaces too?
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

        let delimited = format_with(|f| {
            open_token.format().fmt(f)?;

            let format_content = format_with(|f| f.write_fmt(Arguments::from(content)));

            match mode {
                DelimitedMode::BlockIndent => block_indent(&format_content).fmt(f)?,
                DelimitedMode::SoftBlockIndent(_) => soft_block_indent(&format_content).fmt(f)?,
                DelimitedMode::SoftBlockSpaces(_) => {
                    let mut is_empty = true;

                    let format_content = format_once(|f| {
                        let mut recording = f.start_recording();

                        write!(recording, [format_content])?;

                        is_empty = recording.stop().is_empty();

                        Ok(())
                    });

                    soft_line_indent_or_space(&format_content).fmt(f)?;

                    if !is_empty {
                        soft_line_break_or_space().fmt(f)?;
                    }
                }
            };

            close_token.format().fmt(f)
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

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum DelimitedMode {
    BlockIndent,
    SoftBlockIndent(Option<GroupId>),
    SoftBlockSpaces(Option<GroupId>),
}
