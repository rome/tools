use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::{JsAccessorModifier, JsAccessorModifierFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAccessorModifier;

impl FormatNodeRule<JsAccessorModifier> for FormatJsAccessorModifier {
    fn fmt_fields(&self, node: &JsAccessorModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let JsAccessorModifierFields { modifier_token } = node.as_fields();

        write![f, [modifier_token.format()]]
    }
}
