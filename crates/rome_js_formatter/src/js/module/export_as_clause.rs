use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};

use rome_js_syntax::JsExportAsClause;
use rome_js_syntax::JsExportAsClauseFields;

impl FormatNode for JsExportAsClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportAsClauseFields {
            as_token,
            exported_name,
        } = self.as_fields();

        let as_token = as_token.format(formatter)?;
        let exported_name = exported_name.format(formatter)?;

        Ok(format_elements![as_token, space_token(), exported_name])
    }
}
