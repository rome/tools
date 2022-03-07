use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsExportDeclareClause;
use rome_js_syntax::TsExportDeclareClauseFields;

impl ToFormatElement for TsExportDeclareClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
