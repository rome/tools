use crate::FormatBogusNodeRule;
use rome_js_syntax::JsBogusParameter;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusParameter;

impl FormatBogusNodeRule<JsBogusParameter> for FormatJsBogusParameter {}
