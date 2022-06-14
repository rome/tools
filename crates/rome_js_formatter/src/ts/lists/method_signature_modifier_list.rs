use crate::generated::FormatTsMethodSignatureModifierList;
use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::TsMethodSignatureModifierList;

impl FormatRule<TsMethodSignatureModifierList> for FormatTsMethodSignatureModifierList {
    type Context = JsFormatContext;

    fn fmt(node: &TsMethodSignatureModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space_token())
            .entries(sort_modifiers_by_precedence(node).into_iter().formatted())
            .finish()
    }
}
