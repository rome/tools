use rome_js_syntax::JsForOfStatement;

use crate::prelude::*;
use crate::utils::format_head_body_statement;

use crate::FormatNodeFields;
use rome_js_syntax::JsForOfStatementFields;

impl FormatNodeFields<JsForOfStatement> for FormatNodeRule<JsForOfStatement> {
    fn format_fields(
        node: &JsForOfStatement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsForOfStatementFields {
            for_token,
            await_token,
            l_paren_token,
            initializer,
            of_token,
            expression,
            r_paren_token,
            body,
        } = node.as_fields();

        Ok(group_elements(format_head_body_statement(
            formatter,
            formatted![
                formatter,
                [
                    for_token.format(),
                    space_token(),
                    await_token
                        .format()
                        .with_or_empty(|token| formatted![formatter, [token, space_token()]]),
                    l_paren_token.format(),
                    group_elements(formatted![formatter, [initializer.format()]]?),
                    space_token(),
                    of_token.format(),
                    space_token(),
                    expression.format(),
                    r_paren_token.format()
                ]
            ]?,
            body?,
        )?))
    }
}
