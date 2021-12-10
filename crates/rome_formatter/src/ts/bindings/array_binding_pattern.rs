use crate::{
	format_elements, group_elements, join_elements, space_token, FormatElement, FormatResult,
	Formatter, ToFormatElement,
};
use rslint_parser::ast::{JsAnyArrayBindingPatternElement, JsArrayBindingPattern};

impl ToFormatElement for JsArrayBindingPattern {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let l_bracket = formatter.format_token(&self.l_brack_token()?)?;
		let elements = formatter.format_separated(self.elements())?;
		let r_bracket = formatter.format_token(&self.r_brack_token()?)?;

		Ok(format_elements![group_elements(format_elements![
			l_bracket,
			join_elements(space_token(), elements),
			r_bracket
		])])
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
