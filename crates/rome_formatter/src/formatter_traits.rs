use crate::{empty_element, FormatElement, Formatter};
use rslint_parser::{SyntaxResult, SyntaxToken};

/// Utility trait used to simplify the formatting of optional tokens
pub trait FormatOptionalToken {
    /// This function tries to format an optional [token](SyntaxToken). If the token doesn't exist,
    /// an [empty token](FormatElement::Empty) is created
    ///
    /// ## Panics
    ///
    /// It panics if the formatting fails.
    fn format_or_empty(&self, formatter: &Formatter) -> FormatElement;

    /// This function tries to format an optional [token](SyntaxToken). If the token doesn't exist,
    /// it calls the passed closure, which has to return a [FormatElement]
    ///
    /// ## Panics
    ///
    /// It panics if the formatting fails.
    fn format_or<Op>(&self, formatter: &Formatter, op: Op) -> FormatElement
    where
        Op: FnOnce() -> FormatElement;
}

/// Utility trait used to format simple tokens
pub trait FormatToken {
    /// This function tries to format a token
    ///
    /// ## Panics
    ///
    /// It panics if the formatting fails.
    fn format(&self, formatter: &Formatter) -> FormatElement;
}

impl FormatOptionalToken for Option<SyntaxToken> {
    fn format_or_empty(&self, formatter: &Formatter) -> FormatElement {
        match self {
            None => empty_element(),
            Some(token) => formatter
                .format_token(token)
                .expect("Can't format the token"),
        }
    }

    fn format_or<Op>(&self, formatter: &Formatter, op: Op) -> FormatElement
    where
        Op: FnOnce() -> FormatElement,
    {
        match self {
            None => op(),
            Some(token) => formatter
                .format_token(token)
                .expect("Can't format the token"),
        }
    }
}

impl FormatToken for SyntaxResult<SyntaxToken> {
    fn format(&self, formatter: &Formatter) -> FormatElement {
        let token = self.as_ref().expect("Can't format the token");
        formatter
            .format_token(token)
            .expect("Can't format the token")
    }
}
