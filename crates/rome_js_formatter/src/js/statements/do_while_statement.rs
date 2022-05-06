use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{
    formatted, hard_group_elements, space_token, token, Format, FormatElement,
    FormatNode, Formatter, JsFormatter,
};

use rome_js_syntax::JsDoWhileStatementFields;
use rome_js_syntax::{JsAnyStatement, JsDoWhileStatement};

impl FormatNode for JsDoWhileStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsDoWhileStatementFields {
            do_token,
            body,
            while_token,
            l_paren_token,
            test,
            r_paren_token,
            semicolon_token,
        } = self.as_fields();

        let head = formatted![formatter, do_token.format(formatter)?, space_token(),]?;

        let tail = formatted![
            formatter,
            space_token(),
            while_token.format(formatter)?,
            space_token(),
            formatter.format_delimited_soft_block_indent(
                &l_paren_token?,
                test.format(formatter)?,
                &r_paren_token?,
            )?,
            semicolon_token.format_or(formatter, || token(";"))?
        ]?;

        let body = body?;
        if matches!(body, JsAnyStatement::JsBlockStatement(_)) {
            Ok(hard_group_elements(formatted![
                formatter,
                head,
                body.format(formatter)?,
                tail,
            ]?))
        } else {
            formatted![
                formatter,
                hard_group_elements(head),
                body.format(formatter)?,
                hard_group_elements(tail),
            ]
        }
    }
}
