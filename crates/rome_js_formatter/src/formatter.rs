use crate::prelude::*;

use rome_formatter::{normalize_newlines, FormatResult, GroupId, LINE_TERMINATORS};
use rome_js_syntax::{JsLanguage, JsSyntaxNode, JsSyntaxToken};

use crate::{AsFormat, JsFormatOptions};
use rome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, Language, SyntaxNode, SyntaxTriviaPiece, TextRange,
};
use std::iter::once;

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
pub(super) enum TriviaPrintMode {
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
    FormatVerbatimNode { node }
}

#[derive(Debug, Clone)]
pub struct FormatVerbatimNode<'node> {
    node: &'node JsSyntaxNode,
}

impl Format for FormatVerbatimNode<'_> {
    type Options = JsFormatOptions;

    fn format(&self, formatter: &Formatter<JsFormatOptions>) -> FormatResult<FormatElement> {
        let verbatim = format_verbatim_node_or_token(self.node, formatter);
        Ok(FormatElement::Verbatim(Verbatim::new_verbatim(
            verbatim,
            self.node.text_range().len(),
        )))
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

impl Format for FormatUnknownNode<'_> {
    type Options = JsFormatOptions;

    fn format(&self, formatter: &Formatter<JsFormatOptions>) -> FormatResult<FormatElement> {
        Ok(FormatElement::Verbatim(Verbatim::new_unknown(
            format_verbatim_node_or_token(self.node, formatter),
        )))
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

impl Format for FormatSuppressedNode<'_> {
    type Options = JsFormatOptions;

    fn format(&self, formatter: &Formatter<JsFormatOptions>) -> FormatResult<FormatElement> {
        formatted![
            formatter,
            [
                // Insert a force a line break to ensure the suppression comment is on its own line
                // and correctly registers as a leading trivia on the opening token of this node
                hard_line_break(),
                FormatElement::Verbatim(Verbatim::new_suppressed(format_verbatim_node_or_token(
                    self.node, formatter
                ))),
            ]
        ]
    }
}

fn format_verbatim_node_or_token(
    node: &JsSyntaxNode,
    formatter: &Formatter<JsFormatOptions>,
) -> FormatElement {
    for token in node.descendants_tokens() {
        formatter.track_token(&token);
    }

    fn skip_whitespace<L: Language>(piece: &SyntaxTriviaPiece<L>) -> bool {
        piece.is_newline() || piece.is_whitespace()
    }

    fn trivia_token<L: Language>(piece: SyntaxTriviaPiece<L>) -> Token {
        Token::from_syntax_token_cow_slice(
            normalize_newlines(piece.text(), LINE_TERMINATORS),
            &piece.token(),
            piece.text_range().start(),
        )
    }

    let leading_trivia = node
        .first_leading_trivia()
        .into_iter()
        .flat_map(|trivia| trivia.pieces())
        .skip_while(skip_whitespace)
        .map(trivia_token);

    let content = Token::new_dynamic(
        normalize_newlines(&node.text_trimmed().to_string(), LINE_TERMINATORS).into_owned(),
        node.text_trimmed_range().start(),
    );

    // Clippy false positive: SkipWhile does not implement DoubleEndedIterator
    #[allow(clippy::needless_collect)]
    let trailing_trivia = node
        .last_trailing_trivia()
        .into_iter()
        .flat_map(|trivia| trivia.pieces().rev())
        .skip_while(skip_whitespace)
        .map(trivia_token)
        .collect::<Vec<_>>();

    concat_elements(
        leading_trivia
            .chain(once(content))
            .chain(trailing_trivia.into_iter().rev())
            .map(FormatElement::from),
    )
}

pub(crate) fn format_trailing_trivia(token: &JsSyntaxToken) -> FormatElement {
    format_trailing_trivia_pieces(token.trailing_trivia().pieces())
}

fn format_trailing_trivia_pieces<I>(pieces: I) -> FormatElement
where
    I: IntoIterator<Item = SyntaxTriviaPiece<JsLanguage>>,
{
    let mut elements = Vec::new();

    for piece in pieces {
        if let Some(comment) = piece.as_comments() {
            let is_single_line = comment.text().trim_start().starts_with("//");

            let comment = FormatElement::from(Token::from(comment));

            let content = if !is_single_line {
                format_elements![space_token(), comment, space_token(),]
            } else {
                format_elements![
                    line_suffix(format_elements![space_token(), comment]),
                    expand_parent()
                ]
            };

            elements.push(crate::comment(content));
        }
    }

    concat_elements(elements)
}

pub(super) fn format_leading_trivia(
    token: &JsSyntaxToken,
    trim_mode: TriviaPrintMode,
) -> FormatElement {
    // Checks whether the previous token has any trailing newline
    let has_trailing_newline = token
        .prev_token()
        .and_then(|token| token.trailing_trivia().last())
        .map_or(false, |trivia| trivia.is_newline());

    format_leading_trivia_pieces(
        token.leading_trivia().pieces(),
        trim_mode,
        has_trailing_newline,
    )
    .unwrap_or_else(|_| {
        format_leading_trivia_with_skipped_tokens(token, trim_mode, has_trailing_newline)
    })
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
fn format_leading_trivia_with_skipped_tokens(
    token: &JsSyntaxToken,
    trim_mode: TriviaPrintMode,
    has_trailing_newline: bool,
) -> FormatElement {
    let mut skipped_trivia_range: Option<TextRange> = None;
    // The leading trivia for the first skipped token trivia OR the leading trivia for the token
    let mut trailing_trivia = vec![];
    // The trailing trivia for the last skipped token trivia
    let mut leading_trivia = vec![];
    //  The formatted elements
    let mut elements = vec![];
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
                elements.push(
                    format_leading_trivia_pieces(
                        leading_trivia.drain(..),
                        trim_mode,
                        has_trailing_newline,
                    )
                    .expect("All skipped trivia pieces should have been filtered out"),
                );
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
    elements.push(FormatElement::from(Token::new_syntax_token_slice(
        token,
        skipped_trivia_range,
    )));

    // `print_trailing_trivia_pieces` and `format_leading_trivia_pieces` remove any whitespace except
    // if there's a comment but removing all whitespace may have a different semantic meaning.
    // Insert a:
    // * space if the skipped token has no trailing trivia (`skipped\n`, also works for `skipped//comment` because the comment must either be followed by a line break or the token is the EOF).
    // * new line if the token has any leading trivia. This can only be the case if there was any new line between the skipped trivia and the token
    // * empty: There's literally nothing between skipped and token, so don't insert anything
    let skipped_separator = if !trailing_trivia.is_empty() {
        space_token()
    } else if !leading_trivia.is_empty() {
        hard_line_break()
    } else {
        empty_element()
    };

    elements.push(skipped_separator);
    // Format the trailing pieces of the skipped token trivia
    elements.push(format_trailing_trivia_pieces(trailing_trivia));

    elements.push(
        format_leading_trivia_pieces(leading_trivia.into_iter(), trim_mode, after_newline)
            .expect("All skipped trivia pieces should have been filtered out"),
    );

    concat_elements(elements)
}

/// Formats the leading trivia pieces of a token.
///
/// ## Returns
///
/// Returns [Err] if the leading trivia contains any skipped trivia. Returns the formatted
/// leading trivia otherwise.
fn format_leading_trivia_pieces<I>(
    pieces: I,
    mut trim_mode: TriviaPrintMode,
    has_trailing_newline: bool,
) -> Result<FormatElement, ()>
where
    I: Iterator<Item = SyntaxTriviaPiece<JsLanguage>> + DoubleEndedIterator + ExactSizeIterator,
{
    let mut line_count = 0;
    let mut elements = Vec::new();

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
            return Err(());
        }

        if piece.is_newline() {
            has_leading_newline = true;
        }

        pieces.next();
    }

    // If any newline was found between the previous token and the first comment,
    // it will be prepended with a line break instead of a space
    let prepend_newline = has_trailing_newline || has_leading_newline;

    // This consumes the previously created iterator from the last trivia piece
    // towards the first (that was not consumed by the previous loop)
    for (index, piece) in pieces.rev() {
        if let Some(comment) = piece.as_comments() {
            let is_single_line = comment.text().starts_with("//");

            let comment = FormatElement::from(Token::from(comment));

            let element_before_comment = if prepend_newline && index == first_comment {
                hard_line_break()
            } else {
                space_token()
            };

            let element_after_comment = if is_single_line {
                match line_count {
                    0 | 1 => hard_line_break(),
                    _ => empty_line(),
                }
            } else {
                match line_count {
                    0 => space_token(),
                    1 => hard_line_break(),
                    _ => empty_line(),
                }
            };

            elements.push(crate::comment(format_elements![
                element_before_comment,
                comment,
                element_after_comment,
            ]));

            line_count = 0;
            trim_mode = TriviaPrintMode::Full;
        } else if piece.is_newline() && trim_mode == TriviaPrintMode::Full {
            line_count += 1;
        } else if piece.is_skipped() {
            return Err(());
        }
    }

    Ok(concat_elements(elements.into_iter().rev()))
}

