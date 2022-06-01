use rome_js_syntax::JsForInStatement;

use crate::prelude::*;
use crate::utils::format_head_body_statement;
use crate::FormatNodeFields;
use rome_js_syntax::JsForInStatementFields;

impl FormatNodeFields<JsForInStatement> for FormatNodeRule<JsForInStatement> {
    fn format_fields(
        node: &JsForInStatement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsForInStatementFields {
            for_token,
            l_paren_token,
            initializer,
            in_token,
            expression,
            r_paren_token,
            body,
        } = node.as_fields();

        let for_token = for_token.format();
        let initializer = initializer.format();
        let in_token = in_token.format();
        let expression = expression.format();

        Ok(group_elements(format_head_body_statement(
            formatter,
            formatted![
                formatter,
                [
                    for_token,
                    space_token(),
                    l_paren_token.format(),
                    group_elements(formatted![formatter, [initializer]]?),
                    space_token(),
                    in_token,
                    space_token(),
                    expression,
                    r_paren_token.format(),
                ]
            ]?,
            body?,
        )?))
    }
}
