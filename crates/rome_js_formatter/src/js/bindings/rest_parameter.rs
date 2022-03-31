use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsRestParameter;
use rome_js_syntax::JsRestParameterFields;

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
            type_annotation.format_or_empty(formatter)?
        ])
    }
}
