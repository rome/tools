use crate::Token;
use crate::{
    empty_element, format_elements, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::{AstNode, SyntaxResult, SyntaxToken};

/// Utility trait used to simplify the formatting of optional tokens
pub trait FormatOptionalTokenAndNode {
    /// This function tries to format an optional [token](rslint_parser::SyntaxToken) or [node](rslint_parser::AstNode).
    /// If the token doesn't exist, an [empty token](FormatElement::Empty) is created
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_formatter::{Formatter, empty_element};
    /// use rslint_parser::{SyntaxToken};
    /// use rome_formatter::formatter_traits::{FormatOptionalTokenAndNode};
    ///
    /// let formatter = Formatter::default();
    /// let token: Option<SyntaxToken> = None;
    /// // we wrap the token in [Ok] so we can simulate SyntaxResult.
    /// let result = token.format_or_empty(&formatter);
    ///
    /// assert_eq!(Ok(empty_element()), result)
    fn format_or_empty(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_or(formatter, empty_element)
    }

    /// This function tries to format an optional [token](rslint_parser::SyntaxToken). If the token doesn't exist,
    /// an [empty token](crate::FormatElement::Empty) is created. If exists, the utility
    /// formats the token and passes it to the closure.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_formatter::{Formatter, empty_element, space_token, format_elements, token};
    /// use rslint_parser::{SyntaxToken};
    /// use rome_formatter::formatter_traits::{FormatOptionalTokenAndNode};
    /// use rslint_parser::{SyntaxTreeBuilder, JsSyntaxKind};
    ///
    /// let formatter = Formatter::default();
    /// let empty_token: Option<SyntaxToken> = None;
    ///
    /// let mut builder = SyntaxTreeBuilder::new();
    ///
    /// builder.start_node(JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION);
    /// builder.token(JsSyntaxKind::JS_STRING_LITERAL, "'abc'");
    /// builder.finish_node();
    /// let node = builder.finish();
    /// let syntax_token = node.first_token();
    ///
    /// // we wrap the token in [Ok] so we can simulate SyntaxResult.
    /// let empty_result = empty_token.format_with_or_empty(&formatter, |token| token);
    /// let with_result = syntax_token.format_with_or_empty(&formatter, |token| {
    ///     format_elements![space_token(), token]
    /// });
    ///
    /// assert_eq!(Ok(empty_element()), empty_result);
    /// assert_eq!(Ok(format_elements![space_token(), token("'abc'")]), with_result);
    fn format_with_or_empty<With>(
        &self,
        formatter: &Formatter,
        with: With,
    ) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> FormatElement,
    {
        self.format_with_or(formatter, with, empty_element)
    }

    /// This function tries to format an optional [token](rslint_parser::SyntaxToken) as is. If the token doesn't exist,
    /// it calls the passed closure, which has to return a [create::FormatElement]
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_formatter::{Formatter, token};
    /// use rslint_parser::{SyntaxToken};
    /// use rome_formatter::formatter_traits::{FormatOptionalTokenAndNode};
    ///
    /// let formatter = Formatter::default();
    /// let empty_token: Option<SyntaxToken> = None;
    ///
    /// let result = empty_token.format_or(&formatter, || token(" other result"));
    ///
    /// assert_eq!(Ok(token(" other result")), result);
    fn format_or<Or>(&self, formatter: &Formatter, op: Or) -> FormatResult<FormatElement>
    where
        Or: FnOnce() -> FormatElement,
    {
        self.format_with_or(formatter, |token| token, op)
    }

    /// If the token/node exists, it will call the first closure which will accept formatted element.
    ///
    /// If the token/node don't exist, the second closure will be called.
    ///
    /// Both closures have to return a [create::FormatElement]. This function will make sure the wrap them into [Ok].
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_formatter::{Formatter, empty_element, space_token, format_elements, token};
    /// use rslint_parser::{SyntaxToken};
    /// use rome_formatter::formatter_traits::{FormatOptionalTokenAndNode};
    /// use rslint_parser::{SyntaxTreeBuilder, JsSyntaxKind};
    ///
    /// let formatter = Formatter::default();
    /// let empty_token: Option<SyntaxToken> = None;
    ///
    /// let mut builder = SyntaxTreeBuilder::new();
    ///
    /// builder.start_node(JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION);
    /// builder.token(JsSyntaxKind::JS_STRING_LITERAL, "'abc'");
    /// builder.finish_node();
    /// let node = builder.finish();
    /// let syntax_token = node.first_token();
    ///
    /// // we wrap the token in [Ok] so we can simulate SyntaxResult.
    /// let empty_result = empty_token.format_with_or(&formatter, |token| token, || {
    ///     token("empty")
    /// });
    /// let with_result = syntax_token.format_with_or(&formatter, |token| {
    ///     format_elements![space_token(), token]
    /// }, || empty_element());
    ///
    /// assert_eq!(Ok(token("empty")), empty_result);
    /// assert_eq!(Ok(format_elements![space_token(), token("'abc'")]), with_result);
    fn format_with_or<With, Or>(
        &self,
        formatter: &Formatter,
        with: With,
        op: Or,
    ) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> FormatElement,
        Or: FnOnce() -> FormatElement;
}

/// Utility trait to help to format nodes and tokens
pub trait FormatTokenAndNode {
    /// Simply format a token or node by calling [create::Formatter::format_node] or [create::Formatter::format_token]
    /// respectively.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_formatter::{Formatter, token, space_token};
    /// use rome_formatter::formatter_traits::FormatTokenAndNode;
    /// use rslint_parser::{SyntaxTreeBuilder, JsSyntaxKind};
    ///
    /// let mut builder = SyntaxTreeBuilder::new();
    ///
    /// builder.start_node(JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION);
    /// builder.token(JsSyntaxKind::JS_STRING_LITERAL, "'abc'");
    /// builder.finish_node();
    /// let node = builder.finish();
    /// let syntax_token = node.first_token().unwrap();
    ///
    /// let formatter = Formatter::default();
    /// // we wrap the token in [Ok] so we can simulate SyntaxResult.
    /// let result = Ok(syntax_token).format(&formatter);
    ///
    /// assert_eq!(Ok(token("'abc'")), result)
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_with(formatter, |token| token)
    }

    /// Allows to chain a formatted token/node with another [elements](FormatElement)
    ///
    /// The function will decorate the result with [Ok]
    ///
    /// The formatted element is passed to the closure, which then can appended to additional elements.
    /// This method is useful in case, for example, a token has to be chained with a space.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_formatter::{Formatter, token, format_elements, space_token};
    /// use rslint_parser::{SyntaxNode, SyntaxTreeBuilder, JsSyntaxKind};
    /// use rome_formatter::formatter_traits::FormatTokenAndNode;
    ///
    /// let mut builder = SyntaxTreeBuilder::new();
    /// builder.start_node(JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION);
    /// builder.token(JsSyntaxKind::JS_STRING_LITERAL, "'abc'");
    /// builder.finish_node();
    /// let node = builder.finish();
    /// let syntax_token = node.first_token().unwrap();
    ///
    /// let formatter = Formatter::default();
    /// // we wrap the token in [Ok] so we can simulate SyntaxResult.
    /// let result = Ok(syntax_token).format_with(&formatter, |token| {
    ///     format_elements![token.clone(), space_token(), token.clone()]
    /// });
    ///
    /// assert_eq!(Ok(format_elements![token("'abc'"), space_token(), token("'abc'")]), result)
    fn format_with<With>(&self, formatter: &Formatter, with: With) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> FormatElement;
}

