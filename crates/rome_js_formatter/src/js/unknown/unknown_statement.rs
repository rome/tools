use crate::FormatUnknownNodeRule;
use rome_js_syntax::JsUnknownStatement;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsUnknownStatement;

impl FormatUnknownNodeRule<JsUnknownStatement> for FormatJsUnknownStatement {}
