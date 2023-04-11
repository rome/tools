use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsImportBareClause;
use rome_js_syntax::JsImportBareClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportBareClause;

impl FormatNodeRule<JsImportBareClause> for FormatJsImportBareClause {
    fn fmt_fields(&self, node: &JsImportBareClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportBareClauseFields { source, attribute } = node.as_fields();

        write!(f, [source.format()])?;

        if let Some(attribute) = attribute {
            write!(f, [space(), attribute.format()])?;
        }

        Ok(())
    }
}
