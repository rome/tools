use crate::generated::FormatTsIndexSignatureModifierList;
use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::TsIndexSignatureModifierList;

impl FormatRule<TsIndexSignatureModifierList> for FormatTsIndexSignatureModifierList {
    type Context = JsFormatContext;

    fn format(
        node: &TsIndexSignatureModifierList,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_all(sort_modifiers_by_precedence(node).into_iter().formatted())?,
        ))
    }
}
