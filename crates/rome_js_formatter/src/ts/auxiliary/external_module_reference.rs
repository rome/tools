use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsExternalModuleReference;
use rome_js_syntax::TsExternalModuleReferenceFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsExternalModuleReference;

impl FormatNodeRule<TsExternalModuleReference> for FormatTsExternalModuleReference {
    fn fmt_fields(node: &TsExternalModuleReference, f: &mut JsFormatter) -> FormatResult<()> {
        let TsExternalModuleReferenceFields {
            require_token,
            l_paren_token,
            source,
            r_paren_token,
        } = node.as_fields();

        write![
            f,
            [
                require_token.format(),
                l_paren_token.format(),
                source.format(),
                r_paren_token.format(),
            ]
        ]
    }
}
