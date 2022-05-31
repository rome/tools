use crate::formatter::{FormatSeparatedExtension, FormatSeparatedOptions, TrailingSeparator};
use crate::generated::FormatJsObjectBindingPatternPropertyList;
use crate::prelude::*;
use rome_js_syntax::{JsAnyObjectBindingPatternMember, JsObjectBindingPatternPropertyList};

impl FormatRule<JsObjectBindingPatternPropertyList> for FormatJsObjectBindingPatternPropertyList {
    type Context = JsFormatContext;

    fn format(node: &JsObjectBindingPatternPropertyList, f: &mut JsFormatter) -> FormatResult<()> {
        // The trailing separator is disallowed after a rest element
        let has_trailing_rest = match node.into_iter().last() {
            Some(elem) => matches!(
                elem?,
                JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(_)
            ),
            None => false,
        };

        let trailing_separator = if has_trailing_rest {
            TrailingSeparator::Disallowed
        } else {
            TrailingSeparator::Allowed
        };

        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated_with_options(
                token(","),
                FormatSeparatedOptions::default().with_trailing_separator(trailing_separator),
            ))
            .finish()
    }
}
