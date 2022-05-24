use crate::generated::FormatTsPropertySignatureModifierList;
use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::TsPropertySignatureModifierList;

impl FormatRule<TsPropertySignatureModifierList> for FormatTsPropertySignatureModifierList {
    type Options = JsFormatOptions;

    fn format(
        node: &TsPropertySignatureModifierList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_all(sort_modifiers_by_precedence(node).into_iter().formatted())?,
        ))
    }
}
