use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::TsReadonlyModifier;
use rslint_syntax::TsReadonlyModifierFields;

impl ToFormatElement for TsReadonlyModifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsReadonlyModifierFields { modifier_token } = self.as_fields();
        modifier_token.format(formatter)
    }
}
