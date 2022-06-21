use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::JsMethodModifierList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsMethodModifierList;

impl FormatNodeRule<JsMethodModifierList> for FormatJsMethodModifierList {
    fn fmt_fields(&self, node: &JsMethodModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space_token())
            .entries(sort_modifiers_by_precedence(node).into_iter().formatted())
            .finish()
    }
}
