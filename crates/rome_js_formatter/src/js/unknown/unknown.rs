use crate::FormatUnknownNodeRule;
use rome_js_syntax::JsUnknown;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsUnknown;

impl FormatUnknownNodeRule<JsUnknown> for FormatJsUnknown {}
