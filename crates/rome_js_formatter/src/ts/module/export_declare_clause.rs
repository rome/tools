use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsExportDeclareClause;
use rome_js_syntax::TsExportDeclareClauseFields;

impl FormatNodeFields<TsExportDeclareClause> for FormatNodeRule<TsExportDeclareClause> {
    fn fmt_fields(node: &TsExportDeclareClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsExportDeclareClauseFields {
            declare_token,
            declaration,
        } = node.as_fields();

        write![
            f,
            [declare_token.format(), space_token(), declaration.format(),]
        ]
    }
}
