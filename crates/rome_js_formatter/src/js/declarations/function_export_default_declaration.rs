use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsAnyFunction;
use rome_js_syntax::JsFunctionExportDefaultDeclaration;

#[derive(Debug, Clone, Default)]
pub struct FormatJsFunctionExportDefaultDeclaration;

impl FormatNodeRule<JsFunctionExportDefaultDeclaration>
    for FormatJsFunctionExportDefaultDeclaration
{
    fn fmt_fields(
        &self,
        node: &JsFunctionExportDefaultDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write![f, [JsAnyFunction::from(node.clone()).format()]]
    }
}
