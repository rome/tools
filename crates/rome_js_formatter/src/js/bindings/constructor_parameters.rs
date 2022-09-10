use crate::prelude::*;

use crate::js::bindings::parameters::FormatJsAnyParameters;
use rome_js_syntax::JsConstructorParameters;

#[derive(Debug, Clone, Default)]
pub struct FormatJsConstructorParameters;

impl FormatNodeRule<JsConstructorParameters> for FormatJsConstructorParameters {
    fn fmt_fields(&self, node: &JsConstructorParameters, f: &mut JsFormatter) -> FormatResult<()> {
        FormatJsAnyParameters::from(node.clone()).fmt(f)
    }

    fn formats_dangling_comments(&self) -> bool {
        true
    }
}
