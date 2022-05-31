use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsOverrideModifier;
use rome_js_syntax::TsOverrideModifierFields;

impl FormatNodeFields<TsOverrideModifier> for FormatNodeRule<TsOverrideModifier> {
    fn format_fields(
        node: &TsOverrideModifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsOverrideModifierFields { modifier_token } = node.as_fields();
        formatted![formatter, [modifier_token.format()]]
    }
}
