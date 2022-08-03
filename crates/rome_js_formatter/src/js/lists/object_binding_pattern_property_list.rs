use crate::prelude::*;
use rome_js_syntax::{
    JsAnyObjectBindingPatternMember, JsObjectBindingPatternPropertyList, JsSyntaxKind,
};

#[derive(Debug, Clone, Default)]
pub struct FormatJsObjectBindingPatternPropertyList;

impl FormatRule<JsObjectBindingPatternPropertyList> for FormatJsObjectBindingPatternPropertyList {
    type Context = JsFormatContext;

    fn fmt(
        &self,
        node: &JsObjectBindingPatternPropertyList,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
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
