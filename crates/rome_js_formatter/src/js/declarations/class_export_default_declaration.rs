use crate::prelude::*;
use crate::utils::format_class::FormatClass;

use rome_js_syntax::JsClassExportDefaultDeclaration;

#[derive(Debug, Clone, Default)]
pub struct FormatJsClassExportDefaultDeclaration;

impl FormatNodeRule<JsClassExportDefaultDeclaration> for FormatJsClassExportDefaultDeclaration {
    fn fmt_fields(
        &self,
        node: &JsClassExportDefaultDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatClass::from(&node.clone().into()).fmt(f)
    }
}
