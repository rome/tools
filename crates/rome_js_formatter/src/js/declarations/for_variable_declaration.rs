use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsForVariableDeclaration;
use rome_js_syntax::JsForVariableDeclarationFields;

impl FormatNodeFields<JsForVariableDeclaration> for FormatNodeRule<JsForVariableDeclaration> {
    fn format_fields(
        node: &JsForVariableDeclaration,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsForVariableDeclarationFields {
            kind_token,
            declarator,
        } = node.as_fields();

        formatted![
            formatter,
            [kind_token.format(), space_token(), declarator.format(),]
        ]
    }
}
