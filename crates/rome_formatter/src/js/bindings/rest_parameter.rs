use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsRestParameter;
use rslint_parser::ast::JsRestParameterFields;

impl ToFormatElement for JsRestParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsRestParameterFields {
            dotdotdot_token,
            binding,
            type_annotation,
        } = self.as_fields();

        Ok(format_elements![
            dotdotdot_token.format(formatter)?,
            binding.format(formatter)?,
        ])
    }
}
