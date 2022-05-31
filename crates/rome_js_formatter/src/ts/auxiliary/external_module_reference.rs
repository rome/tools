use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsExternalModuleReference;
use rome_js_syntax::TsExternalModuleReferenceFields;

impl FormatNodeFields<TsExternalModuleReference> for FormatNodeRule<TsExternalModuleReference> {
    fn format_fields(
        node: &TsExternalModuleReference,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsExternalModuleReferenceFields {
            require_token,
            l_paren_token,
            source,
            r_paren_token,
        } = node.as_fields();

        formatted![
            formatter,
            [
                require_token.format(),
                l_paren_token.format(),
                source.format(),
                r_paren_token.format(),
            ]
        ]
    }
}
