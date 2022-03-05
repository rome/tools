use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::JsStaticModifier;
use rslint_syntax::JsStaticModifierFields;

impl ToFormatElement for JsStaticModifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStaticModifierFields { modifier_token } = self.as_fields();
        modifier_token.format(formatter)
    }
}
