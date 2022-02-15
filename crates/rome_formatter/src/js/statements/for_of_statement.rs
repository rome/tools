use rslint_parser::ast::JsForOfStatement;

use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, soft_line_break_or_space, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

impl ToFormatElement for JsForOfStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let for_token = self.for_token().format(formatter)?;
        let await_token = self
            .await_token()
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;
        let initializer = self.initializer().format(formatter)?;
        let of_token = self.of_token().format(formatter)?;
        let expression = self.expression().format(formatter)?;
        let body = self.body().format(formatter)?;

        Ok(format_elements![
            for_token,
            space_token(),
            await_token,
            formatter.format_delimited_soft_block_indent(
                &self.l_paren_token()?,
                format_elements![
                    initializer,
                    soft_line_break_or_space(),
                    of_token,
                    soft_line_break_or_space(),
                    expression,
                ],
                &self.r_paren_token()?
            )?,
            space_token(),
            body
        ])
    }
}
