use crate::{formatted, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsObjectBindingPatternRest;
use rome_js_syntax::JsObjectBindingPatternRestFields;

impl FormatNode for JsObjectBindingPatternRest {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectBindingPatternRestFields {
            dotdotdot_token,
            binding,
        } = self.as_fields();

        formatted![
            formatter,
            dotdotdot_token.format(formatter)?,
            binding.format(formatter)?,
        ]
    }
}
