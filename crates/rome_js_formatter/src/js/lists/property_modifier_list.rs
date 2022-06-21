use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::JsPropertyModifierList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsPropertyModifierList;

impl FormatRule<JsPropertyModifierList> for FormatJsPropertyModifierList {
    type Context = JsFormatContext;

    fn fmt(node: &JsPropertyModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space_token())
            .entries(sort_modifiers_by_precedence(node).into_iter().formatted())
            .finish()
    }
}
