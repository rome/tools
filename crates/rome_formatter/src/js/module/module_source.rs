use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsModuleSource;
use rslint_syntax::JsModuleSourceFields;

impl ToFormatElement for JsModuleSource {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsModuleSourceFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
