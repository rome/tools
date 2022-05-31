use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsQualifiedModuleName;
use rome_js_syntax::TsQualifiedModuleNameFields;

impl FormatNodeFields<TsQualifiedModuleName> for FormatNodeRule<TsQualifiedModuleName> {
    fn format_fields(
        node: &TsQualifiedModuleName,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsQualifiedModuleNameFields {
            left,
            dot_token,
            right,
        } = node.as_fields();

        formatted![
            formatter,
            [left.format(), dot_token.format(), right.format(),]
        ]
    }
}
