use crate::{
    empty_element, format_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsExportNamedShorthandSpecifier;

impl ToFormatElement for JsExportNamedShorthandSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let type_token = if let Some(type_token) = self.type_token() {
            format_elements![formatter.format_token(&type_token)?, space_token()]
        } else {
            empty_element()
        };

        let name = formatter.format_node(self.name()?)?;

        Ok(format_elements![type_token, name])
    }
}
