use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsVariableDeclaration;
use rome_js_syntax::JsVariableDeclarationFields;

impl FormatNodeFields<JsVariableDeclaration> for FormatNodeRule<JsVariableDeclaration> {
    fn format_fields(
        node: &JsVariableDeclaration,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsVariableDeclarationFields { kind, declarators } = node.as_fields();

        formatted![
            formatter,
            [kind.format(), space_token(), declarators.format()]
        ]
    }
}
