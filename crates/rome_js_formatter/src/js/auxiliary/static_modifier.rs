use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsStaticModifier;
use rome_js_syntax::JsStaticModifierFields;

impl FormatNodeFields<JsStaticModifier> for FormatNodeRule<JsStaticModifier> {
    fn fmt_fields(node: &JsStaticModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsStaticModifierFields { modifier_token } = node.as_fields();

        write![f, [modifier_token.format()]]
    }
}
