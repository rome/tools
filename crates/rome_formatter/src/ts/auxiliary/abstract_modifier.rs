use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsAbstractModifier;
use rslint_parser::ast::TsAbstractModifierFields;

impl ToFormatElement for TsAbstractModifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAbstractModifierFields { modifier_token } = self.as_fields();

        modifier_token.format(formatter)
    }
}
