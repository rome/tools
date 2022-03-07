use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsImportBareClause;
use rome_js_syntax::JsImportBareClauseFields;

impl ToFormatElement for JsImportBareClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportBareClauseFields { source, assertion } = self.as_fields();

        let source = source.format(formatter)?;
        let assertion = assertion.format_with_or_empty(formatter, |assertion| {
            format_elements![space_token(), assertion]
        })?;

        Ok(format_elements![source, assertion])
    }
}
