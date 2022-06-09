use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsAnyClass;
use rome_js_syntax::JsClassExportDefaultDeclaration;

impl FormatNodeFields<JsClassExportDefaultDeclaration>
    for FormatNodeRule<JsClassExportDefaultDeclaration>
{
    fn fmt_fields(node: &JsClassExportDefaultDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyClass::from(node.clone()).format()]]
    }
}
