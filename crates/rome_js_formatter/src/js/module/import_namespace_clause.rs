use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsImportNamespaceClause;
use rome_js_syntax::JsImportNamespaceClauseFields;

impl FormatNodeFields<JsImportNamespaceClause> for FormatNodeRule<JsImportNamespaceClause> {
    fn fmt_fields(node: &JsImportNamespaceClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportNamespaceClauseFields {
            type_token,
            star_token,
            as_token,
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
                star_token.format(),
                space_token(),
                as_token.format(),
                space_token(),
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
