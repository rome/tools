use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsOverrideModifier;
use rome_js_syntax::TsOverrideModifierFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsOverrideModifier;

impl FormatNodeRule<TsOverrideModifier> for FormatTsOverrideModifier {
    fn fmt_fields(node: &TsOverrideModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsOverrideModifierFields { modifier_token } = node.as_fields();
        write![f, [modifier_token.format()]]
    }
}
