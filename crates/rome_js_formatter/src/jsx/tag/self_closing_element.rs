use crate::formatter_traits::FormatTokenAndNode;
use crate::group_elements;
use crate::{
    format_elements, soft_block_indent, soft_line_break_or_space, space_token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};
use rome_formatter::empty_element;
use rome_js_syntax::JsxSelfClosingElement;

impl ToFormatElement for JsxSelfClosingElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let mut attributes = Vec::new();
        for attribute in self.attributes() {
            attributes.push(attribute.format(formatter)?);
            attributes.push(soft_line_break_or_space());
        }

        let type_arguments = self
            .type_arguments()
            .map(|arg| arg.format(formatter))
            .transpose()?
            .unwrap_or_else(empty_element);

        Ok(format_elements![
            self.l_angle_token().format(formatter)?,
            self.name().format(formatter)?,
            type_arguments,
            space_token(),
            group_elements(soft_block_indent(concat_elements(attributes))),
            self.slash_token().format(formatter)?,
            self.r_angle_token().format(formatter)?
        ])
    }
}
