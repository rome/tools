use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{formatted, space_token, Format, FormatElement, FormatNode, Formatter};

use rome_js_syntax::JsImportNamedClause;
use rome_js_syntax::JsImportNamedClauseFields;

impl FormatNode for JsImportNamedClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportNamedClauseFields {
            type_token,
            default_specifier,
            named_import,
            from_token,
            source,
            assertion,
        } = self.as_fields();

        let type_token =
            type_token.with_or_empty(|token| formatted![formatter, token, space_token()]);

        let source = source.format(formatter)?;

        let default = default_specifier
            .with_or_empty(|specifier| formatted![formatter, specifier, space_token()]);
        let from = from_token.format(formatter)?;
        let name = named_import.format(formatter)?;
        let assertion =
            assertion.with_or_empty(|assertion| formatted![formatter, space_token(), assertion]);
        formatted![
            formatter,
            type_token,
            default,
            name,
            space_token(),
            from,
            space_token(),
            source,
            assertion
        ]
    }
}
