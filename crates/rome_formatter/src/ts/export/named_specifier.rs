use crate::{
    empty_element, format_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsExportNamedSpecifier;

impl ToFormatElement for JsExportNamedSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let type_token = if let Some(type_token) = self.type_token() {
            format_elements![formatter.format_token(&type_token)?, space_token()]
        } else {
            empty_element()
        };

        let as_token = formatter.format_token(&self.as_token()?)?;
        let local_name = formatter.format_node(&self.local_name()?)?;
        let exported_name = formatter.format_node(&self.exported_name()?)?;

        Ok(format_elements![
            type_token,
            local_name,
            space_token(),
            as_token,
            space_token(),
            exported_name
        ])
    }
}
