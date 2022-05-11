use crate::Format;
use crate::{empty_element, FormatElement, Formatter};
use rome_formatter::FormatResult;

use rome_rowan::SyntaxResult;

/// Utility trait used to simplify the formatting of optional objects that are formattable.
///
/// In order to take advantage of all the functions, you only need to implement the [FormatOptionalTokenAndNode::with_or]
/// function.
pub trait FormatOptional {
    /// This function tries to format an optional object. If the object is [None]
    /// an [empty token](crate::FormatElement::Empty) is created. If exists, the utility
    /// formats the object and passes it to the closure.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::JsSyntaxTreeBuilder;
    /// use rome_js_formatter::{Formatter, empty_element, space_token, format_elements, token, formatted};
    /// use rome_js_syntax::{JsSyntaxToken};
    /// use rome_js_formatter::prelude::*;
    /// use rome_js_syntax::JsSyntaxKind;
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
    /// let empty_result = empty_token.with_or_empty(|token| token);
    /// let with_result = syntax_token.with_or_empty(|token| {
    ///     formatted![&formatter, space_token(), token]
    /// });
    ///
    /// assert_eq!(Ok(empty_element()), formatted![&formatter, empty_result]);
    /// assert_eq!(formatted![&formatter, space_token(), token("'abc'")], formatted![&formatter, with_result]);
    fn with_or_empty<With, WithResult>(
        &self,
        with: With,
    ) -> FormatWithOr<With, fn() -> FormatElement, WithResult, FormatElement>
    where
        With: Fn(FormatElement) -> WithResult,
        WithResult: IntoFormatResult,
    {
        self.with_or(with, empty_element)
    }

    /// This function tries to format an optional formattable object as is. If the object is [None],
    /// it calls the passed closure, which has to return a [crate::FormatElement]
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_formatter::{formatted, Formatter, token};
    /// use rome_js_syntax::{JsSyntaxToken};
    /// use rome_js_formatter::prelude::*;
    ///
    /// let formatter = Formatter::default();
    /// let empty_token: Option<JsSyntaxToken> = None;
    ///
    /// let result = empty_token.or_format(|| token(" other result"));
    ///
    /// assert_eq!(Ok(token(" other result")), formatted![&formatter, result]);
    fn or_format<Or, OrResult>(
        &self,
        op: Or,
    ) -> FormatWithOr<fn(FormatElement) -> FormatElement, Or, FormatElement, OrResult>
    where
        Or: Fn() -> OrResult,
        OrResult: IntoFormatResult,
        Self: Sized,
    {
        self.with_or(|token| token, op)
    }

    /// If the object isn't [None], it will call the first closure which will accept formatted element.
    ///
    /// If the object is [None], the second closure will be called.
    ///
    /// Both closures have to return a [crate::FormatElement]. This function will make sure to wrap them into [Ok].
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::JsSyntaxTreeBuilder;
    /// use rome_js_formatter::{Formatter, empty_element, space_token, format_elements, token, formatted};
    /// use rome_js_syntax::{JsSyntaxToken};
    /// use rome_js_formatter::prelude::*;
    /// use rome_js_syntax::JsSyntaxKind;
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
    /// let empty_result = empty_token.with_or(|token| token, || {
    ///     token("empty")
    /// });
    /// let with_result = syntax_token.with_or(|token| {
    ///     formatted![&formatter, space_token(), token]
    /// }, || empty_element());
    ///
    /// assert_eq!(Ok(token("empty")), formatted![&formatter, empty_result]);
    /// assert_eq!(formatted![&formatter, space_token(), token("'abc'")], formatted![&formatter, with_result]);
    fn with_or<With, Or, WithResult, OrResult>(
        &self,
        with: With,
        op: Or,
    ) -> FormatWithOr<With, Or, WithResult, OrResult>
    where
        With: Fn(FormatElement) -> WithResult,
        WithResult: IntoFormatResult,
        Or: Fn() -> OrResult,
        OrResult: IntoFormatResult;
}

