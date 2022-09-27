use crate::format_element::tag::VerbatimKind;
use crate::prelude::*;
use crate::trivia::{FormatLeadingComments, FormatTrailingComments};
use crate::{write, CstFormatContext, TextLen, TextSize};
use rome_rowan::{
    Direction, Language, SyntaxElement, SyntaxNode, SyntaxToken, SyntaxTokenText, TextRange,
};
use std::iter::FusedIterator;
use std::str::CharIndices;

/// "Formats" a node according to its original formatting in the source text. Being able to format
/// a node "as is" is useful if a node contains syntax errors. Formatting a node with syntax errors
/// has the risk that Rome misinterprets the structure of the code and formatting it could
/// "mess up" the developers, yet incomplete, work or accidentally introduce new syntax errors.
///
/// You may be inclined to call `node.text` directly. However, using `text` doesn't track the nodes
/// nor its children source mapping information, resulting in incorrect source maps for this subtree.
///
/// These nodes and tokens get tracked as [VerbatimKind::Verbatim], useful to understand
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
                    let comments = f.context().comments();
                    comments.mark_suppression_checked(&node);

                    for comment in comments.leading_dangling_trailing_comments(&node) {
                        comment.mark_formatted();
                    }
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

        f.write_element(FormatElement::Tag(Tag::StartVerbatim(self.kind)))?;

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

            let (outside_trimmed_range, in_trimmed_range) =
                leading_comments.split_at(outside_trimmed_range);

            write!(f, [FormatLeadingComments::Comments(outside_trimmed_range)])?;

            for comment in in_trimmed_range {
                comment.mark_formatted();
            }
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
                source_map.text()[trimmed_source_range.cover_offset(start_source)].to_string()
            },
        );

        normalize_newlines(
            &original_source,
            LINE_TERMINATORS,
            self.node.text_trimmed_range().start(),
        )
        .fmt(f)?;

        for comment in f.context().comments().dangling_comments(self.node) {
            comment.mark_formatted();
        }

        // Format all trailing comments that are outside of the trimmed range.
        if self.format_comments {
            let comments = f.context().comments().clone();

            let trailing_comments = comments.trailing_comments(self.node);

            let outside_trimmed_range_start = trailing_comments.partition_point(|comment| {
                source_range(f, comment.piece().text_range()).end() <= trimmed_source_range.end()
            });

            let (in_trimmed_range, outside_trimmed_range) =
                trailing_comments.split_at(outside_trimmed_range_start);

            for comment in in_trimmed_range {
                comment.mark_formatted();
            }

            write!(f, [FormatTrailingComments::Comments(outside_trimmed_range)])?;
        }

        f.write_element(FormatElement::Tag(Tag::EndVerbatim))
    }
}

impl<L: Language> FormatVerbatimNode<'_, L> {
    pub fn skip_comments(mut self) -> Self {
        self.format_comments = false;
        self
    }
}

/// Formats unknown nodes. The difference between this method  and `format_verbatim` is that this method
/// doesn't track nodes/tokens as [VerbatimKind::Verbatim]. They are just printed as they are.
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

/// Replace the line terminators matching the provided list with "literalline".
pub const fn normalize_newlines<const N: usize>(
    text: &str,
    terminators: [char; N],
    source_position: TextSize,
) -> NormalizeNewLines<N> {
    NormalizeNewLines {
        text,
        terminators,
        source_position,
    }
}

const LINE_SEPARATOR: char = '\u{2028}';
const PARAGRAPH_SEPARATOR: char = '\u{2029}';
pub const LINE_TERMINATORS: [char; 4] = ['\n', '\r', LINE_SEPARATOR, PARAGRAPH_SEPARATOR];

pub struct NormalizeNewLines<'a, const N: usize> {
    text: &'a str,
    source_position: TextSize,
    terminators: [char; N],
}

impl<const N: usize, Context> Format<Context> for NormalizeNewLines<'_, N> {
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        for part in NewlineParts::new(self.text, self.terminators) {
            match part {
                NewlineOrText::Newline => {
                    write!(f, [literal_line()])?;
                }
                NewlineOrText::Text((start, text)) => {
                    write!(f, [dynamic_text(text, self.source_position + start)])?;
                }
            }
        }

        Ok(())
    }
}

/// Replace the line terminators matching the provided list with "literalline".
pub fn normalize_token_text_new_lines<const N: usize, L: Language>(
    token: &SyntaxToken<L>,
    range: TextRange,
    terminators: [char; N],
) -> NormalizeTokenTextNewLines<N> {
    let relative_range = range - token.text_range().start();
    let text = token.token_text().slice(relative_range);

    NormalizeTokenTextNewLines {
        text,
        terminators,
        source_position: range.start(),
    }
}

