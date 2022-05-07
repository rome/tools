use crate::prelude::*;

use rome_js_syntax::JsYieldArgument;
use rome_js_syntax::JsYieldArgumentFields;

impl FormatNode for JsYieldArgument {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsYieldArgumentFields {
            star_token,
            expression,
        } = self.as_fields();

        formatted![
            formatter,
            star_token,
            space_token(),
            expression.format(formatter)?
        ]
    }
}
