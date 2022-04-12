use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::group_elements;
use crate::{
    format_elements, soft_block_indent, soft_line_break_or_space, space_token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};
use rome_formatter::join_elements;
use rome_js_syntax::JsxSelfClosingElement;

impl ToFormatElement for JsxSelfClosingElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let attributes = join_elements(
            soft_line_break_or_space(),
            formatter.format_nodes(self.attributes())?,
        );

        let type_arguments = self.type_arguments().format_or_empty(formatter)?;

        Ok(format_elements![
            self.l_angle_token().format(formatter)?,
            self.name().format(formatter)?,
            type_arguments,
            space_token(),
            group_elements(soft_block_indent(attributes)),
            space_token(),
            self.slash_token().format(formatter)?,
            self.r_angle_token().format(formatter)?
        ])
    }
}
