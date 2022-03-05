use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_syntax::TsExportAsNamespaceClause;
use rslint_syntax::TsExportAsNamespaceClauseFields;

impl ToFormatElement for TsExportAsNamespaceClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsExportAsNamespaceClauseFields {
            as_token,
            namespace_token,
            name,
            semicolon_token,
        } = self.as_fields();

        Ok(format_elements![
            as_token.format(formatter)?,
            space_token(),
            namespace_token.format(formatter)?,
            space_token(),
            name.format(formatter)?,
            semicolon_token.format_or(formatter, || token(";"))?,
        ])
    }
}
