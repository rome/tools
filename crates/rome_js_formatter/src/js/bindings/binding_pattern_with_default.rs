use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsBindingPatternWithDefault;
use rome_js_syntax::JsBindingPatternWithDefaultFields;

impl FormatNode for JsBindingPatternWithDefault {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBindingPatternWithDefaultFields {
            pattern,
            eq_token,
            default,
        } = self.as_fields();

        Ok(format_elements![
            pattern.format(formatter)?,
            space_token(),
            eq_token.format(formatter)?,
            space_token(),
            default.format(formatter)?
        ])
    }
}