/// JS specific formatter extensions
pub(crate) trait JsFormatter {
    fn as_formatter(&self) -> &Formatter<JsFormatOptions>;

    #[must_use]
    fn delimited<'a, 'fmt>(
        &'fmt self,
        open_token: &'a JsSyntaxToken,
        content: FormatElement,
        close_token: &'a JsSyntaxToken,
    ) -> FormatDelimited<'a, 'fmt> {
        FormatDelimited::new(open_token, content, close_token, self.as_formatter())
    }

    /// Print out a `token` from the original source with a different `content`.
    ///
    /// This will print the trivias that belong to `token` to `content`;
    /// `token` is then marked as consumed by the formatter.
    fn format_replaced(
        &self,
        current_token: &JsSyntaxToken,
        content_to_replace_with: FormatElement,
    ) -> FormatElement {
        let formatter = self.as_formatter();

        formatter.track_token(current_token);

        format_elements![
            format_leading_trivia(current_token, TriviaPrintMode::Full),
            content_to_replace_with,
            format_trailing_trivia(current_token),
        ]
    }

    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    fn format_separated<L, F>(
        &self,
        list: &L,
        separator_factory: F,
    ) -> FormatResult<std::vec::IntoIter<FormatElement>>
    where
        L: AstSeparatedList<Language = JsLanguage>,
        for<'a> L::Node: AstNode<Language = JsLanguage> + AsFormat<'a>,
        F: Fn() -> FormatElement,
    {
        self.format_separated_with_options(
            list,
            separator_factory,
            FormatSeparatedOptions::default(),
        )
    }

    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    fn format_separated_with_options<L, F>(
        &self,
        list: &L,
        separator_factory: F,
        options: FormatSeparatedOptions,
    ) -> FormatResult<std::vec::IntoIter<FormatElement>>
    where
        L: AstSeparatedList<Language = JsLanguage>,
        for<'a> L::Node: AstNode<Language = JsLanguage> + AsFormat<'a>,
        F: Fn() -> FormatElement,
    {
        let mut result = Vec::with_capacity(list.len());
        let last_index = list.len().saturating_sub(1);
        let formatter = self.as_formatter();

        let trailing_separator_factory = || {
            if let Some(group_id) = options.group_id {
                if_group_with_id_breaks(separator_factory(), group_id)
            } else {
                if_group_breaks(separator_factory())
            }
        };

        let trailing_separator = options.trailing_separator;

        for (index, element) in list.elements().enumerate() {
            let node = formatted![formatter, [element.node()?.format()]]?;

            // Reuse the existing trailing separator or create it if it wasn't in the
            // input source. Only print the last trailing token if the outer group breaks
            let separator = if let Some(separator) = element.trailing_separator()? {
                if index == last_index {
                    if trailing_separator.is_allowed() {
                        // Use format_replaced instead of wrapping the result of format_token
                        // in order to remove only the token itself when the group doesn't break
                        // but still print its associated trivias unconditionally
                        self.format_replaced(separator, trailing_separator_factory())
                    } else if trailing_separator.is_mandatory() {
                        formatted![formatter, [separator.format()]]?
                    } else {
                        empty_element()
                    }
                } else {
                    formatted![formatter, [separator.format()]]?
                }
            } else if index == last_index {
                if trailing_separator.is_allowed() {
                    trailing_separator_factory()
                } else if trailing_separator.is_mandatory() {
                    separator_factory()
                } else {
                    empty_element()
                }
            } else {
                separator_factory()
            };

            result.push(format_elements![group_elements(node), separator]);
        }

        Ok(result.into_iter())
    }

