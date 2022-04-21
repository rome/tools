use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsAccessibilityModifier;
use rome_js_syntax::TsAccessibilityModifierFields;

impl FormatNode for TsAccessibilityModifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAccessibilityModifierFields { modifier_token } = self.as_fields();
        modifier_token.format(formatter)
    }
}
