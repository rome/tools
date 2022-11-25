use crate::FormatUnknownNodeRule;
use rome_js_syntax::JsUnknownImportAssertionEntry;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsUnknownImportAssertionEntry;

impl FormatUnknownNodeRule<JsUnknownImportAssertionEntry> for FormatJsUnknownImportAssertionEntry {}
