use rome_js_syntax::JsForOfStatement;

use crate::prelude::*;
use crate::utils::format_head_body_statement;

use rome_js_syntax::JsForOfStatementFields;

impl FormatNode for JsForOfStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsForOfStatementFields {
            for_token,
            await_token,
            l_paren_token,
            initializer,
            of_token,
            expression,
            r_paren_token,
            body,
        } = self.as_fields();

        let for_token = for_token.format(formatter)?;
        let await_token =
            await_token.with_or_empty(|token| formatted![formatter, token, space_token()]);
        let initializer = initializer.format(formatter)?;
        let of_token = of_token.format(formatter)?;
        let expression = expression.format(formatter)?;

        format_head_body_statement(
            formatter,
            formatted![
                formatter,
                for_token,
                space_token(),
                await_token,
                formatter.format_delimited_soft_block_indent(
                    &l_paren_token?,
                    formatted![
                        formatter,
                        initializer,
                        soft_line_break_or_space(),
                        of_token,
                        soft_line_break_or_space(),
                        expression,
                    ]?,
                    &r_paren_token?
                )?,
            ]?,
            body?,
        )
    }
}
