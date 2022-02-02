use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsNamespaceImportSpecifier;

impl ToFormatElement for JsNamespaceImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let star = self.star_token().format(formatter)?;
        let as_token = self.as_token().format(formatter)?;
        let local_name = self.local_name().format(formatter)?;

        Ok(format_elements![
            star,
            space_token(),
            as_token,
            space_token(),
            local_name
        ])
    }
}
