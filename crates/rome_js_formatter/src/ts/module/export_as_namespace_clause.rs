use crate::prelude::*;
use crate::utils::format_with_semicolon;
use crate::FormatNodeFields;
use rome_js_syntax::TsExportAsNamespaceClause;
use rome_js_syntax::TsExportAsNamespaceClauseFields;

impl FormatNodeFields<TsExportAsNamespaceClause> for FormatNodeRule<TsExportAsNamespaceClause> {
    fn format_fields(
        node: &TsExportAsNamespaceClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsExportAsNamespaceClauseFields {
            as_token,
            namespace_token,
            name,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                [
                    as_token.format(),
                    space_token(),
                    namespace_token.format(),
                    space_token(),
                    name.format(),
                ]
            ]?,
            semicolon_token,
        )
    }
}
