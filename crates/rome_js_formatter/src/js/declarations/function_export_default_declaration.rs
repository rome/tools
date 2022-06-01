use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsAnyFunction;
use rome_js_syntax::JsFunctionExportDefaultDeclaration;

impl FormatNodeFields<JsFunctionExportDefaultDeclaration>
    for FormatNodeRule<JsFunctionExportDefaultDeclaration>
{
    fn format_fields(
        node: &JsFunctionExportDefaultDeclaration,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        formatted![formatter, [JsAnyFunction::from(node.clone()).format()]]
    }
}
