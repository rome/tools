use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsExportDefaultExpressionClause;
use rslint_parser::ast::JsExportDefaultExpressionClauseFields;

impl ToFormatElement for JsExportDefaultExpressionClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportDefaultExpressionClauseFields {
            default_token,
            expression,
            semicolon_token,
        } = self.as_fields();

        let default_token = default_token.format(formatter)?;
        let class = expression.format(formatter)?;
        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;
        Ok(format_elements![
            default_token,
            space_token(),
            class,
            semicolon
        ])
    }
}
