use crate::prelude::*;
use rome_js_syntax::JsExportDefaultDeclarationClause;

impl FormatNode for JsExportDefaultDeclarationClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatted![
            formatter,
            self.default_token().format(formatter)?,
            space_token(),
            self.declaration().format(formatter)?
        ]
    }
}
