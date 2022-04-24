use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

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

        // EXAMPLE: Remove feature flag example before merging
        if members.is_empty() || rome_features::flags().new_spacing {
            formatter.format_delimited_soft_block_indent(&l_curly_token?, members, &r_curly_token?)
        } else {
            formatter.format_delimited_soft_block_spaces(&l_curly_token?, members, &r_curly_token?)
        }
    }
}
