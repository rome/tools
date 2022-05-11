use crate::generated::FormatJsPropertyModifierList;
use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::JsPropertyModifierList;

impl FormatRule<JsPropertyModifierList> for FormatJsPropertyModifierList {
    fn format(node: &JsPropertyModifierList, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_all(sort_modifiers_by_precedence(node).into_iter().formatted())?,
        ))
    }
}
