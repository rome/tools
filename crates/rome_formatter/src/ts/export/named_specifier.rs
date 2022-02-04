use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsExportNamedSpecifier;

impl ToFormatElement for JsExportNamedSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let type_token = self
            .type_token()
            .format_with_or_empty(formatter, |type_token| {
                format_elements![type_token, space_token()]
            })?;
        let as_token = self.as_token().format(formatter)?;
        let local_name = self.local_name().format(formatter)?;
        let exported_name = self.exported_name().format(formatter)?;

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
