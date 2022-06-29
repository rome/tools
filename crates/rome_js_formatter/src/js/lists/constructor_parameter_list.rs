use crate::prelude::*;
use rome_js_syntax::{JsAnyConstructorParameter, JsConstructorParameterList, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub struct FormatJsConstructorParameterList;

impl FormatRule<JsConstructorParameterList> for FormatJsConstructorParameterList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsConstructorParameterList, f: &mut JsFormatter) -> FormatResult<()> {
        // The trailing separator is disallowed if the last element in the list is a rest parameter
        let has_trailing_rest = match node.into_iter().last() {
            Some(elem) => matches!(elem?, JsAnyConstructorParameter::JsRestParameter(_)),
            None => false,
        };

        let trailing_separator = if has_trailing_rest {
            TrailingSeparator::Disallowed
        } else {
            TrailingSeparator::Allowed
        };
        f.join_with(&soft_line_break_or_space())
            .entries(
                node.format_separated(JsSyntaxKind::COMMA)
                    .with_trailing_separator(trailing_separator),
            )
            .finish()
    }
}
