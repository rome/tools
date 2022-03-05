use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::TsAbstractModifier;
use rslint_syntax::TsAbstractModifierFields;

impl ToFormatElement for TsAbstractModifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAbstractModifierFields { modifier_token } = self.as_fields();

        modifier_token.format(formatter)
    }
}
