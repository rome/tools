use crate::{space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsExportDeclareClause;
use rome_js_syntax::TsExportDeclareClauseFields;

impl FormatNode for TsExportDeclareClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsExportDeclareClauseFields {
            declare_token,
            declaration,
        } = self.as_fields();

        formatted![
            formatter,
            declare_token.format(formatter)?,
            space_token(),
            declaration.format(formatter)?,
        ]
    }
}
