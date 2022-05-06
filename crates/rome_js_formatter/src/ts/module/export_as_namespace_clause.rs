use crate::utils::format_with_semicolon;
use crate::{space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsExportAsNamespaceClause;
use rome_js_syntax::TsExportAsNamespaceClauseFields;

impl FormatNode for TsExportAsNamespaceClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsExportAsNamespaceClauseFields {
            as_token,
            namespace_token,
            name,
            semicolon_token,
        } = self.as_fields();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                as_token.format(formatter)?,
                space_token(),
                namespace_token.format(formatter)?,
                space_token(),
                name.format(formatter)?,
            ]?,
            semicolon_token,
        )
    }
}
