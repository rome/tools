use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsReadonlyModifier;
use rome_js_syntax::TsReadonlyModifierFields;

impl FormatNodeFields<TsReadonlyModifier> for FormatNodeRule<TsReadonlyModifier> {
    fn format_fields(
        node: &TsReadonlyModifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsReadonlyModifierFields { modifier_token } = node.as_fields();
        formatted![formatter, [modifier_token.format()]]
    }
}
