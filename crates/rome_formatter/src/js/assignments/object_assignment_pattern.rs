use crate::{
    formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsObjectAssignmentPattern;
use rslint_parser::ast::JsObjectAssignmentPatternFields;

impl ToFormatElement for JsObjectAssignmentPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectAssignmentPatternFields {
            l_curly_token,
            properties,
            r_curly_token,
        } = self.as_fields();

        formatter.format_delimited_soft_block_spaces(
            &l_curly_token?,
            properties.format(formatter)?,
            &r_curly_token?,
        )
    }
}
