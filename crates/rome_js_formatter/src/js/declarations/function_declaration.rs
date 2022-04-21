use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::{JsAnyFunction, JsFunctionDeclaration};

impl FormatNode for JsFunctionDeclaration {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyFunction::from(self.clone()).format(formatter)
    }
}
