use crate::prelude::*;

use rome_js_syntax::JsReferenceIdentifier;
use rome_js_syntax::JsReferenceIdentifierFields;

impl FormatNode for JsReferenceIdentifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsReferenceIdentifierFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
