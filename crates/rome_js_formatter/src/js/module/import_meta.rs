use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_formatter::write;
use rome_js_syntax::ImportMetaFields;
use rome_js_syntax::{ImportMeta, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatImportMeta;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportMeta;

impl FormatNodeRule<ImportMeta> for FormatImportMeta {
    fn fmt_fields(&self, node: &ImportMeta, f: &mut JsFormatter) -> FormatResult<()> {
        let ImportMetaFields {
            import_token,
            dot_token,
            meta_token,
        } = node.as_fields();

        write![
            f,
            [
                import_token.format(),
                dot_token.format(),
                meta_token.format(),
            ]
        ]
    }

    fn needs_parentheses(&self, item: &ImportMeta) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for ImportMeta {
    fn needs_parentheses(&self) -> bool {
        false
    }

    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
