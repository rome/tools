use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::JsMethodModifierList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsMethodModifierList;

impl FormatRule<JsMethodModifierList> for FormatJsMethodModifierList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsMethodModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space())
            .entries(sort_modifiers_by_precedence(node).into_iter().formatted())
            .finish()
    }
}
