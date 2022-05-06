
use rome_formatter::FormatResult;

use crate::{
    formatted, group_elements, soft_line_break_or_space, space_token, token, Format, FormatElement,
    FormatNode, Formatter, JsFormatter,
};

use rome_js_syntax::JsAnyStatement;
use rome_js_syntax::JsForStatement;
use rome_js_syntax::JsForStatementFields;

impl FormatNode for JsForStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsForStatementFields {
            for_token,
            l_paren_token,
            initializer,
            first_semi_token,
            test,
            second_semi_token,
            update,
            r_paren_token,
            body,
        } = self.as_fields();

        let inner = if initializer.is_some() || test.is_some() || update.is_some() {
            formatted![
                formatter,
                initializer,
                first_semi_token.format(formatter)?,
                soft_line_break_or_space(),
                test,
                second_semi_token.format(formatter)?,
                soft_line_break_or_space(),
                update,
            ]?
        } else {
            formatted![
                formatter,
                first_semi_token.format(formatter)?,
                second_semi_token.format(formatter)?,
            ]?
        };

        // Force semicolon insertion for empty bodies
        let body = body?;
        let body = if matches!(body, JsAnyStatement::JsEmptyStatement(_)) {
            formatted![formatter, body.format(formatter)?, token(";")]?
        } else {
            formatted![formatter, space_token(), body.format(formatter)?]?
        };

        Ok(group_elements(formatted![
            formatter,
            for_token.format(formatter)?,
            space_token(),
            formatter.format_delimited_soft_block_indent(
                &l_paren_token?,
                inner,
                &r_paren_token?,
            )?,
            body
        ]?))
    }
}
