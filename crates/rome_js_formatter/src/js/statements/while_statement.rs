use crate::utils::format_head_body_statement;
use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, Formatter, JsFormatter,
};
use rome_formatter::FormatResult;

use rome_js_syntax::JsWhileStatement;
use rome_js_syntax::JsWhileStatementFields;

impl FormatNode for JsWhileStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsWhileStatementFields {
            while_token,
            l_paren_token,
            test,
            r_paren_token,
            body,
        } = self.as_fields();

        format_head_body_statement(
            formatter,
            format_elements![
                while_token.format(formatter)?,
                space_token(),
                formatter.format_delimited_soft_block_indent(
                    &l_paren_token?,
                    test.format(formatter)?,
                    &r_paren_token?,
                )?,
            ],
            body?,
        )
    }
}
