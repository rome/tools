use crate::utils::{format_conditional, Conditional};
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsConditionalType;

impl ToFormatElement for TsConditionalType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_conditional(Conditional::Type(self.clone()), formatter, false)
    }
}
