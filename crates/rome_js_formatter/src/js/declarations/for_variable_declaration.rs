use crate::prelude::*;

use rome_js_syntax::JsForVariableDeclaration;
use rome_js_syntax::JsForVariableDeclarationFields;

impl FormatNode for JsForVariableDeclaration {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsForVariableDeclarationFields {
            kind_token,
            declarator,
        } = self.as_fields();

        formatted![
            formatter,
            kind_token.format(formatter)?,
            space_token(),
            declarator.format(formatter)?,
        ]
    }
}
