use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsAnyFunction;
use rome_js_syntax::JsFunctionExportDefaultDeclaration;

impl FormatNode for JsFunctionExportDefaultDeclaration {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyFunction::from(self.clone()).format(formatter)
    }
}
