use crate::formatter::TrailingSeparator;
use crate::prelude::*;
use rome_js_syntax::{JsAnyObjectAssignmentPatternMember, JsObjectAssignmentPatternPropertyList};

impl Format for JsObjectAssignmentPatternPropertyList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        // The trailing separator is disallowed after a rest element
        let has_trailing_rest = match self.into_iter().last() {
            Some(elem) => matches!(
                elem?,
                JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(_)
            ),
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
