use crate::generated::FormatTsIndexSignatureModifierList;
use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::TsIndexSignatureModifierList;

impl FormatRule<TsIndexSignatureModifierList> for FormatTsIndexSignatureModifierList {
    fn format(
        node: &TsIndexSignatureModifierList,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_all(sort_modifiers_by_precedence(node).into_iter().formatted())?,
        ))
    }
}
