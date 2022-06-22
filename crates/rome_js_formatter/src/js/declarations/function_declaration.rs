use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{JsAnyFunction, JsFunctionDeclaration};

#[derive(Debug, Clone, Default)]
pub struct FormatJsFunctionDeclaration;

impl FormatNodeRule<JsFunctionDeclaration> for FormatJsFunctionDeclaration {
    fn fmt_fields(&self, node: &JsFunctionDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyFunction::from(node.clone()).format()]]
    }
}
