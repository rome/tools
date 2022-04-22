use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_formatter::{
    block_indent, format_elements, group_elements, hard_line_break, indent, soft_block_indent,
    soft_line_break, soft_line_break_or_space,
};
use rome_js_syntax::{JsxFragment, JsxFragmentFields};
use rome_rowan::AstNode;

impl FormatNode for JsxFragment {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxFragmentFields {
            opening_fragment,
            children,
            closing_fragment,
        } = self.as_fields();

        let children = children.format(formatter)?;
        Ok(format_elements![
            opening_fragment.format(formatter)?,
            soft_block_indent(children),
            closing_fragment.format(formatter)?
        ])
    }
}
