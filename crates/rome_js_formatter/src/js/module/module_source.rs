use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use rome_formatter::write;
use rome_js_syntax::JsModuleSource;
use rome_js_syntax::JsModuleSourceFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsModuleSource;

impl FormatNodeRule<JsModuleSource> for FormatJsModuleSource {
    fn fmt_fields(&self, node: &JsModuleSource, f: &mut JsFormatter) -> FormatResult<()> {
        let JsModuleSourceFields { value_token } = node.as_fields();

        write!(
            f,
            [FormatLiteralStringToken::new(
                &value_token?,
                StringLiteralParentKind::Expression
            )]
        )
    }
}
