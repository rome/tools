use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::utils::format_with_semicolon;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsExportFromClause;
use rome_js_syntax::JsExportFromClauseFields;

impl ToFormatElement for JsExportFromClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportFromClauseFields {
            star_token,
            export_as,
            from_token,
            source,
            assertion,
            semicolon_token,
        } = self.as_fields();

        let star = star_token.format(formatter)?;

        let export_as = export_as.format_with_or_empty(formatter, |as_token| {
            format_elements![as_token, space_token()]
        })?;
        let from = from_token.format(formatter)?;
        let source = source.format(formatter)?;
        let assertion = assertion.format_with_or_empty(formatter, |assertion| {
            format_elements![space_token(), assertion]
        })?;

        format_with_semicolon(
            formatter,
            format_elements![
                star,
                space_token(),
                export_as,
                from,
                space_token(),
                source,
                assertion,
            ],
            semicolon_token,
        )
    }
}
