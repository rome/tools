use rome_js_syntax::JsForInStatement;

use crate::prelude::*;
use crate::utils::format_head_body_statement;
use rome_js_syntax::JsForInStatementFields;

impl FormatNode for JsForInStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsForInStatementFields {
            for_token,
            l_paren_token,
            initializer,
            in_token,
            expression,
            r_paren_token,
            body,
        } = self.as_fields();

        let for_token = for_token.format(formatter)?;
        let initializer = initializer.format(formatter)?;
        let in_token = in_token.format(formatter)?;
        let expression = expression.format(formatter)?;

        format_head_body_statement(
            formatter,
            formatted![
                formatter,
                for_token,
                space_token(),
                formatter.format_delimited_soft_block_indent(
                    &l_paren_token?,
                    formatted![
                        formatter,
                        initializer,
                        soft_line_break_or_space(),
                        in_token,
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
