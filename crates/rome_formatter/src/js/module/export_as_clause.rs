use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsExportAsClause;
use rome_js_syntax::JsExportAsClauseFields;

impl ToFormatElement for JsExportAsClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportAsClauseFields {
            as_token,
            exported_name,
        } = self.as_fields();

        let as_token = as_token.format(formatter)?;
        let exported_name = exported_name.format(formatter)?;

        Ok(format_elements![as_token, space_token(), exported_name])
    }
}
