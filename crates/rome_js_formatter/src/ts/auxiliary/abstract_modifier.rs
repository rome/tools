use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsAbstractModifier;
use rome_js_syntax::TsAbstractModifierFields;

impl FormatNodeFields<TsAbstractModifier> for FormatNodeRule<TsAbstractModifier> {
    fn fmt_fields(node: &TsAbstractModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAbstractModifierFields { modifier_token } = node.as_fields();

        write![f, [modifier_token.format()]]
    }
}
