use crate::generated::FormatJsConstructorModifierList;
use crate::prelude::*;
use rome_js_syntax::JsConstructorModifierList;
use rome_rowan::AstNodeList;

impl FormatRule<JsConstructorModifierList> for FormatJsConstructorModifierList {
    type Context = JsFormatContext;

    fn fmt(node: &JsConstructorModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space_token())
            .entries(node.iter().formatted())
            .finish()
    }
}
