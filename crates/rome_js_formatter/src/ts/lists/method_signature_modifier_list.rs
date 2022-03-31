use crate::utils::sort_modifiers_by_precedence;
use crate::{join_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsMethodSignatureModifierList;

impl ToFormatElement for TsMethodSignatureModifierList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_nodes(sort_modifiers_by_precedence(self))?,
        ))
    }
}
