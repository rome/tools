use crate::prelude::*;
use crate::trivia::{FormatLeadingComments, FormatTrailingComments};
use crate::VecBuffer;
use crate::{write, CstFormatContext};
use rome_rowan::{Direction, Language, SyntaxElement, SyntaxNode, TextRange};

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
pub fn format_verbatim_node<L: Language>(node: &SyntaxNode<L>) -> FormatVerbatimNode<L> {
    FormatVerbatimNode {
        node,
        kind: VerbatimKind::Verbatim {
            length: node.text_range().len(),
        },
        format_comments: true,
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FormatVerbatimNode<'node, L: Language> {
    node: &'node SyntaxNode<L>,
    kind: VerbatimKind,
    format_comments: bool,
}

impl<Context> Format<Context> for FormatVerbatimNode<'_, Context::Language>
where
    Context: CstFormatContext,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
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
            [format_with(|f: &mut Formatter<Context>| {
                fn source_range<Context>(f: &Formatter<Context>, range: TextRange) -> TextRange
                where
                    Context: CstFormatContext,
                {
                    f.context()
                        .source_map()
                        .map_or_else(|| range, |source_map| source_map.source_range(range))
                }

                // Format all leading comments that are outside of the node's source range.
                if self.format_comments {
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
                }

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
                    .unwrap_or_else(|| trimmed_source_range.start());

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
                if self.format_comments {
                    let comments = f.context().comments().clone();

                    let trailing_comments = comments.trailing_comments(self.node);

                    let outside_trimmed_range_start =
                        trailing_comments.partition_point(|comment| {
                            source_range(f, comment.piece().text_range()).end()
                                <= trimmed_source_range.end()
                        });

                    write!(
                        f,
                        [FormatTrailingComments::Comments(
                            &trailing_comments[outside_trimmed_range_start..]
                        )]
                    )?;
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

impl<L: Language> FormatVerbatimNode<'_, L> {
    pub fn skip_comments(mut self) -> Self {
        self.format_comments = false;
        self
    }
}

/// Formats unknown nodes. The difference between this method  and `format_verbatim` is that this method
/// doesn't track nodes/tokens as [FormatElement::Verbatim]. They are just printed as they are.
pub fn format_unknown_node<L: Language>(node: &SyntaxNode<L>) -> FormatVerbatimNode<L> {
    FormatVerbatimNode {
        node,
        kind: VerbatimKind::Unknown,
        format_comments: true,
    }
}

/// Format a node having formatter suppression comment applied to it
pub fn format_suppressed_node<L: Language>(node: &SyntaxNode<L>) -> FormatVerbatimNode<L> {
    FormatVerbatimNode {
        node,
        kind: VerbatimKind::Suppressed,
        format_comments: true,
    }
}
