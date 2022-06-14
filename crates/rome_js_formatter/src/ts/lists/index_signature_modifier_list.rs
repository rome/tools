use crate::generated::FormatTsIndexSignatureModifierList;
use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;

use rome_js_syntax::TsIndexSignatureModifierList;

impl FormatRule<TsIndexSignatureModifierList> for FormatTsIndexSignatureModifierList {
    type Context = JsFormatContext;

    fn fmt(node: &TsIndexSignatureModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space_token())
            .entries(sort_modifiers_by_precedence(node).into_iter().formatted())
            .finish()
    }
}
