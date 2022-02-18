use rslint_parser::ast::JsForInStatement;

use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, soft_line_break_or_space, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsForInStatementFields;

impl ToFormatElement for JsForInStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
        let body = body.format(formatter)?;

        Ok(format_elements![
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
            space_token(),
            body
        ])
    }
}
