use crate::prelude::*;

use crate::js::bindings::parameters::FormatParameters;
use rome_js_syntax::JsConstructorParameters;

#[derive(Debug, Clone, Default)]
pub struct FormatJsConstructorParameters;

impl FormatNodeRule<JsConstructorParameters> for FormatJsConstructorParameters {
    fn fmt_fields(&self, node: &JsConstructorParameters, f: &mut JsFormatter) -> FormatResult<()> {
        FormatParameters::from(node.clone()).fmt(f)
    }
}
