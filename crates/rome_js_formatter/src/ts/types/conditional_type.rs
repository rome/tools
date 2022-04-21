use crate::utils::{format_conditional, Conditional};
use crate::{FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsConditionalType;

impl FormatNode for TsConditionalType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_conditional(Conditional::Type(self.clone()), formatter, false)
    }
}
