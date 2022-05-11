use crate::generated::FormatJsConstructorModifierList;
use crate::prelude::*;
use rome_js_syntax::JsConstructorModifierList;
use rome_rowan::AstNodeList;

impl FormatRule<JsConstructorModifierList> for FormatJsConstructorModifierList {
    fn format(
        node: &JsConstructorModifierList,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_all(node.iter().formatted())?,
        ))
    }
}
