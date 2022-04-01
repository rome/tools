use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::has_formatter_suppressions;
use crate::{
    block_indent, concat_elements, empty_element, empty_line, format_elements, group_elements,
    hard_line_break, if_group_breaks, if_group_fits_on_single_line, indent,
    join_elements_hard_line, line_suffix, soft_block_indent, soft_line_break_or_space, space_token,
    FormatElement, FormatOptions, FormatResult, TextRange, ToFormatElement, Token, Verbatim,
};
use rome_formatter::{normalize_newlines, LINE_TERMINATORS};
use rome_js_syntax::{AstNode, AstNodeList, AstSeparatedList, JsLanguage, SyntaxNode, SyntaxToken};
use rome_rowan::{Language, SyntaxTriviaPiece};
#[cfg(debug_assertions)]
use std::cell::RefCell;
#[cfg(debug_assertions)]
use std::collections::HashSet;
use std::iter::once;

/// Handles the formatting of a CST and stores the options how the CST should be formatted (user preferences).
/// The formatter is passed to the [ToFormatElement] implementation of every node in the CST so that they
/// can use it to format their children.
#[derive(Debug, Default)]
pub struct Formatter {
    options: FormatOptions,
    // This is using a RefCell as it only exists in debug mode,
    // the Formatter is still completely immutable in release builds
    #[cfg(debug_assertions)]
    pub(super) printed_tokens: RefCell<HashSet<SyntaxToken>>,
}

#[derive(Debug)]
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

impl Formatter {
    /// Creates a new context that uses the given formatter options
    pub fn new(options: FormatOptions) -> Self {
        Self {
            options,
            #[cfg(debug_assertions)]
            printed_tokens: RefCell::default(),
        }
    }

    /// Returns the [FormatOptions] specifying how to format the current CST
    #[inline]
    pub fn options(&self) -> &FormatOptions {
        &self.options
    }

