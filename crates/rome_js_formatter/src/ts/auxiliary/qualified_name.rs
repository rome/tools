use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsQualifiedName;
use rome_js_syntax::TsQualifiedNameFields;

impl FormatNodeFields<TsQualifiedName> for FormatNodeRule<TsQualifiedName> {
    fn format_fields(
        node: &TsQualifiedName,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsQualifiedNameFields {
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
