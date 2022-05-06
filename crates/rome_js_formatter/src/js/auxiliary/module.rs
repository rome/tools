use crate::utils::format_interpreter;
use crate::{
    format_elements, hard_line_break, Format, FormatElement, FormatNode, Formatter, JsFormatter,
};
use rome_formatter::FormatResult;
use rome_js_syntax::JsModule;
use rome_js_syntax::JsModuleFields;

impl FormatNode for JsModule {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsModuleFields {
            interpreter_token,
            directives,
            items,
            eof_token,
        } = self.as_fields();

        Ok(format_elements![
            format_interpreter(interpreter_token, formatter)?,
            directives.format(formatter)?,
            formatter.format_list(items),
            eof_token.format(formatter)?,
            hard_line_break()
        ])
    }
}
