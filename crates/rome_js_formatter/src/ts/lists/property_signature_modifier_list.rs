use crate::generated::FormatTsPropertySignatureModifierList;
use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::TsPropertySignatureModifierList;

impl FormatRule<TsPropertySignatureModifierList> for FormatTsPropertySignatureModifierList {
    type Context = JsFormatContext;

    fn fmt(node: &TsPropertySignatureModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space_token())
            .entries(sort_modifiers_by_precedence(node).into_iter().formatted())
            .finish()
    }
}
