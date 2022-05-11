use crate::prelude::*;

use rome_js_syntax::JsInitializerClause;
use rome_js_syntax::JsInitializerClauseFields;

impl FormatNode for JsInitializerClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsInitializerClauseFields {
            eq_token,
            expression,
        } = self.as_fields();

        formatted![
            formatter,
            hard_group_elements(eq_token.format(formatter)?),
            space_token(),
            expression.format(formatter)?
        ]
    }
}
