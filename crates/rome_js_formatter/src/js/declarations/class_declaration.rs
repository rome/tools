use crate::prelude::*;
use crate::utils::format_class::FormatClass;

use rome_js_syntax::JsClassDeclaration;

#[derive(Debug, Clone, Default)]
pub struct FormatJsClassDeclaration;

impl FormatNodeRule<JsClassDeclaration> for FormatJsClassDeclaration {
    fn fmt_fields(&self, node: &JsClassDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        FormatClass::from(&node.clone().into()).fmt(f)
    }
}
