use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{JsAnyFunction, JsFunctionDeclaration};

impl FormatNodeFields<JsFunctionDeclaration> for FormatNodeRule<JsFunctionDeclaration> {
    fn format_fields(node: &JsFunctionDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyFunction::from(node.clone()).format()]]
    }
}
