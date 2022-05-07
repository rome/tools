use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::utils::format_with_semicolon;
use crate::{formatted, space_token, Format, FormatElement, FormatNode, Formatter};

use rome_js_syntax::JsExportFromClause;
use rome_js_syntax::JsExportFromClauseFields;

impl FormatNode for JsExportFromClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportFromClauseFields {
            star_token,
            export_as,
            from_token,
            source,
            assertion,
            semicolon_token,
        } = self.as_fields();

        let star = star_token.format(formatter)?;

        let export_as =
            export_as.with_or_empty(|as_token| formatted![formatter, as_token, space_token()]);
        let from = from_token.format(formatter)?;
        let source = source.format(formatter)?;
        let assertion =
            assertion.with_or_empty(|assertion| formatted![formatter, space_token(), assertion]);

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                star,
                space_token(),
                export_as,
                from,
                space_token(),
                source,
                assertion,
            ]?,
            semicolon_token,
        )
    }
}
