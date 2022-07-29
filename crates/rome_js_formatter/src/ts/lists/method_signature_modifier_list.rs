use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::TsMethodSignatureModifierList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsMethodSignatureModifierList;

impl FormatRule<TsMethodSignatureModifierList> for FormatTsMethodSignatureModifierList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsMethodSignatureModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space())
            .entries(sort_modifiers_by_precedence(node).into_iter().formatted())
            .finish()
    }
}
