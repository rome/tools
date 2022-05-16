use crate::prelude::*;

use crate::utils::format_string_literal_token;
use crate::FormatNodeFields;
use rome_js_syntax::JsModuleSource;
use rome_js_syntax::JsModuleSourceFields;

impl FormatNodeFields<JsModuleSource> for FormatNodeRule<JsModuleSource> {
    fn format_fields(
        node: &JsModuleSource,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsModuleSourceFields { value_token } = node.as_fields();

        Ok(format_string_literal_token(value_token?, formatter))
    }
}
