use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{JsAnyClass, JsClassDeclaration};

impl FormatNodeFields<JsClassDeclaration> for FormatNodeRule<JsClassDeclaration> {
    fn fmt_fields(node: &JsClassDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyClass::from(node.clone()).format()]]
    }
}
