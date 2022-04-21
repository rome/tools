use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
use rome_js_syntax::TsExportDeclareClause;
use rome_js_syntax::TsExportDeclareClauseFields;

impl FormatNode for TsExportDeclareClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsExportDeclareClauseFields {
            declare_token,
            declaration,
        } = self.as_fields();

        Ok(format_elements![
            declare_token.format(formatter)?,
            space_token(),
            declaration.format(formatter)?,
        ])
    }
}
