use crate::{join_elements, space_token, Format, FormatElement, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsConstructorModifierList;
use rome_rowan::AstNodeList;

impl Format for JsConstructorModifierList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_all(self.iter())?,
        ))
    }
}
