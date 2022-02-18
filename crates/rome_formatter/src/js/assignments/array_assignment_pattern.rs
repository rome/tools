use crate::{
    formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsArrayAssignmentPattern;
use rslint_parser::ast::JsArrayAssignmentPatternFields;

impl ToFormatElement for JsArrayAssignmentPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsArrayAssignmentPatternFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = self.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_brack_token?,
            elements.format(formatter)?,
            &r_brack_token?,
        )
    }
}
