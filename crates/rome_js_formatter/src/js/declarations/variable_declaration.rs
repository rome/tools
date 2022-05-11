use crate::prelude::*;

use rome_js_syntax::JsVariableDeclaration;
use rome_js_syntax::JsVariableDeclarationFields;

impl FormatNode for JsVariableDeclaration {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsVariableDeclarationFields { kind, declarators } = self.as_fields();

        formatted![
            formatter,
            kind.format(formatter)?,
            space_token(),
            declarators.format(formatter)?,
        ]
    }
}
