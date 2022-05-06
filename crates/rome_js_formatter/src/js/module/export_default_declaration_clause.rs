use crate::{
    formatted, space_token, Format, FormatElement, FormatNode, Formatter,
};
use rome_formatter::FormatResult;
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
