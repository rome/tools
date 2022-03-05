use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::TsOverrideModifier;
use rslint_syntax::TsOverrideModifierFields;

impl ToFormatElement for TsOverrideModifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsOverrideModifierFields { modifier_token } = self.as_fields();
        modifier_token.format(formatter)
    }
}
