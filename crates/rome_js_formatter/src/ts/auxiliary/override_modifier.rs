use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsOverrideModifier;
use rome_js_syntax::TsOverrideModifierFields;

impl FormatNode for TsOverrideModifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsOverrideModifierFields { modifier_token } = self.as_fields();
        modifier_token.format(formatter)
    }
}
