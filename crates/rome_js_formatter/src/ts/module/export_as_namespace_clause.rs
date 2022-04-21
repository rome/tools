use crate::utils::format_with_semicolon;
use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
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
            format_elements![
                as_token.format(formatter)?,
                space_token(),
                namespace_token.format(formatter)?,
                space_token(),
                name.format(formatter)?,
            ],
            semicolon_token,
        )
    }
}
