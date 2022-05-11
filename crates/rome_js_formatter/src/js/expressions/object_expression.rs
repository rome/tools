use crate::prelude::*;

use rome_js_syntax::JsObjectExpression;
use rome_js_syntax::JsObjectExpressionFields;

impl FormatNode for JsObjectExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
