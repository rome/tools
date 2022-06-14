use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsReadonlyModifier;
use rome_js_syntax::TsReadonlyModifierFields;

impl FormatNodeFields<TsReadonlyModifier> for FormatNodeRule<TsReadonlyModifier> {
    fn fmt_fields(node: &TsReadonlyModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsReadonlyModifierFields { modifier_token } = node.as_fields();
        write![f, [modifier_token.format()]]
    }
}
