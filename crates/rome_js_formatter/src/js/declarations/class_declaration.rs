use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::{JsAnyClass, JsClassDeclaration};

impl FormatNodeFields<JsClassDeclaration> for FormatNodeRule<JsClassDeclaration> {
    fn format_fields(
        node: &JsClassDeclaration,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        formatted![formatter, [JsAnyClass::from(node.clone()).format()]]
    }
}
