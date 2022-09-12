

use crate::FormatUnknownNodeRule;
use rome_js_syntax::JsUnknownBinding;


#[derive(Debug, Clone, Default)]
pub struct FormatJsUnknownBinding;

impl FormatUnknownNodeRule<JsUnknownBinding> for FormatJsUnknownBinding {}
