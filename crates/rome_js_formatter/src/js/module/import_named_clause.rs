use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsImportNamedClause;
use rome_js_syntax::JsImportNamedClauseFields;

impl FormatNodeFields<JsImportNamedClause> for FormatNodeRule<JsImportNamedClause> {
    fn fmt_fields(node: &JsImportNamedClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportNamedClauseFields {
            type_token,
            default_specifier,
            named_import,
            from_token,
            source,
            assertion,
        } = node.as_fields();

        if let Some(type_token) = type_token {
            write!(f, [type_token.format(), space_token()])?;
        }

        if let Some(default_specifier) = default_specifier {
            write!(f, [default_specifier.format(), space_token()])?;
        }

        write![
            f,
            [
                named_import.format(),
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
