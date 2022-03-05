use crate::utils::sort_modifiers_by_precedence;
use crate::{join_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::TsPropertySignatureModifierList;

impl ToFormatElement for TsPropertySignatureModifierList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_nodes(sort_modifiers_by_precedence(self))?,
        ))
    }
}
