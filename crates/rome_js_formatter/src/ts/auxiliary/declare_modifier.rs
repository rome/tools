use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsDeclareModifier;
use rome_js_syntax::TsDeclareModifierFields;

impl FormatNodeFields<TsDeclareModifier> for FormatNodeRule<TsDeclareModifier> {
    fn format_fields(
        node: &TsDeclareModifier,
        formatter: &Formatter<JsFormatContext>,
    ) -> FormatResult<FormatElement> {
        let TsDeclareModifierFields { modifier_token } = node.as_fields();
        formatted![formatter, [modifier_token.format()]]
    }
}
