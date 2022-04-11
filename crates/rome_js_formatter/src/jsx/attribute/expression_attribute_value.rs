use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, hard_group_elements, soft_block_indent, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};
use rome_js_syntax::{JsxExpressionAttributeValue, JsxExpressionAttributeValueFields};

impl ToFormatElement for JsxExpressionAttributeValue {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxExpressionAttributeValueFields {
            l_curly_token,
            expression,
            r_curly_token,
        } = self.as_fields();

        Ok(hard_group_elements(format_elements![
            l_curly_token.format(formatter)?,
            soft_block_indent(expression.format(formatter)?),
            r_curly_token.format(formatter)?,
        ]))
    }
}
