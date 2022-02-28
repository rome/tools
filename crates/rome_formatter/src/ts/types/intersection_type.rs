use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, group_elements, if_group_breaks, indent, soft_line_break, space_token, token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsIntersectionType;
use rslint_parser::ast::TsIntersectionTypeFields;

impl ToFormatElement for TsIntersectionType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsIntersectionTypeFields {
            leading_separator_token,
            types,
        } = self.as_fields();

        Ok(group_elements(indent(format_elements![
            soft_line_break(),
            if_group_breaks(format_elements![
                leading_separator_token.format_or(formatter, || token("&"))?,
                space_token()
            ]),
            types.format(formatter)?,
        ])))
    }
}
