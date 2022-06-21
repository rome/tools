use crate::prelude::*;
use crate::utils::format_class::FormatClass;
use crate::FormatNodeFields;
use rome_js_syntax::JsClassExportDefaultDeclaration;

impl FormatNodeFields<JsClassExportDefaultDeclaration>
    for FormatNodeRule<JsClassExportDefaultDeclaration>
{
    fn fmt_fields(node: &JsClassExportDefaultDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        FormatClass::from(&node.clone().into()).fmt(f)
    }
}
