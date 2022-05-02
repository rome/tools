use crate::{
    soft_block_indent, Format, FormatElement, FormatNode, FormatResult, Formatter, JsFormatter,
};
use rome_formatter::format_elements;
use rome_js_syntax::{JsxElement, JsxElementFields};

impl FormatNode for JsxElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxElementFields {
            opening_element,
            children,
            closing_element,
        } = self.as_fields();

        Ok(format_elements![
            opening_element.format(formatter)?,
            soft_block_indent(formatter.format_list(children)),
            closing_element.format(formatter)?
        ])
    }
}
