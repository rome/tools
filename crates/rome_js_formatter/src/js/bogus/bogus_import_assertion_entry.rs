use crate::FormatBogusNodeRule;
use rome_js_syntax::JsBogusImportAssertionEntry;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusImportAssertionEntry;

impl FormatBogusNodeRule<JsBogusImportAssertionEntry> for FormatJsBogusImportAssertionEntry {}
