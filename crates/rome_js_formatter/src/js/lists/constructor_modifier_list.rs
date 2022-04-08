use crate::{join_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsConstructorModifierList;
use rome_rowan::AstNodeList;

impl ToFormatElement for JsConstructorModifierList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_nodes(self.iter())?,
        ))
    }
}
