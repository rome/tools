use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::{TsConstModifier, TsConstModifierFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsConstModifier;

impl FormatNodeRule<TsConstModifier> for FormatTsConstModifier {
    fn fmt_fields(&self, node: &TsConstModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsConstModifierFields { modifier_token } = node.as_fields();
        write![f, [modifier_token.format()]]
    }
}
