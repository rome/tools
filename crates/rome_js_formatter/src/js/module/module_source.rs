use crate::prelude::*;
use crate::utils::FormatLiteralStringToken;
use crate::FormatNodeFields;
use rome_js_syntax::JsModuleSource;
use rome_js_syntax::JsModuleSourceFields;

impl FormatNodeFields<JsModuleSource> for FormatNodeRule<JsModuleSource> {
    fn format_fields(
        node: &JsModuleSource,
        formatter: &Formatter<JsFormatContext>,
    ) -> FormatResult<FormatElement> {
        let JsModuleSourceFields { value_token } = node.as_fields();

        FormatLiteralStringToken::from_string(&value_token?).format(formatter)
    }
}
