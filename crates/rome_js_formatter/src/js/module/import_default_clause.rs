use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsImportDefaultClause;
use rome_js_syntax::JsImportDefaultClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsImportDefaultClause;

impl FormatNodeRule<JsImportDefaultClause> for FormatJsImportDefaultClause {
    fn fmt_fields(&self, node: &JsImportDefaultClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportDefaultClauseFields {
            type_token,
            local_name,
            from_token,
            source,
            assertion,
        } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space_token()])?;
        }

        write![
            f,
            [
                local_name.format(),
                space_token(),
                from_token.format(),
                space_token(),
                source.format(),
            ]
        ]?;

        if let Some(assertion) = assertion {
            write!(f, [space_token(), assertion.format()])?;
        }

        Ok(())
    }
}
