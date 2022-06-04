use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsAnyFunction;
use rome_js_syntax::JsFunctionExportDefaultDeclaration;

impl FormatNodeFields<JsFunctionExportDefaultDeclaration>
    for FormatNodeRule<JsFunctionExportDefaultDeclaration>
{
    fn fmt_fields(
        node: &JsFunctionExportDefaultDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write![f, [JsAnyFunction::from(node.clone()).format()]]
    }
}
