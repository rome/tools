use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::TsAccessibilityModifier;
use rslint_syntax::TsAccessibilityModifierFields;

impl ToFormatElement for TsAccessibilityModifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAccessibilityModifierFields { modifier_token } = self.as_fields();
        modifier_token.format(formatter)
    }
}
