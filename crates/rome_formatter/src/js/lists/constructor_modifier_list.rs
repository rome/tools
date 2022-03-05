use crate::{join_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::AstNodeList;
use rslint_syntax::JsConstructorModifierList;

impl ToFormatElement for JsConstructorModifierList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_nodes(self.iter())?,
        ))
    }
}
