use crate::format_traits::FormatOptional;
use rome_formatter::{hard_line_break, FormatResult};

use crate::{
    format_elements, hard_group_elements, space_token, token, Format, FormatElement, FormatNode,
    Formatter,
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

        let head = do_token.format(formatter)?;

        let tail = format_elements![
            space_token(),
            while_token.format(formatter)?,
            space_token(),
            formatter.format_delimited_soft_block_indent(
                &l_paren_token?,
                test.format(formatter)?,
                &r_paren_token?,
            )?,
            semicolon_token.format_or(formatter, || token(";"))?
        ];

        let body = body?;
        if matches!(body, JsAnyStatement::JsBlockStatement(_)) {
            Ok(hard_group_elements(format_elements![
                head,
                space_token(),
                body.format(formatter)?,
                tail,
            ]))
        } else if matches!(body, JsAnyStatement::JsEmptyStatement(_)) {
            Ok(format_elements![
                hard_group_elements(format_elements![head, body.format(formatter)?,]),
                hard_line_break(),
                tail,
            ])
        } else {
            Ok(format_elements![
                hard_group_elements(format_elements![head, space_token()]),
                body.format(formatter)?,
                hard_group_elements(tail),
            ])
        }
    }
}
