use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsNamespaceImportSpecifier;
use rome_js_syntax::JsNamespaceImportSpecifierFields;

impl ToFormatElement for JsNamespaceImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNamespaceImportSpecifierFields {
            star_token,
            as_token,
            local_name,
        } = self.as_fields();

        let star = star_token.format(formatter)?;
        let as_token = as_token.format(formatter)?;
        let local_name = local_name.format(formatter)?;

        Ok(format_elements![
            star,
            space_token(),
            as_token,
            space_token(),
            local_name
        ])
    }
}
