use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsGlobalDeclaration;
use rome_js_syntax::TsGlobalDeclarationFields;

impl FormatNodeFields<TsGlobalDeclaration> for FormatNodeRule<TsGlobalDeclaration> {
    fn format_fields(
        node: &TsGlobalDeclaration,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsGlobalDeclarationFields { global_token, body } = node.as_fields();

        formatted![
            formatter,
            [global_token.format(), space_token(), body.format()]
        ]
    }
}
