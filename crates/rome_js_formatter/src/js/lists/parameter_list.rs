use crate::formatter::TrailingSeparator;
use crate::{join_elements, soft_line_break_or_space, token, Format, FormatElement, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::{JsAnyParameter, JsParameterList};

impl Format for JsParameterList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        // The trailing separator is disallowed if the last element in the list is a rest parameter
        let has_trailing_rest = match self.into_iter().last() {
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
            formatter.format_separated(self, || token(","), trailing_separator)?,
        ))
    }
}
