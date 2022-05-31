use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsDeclareStatement;
use rome_js_syntax::TsDeclareStatementFields;

impl FormatNodeFields<TsDeclareStatement> for FormatNodeRule<TsDeclareStatement> {
    fn format_fields(
        node: &TsDeclareStatement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsDeclareStatementFields {
            declaration,
            declare_token,
        } = node.as_fields();
        formatted![
            formatter,
            [declare_token.format(), space_token(), declaration.format()]
        ]
    }
}
