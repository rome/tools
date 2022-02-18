use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsDoWhileStatement;
use rslint_parser::ast::JsDoWhileStatementFields;

impl ToFormatElement for JsDoWhileStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsDoWhileStatementFields {
            do_token,
            body,
            while_token,
            l_paren_token,
            test,
            r_paren_token,
            semicolon_token,
        } = self.as_fields();

        Ok(format_elements![
            do_token.format(formatter)?,
            space_token(),
            body.format(formatter)?,
            space_token(),
            while_token.format(formatter)?,
            space_token(),
            formatter.format_delimited_soft_block_indent(
                &l_paren_token?,
                test.format(formatter)?,
                &r_paren_token?,
            )?,
            semicolon_token.format_or(formatter, || token(";"))?
        ])
    }
}
