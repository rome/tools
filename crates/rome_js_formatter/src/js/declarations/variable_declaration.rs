use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsVariableDeclaration;
use rome_js_syntax::JsVariableDeclarationFields;

impl FormatNode for JsVariableDeclaration {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsVariableDeclarationFields { kind, declarators } = self.as_fields();

        Ok(format_elements![
            kind.format(formatter)?,
            space_token(),
            declarators.format(formatter)?,
        ])
    }
}