    /// Formats a CST
    #[tracing::instrument(level = "debug", skip_all)]
    pub(crate) fn format_root(self, root: &SyntaxNode) -> FormatResult<FormatElement> {
        let element = self.format_syntax_node(root)?;

        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                let printed_tokens = self.printed_tokens.into_inner();
                for token in root.descendants_tokens() {
                    assert!(
                        printed_tokens.contains(&token),
                        "token was not seen by the formatter: {:?}",
                        token
                    );
                }
            }
        }

        Ok(element)
    }

    fn format_syntax_node(&self, node: &SyntaxNode) -> FormatResult<FormatElement> {
        if has_formatter_suppressions(node) {
            return Ok(self.format_suppressed(node));
        }

        node.to_format_element(self)
    }

    /// Formats a group delimited by an opening and closing token,
    /// such as a function body delimited by '{' and '}' tokens
    ///
    /// Calling this method is required to correctly handle the comments attached
    /// to the opening and closing tokens and insert them inside the group block
    fn format_delimited(
        &self,
        open_token: &SyntaxToken,
        content: impl FnOnce(FormatElement, FormatElement) -> FormatElement,
        close_token: &SyntaxToken,
    ) -> FormatResult<FormatElement> {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                let mut printed_tokens = self.printed_tokens.borrow_mut();
                assert!(printed_tokens.insert(open_token.clone()));
                assert!(printed_tokens.insert(close_token.clone()));
                drop(printed_tokens);
            }
        }
        let open_token_trailing_trivia = self.print_trailing_trivia(open_token);
        let close_token_leading_trivia =
            self.print_leading_trivia(close_token, TriviaPrintMode::Trim);

        let open_token_trailing_trivia = if !open_token_trailing_trivia.is_empty() {
            format_elements![open_token_trailing_trivia, soft_line_break_or_space()]
        } else {
            empty_element()
        };
        let close_token_leading_trivia = if !close_token_leading_trivia.is_empty() {
            format_elements![soft_line_break_or_space(), close_token_leading_trivia]
        } else {
            empty_element()
        };
        Ok(format_elements![
            self.print_leading_trivia(open_token, TriviaPrintMode::Full),
            group_elements(format_elements![
                Token::from(open_token),
                content(open_token_trailing_trivia, close_token_leading_trivia),
                Token::from(close_token),
            ]),
            self.print_trailing_trivia(close_token),
        ])
    }

    /// Formats a group delimited by an opening and closing token, placing the
    /// content in a [block_indent] group
    pub(crate) fn format_delimited_block_indent(
        &self,
        open_token: &SyntaxToken,
        content: FormatElement,
        close_token: &SyntaxToken,
    ) -> FormatResult<FormatElement> {
        self.format_delimited(
            open_token,
            move |trailing_trivia, leading_trivia| {
                block_indent(format_elements![trailing_trivia, content, leading_trivia])
            },
            close_token,
        )
    }

    /// Formats a group delimited by an opening and closing token, placing the
    /// content in a [soft_block_indent] group
    pub(crate) fn format_delimited_soft_block_indent(
        &self,
        open_token: &SyntaxToken,
        content: FormatElement,
        close_token: &SyntaxToken,
    ) -> FormatResult<FormatElement> {
        self.format_delimited(
            open_token,
            move |trailing_trivia, leading_trivia| {
                soft_block_indent(format_elements![trailing_trivia, content, leading_trivia])
            },
            close_token,
        )
    }

    /// Formats a group delimited by an opening and closing token, placing the
    /// content in an [indent] group with [soft_line_break_or_space] tokens at the
    /// start and end
    pub(crate) fn format_delimited_soft_block_spaces(
        &self,
        open_token: &SyntaxToken,
        content: FormatElement,
        close_token: &SyntaxToken,
    ) -> FormatResult<FormatElement> {
        self.format_delimited(
            open_token,
            move |trailing_trivia, leading_trivia| {
                if trailing_trivia.is_empty() && content.is_empty() && leading_trivia.is_empty() {
                    empty_element()
                } else {
                    format_elements![
                        indent(format_elements![
                            soft_line_break_or_space(),
                            trailing_trivia,
                            content,
                            leading_trivia,
                        ]),
                        soft_line_break_or_space(),
                    ]
                }
            },
            close_token,
        )
    }

    /// Print out a `token` from the original source with a different `content`.
    ///
    /// This will print the trivias that belong to `token` to `content`;
    /// `token` is then marked as consumed by the formatter.
    pub fn format_replaced(
        &self,
        current_token: &SyntaxToken,
        content_to_replace_with: FormatElement,
    ) -> FormatElement {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                assert!(self.printed_tokens.borrow_mut().insert(current_token.clone()));
            }
        }

        format_elements![
            self.print_leading_trivia(current_token, TriviaPrintMode::Full),
            content_to_replace_with,
            self.print_trailing_trivia(current_token),
        ]
    }

    /// Formats each child and returns the result as a list.
    ///
    /// Returns [None] if a child couldn't be formatted.
    pub fn format_nodes<T: AstNode + ToFormatElement>(
        &self,
        nodes: impl IntoIterator<Item = T>,
    ) -> FormatResult<impl Iterator<Item = FormatElement>> {
        let mut result = Vec::new();

        for node in nodes {
            match node.format(self) {
                Ok(formatted) => {
                    result.push(formatted);
                }
                Err(err) => return Err(err),
            }
        }

        Ok(result.into_iter())
    }

    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    pub fn format_separated<T, L, F>(
        &self,
        list: &L,
        separator_factory: F,
        trailing_separator: TrailingSeparator,
    ) -> FormatResult<impl Iterator<Item = FormatElement>>
    where
        T: AstNode + ToFormatElement + Clone,
        L: AstSeparatedList<T>,
        F: Fn() -> FormatElement,
    {
        let mut result = Vec::with_capacity(list.len());
        let last_index = list.len().saturating_sub(1);

        for (index, element) in list.elements().enumerate() {
            let node = element.node()?.format(self)?;

            // Reuse the existing trailing separator or create it if it wasn't in the
            // input source. Only print the last trailing token if the outer group breaks
            let separator = if let Some(separator) = element.trailing_separator()? {
                if index == last_index {
                    if trailing_separator.is_allowed() {
                        // Use format_replaced instead of wrapping the result of format_token
                        // in order to remove only the token itself when the group doesn't break
                        // but still print its associated trivias unconditionally
                        self.format_replaced(&separator, if_group_breaks(Token::from(&separator)))
                    } else if trailing_separator.is_mandatory() {
                        separator.format(self)?
                    } else {
                        empty_element()
                    }
                } else {
                    FormatTokenAndNode::format(&separator, self)?
                }
            } else if index == last_index {
                if trailing_separator.is_allowed() {
                    if_group_breaks(separator_factory())
                } else if trailing_separator.is_mandatory() {
                    separator_factory()
                } else {
                    empty_element()
                }
            } else {
                separator_factory()
            };

            result.push(format_elements![node, separator]);
        }

        Ok(result.into_iter())
    }

    /// It formats a list of nodes that are not separated. It's an ad-hoc function to
    /// format lists that implement [rome_js_syntax::AstNodeList].
    ///
    /// The elements of the list are joined together using [join_elements_hard_line], which will
    /// end up separated by hard lines or empty lines.
    ///
    /// If the formatter fails to format an element, said element gets printed verbatim.
    pub fn format_list<List, Node: AstNode + ToFormatElement>(&self, list: List) -> FormatElement
    where
        List: AstNodeList<Node>,
    {
        let formatted_list = list.iter().map(|module_item| {
            let snapshot = self.snapshot();
            let elem = match module_item.format(self) {
                Ok(result) => result,
                Err(_) => {
                    self.restore(snapshot);
                    // Lists that yield errors are formatted as they were unknown nodes.
                    // Doing so, the formatter formats the nodes/tokens as is.
                    self.format_unknown(module_item.syntax())
                }
            };

            (module_item.syntax().clone(), elem)
        });
        join_elements_hard_line(formatted_list)
    }

    pub(super) fn print_leading_trivia(
        &self,
        token: &SyntaxToken,
        trim_mode: TriviaPrintMode,
    ) -> FormatElement {
        // Checks whether the previous token has any trailing newline
        let has_trailing_newline = token
            .prev_token()
            .and_then(|token| token.trailing_trivia().last())
            .map_or(false, |trivia| trivia.is_newline());

        self.print_leading_trivia_pieces(
            token.leading_trivia().pieces(),
            trim_mode,
            has_trailing_newline,
        )
        .unwrap_or_else(|_| {
            self.print_leading_trivia_with_skipped_tokens(token, trim_mode, has_trailing_newline)
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
    fn print_leading_trivia_with_skipped_tokens(
        &self,
        token: &SyntaxToken,
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
                        self.print_leading_trivia_pieces(
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

        let skipped_trivia_range = skipped_trivia_range.expect("Only call this method for leading trivia containing at least one skipped token trivia.");

        // Format the skipped token trivia range
        // Compute the offsets relative to the tokens text
        let relative_skipped_range = skipped_trivia_range - token.text_range().start();
        let text = &token.text()[relative_skipped_range];
        elements.push(FormatElement::from(Token::new_dynamic(
            text.to_string(),
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
        elements.push(self.print_trailing_trivia_pieces(trailing_trivia.into_iter()));

        elements.push(
            self.print_leading_trivia_pieces(leading_trivia.into_iter(), trim_mode, after_newline)
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
    ///
    fn print_leading_trivia_pieces<I>(
        &self,
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

                let comment = Token::from(comment);

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

    pub(super) fn print_trailing_trivia(&self, token: &SyntaxToken) -> FormatElement {
        self.print_trailing_trivia_pieces(token.trailing_trivia().pieces())
    }

    fn print_trailing_trivia_pieces<I>(&self, pieces: I) -> FormatElement
    where
        I: Iterator<Item = SyntaxTriviaPiece<JsLanguage>>,
    {
        let mut elements = Vec::new();

        for piece in pieces {
            if let Some(comment) = piece.as_comments() {
                let is_single_line = comment.text().trim_start().starts_with("//");

                let comment = Token::from(comment);

                let content = if !is_single_line {
                    format_elements![
                        if_group_breaks(line_suffix(format_elements![
                            space_token(),
                            comment.clone(),
                            space_token(),
                        ])),
                        if_group_fits_on_single_line(format_elements![
                            space_token(),
                            comment,
                            space_token(),
                        ]),
                    ]
                } else {
                    line_suffix(format_elements![space_token(), comment, space_token()])
                };

                elements.push(crate::comment(content));
            }
        }

        concat_elements(elements)
    }

    /// "Formats" a node according to its original formatting in the source text. Being able to format
    /// a node "as is" is useful if a node contains syntax errors. Formatting a node with syntax errors
    /// has the risk that Rome misinterprets the structure of the code and formatting it could
    /// "mess up" the developers, yet incomplete, work or accidentally introduce new syntax errors.
    ///
    /// You may be inclined to call `node.text` directly. However, using `text` doesn't track the nodes
    ///nor its children source mapping information, resulting in incorrect source maps for this subtree.
    ///
    /// These nodes and tokens get tracked as [FormatElement::Verbatim], useful to understand
    /// if these nodes still need to have their own implementation.
    pub fn format_verbatim(&self, node: &SyntaxNode) -> FormatElement {
        let verbatim = self.format_verbatim_node_or_token(node);
        FormatElement::Verbatim(Verbatim::new_verbatim(
            verbatim,
            node.to_string(),
            node.text_range(),
        ))
    }

    /// Formats unknown nodes. The difference between this method  and `format_verbatim` is that this method
    /// doesn't track nodes/tokens as [FormatElement::Verbatim]. They are just printed as they are.
    pub fn format_unknown(&self, node: &SyntaxNode) -> FormatElement {
        FormatElement::Verbatim(Verbatim::new_unknown(
            self.format_verbatim_node_or_token(node),
        ))
    }

    /// Format a node having formatter suppression comment applied to it
    pub fn format_suppressed(&self, node: &SyntaxNode) -> FormatElement {
        format_elements![
            // Insert a force a line break to ensure the suppression comment is on its own line
            // and correctly registers as a leading trivia on the opening token of this node
            hard_line_break(),
            FormatElement::Verbatim(Verbatim::new_suppressed(
                self.format_verbatim_node_or_token(node)
            )),
        ]
    }

    fn format_verbatim_node_or_token(&self, node: &SyntaxNode) -> FormatElement {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                for token in node.descendants_tokens() {
                    assert!(self.printed_tokens.borrow_mut().insert(token.clone()));
                }
            }
        }

        fn skip_whitespace<L: Language>(piece: &SyntaxTriviaPiece<L>) -> bool {
            piece.is_newline() || piece.is_whitespace()
        }

        fn trivia_token<L: Language>(piece: SyntaxTriviaPiece<L>) -> Token {
            Token::new_dynamic(
                normalize_newlines(piece.text(), LINE_TERMINATORS).into_owned(),
                piece.text_range(),
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
            node.text_trimmed_range(),
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
}

/// Determines if the whitespace separating comment trivias
/// from their associated tokens should be printed or trimmed
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(super) enum TriviaPrintMode {
    Full,
    Trim,
}

/// Snapshot of the formatter state  used to handle backtracking if
/// errors are encountered in the formatting process and the formatter
/// has to fallback to printing raw tokens
///
/// In practice this only saves the set of printed tokens in debug
/// mode and compiled to nothing in release mode
pub struct FormatterSnapshot {
    #[cfg(debug_assertions)]
    printed_tokens: HashSet<SyntaxToken>,
}

impl Formatter {
    /// Take a snapshot of the state of the formatter
    pub fn snapshot(&self) -> FormatterSnapshot {
        FormatterSnapshot {
            #[cfg(debug_assertions)]
            printed_tokens: self.printed_tokens.borrow().clone(),
        }
    }

    #[cfg(debug_assertions)]
    /// Restore the state of the formatter to a previous snapshot
    pub fn restore(&self, snapshot: FormatterSnapshot) {
        *self.printed_tokens.borrow_mut() = snapshot.printed_tokens;
    }

    #[cfg(not(debug_assertions))]
    /// Restore the state of the formatter to a previous snapshot
    pub fn restore(&self, _: FormatterSnapshot) {}
}
