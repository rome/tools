use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsExtendsClause;
use rome_js_syntax::JsExtendsClauseFields;

impl FormatNodeFields<JsExtendsClause> for FormatNodeRule<JsExtendsClause> {
    fn fmt_fields(node: &JsExtendsClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExtendsClauseFields {
            extends_token,
            super_class,
            type_arguments,
        } = node.as_fields();

        write![
            f,
            [
                extends_token.format(),
                space_token(),
                super_class.format(),
                type_arguments.format(),
            ]
        ]
    }
}
