use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsAnyClass;
use rome_js_syntax::JsClassExportDefaultDeclaration;

impl FormatNodeFields<JsClassExportDefaultDeclaration>
    for FormatNodeRule<JsClassExportDefaultDeclaration>
{
    fn format_fields(
        node: &JsClassExportDefaultDeclaration,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        formatted![formatter, [JsAnyClass::from(node.clone()).format()]]
    }
}
