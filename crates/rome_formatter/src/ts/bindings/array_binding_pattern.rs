use crate::{
    format_elements, group_elements, join_elements, soft_indent, soft_line_break_or_space, token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{JsAnyArrayBindingPatternElement, JsArrayBindingPattern};

impl ToFormatElement for JsArrayBindingPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let elements = formatter.format_separated(self.elements(), || token(","))?;

        Ok(group_elements(formatter.format_delimited_group(
            &self.l_brack_token()?,
            |leading, trailing| {
                Ok(soft_indent(format_elements![
                    leading,
                    join_elements(soft_line_break_or_space(), elements),
                    trailing
                ]))
            },
            &self.r_brack_token()?,
        )?))
    }
}

impl ToFormatElement for JsAnyArrayBindingPatternElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyArrayBindingPatternElement::JsArrayHole(hole) => hole.to_format_element(formatter),
            JsAnyArrayBindingPatternElement::JsAnyBindingPattern(binding) => {
                binding.to_format_element(formatter)
            }
            JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(with_default) => {
                with_default.to_format_element(formatter)
            }
            JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(binding) => {
                binding.to_format_element(formatter)
            }
        }
    }
}
