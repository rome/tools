use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsImportBareClause;
use rome_js_syntax::JsImportBareClauseFields;

impl FormatNodeFields<JsImportBareClause> for FormatNodeRule<JsImportBareClause> {
    fn fmt_fields(node: &JsImportBareClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportBareClauseFields { source, assertion } = node.as_fields();

        write!(f, [source.format()])?;

        if let Some(assertion) = assertion {
            write!(f, [space_token(), assertion.format()])?;
        }

        Ok(())
    }
}
