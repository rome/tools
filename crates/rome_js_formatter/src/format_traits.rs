use crate::Format;
use crate::{empty_element, FormatElement, Formatter};
use rome_formatter::FormatResult;

use rome_rowan::SyntaxResult;

/// Utility trait used to simplify the formatting of optional tokens
///
/// In order to take advantage of all the functions, you only need to implement the [FormatOptionalTokenAndNode::format_with_or]
/// function.
pub trait FormatOptional {
    /// This function tries to format an optional [token](rome_js_syntax::SyntaxToken) or [node](rome_js_syntax::AstNode).
    /// If the token doesn't exist, an [empty token](FormatElement::Empty) is created
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_formatter::{Formatter, empty_element};
    /// use rome_js_syntax::{JsSyntaxToken};
    /// use rome_js_formatter::prelude::*;
    ///
    /// let formatter = Formatter::default();
    /// let token: Option<JsSyntaxToken> = None;
    /// // we wrap the token in [Ok] so we can simulate SyntaxResult.
    /// let result = token.format_or_empty(&formatter);
    ///
    /// assert_eq!(Ok(empty_element()), result)
    fn format_or_empty(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.format_or(formatter, empty_element)
    }

    /// This function tries to format an optional [token](rome_js_syntax::SyntaxToken). If the token doesn't exist,
    /// an [empty token](crate::FormatElement::Empty) is created. If exists, the utility
    /// formats the token and passes it to the closure.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use rome_js_formatter::{Formatter, empty_element, space_token, format_elements, token};
    /// use rome_js_syntax::{JsSyntaxToken};
    /// use rome_js_formatter::prelude::*;
    /// use rome_js_syntax::{JsSyntaxTreeBuilder, JsSyntaxKind};
    ///
    /// let formatter = Formatter::default();
    /// let empty_token: Option<JsSyntaxToken> = None;
    ///
    /// let mut builder = JsSyntaxTreeBuilder::new();
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
    /// ```
    fn format_with_or_empty<With, WithResult>(
        &self,
        formatter: &Formatter,
        with: With,
    ) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> WithResult,
        WithResult: IntoFormatResult,
    {
        self.format_with_or(formatter, with, empty_element)
    }

    /// This function tries to format an optional [token](rome_js_syntax::SyntaxToken) as is. If the token doesn't exist,
    /// it calls the passed closure, which has to return a [crate::FormatElement]
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_formatter::{Formatter, token};
    /// use rome_js_syntax::{JsSyntaxToken};
    /// use rome_js_formatter::prelude::*;
    ///
    /// let formatter = Formatter::default();
    /// let empty_token: Option<JsSyntaxToken> = None;
    ///
    /// let result = empty_token.format_or(&formatter, || token(" other result"));
    ///
    /// assert_eq!(Ok(token(" other result")), result);
    fn format_or<Or, OrResult>(&self, formatter: &Formatter, op: Or) -> FormatResult<FormatElement>
    where
        Or: FnOnce() -> OrResult,
        OrResult: IntoFormatResult,
    {
        self.format_with_or(formatter, |token| token, op)
            .into_format_result()
    }

    /// If the token/node exists, it will call the first closure which will accept formatted element.
    ///
    /// If the token/node don't exist, the second closure will be called.
    ///
    /// Both closures have to return a [crate::FormatElement]. This function will make sure the wrap them into [Ok].
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_formatter::{Formatter, empty_element, space_token, format_elements, token};
    /// use rome_js_syntax::{JsSyntaxToken};
    /// use rome_js_formatter::prelude::*;
    /// use rome_js_syntax::{JsSyntaxTreeBuilder, JsSyntaxKind};
    ///
    /// let formatter = Formatter::default();
    /// let empty_token: Option<JsSyntaxToken> = None;
    ///
    /// let mut builder = JsSyntaxTreeBuilder::new();
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
    fn format_with_or<With, Or, WithResult, OrResult>(
        &self,
        formatter: &Formatter,
        with: With,
        op: Or,
    ) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> WithResult,
        WithResult: IntoFormatResult,
        Or: FnOnce() -> OrResult,
        OrResult: IntoFormatResult;
}

/// Utility trait to help to format nodes and tokens
pub trait FormatWith {
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
    /// use rome_js_formatter::{Formatter, token, format_elements, space_token};
    /// use rome_js_syntax::{JsSyntaxNode, JsSyntaxTreeBuilder, JsSyntaxKind};
    /// use rome_js_formatter::prelude::*;
    ///
    /// let mut builder = JsSyntaxTreeBuilder::new();
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
    fn format_with<With, WithResult>(
        &self,
        formatter: &Formatter,
        with: With,
    ) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> WithResult,
        WithResult: IntoFormatResult;
}

/// Utility trait to convert [crate::FormatElement] to [FormatResult]
pub trait IntoFormatResult {
    /// Consumes a [crate::FormatElement] to return a [FormatResult::FormatElement]
    ///
    /// This function in important when working with closures and the rest of the traits
    /// that belong to this module.
    fn into_format_result(self) -> FormatResult<FormatElement>;
}

impl IntoFormatResult for FormatElement {
    fn into_format_result(self) -> FormatResult<FormatElement> {
        Ok(self)
    }
}

impl IntoFormatResult for FormatResult<FormatElement> {
    fn into_format_result(self) -> FormatResult<FormatElement> {
        self
    }
}

impl<F: Format> FormatWith for F {
    fn format_with<With, WithResult>(
        &self,
        formatter: &Formatter,
        with: With,
    ) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> WithResult,
        WithResult: IntoFormatResult,
    {
        let element = self.format(formatter)?;

        with(element).into_format_result()
    }
}

impl<F: FormatOptional> FormatOptional for SyntaxResult<F> {
    fn format_with_or<With, Or, WithResult, OrResult>(
        &self,
        formatter: &Formatter,
        with: With,
        op: Or,
    ) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> WithResult,
        WithResult: IntoFormatResult,
        Or: FnOnce() -> OrResult,
        OrResult: IntoFormatResult,
    {
        match self {
            Ok(token) => token.format_with_or(formatter, with, op),
            Err(err) => Err(err.into()),
        }
    }
}

impl<F: FormatWith> FormatOptional for Option<F> {
    fn format_with_or<With, Or, WithResult, OrResult>(
        &self,
        formatter: &Formatter,
        with: With,
        op: Or,
    ) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> WithResult,
        WithResult: IntoFormatResult,
        Or: FnOnce() -> OrResult,
        OrResult: IntoFormatResult,
    {
        match self {
            None => op().into_format_result(),
            Some(token) => token.format_with(formatter, with).into_format_result(),
        }
    }
}
