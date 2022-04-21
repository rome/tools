use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsReadonlyModifier;
use rome_js_syntax::TsReadonlyModifierFields;

impl FormatNode for TsReadonlyModifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsReadonlyModifierFields { modifier_token } = self.as_fields();
        modifier_token.format(formatter)
    }
}
