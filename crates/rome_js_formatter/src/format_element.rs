use rome_formatter::intersperse::IntersperseFn;
use rome_formatter::{
    concat_elements, empty_element, empty_line, hard_line_break, soft_line_break_or_space,
    FormatElement,
};
use rome_js_syntax::{AstNode, SyntaxNode};

/// Specialized version of [join_elements] for joining SyntaxNodes separated by a space, soft
/// line break or empty line depending on the input file.
///
/// This functions inspects the input source and separates consecutive elements with either
/// a [soft_line_break_or_space] or [empty_line] depending on how many line breaks were
/// separating the elements in the original file.
#[inline]
pub fn join_elements_soft_line<I, N>(elements: I) -> FormatElement
where
    I: IntoIterator<Item = (N, FormatElement)>,
    N: AstNode,
{
    join_elements_with(elements, soft_line_break_or_space)
}

/// Specialized version of [join_elements] for joining SyntaxNodes separated by one or more
/// line breaks depending on the input file.
///
/// This functions inspects the input source and separates consecutive elements with either
/// a [hard_line_break] or [empty_line] depending on how many line breaks were separating the
/// elements in the original file.
#[inline]
pub fn join_elements_hard_line<I, N>(elements: I) -> FormatElement
where
    I: IntoIterator<Item = (N, FormatElement)>,
    N: AstNode,
{
    join_elements_with(elements, hard_line_break)
}

#[inline]
pub fn join_elements_with<I, N>(elements: I, separator: fn() -> FormatElement) -> FormatElement
where
    I: IntoIterator<Item = (N, FormatElement)>,
    N: AstNode,
{
    /// Get the number of line breaks between two consecutive SyntaxNodes in the tree
    fn get_lines_between_nodes(prev_node: &SyntaxNode, next_node: &SyntaxNode) -> usize {
        // Ensure the two nodes are actually siblings on debug
        debug_assert_eq!(prev_node.next_sibling().as_ref(), Some(next_node));
        debug_assert_eq!(next_node.prev_sibling().as_ref(), Some(prev_node));

        // Count the lines separating the two statements,
        // starting with the trailing trivia of the previous node
        let mut line_count = prev_node
            .last_trailing_trivia()
            .and_then(|prev_token| {
                // Newline pieces can only come last in trailing trivias, skip to it directly
                prev_token.pieces().next_back()?.as_newline()
            })
            .is_some() as usize;

        // Then add the newlines in the leading trivia of the next node
        if let Some(leading_trivia) = next_node.first_leading_trivia() {
            for piece in leading_trivia.pieces() {
                if piece.is_newline() {
                    line_count += 1;
                } else if piece.is_comments() {
                    // Stop at the first comment piece, the comment printer
                    // will handle newlines between the comment and the node
                    break;
                }
            }
        }

        line_count
    }

    concat_elements(IntersperseFn::new(
        elements.into_iter(),
        |prev_node, next_node, next_elem| {
            if next_elem.is_empty() {
                empty_element()
            } else if get_lines_between_nodes(prev_node.syntax(), next_node.syntax()) > 1 {
                empty_line()
            } else {
                separator()
            }
        },
    ))
}
