use crate::{Format, FormatElement, Formatter};
use rome_formatter::FormatResult;
use rome_formatter::{group_elements, join_elements, soft_block_indent, soft_line_break_or_space};
use rome_js_syntax::JsxAttributeList;
impl Format for JsxAttributeList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let attributes = join_elements(soft_line_break_or_space(), formatter.format_all(self)?);

        Ok(group_elements(soft_block_indent(attributes)))
    }
}
