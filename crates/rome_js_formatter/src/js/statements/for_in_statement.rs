use rome_formatter::FormatResult;
use rome_js_syntax::JsForInStatement;

use crate::utils::format_head_body_statement;
use crate::{
    format_elements, soft_line_break_or_space, space_token, Format, FormatElement, FormatNode,
    Formatter, JsFormatter,
};
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
            format_elements![
                for_token,
                space_token(),
                formatter.format_delimited_soft_block_indent(
                    &l_paren_token?,
                    format_elements![
                        initializer,
                        soft_line_break_or_space(),
                        in_token,
                        soft_line_break_or_space(),
                        expression,
                    ],
                    &r_paren_token?
                )?,
            ],
            body?,
        )
    }
}
