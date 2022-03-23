use crate::formatter_traits::FormatTokenAndNode;

use crate::utils::format_with_semicolon;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsExportDefaultExpressionClause;
use rome_js_syntax::JsExportDefaultExpressionClauseFields;

impl ToFormatElement for JsExportDefaultExpressionClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportDefaultExpressionClauseFields {
            default_token,
            expression,
            semicolon_token,
        } = self.as_fields();

        let default_token = default_token.format(formatter)?;
        let class = expression.format(formatter)?;

        format_with_semicolon(
            formatter,
            format_elements![default_token, space_token(), class],
            semicolon_token,
        )
    }
}
