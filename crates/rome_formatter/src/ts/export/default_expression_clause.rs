use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsExportDefaultExpressionClause;

impl ToFormatElement for JsExportDefaultExpressionClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let default_token = self.default_token().format(formatter)?;
        let class = self.expression().format(formatter)?;
        let semicolon = self.semicolon_token().format_or(formatter, || token(";"))?;
        Ok(format_elements![
            default_token,
            space_token(),
            class,
            semicolon
        ])
    }
}
