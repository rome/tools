use crate::generated::FormatJsMethodModifierList;
use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::JsMethodModifierList;

impl FormatRule<JsMethodModifierList> for FormatJsMethodModifierList {
    type Context = JsFormatContext;

    fn fmt(node: &JsMethodModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space_token())
            .entries(sort_modifiers_by_precedence(node).into_iter().formatted())
            .finish()
    }
}
