use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, hard_group_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::JsWhileStatementFields;
use rslint_parser::ast::{JsAnyStatement, JsWhileStatement};

impl ToFormatElement for JsWhileStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsWhileStatementFields {
            while_token,
            l_paren_token,
            test,
            r_paren_token,
            body,
        } = self.as_fields();

        let head = format_elements![
            while_token.format(formatter)?,
            space_token(),
            formatter.format_delimited_soft_block_indent(
                &l_paren_token?,
                test.format(formatter)?,
                &r_paren_token?,
            )?,
            space_token(),
        ];

        let body = body?;
        if matches!(body, JsAnyStatement::JsBlockStatement(_)) {
            Ok(hard_group_elements(format_elements![
                head,
                body.format(formatter)?
            ]))
        } else {
            Ok(format_elements![
                hard_group_elements(head),
                body.format(formatter)?
            ])
        }
    }
}
