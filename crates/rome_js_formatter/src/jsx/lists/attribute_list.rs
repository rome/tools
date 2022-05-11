use crate::generated::FormatJsxAttributeList;
use crate::prelude::*;
use rome_js_syntax::JsxAttributeList;

impl FormatRule<JsxAttributeList> for FormatJsxAttributeList {
    fn format(node: &JsxAttributeList, formatter: &Formatter) -> FormatResult<FormatElement> {
        let attributes = join_elements(
            soft_line_break_or_space(),
            formatter.format_all(node.iter().formatted())?,
        );

        Ok(group_elements(soft_block_indent(attributes)))
    }
}
