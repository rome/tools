use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsAbstractModifier;
use rome_js_syntax::TsAbstractModifierFields;

impl FormatNodeFields<TsAbstractModifier> for FormatNodeRule<TsAbstractModifier> {
    fn format_fields(
        node: &TsAbstractModifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsAbstractModifierFields { modifier_token } = node.as_fields();

        formatted![formatter, [modifier_token.format()]]
    }
}
