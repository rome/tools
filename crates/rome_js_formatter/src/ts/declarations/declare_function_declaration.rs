use crate::prelude::*;
use crate::utils::FormatWithSemicolon;

use crate::js::declarations::function_declaration::FormatFunction;
use rome_formatter::write;
use rome_js_syntax::TsDeclareFunctionDeclaration;

#[derive(Debug, Clone, Default)]
pub struct FormatTsDeclareFunctionDeclaration;

impl FormatNodeRule<TsDeclareFunctionDeclaration> for FormatTsDeclareFunctionDeclaration {
    fn fmt_fields(
        &self,
        node: &TsDeclareFunctionDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write!(
            f,
            [FormatWithSemicolon::new(
                &FormatFunction::from(node.clone()),
                node.semicolon_token().as_ref()
            )]
        )
    }
}
