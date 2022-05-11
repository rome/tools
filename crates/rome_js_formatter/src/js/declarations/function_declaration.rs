use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::{JsAnyFunction, JsFunctionDeclaration};

impl FormatNodeFields<JsFunctionDeclaration> for FormatNodeRule<JsFunctionDeclaration> {
    fn format_fields(
        node: &JsFunctionDeclaration,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        formatted![formatter, [JsAnyFunction::from(node.clone()).format()]]
    }
}
