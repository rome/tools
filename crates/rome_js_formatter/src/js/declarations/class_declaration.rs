use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{JsAnyClass, JsClassDeclaration};

impl FormatNodeFields<JsClassDeclaration> for FormatNodeRule<JsClassDeclaration> {
    fn format_fields(node: &JsClassDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyClass::from(node.clone()).format()]]
    }
}
