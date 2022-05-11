use crate::generated::FormatTsMethodSignatureModifierList;
use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::TsMethodSignatureModifierList;

impl FormatRule<TsMethodSignatureModifierList> for FormatTsMethodSignatureModifierList {
    fn format(
        node: &TsMethodSignatureModifierList,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_all(sort_modifiers_by_precedence(node).into_iter().formatted())?,
        ))
    }
}