impl<F: FormatTokenAndNode> FormatTokenAndNode for SyntaxResult<F> {
    fn format_with<With>(&self, formatter: &Formatter, with: With) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> FormatElement,
    {
        match self {
            Ok(token) => Ok(with(token.format(formatter)?)),
            Err(err) => Err(err.into()),
        }
    }
}

impl FormatTokenAndNode for SyntaxToken {
    fn format_with<With>(&self, formatter: &Formatter, with: With) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> FormatElement,
    {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                assert!(formatter.printed_tokens.borrow_mut().insert(self.clone()));
            }
        }

        Ok(with(format_elements![
            formatter.print_leading_trivia(self),
            Token::from(self),
            formatter.print_trailing_trivia(self),
        ]))
    }
}

impl<N: AstNode + ToFormatElement> FormatTokenAndNode for N {
    fn format_with<With>(&self, formatter: &Formatter, with: With) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> FormatElement,
    {
        let leading = formatter.format_node_start(self.syntax());
        let trailing = formatter.format_node_end(self.syntax());
        Ok(with(format_elements![
            leading,
            self.to_format_element(formatter)?,
            trailing,
        ]))
    }
}

impl<F: FormatOptionalTokenAndNode> FormatOptionalTokenAndNode for SyntaxResult<F> {
    fn format_with_or<With, Or>(
        &self,
        formatter: &Formatter,
        with: With,
        op: Or,
    ) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> FormatElement,
        Or: FnOnce() -> FormatElement,
    {
        match self {
            Ok(token) => token.format_with_or(formatter, with, op),
            Err(err) => Err(err.into()),
        }
    }
}

impl<F: FormatTokenAndNode> FormatOptionalTokenAndNode for Option<F> {
    fn format_with_or<With, Or>(
        &self,
        formatter: &Formatter,
        with: With,
        op: Or,
    ) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> FormatElement,
        Or: FnOnce() -> FormatElement,
    {
        match self {
            None => Ok(op()),
            Some(token) => token.format_with(formatter, with),
        }
    }
}
