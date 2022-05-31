use crate::prelude::*;

use crate::utils::format_with_semicolon;

use crate::FormatNodeFields;
use rome_js_syntax::JsExportFromClause;
use rome_js_syntax::JsExportFromClauseFields;

impl FormatNodeFields<JsExportFromClause> for FormatNodeRule<JsExportFromClause> {
    fn format_fields(
        node: &JsExportFromClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsExportFromClauseFields {
            star_token,
            export_as,
            from_token,
            source,
            assertion,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                [
                    star_token.format(),
                    space_token(),
                    export_as
                        .format()
                        .with_or_empty(|as_token| formatted![formatter, [as_token, space_token()]]),
                    from_token.format(),
                    space_token(),
                    source.format(),
                    assertion.format().with_or_empty(|assertion| formatted![
                        formatter,
                        [space_token(), assertion]
                    ]),
                ]
            ]?,
            semicolon_token,
        )
    }
}
