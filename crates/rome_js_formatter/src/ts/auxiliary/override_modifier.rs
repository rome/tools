use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::TsOverrideModifier;
use rome_js_syntax::TsOverrideModifierFields;

impl FormatNodeFields<TsOverrideModifier> for FormatNodeRule<TsOverrideModifier> {
    fn format_fields(node: &TsOverrideModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsOverrideModifierFields { modifier_token } = node.as_fields();
        write![f, [modifier_token.format()]]
    }
}