    /// It formats a list of nodes that are not separated. Essentially a wrapper for [format_list] that
    /// joins the result together with [join_elements_hard_line].
    fn format_list_with_hard_line<List, Node>(&self, list: &List) -> FormatElement
    where
        List: AstNodeList<Language = JsLanguage, Node = Node>,
        for<'a> Node: AstNode<Language = JsLanguage> + AsFormat<'a>,
    {
        join_elements_hard_line(self.format_list(list))
    }

    /// It formats a list of nodes that are not separated. It's an ad-hoc function to
    /// format lists that implement [rome_js_syntax::AstNodeList].
    ///
    /// Returns a list of syntax nodes and format elements.
    ///
    /// If the formatter fails to format an element, said element gets printed verbatim.
    fn format_list<List, Node>(&self, list: &List) -> Vec<(SyntaxNode<JsLanguage>, FormatElement)>
    where
        List: AstNodeList<Language = JsLanguage, Node = Node>,
        for<'a> Node: AstNode<Language = JsLanguage> + AsFormat<'a>,
    {
        let formatter = self.as_formatter();
        let formatted_list = list.iter().map(|module_item| {
            let snapshot = formatter.snapshot();
            let format = formatted![formatter, [module_item.format()]];

            let elem = match formatted![formatter, [format]] {
                Ok(result) => result,
                Err(_) => {
                    formatter.restore(snapshot);

                    // Lists that yield errors are formatted as they were unknown nodes.
                    // Doing so, the formatter formats the nodes/tokens as is.
                    // SAFETY: `FormatUnknownNode` always returns Ok
                    unknown_node(module_item.syntax())
                        .format(formatter)
                        .unwrap()
                }
            };

            (module_item.syntax().clone(), elem)
        });

        formatted_list.collect()
    }
}

