use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsDeclareStatement;
use rome_js_syntax::TsDeclareStatementFields;

impl FormatNodeFields<TsDeclareStatement> for FormatNodeRule<TsDeclareStatement> {
    fn fmt_fields(node: &TsDeclareStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let TsDeclareStatementFields {
            declaration,
            declare_token,
        } = node.as_fields();
        write![
            f,
            [declare_token.format(), space_token(), declaration.format()]
        ]
    }
}
