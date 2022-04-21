use crate::format_traits::FormatOptional;

use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};

use rome_js_syntax::JsYieldArgument;
use rome_js_syntax::JsYieldArgumentFields;

impl FormatNode for JsYieldArgument {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsYieldArgumentFields {
            star_token,
            expression,
        } = self.as_fields();

        let star_token = star_token.format_or_empty(formatter)?;

        Ok(format_elements![
            star_token,
            space_token(),
            expression.format(formatter)?
        ])
    }
}
