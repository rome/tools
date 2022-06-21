use crate::prelude::*;
use crate::utils::format_class::FormatClass;
use crate::FormatNodeFields;
use rome_js_syntax::JsClassDeclaration;

impl FormatNodeFields<JsClassDeclaration> for FormatNodeRule<JsClassDeclaration> {
    fn fmt_fields(node: &JsClassDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        FormatClass::from(&node.clone().into()).fmt(f)
    }
}
