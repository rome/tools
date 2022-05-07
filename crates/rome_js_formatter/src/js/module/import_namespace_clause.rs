use crate::prelude::*;

use rome_js_syntax::JsImportNamespaceClause;
use rome_js_syntax::JsImportNamespaceClauseFields;

impl FormatNode for JsImportNamespaceClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportNamespaceClauseFields {
            type_token,
            star_token,
            as_token,
            local_name,
            from_token,
            source,
            assertion,
        } = self.as_fields();

        let type_token =
            type_token.with_or_empty(|token| formatted![formatter, token, space_token()]);

        let star = star_token.format(formatter)?;
        let as_token = as_token.format(formatter)?;
        let local_name = local_name.format(formatter)?;
        let source = source.format(formatter)?;
        let from = from_token.format(formatter)?;
        let assertion =
            assertion.with_or_empty(|assertion| formatted![formatter, space_token(), assertion]);
        formatted![
            formatter,
            type_token,
            star,
            space_token(),
            as_token,
            space_token(),
            local_name,
            space_token(),
            from,
            space_token(),
            source,
            assertion
        ]
    }
}
