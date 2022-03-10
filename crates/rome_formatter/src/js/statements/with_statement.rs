use crate::formatter_traits::FormatTokenAndNode;

use crate::utils::format_head_body_statement;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsWithStatement;
use rome_js_syntax::JsWithStatementFields;

impl ToFormatElement for JsWithStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
