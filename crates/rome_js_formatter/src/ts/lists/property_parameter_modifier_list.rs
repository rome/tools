use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::TsPropertyParameterModifierList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsPropertyParameterModifierList;

impl FormatNodeRule<TsPropertyParameterModifierList> for FormatTsPropertyParameterModifierList {
    fn fmt_fields(
        &self,
        node: &TsPropertyParameterModifierList,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        f.join_with(&space_token())
            .entries(sort_modifiers_by_precedence(node).into_iter().formatted())
            .finish()
    }
}
