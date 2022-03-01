use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsAccessibilityModifier;
use rslint_parser::ast::TsAccessibilityModifierFields;

impl ToFormatElement for TsAccessibilityModifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAccessibilityModifierFields { modifier_token } = self.as_fields();
        modifier_token.format(formatter)
    }
}
