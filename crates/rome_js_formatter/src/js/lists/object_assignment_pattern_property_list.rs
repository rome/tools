use crate::generated::FormatJsObjectAssignmentPatternPropertyList;
use crate::prelude::*;
use rome_js_syntax::{JsAnyObjectAssignmentPatternMember, JsObjectAssignmentPatternPropertyList};

impl FormatRule<JsObjectAssignmentPatternPropertyList>
    for FormatJsObjectAssignmentPatternPropertyList
{
    type Context = JsFormatContext;

    fn fmt(node: &JsObjectAssignmentPatternPropertyList, f: &mut JsFormatter) -> FormatResult<()> {
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

        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(token(",")).with_options(
                FormatSeparatedOptions::default().with_trailing_separator(trailing_separator),
            ))
            .finish()
    }
}
