use crate::formatter::{FormatSeparatedOptions, TrailingSeparator};
use crate::generated::FormatJsObjectAssignmentPatternPropertyList;
use crate::prelude::*;
use rome_js_syntax::{JsAnyObjectAssignmentPatternMember, JsObjectAssignmentPatternPropertyList};

impl FormatRule<JsObjectAssignmentPatternPropertyList>
    for FormatJsObjectAssignmentPatternPropertyList
{
    type Context = JsFormatContext;

    fn format(
        node: &JsObjectAssignmentPatternPropertyList,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        // The trailing separator is disallowed after a rest element
        let has_trailing_rest = match node.into_iter().last() {
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
            formatter.format_separated_with_options(
                node,
                || token(","),
                FormatSeparatedOptions::default().with_trailing_separator(trailing_separator),
            )?,
        ))
    }
}
