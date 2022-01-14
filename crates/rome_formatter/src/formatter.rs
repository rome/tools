use std::cell::RefCell;
use std::collections::HashSet;

use crate::printer::Printer;
use crate::{
    concat_elements, empty_element, format_elements, hard_line_break, if_group_breaks,
    if_group_fits_on_single_line, line_suffix, space_token, token, FormatElement, FormatOptions,
    FormatResult, Formatted, ToFormatElement,
};
use rome_rowan::api::SyntaxTriviaPieceComments;
use rome_rowan::{Language, SyntaxElement};
use rslint_parser::{AstNode, AstSeparatedList, SyntaxNode, SyntaxNodeExt, SyntaxToken};

/// Handles the formatting of a CST and stores the options how the CST should be formatted (user preferences).
/// The formatter is passed to the [ToFormatElement] implementation of every node in the CST so that they
/// can use it to format their children.
#[derive(Debug, Default)]
pub struct Formatter {
    options: FormatOptions,
    // This is using a RefCell as it only exists in debug mode,
    // the Formatter is still completely immutable in release builds
    #[cfg(debug_assertions)]
    printed_tokens: RefCell<HashSet<SyntaxToken>>,
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
    pub fn format_root(self, root: &SyntaxNode) -> FormatResult<Formatted> {
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

        let printer = Printer::new(self.options);
        Ok(printer.print(&element))
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
    pub(crate) fn format_delimited_group(
        &self,
        open_token: &SyntaxToken,
        content: impl FnOnce(FormatElement, FormatElement) -> FormatResult<FormatElement>,
        close_token: &SyntaxToken,
    ) -> FormatResult<FormatElement> {
        debug_assert!(self.printed_tokens.borrow_mut().insert(open_token.clone()));
        debug_assert!(self.printed_tokens.borrow_mut().insert(close_token.clone()));

        Ok(format_elements![
            self.print_leading_trivia(open_token),
            token(open_token.text_trimmed()),
            content(
                self.print_trailing_trivia(open_token),
                self.print_leading_trivia(close_token),
            )?,
            token(close_token.text_trimmed()),
            self.print_trailing_trivia(close_token),
        ])
    }

    /// Recursively formats the ast node and all its children
    ///
    /// Returns `None` if the node couldn't be formatted because of syntax errors in its sub tree.
    /// The parent may use `format_raw` to insert the node content as is.
    pub fn format_node<T: AstNode + ToFormatElement>(
        &self,
        node: T,
    ) -> FormatResult<FormatElement> {
        let leading = self.format_node_start(node.syntax());
        let trailing = self.format_node_end(node.syntax());
        Ok(format_elements![
            leading,
            node.to_format_element(self)?,
            trailing,
        ])
    }

    /// Helper function that returns what should be printed before the node that work on
    /// the non-generic [SyntaxNode] to avoid unrolling the logic for every [AstNode] type.
    fn format_node_start(&self, _node: &SyntaxNode) -> FormatElement {
        // TODO: Set the marker for the start source map location, ...
        empty_element()
    }

    /// Helper function that returns what should be printed after the node that work on
    /// the non-generic [SyntaxNode] to avoid unrolling the logic for every [AstNode] type.
    fn format_node_end(&self, _node: &SyntaxNode) -> FormatElement {
        // TODO: Sets the marker for the end source map location, ...
        empty_element()
    }

    /// Formats the passed in token.
    ///
    /// May return `None` if the token wasn't present in the original source but was inserted
    /// by the parser to "fix" a syntax error and generate a valid tree.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use rome_formatter::{Formatter, token};
    /// use rslint_parser::{SyntaxNode, T, SyntaxToken, JsLanguage, JsSyntaxKind, SyntaxTreeBuilder};
    /// use rome_rowan::{NodeOrToken};
    ///
    /// let mut builder = SyntaxTreeBuilder::new();
    /// builder.start_node(JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION);
    /// builder.token(JsSyntaxKind::JS_STRING_LITERAL, "'abc'");
    /// builder.finish_node();
    /// let node = builder.finish();
    ///
    /// let syntax_token = node.first_token().unwrap();
    ///
    /// let formatter = Formatter::default();
    /// let result = formatter.format_token(&syntax_token);
    ///
    /// assert_eq!(Ok(token("'abc'")), result)
    /// ```
    pub fn format_token(&self, syntax_token: &SyntaxToken) -> FormatResult<FormatElement> {
        debug_assert!(self
            .printed_tokens
            .borrow_mut()
            .insert(syntax_token.clone()));

        Ok(format_elements![
            self.print_leading_trivia(syntax_token),
            token(syntax_token.text_trimmed()),
            self.print_trailing_trivia(syntax_token),
        ])
    }

    /// Print out a token from the original source with a different content
    ///
    /// This will print the associated trivias from the token as well as mark
    /// it as having been consumed by the formatter
    pub fn format_replaced_token(
        &self,
        token: &SyntaxToken,
        content: FormatElement,
    ) -> FormatResult<FormatElement> {
        debug_assert!(self.printed_tokens.borrow_mut().insert(token.clone()));
        Ok(format_elements![
            self.print_leading_trivia(token),
            content,
            self.print_trailing_trivia(token),
        ])
    }

    /// Format an existing optional token, or calls the factory function to
    /// create a new one if didn't exist in the input
    pub fn format_or_create_token(
        &self,
        token: Option<SyntaxToken>,
        token_factory: impl FnOnce() -> FormatElement,
    ) -> FormatResult<FormatElement> {
        if let Some(token) = token {
            self.format_token(&token)
        } else {
            Ok(token_factory())
        }
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
            match self.format_node(node) {
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
        list: L,
        separator_factory: F,
    ) -> FormatResult<impl Iterator<Item = FormatElement>>
    where
        T: AstNode + ToFormatElement + Clone,
        L: AstSeparatedList<T>,
        F: Fn() -> FormatElement,
    {
        let mut result = Vec::with_capacity(list.len());
        let last_index = list.len().saturating_sub(1);

        for (index, element) in list.elements().enumerate() {
            let node = self.format_node(element.node()?)?;

            // Reuse the existing trailing separator or create it if it wasn't in the
            // input source. Only print the last trailing token if the outer group breaks
            let separator = if let Some(separator) = element.trailing_separator()? {
                if index == last_index {
                    // Use format_replaced_token instead of wrapping the result of format_token
                    // in order to remove only the token itself when the group doesn't break
                    // but still print its associated trivias unconditionally
                    self.format_replaced_token(
                        &separator,
                        if_group_breaks(token(separator.text_trimmed())),
                    )?
                } else {
                    self.format_token(&separator)?
                }
            } else {
                if index == last_index {
                    if_group_breaks(separator_factory())
                } else {
                    separator_factory()
                }
            };

            result.push(format_elements![node, separator]);
        }

        Ok(result.into_iter())
    }

    fn print_leading_trivia(&self, token: &SyntaxToken) -> FormatElement {
        // False positive: the trivias need to be collected in a vector as they
        // are iterated on in reverse order later, but SyntaxTriviaPiecesIterator
        // doesn't implement DoubleEndedIterator (rust-lang/rust-clippy#8132)
        #[allow(clippy::needless_collect)]
        let pieces: Vec<_> = token.leading_trivia().pieces().collect();

        let mut line_count = 0;
        let mut elements = Vec::new();

        for piece in pieces.into_iter().rev() {
            if let Some(comment) = piece.as_comments() {
                let is_single_line = comment.text().trim_start().starts_with("//");

                let comment = self.format_comment(comment);

                let line_break = if is_single_line {
                    hard_line_break()
                } else {
                    match line_count {
                        0 => space_token(),
                        1 => hard_line_break(),
                        _ => format_elements![hard_line_break(), hard_line_break()],
                    }
                };

                elements.push(format_elements![comment, line_break]);

                line_count = 0;
            }

            if piece.as_newline().is_some() {
                line_count += 1;
            }
        }

        concat_elements(elements.into_iter().rev())
    }

    fn print_trailing_trivia(&self, token: &SyntaxToken) -> FormatElement {
        let mut line_count = 0;
        let mut elements = Vec::new();

        for piece in token.trailing_trivia().pieces() {
            if let Some(comment) = piece.as_comments() {
                let is_single_line = comment.text().trim_start().starts_with("//");

                let comment = self.format_comment(comment);

                elements.push(if line_count >= 1 {
                    line_suffix(format_elements![
                        if line_count > 1 {
                            hard_line_break()
                        } else {
                            space_token()
                        },
                        hard_line_break(),
                        comment,
                        space_token(),
                    ])
                } else if !is_single_line {
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

                line_count = 0;
            }

            if piece.as_newline().is_some() {
                line_count += 1;
            }
        }

        concat_elements(elements)
    }

    fn format_comment<L: Language>(&self, trivia: SyntaxTriviaPieceComments<L>) -> FormatElement {
        token(trivia.text().trim())
    }

    /// "Formats" a node according to its original formatting in the source text. Being able to format
    /// a node "as is" is useful if a node contains syntax errors. Formatting a node with syntax errors
    /// has the risk that Rome misinterprets the structure of the code and formatting it could
    /// "mess up" the developers, yet incomplete, work or accidentally introduce new syntax errors.
    ///
    /// You may be inclined to call `node.text` directly. However, using `text` doesn't track the nodes
    ///nor its children source mapping information, resulting in incorrect source maps for this subtree.
    pub fn format_raw(&self, node: &SyntaxNode) -> FormatElement {
        concat_elements(node.children_with_tokens().map(|child| match child {
            SyntaxElement::Node(child_node) => {
                // TODO: Add source map markers before/after node as well as any additional elements that
                // need to be tracked for every node.
                self.format_raw(&child_node)
            }
            SyntaxElement::Token(syntax_token) => {
                debug_assert!(self
                    .printed_tokens
                    .borrow_mut()
                    .insert(syntax_token.clone()));

                token(syntax_token.text())
            }
        }))
    }
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

    /// Restore the state of the formatter to a previous snapshot
    pub fn restore(&self, snapshot: FormatterSnapshot) {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                *self.printed_tokens.borrow_mut() = snapshot.printed_tokens;
            }
        }
    }
}
