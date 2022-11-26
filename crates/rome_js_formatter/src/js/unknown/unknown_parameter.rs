use crate::FormatUnknownNodeRule;
use rome_js_syntax::JsUnknownParameter;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsUnknownParameter;

impl FormatUnknownNodeRule<JsUnknownParameter> for FormatJsUnknownParameter {}
