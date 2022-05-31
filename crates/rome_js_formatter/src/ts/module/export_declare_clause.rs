use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsExportDeclareClause;
use rome_js_syntax::TsExportDeclareClauseFields;

impl FormatNodeFields<TsExportDeclareClause> for FormatNodeRule<TsExportDeclareClause> {
    fn format_fields(
        node: &TsExportDeclareClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsExportDeclareClauseFields {
            declare_token,
            declaration,
        } = node.as_fields();

        formatted![
            formatter,
            [declare_token.format(), space_token(), declaration.format(),]
        ]
    }
}
