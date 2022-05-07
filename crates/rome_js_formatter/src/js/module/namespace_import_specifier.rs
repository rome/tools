use crate::prelude::*;

use rome_js_syntax::JsNamespaceImportSpecifier;
use rome_js_syntax::JsNamespaceImportSpecifierFields;

impl FormatNode for JsNamespaceImportSpecifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNamespaceImportSpecifierFields {
            star_token,
            as_token,
            local_name,
        } = self.as_fields();

        let star = star_token.format(formatter)?;
        let as_token = as_token.format(formatter)?;
        let local_name = local_name.format(formatter)?;

        formatted![
            formatter,
            star,
            space_token(),
            as_token,
            space_token(),
            local_name
        ]
    }
}
