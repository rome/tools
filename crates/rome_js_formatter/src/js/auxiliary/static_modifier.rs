use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsStaticModifier;
use rome_js_syntax::JsStaticModifierFields;

impl FormatNode for JsStaticModifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStaticModifierFields { modifier_token } = self.as_fields();
        modifier_token.format(formatter)
    }
}
