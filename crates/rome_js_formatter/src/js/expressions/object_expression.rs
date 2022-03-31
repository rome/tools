use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsObjectExpression;
use rome_js_syntax::JsObjectExpressionFields;

impl ToFormatElement for JsObjectExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectExpressionFields {
            l_curly_token,
            members,
            r_curly_token,
        } = self.as_fields();

        let members = members.format(formatter)?;

        if members.is_empty() {
            formatter.format_delimited_soft_block_indent(&l_curly_token?, members, &r_curly_token?)
        } else {
            formatter.format_delimited_soft_block_spaces(&l_curly_token?, members, &r_curly_token?)
        }
    }
}
