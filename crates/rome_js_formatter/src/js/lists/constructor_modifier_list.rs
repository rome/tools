use crate::prelude::*;
use rome_js_syntax::JsConstructorModifierList;
use rome_rowan::AstNodeList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsConstructorModifierList;

impl FormatRule<JsConstructorModifierList> for FormatJsConstructorModifierList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsConstructorModifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&space())
            .entries(node.iter().formatted())
            .finish()
    }
}
