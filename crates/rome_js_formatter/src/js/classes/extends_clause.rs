use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsExtendsClause;
use rome_js_syntax::JsExtendsClauseFields;

impl FormatNodeFields<JsExtendsClause> for FormatNodeRule<JsExtendsClause> {
    fn format_fields(
        node: &JsExtendsClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsExtendsClauseFields {
            extends_token,
            super_class,
            type_arguments,
        } = node.as_fields();

        Ok(formatted![
            formatter,
            [
                extends_token.format(),
                space_token(),
                super_class.format(),
                type_arguments.format(),
            ]
        ]?)
    }
}
