use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsAnyStatement;
use rome_js_syntax::JsForStatement;
use rome_js_syntax::JsForStatementFields;

impl FormatNodeFields<JsForStatement> for FormatNodeRule<JsForStatement> {
    fn format_fields(
        node: &JsForStatement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsForStatementFields {
            for_token,
            l_paren_token,
            initializer,
            first_semi_token,
            test,
            second_semi_token,
            update,
            r_paren_token,
            body,
        } = node.as_fields();

        let inner = if initializer.is_some() || test.is_some() || update.is_some() {
            formatted![
                formatter,
                [
                    initializer.format(),
                    first_semi_token.format(),
                    soft_line_break_or_space(),
                    test.format(),
                    second_semi_token.format(),
                    soft_line_break_or_space(),
                    update.format(),
                ]
            ]?
        } else {
            formatted![
                formatter,
                [first_semi_token.format(), second_semi_token.format(),]
            ]?
        };

        // Force semicolon insertion for empty bodies
        let body = body?;
        let body = if matches!(body, JsAnyStatement::JsEmptyStatement(_)) {
            formatted![formatter, [body.format(), token(";")]]?
        } else {
            formatted![formatter, [space_token(), body.format()]]?
        };

        Ok(group_elements(formatted![
            formatter,
            [
                for_token.format(),
                space_token(),
                formatter
                    .delimited(&l_paren_token?, inner, &r_paren_token?,)
                    .soft_block_indent()
                    .finish()?,
                body
            ]
        ]?))
    }
}
