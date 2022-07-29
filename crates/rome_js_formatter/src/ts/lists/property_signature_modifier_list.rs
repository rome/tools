use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::TsPropertySignatureModifierList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsPropertySignatureModifierList;

impl FormatRule<TsPropertySignatureModifierList> for FormatTsPropertySignatureModifierList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsPropertySignatureModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space())
            .entries(sort_modifiers_by_precedence(node).into_iter().formatted())
            .finish()
    }
}
