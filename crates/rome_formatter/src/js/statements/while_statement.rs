use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsWhileStatement;
use rslint_parser::ast::JsWhileStatementFields;

impl ToFormatElement for JsWhileStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsWhileStatementFields {
            while_token,
            l_paren_token,
            test,
            r_paren_token,
            body,
        } = self.as_fields();

        Ok(format_elements![
            while_token.format(formatter)?,
            space_token(),
            formatter.format_delimited_soft_block_indent(
                &l_paren_token?,
                test.format(formatter)?,
                &r_paren_token?,
            )?,
            space_token(),
            body.format(formatter)?
        ])
    }
}
