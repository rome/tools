use crate::{
	format_elements, group_elements, join_elements, space_token, FormatElement, FormatResult,
	Formatter, ToFormatElement,
};
use rslint_parser::ast::{JsAnyArrayElementBinding, JsArrayBinding};

impl ToFormatElement for JsArrayBinding {
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

impl ToFormatElement for JsAnyArrayElementBinding {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyArrayElementBinding::JsArrayHole(hole) => hole.to_format_element(formatter),
			JsAnyArrayElementBinding::JsAnyBinding(binding) => binding.to_format_element(formatter),
			JsAnyArrayElementBinding::JsBindingWithDefault(with_default) => {
				with_default.to_format_element(formatter)
			}
			JsAnyArrayElementBinding::JsArrayRestBinding(_) => todo!(),
		}
	}
}
