use crate::prelude::*;
use rome_js_syntax::{
    JsAnyObjectAssignmentPatternMember, JsObjectAssignmentPatternPropertyList, JsSyntaxKind,
};

#[derive(Debug, Clone, Default)]
pub struct FormatJsObjectAssignmentPatternPropertyList;

impl FormatRule<JsObjectAssignmentPatternPropertyList>
    for FormatJsObjectAssignmentPatternPropertyList
{
    type Context = JsFormatContext;

    fn fmt(
        &self,
        node: &JsObjectAssignmentPatternPropertyList,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
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

        let entries = node
            .format_separated(JsSyntaxKind::COMMA)
            .with_trailing_separator(trailing_separator)
            .zip(node.iter());

        let mut join = f.join_nodes_with_soft_line();

        for (format_entry, node) in entries {
            join.entry(node?.syntax(), &format_entry);
        }

        join.finish()
    }
}
