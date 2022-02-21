#[cfg(debug_assertions)]
use std::cell::RefCell;
#[cfg(debug_assertions)]
use std::collections::HashSet;

use crate::format_element::Verbatim;
use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    block_indent, concat_elements, empty_element, empty_line,
    format_element::{normalize_newlines, Token, LINE_TERMINATORS},
    format_elements, group_elements, hard_line_break, if_group_breaks,
    if_group_fits_on_single_line, indent, join_elements_hard_line, line_suffix, soft_block_indent,
    soft_line_break_or_space, space_token, FormatElement, FormatOptions, FormatResult,
    ToFormatElement,
};
use rome_rowan::SyntaxElement;
#[cfg(debug_assertions)]
use rslint_parser::SyntaxNodeExt;
use rslint_parser::{AstNode, AstNodeList, AstSeparatedList, SyntaxNode, SyntaxToken};

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
}

impl TrailingSeparator {
    pub fn is_allowed(&self) -> bool {
        matches!(self, TrailingSeparator::Allowed)
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
    pub(crate) fn format_root(self, root: &SyntaxNode) -> FormatResult<FormatElement> {
        let element = self.format_syntax_node(root)?;

        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                let printed_tokens = self.printed_tokens.into_inner();
                for token in root.tokens() {
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
        let start = self.format_node_start(node);
        let content = node.to_format_element(self)?;
        Ok(concat_elements(vec![
            start,
            content,
            self.format_node_end(node),
        ]))
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

    /// Helper function that returns what should be printed before the node that work on
    /// the non-generic [SyntaxNode] to avoid unrolling the logic for every [AstNode] type.
    pub(super) fn format_node_start(&self, _node: &SyntaxNode) -> FormatElement {
        // TODO: Set the marker for the start source map location, ...
        empty_element()
    }

    /// Helper function that returns what should be printed after the node that work on
    /// the non-generic [SyntaxNode] to avoid unrolling the logic for every [AstNode] type.
    pub(super) fn format_node_end(&self, _node: &SyntaxNode) -> FormatElement {
        // TODO: Sets the marker for the end source map location, ...
        empty_element()
    }

    /// Print out a `token` from the original source with a different `content`.
    ///
    /// This will print the trivias that belong to `token` to `content`;
    /// `token` is then marked as consumed by the formatter.
    pub fn format_replaced(
        &self,
        token: &SyntaxToken,
        content: FormatElement,
    ) -> FormatResult<FormatElement> {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                assert!(self.printed_tokens.borrow_mut().insert(token.clone()));
            }
        }

        Ok(format_elements![
            self.print_leading_trivia(token, TriviaPrintMode::Full),
            content,
            self.print_trailing_trivia(token),
        ])
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
                        self.format_replaced(&separator, if_group_breaks(Token::from(&separator)))?
                    } else {
                        empty_element()
                    }
                } else {
                    FormatTokenAndNode::format(&separator, self)?
                }
            } else if index == last_index {
                if trailing_separator.is_allowed() {
                    if_group_breaks(separator_factory())
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

    /// It formats a list of nodes that are not separated. It's a ad-hoc function to
    /// format lists that implement [rslint_parser::AstNodeList].
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
                    self.format_verbatim(module_item.syntax())
                        .trim_start()
                        .trim_end()
                }
            };

            (module_item, elem)
        });
        join_elements_hard_line(formatted_list)
    }

    pub(super) fn print_leading_trivia(
        &self,
        token: &SyntaxToken,
        mut trim_mode: TriviaPrintMode,
    ) -> FormatElement {
        let mut line_count = 0;
        let mut elements = Vec::new();

        for piece in token.leading_trivia().pieces().rev() {
            if let Some(comment) = piece.as_comments() {
                let is_single_line = comment.text().trim_start().starts_with("//");

                let comment = Token::from(comment);

                let line_break = if is_single_line {
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

                elements.push(format_elements![comment, line_break]);

                line_count = 0;
                trim_mode = TriviaPrintMode::Full;
            } else if piece.as_newline().is_some() && trim_mode == TriviaPrintMode::Full {
                line_count += 1;
            }
        }

        concat_elements(elements.into_iter().rev())
    }

    pub(super) fn print_trailing_trivia(&self, token: &SyntaxToken) -> FormatElement {
        let mut elements = Vec::new();

        for piece in token.trailing_trivia().pieces() {
            if let Some(comment) = piece.as_comments() {
                let is_single_line = comment.text().trim_start().starts_with("//");

                let comment = Token::from(comment);

                elements.push(if !is_single_line {
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
                });
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
        FormatElement::Verbatim(Verbatim::new(verbatim, node.to_string(), node.text_range()))
    }

    /// Formats unknown nodes. The different between `format_verbatim` is that unknown nodes (and its children/tokens)
    /// are not tracked by the printer.
    pub fn format_unknown(&self, node: &SyntaxNode) -> FormatElement {
        self.format_verbatim_node_or_token(node)
    }

    fn format_verbatim_node_or_token(&self, node: &SyntaxNode) -> FormatElement {
        concat_elements(node.children_with_tokens().map(|child| {
            match child {
                SyntaxElement::Node(child_node) => {
                    // TODO: Add source map markers before/after node as well as any additional elements that
                    // need to be tracked for every node.

                    // Here we call `format_unknown` because we don't want to track it as [FormatElement::Verbatim]
                    // all the possible children. The first node to call `format_verbatim` should be node to be tracked
                    self.format_unknown(&child_node)
                }
                SyntaxElement::Token(syntax_token) => {
                    cfg_if::cfg_if! {
                        if #[cfg(debug_assertions)] {
                            assert!(self.printed_tokens.borrow_mut().insert(syntax_token.clone()));
                        }
                    }

                    // Print the full (not trimmed) text of the token
                    FormatElement::from(Token::new_dynamic(
                        normalize_newlines(syntax_token.text(), LINE_TERMINATORS).into_owned(),
                        syntax_token.text_range(),
                    ))
                }
            }
        }))
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