pub struct NormalizeTokenTextNewLines<const N: usize> {
    pub text: SyntaxTokenText,
    pub terminators: [char; N],
    pub source_position: TextSize,
}

impl<const N: usize, Context> Format<Context> for NormalizeTokenTextNewLines<N> {
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        for part in NewlineParts::new(&self.text, self.terminators) {
            match part {
                NewlineOrText::Newline => {
                    write!(f, [literal_line()])?;
                }
                NewlineOrText::Text((start, text)) => {
                    f.write_element(FormatElement::Text(Text::SyntaxTokenTextSlice {
                        source_position: self.source_position + start,
                        slice: self.text.clone().slice(TextRange::at(
                            start + self.text.range().start(),
                            text.text_len(),
                        )),
                    }))?;
                }
            }
        }

        Ok(())
    }
}

struct NewlineParts<'a, const N: usize> {
    text: &'a str,
    chars: std::iter::Peekable<CharIndices<'a>>,
    terminators: [char; N],
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum NewlineOrText<'a> {
    Newline,
    Text((TextSize, &'a str)),
}

impl<'a, const N: usize> NewlineParts<'a, N> {
    fn new(text: &'a str, terminators: [char; N]) -> Self {
        Self {
            chars: text.char_indices().peekable(),
            text,
            terminators,
        }
    }
}

impl<'a, const N: usize> Iterator for NewlineParts<'a, N> {
    type Item = NewlineOrText<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, char) = self.chars.next()?;

        if self.terminators.contains(&char) {
            // If the current character is \r and the
            // next is \n, skip over the entire sequence
            if char == '\r' && matches!(self.chars.peek(), Some((_, '\n'))) {
                self.chars.next();
            }

            Some(NewlineOrText::Newline)
        } else {
            let start_position = TextSize::from(start as u32);
            loop {
                match self.chars.peek() {
                    Some((index, next)) => {
                        if self.terminators.contains(next) {
                            break Some(NewlineOrText::Text((
                                start_position,
                                &self.text[start..*index],
                            )));
                        }

                        self.chars.next();
                    }
                    None => break Some(NewlineOrText::Text((start_position, &self.text[start..]))),
                }
            }
        }
    }
}

impl<const N: usize> FusedIterator for NewlineParts<'_, N> {}

#[cfg(test)]
mod tests {

    use super::{NewlineOrText, NewlineParts, LINE_TERMINATORS};
    use crate::TextSize;

    #[test]
    fn test_normalize_newlines() {
        assert_eq!(
            NewlineParts::new("a\nb", LINE_TERMINATORS).collect::<Vec<_>>(),
            vec![
                NewlineOrText::Text((TextSize::from(0), "a")),
                NewlineOrText::Newline,
                NewlineOrText::Text((TextSize::from(2), "b"))
            ]
        );
        assert_eq!(
            NewlineParts::new("a\n\n\nb", LINE_TERMINATORS).collect::<Vec<_>>(),
            vec![
                NewlineOrText::Text((TextSize::from(0), "a")),
                NewlineOrText::Newline,
                NewlineOrText::Newline,
                NewlineOrText::Newline,
                NewlineOrText::Text((TextSize::from(4), "b"))
            ]
        );

        assert_eq!(
            NewlineParts::new("a\rb", LINE_TERMINATORS).collect::<Vec<_>>(),
            vec![
                NewlineOrText::Text((TextSize::from(0), "a")),
                NewlineOrText::Newline,
                NewlineOrText::Text((TextSize::from(2), "b"))
            ]
        );
        assert_eq!(
            NewlineParts::new("a\r\nb", LINE_TERMINATORS).collect::<Vec<_>>(),
            vec![
                NewlineOrText::Text((TextSize::from(0), "a")),
                NewlineOrText::Newline,
                NewlineOrText::Text((TextSize::from(3), "b"))
            ]
        );
        assert_eq!(
            NewlineParts::new("a\u{2028}b", LINE_TERMINATORS).collect::<Vec<_>>(),
            vec![
                NewlineOrText::Text((TextSize::from(0), "a")),
                NewlineOrText::Newline,
                NewlineOrText::Text((TextSize::from(4), "b"))
            ]
        );
        assert_eq!(
            NewlineParts::new("a\u{2029}b", LINE_TERMINATORS).collect::<Vec<_>>(),
            vec![
                NewlineOrText::Text((TextSize::from(0), "a")),
                NewlineOrText::Newline,
                NewlineOrText::Text((TextSize::from(4), "b"))
            ]
        );
    }
}
