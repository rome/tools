use crate::{empty_element, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::{AstNode, SyntaxResult, SyntaxToken};

/// Utility trait used to simplify the formatting of optional tokens
pub trait FormatOptionalToken {
    /// This function tries to format an optional [token](SyntaxToken). If the token doesn't exist,
    /// an [empty token](FormatElement::Empty) is created
    fn format_or_empty(&self, formatter: &Formatter) -> FormatResult<FormatElement>;

    /// This function tries to format an optional [token](SyntaxToken). If the token doesn't exist,
    /// an [empty token](FormatElement::Empty) is created. If exists, the utility
    /// formats the token and passes it to the closure.

    fn format_with_or_empty<With>(
        &self,
        formatter: &Formatter,
        with: With,
    ) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> FormatElement;

    /// This function tries to format an optional [token](SyntaxToken). If the token doesn't exist,
    /// it calls the passed closure, which has to return a [FormatElement]
    fn format_or<Op>(&self, formatter: &Formatter, op: Op) -> FormatResult<FormatElement>
    where
        Op: FnOnce() -> FormatElement;
}

impl FormatOptionalToken for Option<SyntaxToken> {
    fn format_or_empty(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            None => Ok(empty_element()),
            Some(token) => formatter.format_token(token),
        }
    }

    fn format_with_or_empty<With>(
        &self,
        formatter: &Formatter,
        with: With,
    ) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> FormatElement,
    {
        match self {
            None => Ok(empty_element()),
            Some(token) => Ok(with(formatter.format_token(token)?)),
        }
    }

    fn format_or<Op>(&self, formatter: &Formatter, op: Op) -> FormatResult<FormatElement>
    where
        Op: FnOnce() -> FormatElement,
    {
        match self {
            None => Ok(op()),
            Some(token) => formatter.format_token(token),
        }
    }
}

impl<Node: AstNode + ToFormatElement> FormatOptionalToken for Option<Node> {
    fn format_or_empty(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            None => Ok(empty_element()),
            Some(node) => { node.to_owned() formatter.format_node(node.clone()) },
        }
    }

    fn format_with_or_empty<With>(
        &self,
        formatter: &Formatter,
        with: With,
    ) -> FormatResult<FormatElement>
    where
        With: FnOnce(FormatElement) -> FormatElement,
    {
        todo!()
    }

    fn format_or<Op>(&self, formatter: &Formatter, op: Op) -> FormatResult<FormatElement>
    where
        Op: FnOnce() -> FormatElement,
    {
        todo!()
    }
}
