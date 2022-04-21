use crate::format_traits::FormatOptional;

use crate::{
    format_elements, group_elements, soft_line_break_or_space, space_token, token, Format,
    FormatElement, FormatNode, FormatResult, Formatter,
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
            format_elements![
                initializer.format_or_empty(formatter)?,
                first_semi_token.format(formatter)?,
                soft_line_break_or_space(),
                test.format_or_empty(formatter)?,
                second_semi_token.format(formatter)?,
                soft_line_break_or_space(),
                update.format_or_empty(formatter)?,
            ]
        } else {
            format_elements![
                first_semi_token.format(formatter)?,
                second_semi_token.format(formatter)?,
            ]
        };

        // Force semicolon insertion for empty bodies
        let body = body?;
        let body = if matches!(body, JsAnyStatement::JsEmptyStatement(_)) {
            format_elements![body.format(formatter)?, token(";")]
        } else {
            format_elements![space_token(), body.format(formatter)?]
        };

        Ok(group_elements(format_elements![
            for_token.format(formatter)?,
            space_token(),
            formatter.format_delimited_soft_block_indent(
                &l_paren_token?,
                inner,
                &r_paren_token?,
            )?,
            body
        ]))
    }
}