/// Utility trait for formatting a formattable object with some additional content.
pub trait FormatWith {
    /// Allows to chain a formattable object with another [elements](FormatElement)
    ///
    /// The function will decorate the result with [Ok]
    ///
    /// The formatted element is passed to the closure, which then can appended to additional elements.
    /// This method is useful in case, for example, a token has to be chained with a space.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::JsSyntaxTreeBuilder;
    /// use rome_js_formatter::{Formatter, token, format_elements, space_token, formatted};
    /// use rome_js_syntax::{JsSyntaxNode, JsSyntaxKind};
    /// use rome_js_formatter::prelude::*;
    ///
    /// let mut builder = JsSyntaxTreeBuilder::new();
    /// builder.start_node(JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION);
    /// builder.token(JsSyntaxKind::JS_STRING_LITERAL, "'abc'");
    /// builder.finish_node();
    /// let node = builder.finish();
    /// let syntax_token = node.first_token().unwrap();
    /// let formatter = Formatter::default();
    ///
    /// // Wrap the token in [Ok] so we can simulate SyntaxResult.
    /// let result = Ok(syntax_token);
    /// let result = result.with(|string_literal| {
    ///     formatted![&formatter, string_literal, space_token(), token("+")]
    /// });
    ///
    /// assert_eq!(formatted![&formatter, token("'abc'"), space_token(), token("+")], formatted![&formatter, result])
    fn with<With, WithResult>(&self, with: With) -> FormatItemWith<With, WithResult>
    where
        With: Fn(FormatElement) -> WithResult,
        WithResult: IntoFormatResult;
}

pub struct FormatItemWith<'a, With, WithResult>
where
    With: Fn(FormatElement) -> WithResult,
    WithResult: IntoFormatResult,
{
    with: With,
    inner: &'a dyn Format,
}

impl<'a, With, WithResult> Format for FormatItemWith<'a, With, WithResult>
where
    With: Fn(FormatElement) -> WithResult,
    WithResult: IntoFormatResult,
{
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let element = self.inner.format(formatter)?;

        (self.with)(element).into_format_result()
    }
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
    fn with<With, WithResult>(&self, with: With) -> FormatItemWith<With, WithResult>
    where
        With: Fn(FormatElement) -> WithResult,
        WithResult: IntoFormatResult,
    {
        FormatItemWith { with, inner: self }
    }
}

impl<F: Format> FormatOptional for SyntaxResult<Option<F>> {
    fn with_or<With, Or, WithResult, OrResult>(
        &self,
        with: With,
        op: Or,
    ) -> FormatWithOr<With, Or, WithResult, OrResult>
    where
        With: Fn(FormatElement) -> WithResult,
        WithResult: IntoFormatResult,
        Or: Fn() -> OrResult,
        OrResult: IntoFormatResult,
    {
        match self {
            Err(_) => FormatWithOr::With { inner: self, with },
            Ok(Some(value)) => FormatWithOr::With { inner: value, with },
            Ok(None) => FormatWithOr::Or(op),
        }
    }
}

impl<F: Format> FormatOptional for Option<F> {
    fn with_or<With, Or, WithResult, OrResult>(
        &self,
        with: With,
        op: Or,
    ) -> FormatWithOr<With, Or, WithResult, OrResult>
    where
        With: Fn(FormatElement) -> WithResult,
        WithResult: IntoFormatResult,
        Or: Fn() -> OrResult,
        OrResult: IntoFormatResult,
    {
        match self {
            None => FormatWithOr::Or(op),
            Some(value) => FormatWithOr::With { inner: value, with },
        }
    }
}

pub enum FormatWithOr<'a, With, Or, WithResult, OrResult>
where
    With: Fn(FormatElement) -> WithResult,
    Or: Fn() -> OrResult,
    WithResult: IntoFormatResult,
    OrResult: IntoFormatResult,
{
    With { inner: &'a dyn Format, with: With },
    Or(Or),
}

impl<'a, With, Or, WithResult, OrResult> Format for FormatWithOr<'a, With, Or, WithResult, OrResult>
where
    With: Fn(FormatElement) -> WithResult,
    Or: Fn() -> OrResult,
    WithResult: IntoFormatResult,
    OrResult: IntoFormatResult,
{
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            FormatWithOr::Or(op) => op().into_format_result(),
            FormatWithOr::With { inner, with } => {
                with(inner.format(formatter)?).into_format_result()
            }
        }
    }
}
