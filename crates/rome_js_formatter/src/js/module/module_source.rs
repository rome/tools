use crate::prelude::*;
use crate::utils::FormatLiteralStringToken;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsModuleSource;
use rome_js_syntax::JsModuleSourceFields;

impl FormatNodeFields<JsModuleSource> for FormatNodeRule<JsModuleSource> {
    fn format_fields(node: &JsModuleSource, f: &mut JsFormatter) -> FormatResult<()> {
        let JsModuleSourceFields { value_token } = node.as_fields();

        write!(f, [FormatLiteralStringToken::from_string(&value_token?)])
    }
}
