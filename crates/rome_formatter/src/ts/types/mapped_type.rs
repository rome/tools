use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, hard_line_break, indent, space_token, token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};
use rslint_parser::{ast::TsMappedType, AstNode};

impl ToFormatElement for TsMappedType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let l_curly = self.l_curly_token().format(formatter)?;
        let readonly = self
            .readonly_modifier()
            .format_with_or_empty(formatter, |readonly| {
                format_elements![readonly, space_token()]
            })?;
        let l_square = self.l_brack_token().format(formatter)?;
        let property_name = self.property_name().format(formatter)?;
        let in_token = self.in_token().format(formatter)?;
        let keys = self.keys_type().format(formatter)?;
        let as_clause = self.as_clause().format_or_empty(formatter)?;
        let r_square = self.r_brack_token().format(formatter)?;
        let optional_modifier = self.optional_modifier().format_or_empty(formatter)?;
        let mapped_type = self.mapped_type().format_or_empty(formatter)?;
        let r_curly = self.r_curly_token().format(formatter)?;
        let semicolon = self.semicolon_token().format_or(formatter, || token(";"))?;

        Ok(format_elements![
            l_curly,
            indent(format_elements![
                hard_line_break(),
                readonly,
                l_square,
                property_name,
                space_token(),
                in_token,
            ]),
            r_curly
        ])
    }
}
