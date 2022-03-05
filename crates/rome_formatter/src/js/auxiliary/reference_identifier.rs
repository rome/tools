use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsReferenceIdentifier;
use rslint_syntax::JsReferenceIdentifierFields;

impl ToFormatElement for JsReferenceIdentifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsReferenceIdentifierFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
