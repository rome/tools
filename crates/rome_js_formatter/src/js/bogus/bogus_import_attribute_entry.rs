use crate::FormatBogusNodeRule;
use rome_js_syntax::JsBogusImportAttributeEntry;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusImportAttributeEntry;
impl FormatBogusNodeRule<JsBogusImportAttributeEntry> for FormatJsBogusImportAttributeEntry {}
