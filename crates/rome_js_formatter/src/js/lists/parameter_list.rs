use crate::formatter::TrailingSeparator;
use crate::generated::FormatJsParameterList;
use crate::prelude::*;
use rome_js_syntax::{JsAnyParameter, JsParameterList};

impl FormatRule<JsParameterList> for FormatJsParameterList {
    type Options = JsFormatOptions;

    fn format(
        node: &JsParameterList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        // The trailing separator is disallowed if the last element in the list is a rest parameter
        let has_trailing_rest = match node.into_iter().last() {
            Some(elem) => matches!(elem?, JsAnyParameter::JsRestParameter(_)),
            None => false,
        };

        let trailing_separator = if has_trailing_rest {
            TrailingSeparator::Disallowed
        } else {
            TrailingSeparator::Allowed
        };

        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","), trailing_separator)?,
        ))
    }
}
