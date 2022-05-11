use crate::prelude::*;
use crate::utils::sort_modifiers_by_precedence;
use rome_js_syntax::JsPropertyModifierList;

impl Format for JsPropertyModifierList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_all(sort_modifiers_by_precedence(self))?,
        ))
    }
}
