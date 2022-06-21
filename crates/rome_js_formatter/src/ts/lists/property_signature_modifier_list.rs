use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::TsPropertySignatureModifierList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsPropertySignatureModifierList;

impl FormatNodeRule<TsPropertySignatureModifierList> for FormatTsPropertySignatureModifierList {
    fn fmt_fields(
        &self,
        node: &TsPropertySignatureModifierList,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        f.join_with(&space_token())
            .entries(sort_modifiers_by_precedence(node).into_iter().formatted())
            .finish()
    }
}
