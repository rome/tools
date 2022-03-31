use rome_js_syntax::JsForOfStatement;

use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::utils::format_head_body_statement;
use crate::{
    format_elements, soft_line_break_or_space, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rome_js_syntax::JsForOfStatementFields;

impl ToFormatElement for JsForOfStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
        let await_token = await_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let initializer = initializer.format(formatter)?;
        let of_token = of_token.format(formatter)?;
        let expression = expression.format(formatter)?;

        format_head_body_statement(
            formatter,
            format_elements![
                for_token,
                space_token(),
                await_token,
                formatter.format_delimited_soft_block_indent(
                    &l_paren_token?,
                    format_elements![
                        initializer,
                        soft_line_break_or_space(),
                        of_token,
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
