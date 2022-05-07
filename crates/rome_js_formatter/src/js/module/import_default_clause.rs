use crate::prelude::*;

use rome_js_syntax::JsImportDefaultClause;
use rome_js_syntax::JsImportDefaultClauseFields;

impl FormatNode for JsImportDefaultClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportDefaultClauseFields {
            type_token,
            local_name,
            from_token,
            source,
            assertion,
        } = self.as_fields();

        let type_token =
            type_token.with_or_empty(|token| formatted![formatter, token, space_token()]);
        let local_name = local_name.format(formatter)?;
        let from = from_token.format(formatter)?;
        let source = source.format(formatter)?;
        let assertion =
            assertion.with_or_empty(|assertion| formatted![formatter, space_token(), assertion]);

        formatted![
            formatter,
            type_token,
            local_name,
            space_token(),
            from,
            space_token(),
            source,
            assertion
        ]
    }
}
