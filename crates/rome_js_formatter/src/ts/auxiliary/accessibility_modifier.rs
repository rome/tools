use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsAccessibilityModifier;
use rome_js_syntax::TsAccessibilityModifierFields;

impl FormatNodeFields<TsAccessibilityModifier> for FormatNodeRule<TsAccessibilityModifier> {
    fn format_fields(
        node: &TsAccessibilityModifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsAccessibilityModifierFields { modifier_token } = node.as_fields();

        formatted![formatter, [modifier_token.format()]]
    }
}
