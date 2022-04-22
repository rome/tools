use crate::format_traits::FormatOptional;
use crate::group_elements;
use crate::{
    format_elements, soft_block_indent, soft_line_break_or_space, space_token, Format,
    FormatElement, FormatNode, Formatter,
};
use rome_formatter::{join_elements, FormatResult};
use rome_js_syntax::JsxSelfClosingElement;

impl FormatNode for JsxSelfClosingElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let attributes = join_elements(
            soft_line_break_or_space(),
            formatter.format_all(self.attributes())?,
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
