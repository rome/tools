use rslint_parser::ast::{JsAnyObjectMemberName, JsStaticObjectMemberName};

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

impl ToFormatElement for JsAnyObjectMemberName {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyObjectMemberName::JsComputedObjectMemberName(_) => todo!(),
			JsAnyObjectMemberName::JsStringLiteral(literal) => literal.to_format_element(formatter),
			JsAnyObjectMemberName::JsNumberLiteral(literal) => literal.to_format_element(formatter),
			JsAnyObjectMemberName::JsStaticObjectMemberName(ident) => {
				ident.to_format_element(formatter)
			}
		}
	}
}

impl ToFormatElement for JsStaticObjectMemberName {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		formatter.format_token(&self.name_token()?)
	}
}
