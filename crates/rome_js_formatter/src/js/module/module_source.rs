use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use crate::utils::format_string_literal_token;
use rome_js_syntax::JsModuleSource;
use rome_js_syntax::JsModuleSourceFields;

impl ToFormatElement for JsModuleSource {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsModuleSourceFields { value_token } = self.as_fields();

        Ok(format_string_literal_token(value_token?, formatter))
    }
}
