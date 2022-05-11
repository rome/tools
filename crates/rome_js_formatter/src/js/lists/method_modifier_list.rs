use crate::generated::FormatJsMethodModifierList;
use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::JsMethodModifierList;

impl FormatRule<JsMethodModifierList> for FormatJsMethodModifierList {
    fn format(node: &JsMethodModifierList, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_all(sort_modifiers_by_precedence(node).into_iter().formatted())?,
        ))
    }
}
