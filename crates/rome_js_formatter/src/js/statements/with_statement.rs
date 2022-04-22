use crate::utils::format_head_body_statement;
use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsWithStatement;
use rome_js_syntax::JsWithStatementFields;

impl FormatNode for JsWithStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsWithStatementFields {
            with_token,
            l_paren_token,
            object,
            r_paren_token,
            body,
        } = self.as_fields();

        format_head_body_statement(
            formatter,
            format_elements![
                with_token.format(formatter)?,
                space_token(),
                formatter.format_delimited_soft_block_indent(
                    &l_paren_token?,
                    object.format(formatter)?,
                    &r_paren_token?,
                )?,
            ],
            body?,
        )
    }
}
