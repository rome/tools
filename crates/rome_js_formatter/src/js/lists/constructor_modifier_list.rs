use crate::prelude::*;
use rome_js_syntax::JsConstructorModifierList;
use rome_rowan::AstNodeList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsConstructorModifierList;

impl FormatRule<JsConstructorModifierList> for FormatJsConstructorModifierList {
    type Context = JsFormatContext;

    fn fmt(node: &JsConstructorModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space_token())
            .entries(node.iter().formatted())
            .finish()
    }
}
