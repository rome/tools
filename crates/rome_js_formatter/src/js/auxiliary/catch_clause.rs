use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsCatchClause;
use rome_js_syntax::JsCatchClauseFields;

impl FormatNodeFields<JsCatchClause> for FormatNodeRule<JsCatchClause> {
    fn format_fields(node: &JsCatchClause, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let JsCatchClauseFields {
            catch_token,
            declaration,
            body,
        } = node.as_fields();

        formatted![
            formatter,
            [declaration.format().with_or(
                |declaration| {
                    formatted![
                        formatter,
                        [
                            catch_token.format(),
                            space_token(),
                            declaration,
                            space_token(),
                            body.format()
                        ]
                    ]
                },
                || {
                    formatted![
                        formatter,
                        [catch_token.format(), space_token(), body.format()]
                    ]
                },
            )]
        ]
    }
}
