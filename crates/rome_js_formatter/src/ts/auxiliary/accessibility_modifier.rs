use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsAccessibilityModifier;
use rome_js_syntax::TsAccessibilityModifierFields;

impl FormatNodeFields<TsAccessibilityModifier> for FormatNodeRule<TsAccessibilityModifier> {
    fn fmt_fields(node: &TsAccessibilityModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAccessibilityModifierFields { modifier_token } = node.as_fields();

        write![f, [modifier_token.format()]]
    }
}