impl JsFormatter for Formatter<JsFormatOptions> {
    fn as_formatter(&self) -> &Formatter<JsFormatOptions> {
        self
    }
}

/// Formats a group delimited by an opening and closing token,
/// such as a function body delimited by '{' and '}' tokens
///
/// Calling this method is required to correctly handle the comments attached
/// to the opening and closing tokens and insert them inside the group block
pub struct FormatDelimited<'a, 'fmt> {
    open_token: &'a JsSyntaxToken,
    content: FormatElement,
    close_token: &'a JsSyntaxToken,
    mode: DelimitedMode,
    formatter: &'fmt Formatter<JsFormatOptions>,
}

impl<'a, 'fmt> FormatDelimited<'a, 'fmt> {
    fn new(
        open_token: &'a JsSyntaxToken,
        content: FormatElement,
        close_token: &'a JsSyntaxToken,
        formatter: &'fmt Formatter<JsFormatOptions>,
    ) -> Self {
        Self {
            open_token,
            content,
            close_token,
            mode: DelimitedMode::SoftBlockIndent(None),
            formatter,
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

    pub fn finish(self) -> FormatResult<FormatElement> {
        let FormatDelimited {
            formatter,
            open_token,
            close_token,
            content,
            mode,
        } = self;

        formatter.track_token(open_token);
        formatter.track_token(close_token);

        let open_token_trailing_trivia = format_trailing_trivia(open_token);
        let close_token_leading_trivia = format_leading_trivia(close_token, TriviaPrintMode::Trim);

        let open_token_trailing_trivia = if !open_token_trailing_trivia.is_empty() {
            formatted![
                formatter,
                [open_token_trailing_trivia, soft_line_break_or_space()]
            ]?
        } else {
            empty_element()
        };
        let close_token_leading_trivia = if !close_token_leading_trivia.is_empty() {
            formatted![
                formatter,
                [soft_line_break_or_space(), close_token_leading_trivia]
            ]?
        } else {
            empty_element()
        };

        let formatted_content = match mode {
            DelimitedMode::BlockIndent => block_indent(formatted![
                formatter,
                [
                    open_token_trailing_trivia,
                    content,
                    close_token_leading_trivia
                ]
            ]?),
            DelimitedMode::SoftBlockIndent(_) => soft_block_indent(formatted![
                formatter,
                [
                    open_token_trailing_trivia,
                    content,
                    close_token_leading_trivia
                ]
            ]?),
            DelimitedMode::SoftBlockSpaces(_) => {
                if open_token_trailing_trivia.is_empty()
                    && content.is_empty()
                    && close_token_leading_trivia.is_empty()
                {
                    empty_element()
                } else {
                    formatted![
                        formatter,
                        [
                            indent(formatted![
                                formatter,
                                [
                                    soft_line_break_or_space(),
                                    open_token_trailing_trivia,
                                    content,
                                    close_token_leading_trivia,
                                ]
                            ]?),
                            soft_line_break_or_space(),
                        ]
                    ]?
                }
            }
        };

        let delimited = format_elements![
            Token::from(open_token),
            formatted_content,
            Token::from(close_token),
        ];

        let grouped = match mode {
            // Group is useless, the block indent would expand it right anyway
            DelimitedMode::BlockIndent => delimited,
            DelimitedMode::SoftBlockIndent(group_id) | DelimitedMode::SoftBlockSpaces(group_id) => {
                match group_id {
                    None => group_elements(delimited),
                    Some(group_id) => group_elements_with_options(
                        delimited,
                        GroupElementsOptions {
                            group_id: Some(group_id),
                        },
                    ),
                }
            }
        };

        formatted![
            formatter,
            [
                format_leading_trivia(open_token, TriviaPrintMode::Full),
                grouped,
                format_trailing_trivia(close_token),
            ]
        ]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum DelimitedMode {
    BlockIndent,
    SoftBlockIndent(Option<GroupId>),
    SoftBlockSpaces(Option<GroupId>),
}
