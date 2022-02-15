use rslint_parser::ast::JsForInStatement;

use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, soft_line_break_or_space, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

impl ToFormatElement for JsForInStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let for_token = self.for_token().format(formatter)?;
        let initializer = self.initializer().format(formatter)?;
        let in_token = self.in_token().format(formatter)?;
        let expression = self.expression().format(formatter)?;
        let body = self.body().format(formatter)?;

        Ok(format_elements![
            for_token,
            space_token(),
            formatter.format_delimited_soft_block_indent(
                &self.l_paren_token()?,
                format_elements![
                    initializer,
                    soft_line_break_or_space(),
                    in_token,
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
