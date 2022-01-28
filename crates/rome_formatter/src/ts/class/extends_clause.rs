use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsExtendsClause;

impl ToFormatElement for JsExtendsClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.extends_token()?)?,
            space_token(),
            formatter.format_node(&self.super_class()?)?
        ])
    }
}
