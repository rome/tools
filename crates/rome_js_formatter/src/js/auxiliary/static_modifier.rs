use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsStaticModifier;
use rome_js_syntax::JsStaticModifierFields;

impl FormatNodeFields<JsStaticModifier> for FormatNodeRule<JsStaticModifier> {
    fn format_fields(
        node: &JsStaticModifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsStaticModifierFields { modifier_token } = node.as_fields();

        formatted![formatter, [modifier_token.format()]]
    }
}
